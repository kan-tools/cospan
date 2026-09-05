//! The mobile read-API server (mobile Phase 1): the watch-and-fold spine run
//! headless and exposed over a localhost HTTP/WebSocket API.
//!
//! Endpoints:
//!   * `GET /`                — a self-contained mobile-first HTML view of the
//!     fold (fetches `/fold`, live-updates over `/stream`). One embedded page, no
//!     separate codebase or build step — the disposable way to give the read API
//!     a human UX ahead of the full Phase-2 web client.
//!   * `GET /fold`            — the current `substrate::Fold` as a JSON snapshot.
//!   * `WS  /stream`          — the serialized `Fold` pushed on every refold; the
//!     current snapshot arrives on connect.
//!   * `GET /comments`        — index of files that have comments
//!     (`mcp::comment_files`); with `?file=`, that file's comments
//!     (`mcp::list_comments`).
//!   * `GET /thread?file=&id=`— one comment + thread (`mcp::get_thread`).
//!   * `GET /chat`            — index of agent chat sessions (`chat::chat_index`);
//!     with `?session=`, that session's turns (`chat::chat_session`). No local
//!     paths are ever returned.
//!   * `GET /capabilities`    — `{writes, author}`, so the page knows whether to
//!     show write UI.
//!   * With `--allow-writes`, `POST /comments` (add), `POST /thread` (reply), and
//!     `POST /resolve` write comments to the sidecar — attributed `who:"human"`.
//!
//! The reads are a projection of the kan/day log; the writes touch only cospan's
//! own sidecar state (`telos/kan-is-truth`'s stated exception) — **no kan claim
//! is written** and the write seam `command_bus::WriteChannel` stays untouched
//! (`telos/observe-now-control-later`: comment writes are the mildest control
//! step, opt-in via `--allow-writes`). It stays disposable (`telos/disposable`):
//! a foreground process bound to 127.0.0.1, no daemon, no on-disk state beyond
//! the owned sidecar tree.
//!
//! No second async runtime is introduced — `run` builds one `tokio` runtime and
//! `block_on`s the server, exactly as `mcp::run` does, so `fn main` stays sync.

use crate::comments::Author;
use crate::substrate::{self, Fold};
use crate::{chat, filetree, mcp, tui};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, Request, State};
use axum::http::{header::AUTHORIZATION, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Json, Response};
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

/// The default loopback port `cospan serve` binds when `--port` is not given.
pub const DEFAULT_PORT: u16 = 8787;

/// How many refolds a slow `/stream` client may fall behind before the broadcast
/// drops the oldest (a lagged client resyncs from the next push — it never blocks
/// the fold loop). One fold per ~250ms tick, so 32 buffers ~8s of updates.
const STREAM_BUFFER: usize = 32;

/// The fold loop's poll interval — the same ~250ms tick the TUI uses.
const TICK: Duration = Duration::from_millis(250);

/// Default cap on concurrent `/stream` clients (`--max-stream`). Generous for a
/// personal tool; an upgrade past it is cleanly rejected, never queued.
pub const DEFAULT_MAX_STREAM: usize = 64;

/// The auth posture of a running server. `None` (from `--no-auth`) leaves every
/// route open; `Token` gates every route — including the `/stream` upgrade — on a
/// bearer token presented as `?token=` or `Authorization: Bearer`. Held in memory
/// only; never written to disk (`telos/disposable`).
#[derive(Clone)]
pub enum Auth {
    None,
    Token(Arc<str>),
}

/// Mint a fresh URL-safe token from OS randomness (32 bytes → 64 hex chars),
/// held only in memory. Falls back to a time-free fixed-length placeholder only
/// if the OS RNG fails, which on a supported platform it does not.
pub fn mint_token() -> String {
    let mut bytes = [0u8; 32];
    match getrandom::fill(&mut bytes) {
        Ok(()) => bytes.iter().map(|b| format!("{b:02x}")).collect(),
        // getrandom failing is effectively impossible on a supported OS; if it
        // ever does, refuse to mint a guessable token — the caller prints the
        // error path via an empty string it must handle.
        Err(_) => String::new(),
    }
}

/// Constant-time byte equality — no early return on the first differing byte, so
/// the auth check leaks no timing signal about how much of the token matched.
/// (The length comparison itself is not secret: the token length is fixed.)
fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b) {
        diff |= x ^ y;
    }
    diff == 0
}

/// The shared state every handler and the fold loop read: the repo root, the
/// latest folded `Fold`, the broadcast sender that fans a fresh serialized fold
/// out to `/stream` subscribers, the auth posture, and the live-`/stream` counter
/// + its cap. Cheap to `clone` — everything is `Arc` or `Copy`.
#[derive(Clone)]
pub struct Shared {
    repo: Arc<PathBuf>,
    /// The most recent `Fold`. `RwLock` because every request reads it and only
    /// the single fold loop writes it. Held as the `Fold` (not a pre-serialized
    /// string) so `/fold` and a fresh `/stream` connect both serialize the one
    /// source of truth (REQ-2's derives), no shape maintained twice.
    latest: Arc<RwLock<Fold>>,
    /// A serialized `Fold` is broadcast here on every refold. `Arc<str>` so the
    /// JSON is serialized once per refold and shared across all subscribers.
    tx: broadcast::Sender<Arc<str>>,
    /// The auth posture; the auth middleware reads it per request.
    auth: Auth,
    /// The cap on concurrent `/stream` clients, and the live count (shared across
    /// clones so every connection sees the same tally).
    max_stream: usize,
    live_streams: Arc<AtomicUsize>,
    /// Whether comment-write POST routes are mounted (`--allow-writes`). Off by
    /// default — `serve` is read-only unless control is explicitly enabled.
    allow_writes: bool,
    /// The author stamped on a web-written comment/reply (`who:"human"`, id from
    /// `--author`/`COSPAN_WEB_AUTHOR`, default `"web"`).
    web_author: Author,
    /// Serializes comment writes so two concurrent POSTs can't lose an update —
    /// the same load-modify-save guard `CommentServer` uses over MCP.
    writes: Arc<Mutex<()>>,
}

impl Shared {
    /// Build shared state seeded with an initial fold, so `/fold` answers before
    /// the loop's first tick. Auth defaults to open and the stream cap to
    /// `DEFAULT_MAX_STREAM`; `with_auth`/`with_max_stream` set them.
    pub fn seed(repo: PathBuf, initial: Fold) -> Self {
        let (tx, _rx) = broadcast::channel(STREAM_BUFFER);
        Self {
            repo: Arc::new(repo),
            latest: Arc::new(RwLock::new(initial)),
            tx,
            auth: Auth::None,
            max_stream: DEFAULT_MAX_STREAM,
            live_streams: Arc::new(AtomicUsize::new(0)),
            allow_writes: false,
            web_author: Author {
                who: "human".into(),
                id: "web".into(),
            },
            writes: Arc::new(Mutex::new(())),
        }
    }

    /// Set the auth posture (builder-style; keeps `seed`'s call sites — the tests
    /// and smokes — auth-free).
    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    /// Enable comment writes (`--allow-writes`), stamping web writes with `author`
    /// (`who:"human"`, the given id). Default posture is read-only, so `seed`'s
    /// existing call sites stay writeless.
    pub fn with_writes(mut self, author_id: String) -> Self {
        self.allow_writes = true;
        self.web_author = Author {
            who: "human".into(),
            id: if author_id.is_empty() {
                "web".into()
            } else {
                author_id
            },
        };
        self
    }

    /// Set the concurrent-`/stream` cap.
    pub fn with_max_stream(mut self, max: usize) -> Self {
        self.max_stream = max.max(1);
        self
    }

    /// The current fold serialized to JSON. A poisoned lock is recovered rather
    /// than propagated — a read never needs a consistent *writer*, just the last
    /// whole value written (the writer swaps a fully-built `Fold` in one store).
    fn snapshot_json(&self) -> String {
        let f = self.latest.read().unwrap_or_else(|e| e.into_inner());
        serde_json::to_string(&*f).unwrap_or_else(|_| "{}".to_string())
    }
}

/// An acquired `/stream` slot: increments the live count on `acquire` (rejecting
/// past the cap) and decrements on `Drop`, so a normal close, an error, or a
/// panic all release it. Held for the life of the WS task.
struct StreamSlot(Arc<AtomicUsize>);

impl StreamSlot {
    /// Take a slot if the live count is under `max`, else `None`. The optimistic
    /// `fetch_add` may transiently overshoot under a race but self-corrects (the
    /// loser decrements immediately), so the live count never *settles* above the
    /// cap.
    fn acquire(counter: &Arc<AtomicUsize>, max: usize) -> Option<Self> {
        let prev = counter.fetch_add(1, Ordering::SeqCst);
        if prev >= max {
            counter.fetch_sub(1, Ordering::SeqCst);
            None
        } else {
            Some(StreamSlot(counter.clone()))
        }
    }
}

impl Drop for StreamSlot {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Every token a request presents — the `Authorization: Bearer <t>` header and
/// the `token` query parameter, in that order. Both are collected (not
/// header-first-wins) so a stale/wrong header cannot mask a correct query token:
/// the caller accepts the request if *any* presented token matches.
fn presented_tokens(req: &Request) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(v) = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        if let Some(tok) = v.strip_prefix("Bearer ") {
            out.push(tok.trim().to_string());
        }
    }
    if let Some(q) = req.uri().query() {
        if let Some(t) = q
            .split('&')
            .find_map(|kv| kv.strip_prefix("token=").map(|t| t.replace('+', " ")))
        {
            out.push(t);
        }
    }
    out
}

/// Auth middleware: with `Auth::Token`, every route (including the `/stream`
/// upgrade, whose request passes through here before `on_upgrade`) must present
/// the matching token, compared constant-time; otherwise `401`. With `Auth::None`
/// it is a pass-through.
async fn require_auth(State(shared): State<Shared>, req: Request, next: Next) -> Response {
    match &shared.auth {
        Auth::None => next.run(req).await,
        Auth::Token(tok) => {
            // An empty configured token authenticates nothing — defends the
            // (OS-RNG-failure) branch where a token could be blank: never fail
            // open on an empty secret, regardless of what a request presents.
            let ok = !tok.is_empty()
                && presented_tokens(&req)
                    .iter()
                    .any(|p| ct_eq(p.as_bytes(), tok.as_bytes()));
            if ok {
                next.run(req).await
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    "401 unauthorized: append ?token=<token> (printed at startup) or send Authorization: Bearer <token>\n",
                )
                    .into_response()
            }
        }
    }
}

/// One tick of the headless fold loop: if `HEAD`'s mtime moved since `last`,
/// re-fold, swap the shared latest-`Fold`, and broadcast the serialized fold to
/// `/stream` subscribers. Returns the mtime to carry into the next tick.
///
/// This is the exact refold gate the TUI's `run` loop uses (`tui::head_mtime` +
/// `tui::should_refold` around `substrate::fold`), lifted out of the ratatui
/// event loop — nothing here touches a terminal. `substrate::fold` shells out to
/// `kan`/`day`, so callers run it off the async workers (`spawn_blocking`).
fn fold_tick(shared: &Shared, last: Option<SystemTime>) -> Option<SystemTime> {
    let now = tui::head_mtime(&shared.repo);
    if tui::should_refold(last, now) {
        let fold = substrate::fold(&shared.repo);
        let json: Arc<str> = Arc::from(
            serde_json::to_string(&fold)
                .unwrap_or_else(|_| "{}".to_string())
                .as_str(),
        );
        {
            let mut w = shared.latest.write().unwrap_or_else(|e| e.into_inner());
            *w = fold;
        }
        // Err just means no subscribers are connected — the snapshot is already
        // stored for the next `/fold` or connect, so the refold is not lost.
        let _ = shared.tx.send(json);
    }
    now
}

/// The headless fold loop: poll `HEAD`'s mtime every `TICK` and refold on change,
/// forever. Each fold runs under `spawn_blocking` (it shells `kan`/`day`), so the
/// async runtime's workers stay free for the HTTP/WS handlers.
async fn fold_loop(shared: Shared) {
    // `None` forces a refold on the first tick even though `seed` already folded,
    // so the loop's mtime baseline is established from a real read.
    let mut last: Option<SystemTime> = None;
    loop {
        let s = shared.clone();
        last = tokio::task::spawn_blocking(move || fold_tick(&s, last))
            .await
            .unwrap_or(last);
        tokio::time::sleep(TICK).await;
    }
}

/// The single-page human view, embedded in the binary at compile time so `serve`
/// stays one self-contained, disposable process — no asset directory to ship, no
/// build step. Served at `GET /`. It is a pure client of `/fold` + `/stream`;
/// all CSS/JS is inline, so it loads under any CSP and behind a plain proxy
/// (e.g. `tailscale serve`) with no external requests.
const INDEX_HTML: &str = include_str!("web/index.html");

// --- Handlers ---------------------------------------------------------------

/// `GET /` — the embedded mobile-first HTML view of the fold.
async fn get_index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// `GET /fold` — the current serialized `Fold`.
async fn get_fold(State(shared): State<Shared>) -> Response {
    (
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        shared.snapshot_json(),
    )
        .into_response()
}

/// `WS /stream` — upgrade, then push the serialized `Fold` on connect and on
/// every refold. Rejected with `503` when the concurrent-connection cap
/// (`--max-stream`) is already reached; the acquired slot rides into the task and
/// releases on disconnect.
async fn get_stream(ws: WebSocketUpgrade, State(shared): State<Shared>) -> Response {
    match StreamSlot::acquire(&shared.live_streams, shared.max_stream) {
        Some(slot) => ws.on_upgrade(move |socket| stream_folds(socket, shared, slot)),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            "503 too many /stream clients (see --max-stream)\n",
        )
            .into_response(),
    }
}

/// Send the current fold immediately, then forward each broadcast refold until
/// the client disconnects. Client→server frames are ignored (the API is
/// read-only) except a Close, which ends the loop. `_slot` is held for the whole
/// task so the connection count is released exactly when the task ends.
async fn stream_folds(mut socket: WebSocket, shared: Shared, _slot: StreamSlot) {
    let mut rx = shared.tx.subscribe();
    // Snapshot on connect, so a client that connects between refolds still gets
    // the current state without waiting for the next log change.
    if socket
        .send(Message::Text(shared.snapshot_json().into()))
        .await
        .is_err()
    {
        return;
    }
    loop {
        tokio::select! {
            pushed = rx.recv() => match pushed {
                Ok(json) => {
                    if socket.send(Message::Text(json.as_ref().into())).await.is_err() {
                        break;
                    }
                }
                // Lagged: this client fell behind the buffer. Skip the dropped
                // folds and resume — the next push carries the latest state.
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            },
            incoming = socket.recv() => match incoming {
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(_)) => {} // read-only: ignore any client message
                Some(Err(_)) => break,
            },
        }
    }
}

#[derive(Deserialize)]
struct CommentsQuery {
    file: Option<String>,
}

/// `GET /comments` — the comment read collection. With no `file`, returns the
/// index of files that have comments (`mcp::comment_files`); with `?file=<rel>`,
/// that file's comments (`mcp::list_comments`). The blocking sidecar I/O runs
/// under `spawn_blocking`; the single-file path is path-guarded, so a
/// `file=../x` returns the guard error, never a read outside the repo.
async fn get_comments(
    State(shared): State<Shared>,
    Query(q): Query<CommentsQuery>,
) -> Json<serde_json::Value> {
    let repo = shared.repo.clone();
    let v = tokio::task::spawn_blocking(move || match q.file {
        Some(file) => mcp::list_comments(&repo, &file),
        None => mcp::comment_files(&repo),
    })
    .await
    .unwrap_or_else(|e| serde_json::json!({ "error": format!("task join failed: {e}") }));
    Json(v)
}

#[derive(Deserialize)]
struct ThreadQuery {
    file: String,
    id: String,
}

/// `GET /thread?file=<rel>&id=<id>` — one comment with its full reply thread.
async fn get_thread(
    State(shared): State<Shared>,
    Query(q): Query<ThreadQuery>,
) -> Json<serde_json::Value> {
    let repo = shared.repo.clone();
    let v = tokio::task::spawn_blocking(move || mcp::get_thread(&repo, &q.file, &q.id))
        .await
        .unwrap_or_else(|e| serde_json::json!({ "error": format!("task join failed: {e}") }));
    Json(v)
}

#[derive(Deserialize)]
struct ChatQuery {
    session: Option<String>,
}

/// `GET /chat` — the index of the repo's chat sessions; with `?session=<id>`,
/// that session's turns. A read of the transcript stores under `spawn_blocking`
/// (discovery reads files / a SQLite DB); no local path is ever returned.
async fn get_chat(
    State(shared): State<Shared>,
    Query(q): Query<ChatQuery>,
) -> Json<serde_json::Value> {
    let repo = shared.repo.clone();
    let v = tokio::task::spawn_blocking(move || match q.session {
        Some(id) => chat::chat_session(&repo, &id),
        None => chat::chat_index(&repo),
    })
    .await
    .unwrap_or_else(|e| serde_json::json!({ "error": format!("task join failed: {e}") }));
    Json(v)
}

/// `GET /files` — the browsable file list: the same tracked ∪ untracked-not-ignored
/// set the TUI browses (`filetree::list`), each `{path, status}` with `status` the
/// one-char git marker. A read (no `--allow-writes` needed) and the source for the
/// phone's file picker, so a file with **no** comments can still be reached to
/// receive its first one.
async fn get_files(State(shared): State<Shared>) -> Json<serde_json::Value> {
    let repo = shared.repo.clone();
    let v = tokio::task::spawn_blocking(move || {
        let files: Vec<serde_json::Value> = filetree::list(&repo)
            .into_iter()
            .map(|e| {
                serde_json::json!({
                    "path": e.path.to_string_lossy(),
                    "status": filetree::marker(e.status).to_string(),
                })
            })
            .collect();
        serde_json::json!({ "files": files })
    })
    .await
    .unwrap_or_else(|e| serde_json::json!({ "error": format!("task join failed: {e}") }));
    Json(v)
}

#[derive(Deserialize)]
struct FileQuery {
    path: String,
}

/// `GET /file?path=<rel>` — one file's syntax-highlighted content for the viewer
/// (`mcp::file_view`): `{path, lines, truncated, total}`, capped at
/// `mcp::FILE_VIEW_MAX_LINES`. Path-guarded under `spawn_blocking`, so
/// `path=../x` returns the guard error and a non-file path returns `{error}`.
async fn get_file(
    State(shared): State<Shared>,
    Query(q): Query<FileQuery>,
) -> Json<serde_json::Value> {
    let repo = shared.repo.clone();
    let v = tokio::task::spawn_blocking(move || mcp::file_view(&repo, &q.path))
        .await
        .unwrap_or_else(|e| serde_json::json!({ "error": format!("task join failed: {e}") }));
    Json(v)
}

/// `GET /capabilities` — what this server allows, so the page knows whether to
/// show write UI without probing a `405`. `{writes, author}` when writes are on,
/// `{writes:false}` when off.
async fn get_capabilities(State(shared): State<Shared>) -> Json<serde_json::Value> {
    if shared.allow_writes {
        Json(serde_json::json!({
            "writes": true,
            "author": { "who": shared.web_author.who, "id": shared.web_author.id },
        }))
    } else {
        Json(serde_json::json!({ "writes": false }))
    }
}

#[derive(Deserialize)]
struct AddBody {
    line: usize,
    body: String,
}

/// `POST /comments?file=<rel>` `{line, body}` — add a web-authored comment. The
/// write lock serializes the load-modify-save; the core is path-guarded.
async fn post_comment(
    State(shared): State<Shared>,
    Query(q): Query<CommentsQuery>,
    Json(b): Json<AddBody>,
) -> Json<serde_json::Value> {
    let Some(file) = q.file else {
        return Json(serde_json::json!({ "error": "missing ?file=" }));
    };
    write_blocking(shared.clone(), move |s| {
        mcp::add_comment_as(&s.repo, &file, b.line, &b.body, s.web_author.clone())
    })
    .await
}

#[derive(Deserialize)]
struct ReplyBody {
    body: String,
}

/// `POST /thread?file=<rel>&id=<id>` `{body}` — add a web-authored reply.
async fn post_reply(
    State(shared): State<Shared>,
    Query(q): Query<ThreadQuery>,
    Json(b): Json<ReplyBody>,
) -> Json<serde_json::Value> {
    write_blocking(shared.clone(), move |s| {
        mcp::reply_as(&s.repo, &q.file, &q.id, &b.body, s.web_author.clone())
    })
    .await
}

#[derive(Deserialize)]
struct ResolveBody {
    value: Option<bool>,
}

/// `POST /resolve?file=<rel>&id=<id>` `{value}` — set (default true) or clear a
/// comment's resolved flag.
async fn post_resolve(
    State(shared): State<Shared>,
    Query(q): Query<ThreadQuery>,
    Json(b): Json<ResolveBody>,
) -> Json<serde_json::Value> {
    let value = b.value.unwrap_or(true);
    write_blocking(shared.clone(), move |s| {
        mcp::resolve(&s.repo, &q.file, &q.id, value)
    })
    .await
}

/// Run a comment-write core under `spawn_blocking`, holding the write lock across
/// the load-modify-save so concurrent writes cannot lose an update. The lock is a
/// std `Mutex` taken and released inside the blocking closure — never across an
/// `.await`.
async fn write_blocking<F>(shared: Shared, f: F) -> Json<serde_json::Value>
where
    F: FnOnce(&Shared) -> serde_json::Value + Send + 'static,
{
    let v = tokio::task::spawn_blocking(move || {
        let _w = shared.writes.lock().unwrap_or_else(|e| e.into_inner());
        f(&shared)
    })
    .await
    .unwrap_or_else(|e| serde_json::json!({ "error": format!("task join failed: {e}") }));
    Json(v)
}

/// The router over the shared state — the page, the read endpoints, and
/// `/capabilities`, plus the comment-write POST routes when `--allow-writes` is
/// set; all wrapped in the auth middleware (a pass-through under `Auth::None`).
/// Split out so a test can drive the exact app the server serves.
pub fn app(shared: Shared) -> Router {
    // Read endpoints. When writes are enabled, the same `/comments` and `/thread`
    // paths also accept POST (add / reply), plus a `/resolve` POST.
    let (comments, thread) = if shared.allow_writes {
        (
            get(get_comments).post(post_comment),
            get(get_thread).post(post_reply),
        )
    } else {
        (get(get_comments), get(get_thread))
    };
    let mut router = Router::new()
        .route("/", get(get_index))
        .route("/fold", get(get_fold))
        .route("/stream", get(get_stream))
        .route("/comments", comments)
        .route("/thread", thread)
        .route("/chat", get(get_chat))
        .route("/files", get(get_files))
        .route("/file", get(get_file))
        .route("/capabilities", get(get_capabilities));
    if shared.allow_writes {
        router = router.route("/resolve", post(post_resolve));
    }
    router
        .layer(middleware::from_fn_with_state(shared.clone(), require_auth))
        .with_state(shared)
}

/// The loopback socket address `serve` binds — 127.0.0.1 only. A non-loopback
/// address is never constructed (REQ-7): the mobile server is localhost-only in
/// Phase 1; remote exposure is a later, separately-designed phase.
pub fn bind_addr(port: u16) -> SocketAddr {
    SocketAddr::from((Ipv4Addr::LOCALHOST, port))
}

/// Bind the loopback listener for `port` (0 = an OS-assigned ephemeral port).
pub async fn bind(port: u16) -> std::io::Result<TcpListener> {
    TcpListener::bind(bind_addr(port)).await
}

/// Serve the read API on an already-bound listener until the process is stopped
/// (Ctrl-C). Spawns the headless fold loop, then serves the four endpoints. The
/// listener is passed in so a test can bind an ephemeral port and learn it first.
pub async fn serve_on(listener: TcpListener, shared: Shared) -> Result<(), String> {
    tokio::spawn(fold_loop(shared.clone()));
    axum::serve(listener, app(shared))
        .await
        .map_err(|e| e.to_string())
}

/// Resolve the effective auth posture: `--no-auth` wins; otherwise an explicit
/// `--token`/`COSPAN_SERVE_TOKEN` (non-empty) is used, else a fresh token is
/// minted. Split out so it is unit-testable without a running server.
pub fn resolve_auth(token_opt: Option<String>, no_auth: bool) -> Auth {
    if no_auth {
        return Auth::None;
    }
    let tok = token_opt
        .filter(|t| !t.is_empty())
        .or_else(|| {
            std::env::var("COSPAN_SERVE_TOKEN")
                .ok()
                .filter(|t| !t.is_empty())
        })
        .unwrap_or_else(mint_token);
    Auth::Token(Arc::from(tok.as_str()))
}

/// `cospan serve <repo> [--port N] [--token T | --no-auth] [--max-stream N]` —
/// run the read API in the foreground on 127.0.0.1. Builds one tokio runtime and
/// blocks on it (like `mcp::run`), so `fn main` stays synchronous. Auth is on by
/// default: a token is minted and the ready URL (with `?token=`) is printed once
/// to stderr; the token is held in memory only.
#[allow(clippy::too_many_arguments)]
pub fn run(
    repo: PathBuf,
    port: u16,
    token_opt: Option<String>,
    no_auth: bool,
    max_stream: usize,
    allow_writes: bool,
    author_id: String,
) -> Result<(), String> {
    if !repo.join(".kan").is_dir() {
        eprintln!(
            "warning: {} has no .kan/ — is this a kan repo?",
            repo.display()
        );
    }
    let auth = resolve_auth(token_opt, no_auth);
    // A blank token can only arise if the OS RNG failed to mint one; refuse to
    // serve rather than run a gate that an empty `?token=` would satisfy.
    if let Auth::Token(t) = &auth {
        if t.is_empty() {
            return Err(
                "failed to mint an auth token (OS RNG unavailable); refusing to serve without one — pass --token <t> or --no-auth".into(),
            );
        }
    }
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async move {
        let listener = bind(port).await.map_err(|e| e.to_string())?;
        let addr = listener.local_addr().map_err(|e| e.to_string())?;
        match &auth {
            Auth::Token(tok) => {
                eprintln!("cospan serve: http://{addr}/?token={tok}  (read-only; Ctrl-C to stop)");
                eprintln!("  open that URL, or send `Authorization: Bearer {tok}`.");
            }
            Auth::None => {
                eprintln!("cospan serve: http://{addr}/  (read-only; Ctrl-C to stop)");
                eprintln!("  WARNING: --no-auth — this channel is UNAUTHENTICATED; a proxy (e.g. `tailscale serve`) would expose it to your tailnet.");
            }
        }
        if allow_writes {
            let who = if author_id.is_empty() { "web" } else { &author_id };
            eprintln!("  --allow-writes: comment writes (add/reply/resolve) ENABLED, attributed human:{who} (sidecars only; no kan claim, no agent control).");
        }
        // Seed with an initial fold so the first request does not race the loop.
        let initial = substrate::fold(&repo);
        let mut shared = Shared::seed(repo, initial)
            .with_auth(auth)
            .with_max_stream(max_stream);
        if allow_writes {
            shared = shared.with_writes(author_id);
        }
        serve_on(listener, shared).await
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::substrate::{Atom, Claim, Fold, ProcessSnapshot, TelosView, Tension};
    use std::collections::HashMap;

    fn sample_fold() -> Fold {
        let claim = Claim {
            cid: "bafyclaim".into(),
            kind: "Decision".into(),
            subject: "telos/p0-spine".into(),
            author: "did:key:zAbc".into(),
            recorded_at: Some(1_787_000_000_000_000),
            text: Some("the watch-and-fold spine runs".into()),
            title: None,
            artifacts: vec!["Commit(\"deadbeef\")".into()],
            cites: vec![],
            supersedes: None,
        };
        let mut claims: HashMap<String, Vec<Claim>> = HashMap::new();
        claims.insert("telos/p0-spine".into(), vec![claim.clone()]);
        let mut by_cid = HashMap::new();
        by_cid.insert(claim.cid.clone(), claim);
        Fold {
            subjects: vec!["telos/p0-spine".into(), "atom/design".into()],
            claims,
            by_cid,
            process: ProcessSnapshot {
                atoms: vec![Atom {
                    slug: "atom/design".into(),
                    inputs: vec!["intent".into()],
                    outputs: vec!["design-doc".into()],
                    next: vec!["generative-build".into()],
                    done: vec![],
                    revisits: vec![],
                }],
                teloi: vec![TelosView {
                    slug: "telos/p0-spine".into(),
                    title: "P0: the watch-and-fold spine".into(),
                    statement: "the spine runs".into(),
                    witnesses: vec![],
                }],
                tensions: vec![Tension {
                    between: ("telos/disposable".into(), "telos/kan-is-truth".into()),
                    why: "comments are owned state".into(),
                }],
                witnesses: Default::default(),
            },
            day_status: Some("atom: design".into()),
            errors: vec![],
        }
    }

    /// AC-1: every rendered type serializes; the JSON carries the subjects, the
    /// claim's fields, and the process keys — proving the REQ-2 derives cover the
    /// whole shape with no separately-maintained projection.
    #[test]
    fn fold_serializes_with_subjects_claims_and_process() {
        let json = serde_json::to_string(&sample_fold()).expect("Fold serializes");
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();

        // subjects
        assert!(json.contains("telos/p0-spine"));
        // a claim's fields
        assert_eq!(v["claims"]["telos/p0-spine"][0]["cid"], "bafyclaim");
        assert_eq!(v["claims"]["telos/p0-spine"][0]["kind"], "Decision");
        assert_eq!(
            v["claims"]["telos/p0-spine"][0]["artifacts"][0],
            "Commit(\"deadbeef\")"
        );
        assert_eq!(v["by_cid"]["bafyclaim"]["subject"], "telos/p0-spine");
        // the process snapshot: atoms, teloi, tensions
        assert_eq!(v["process"]["atoms"][0]["slug"], "atom/design");
        assert_eq!(v["process"]["atoms"][0]["outputs"][0], "design-doc");
        assert_eq!(v["process"]["teloi"][0]["slug"], "telos/p0-spine");
        assert_eq!(
            v["process"]["tensions"][0]["between"][0],
            "telos/disposable"
        );
        assert_eq!(v["day_status"], "atom: design");
    }

    /// AC-5 (the never-constructed-non-loopback half): the server's bind address
    /// is 127.0.0.1 for any port, so a non-loopback bind cannot be reached from
    /// this code path. (The no-disk-write half is asserted in the smoke, which
    /// starts and stops a real server.)
    #[test]
    fn bind_address_is_always_loopback() {
        for port in [0u16, 80, DEFAULT_PORT, 65535] {
            let addr = bind_addr(port);
            assert!(addr.ip().is_loopback(), "{addr} must be loopback");
            assert_eq!(addr.port(), port);
        }
    }

    /// AC-4 (traversal half, no kan needed): the `/comments` core rejects a
    /// `..`-escaping path with its guard error, never reading outside the repo —
    /// the exact `mcp::list_comments` the handler wraps under `spawn_blocking`.
    #[test]
    fn comments_core_rejects_path_traversal() {
        let repo = std::env::temp_dir().join(format!("cospan-serve-guard-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).unwrap();
        let v = mcp::list_comments(&repo, "../../etc/passwd");
        assert!(v.get("error").is_some(), "escaping read must error: {v}");
        std::fs::remove_dir_all(&repo).ok();
    }

    /// The embedded human view is present and wired to the API it consumes — a
    /// non-empty page that references `/fold` and `/stream`, so an accidentally
    /// empty `include_str!` or a renamed endpoint is caught at CI without needing
    /// a running server.
    #[test]
    fn index_html_is_embedded_and_wired_to_the_api() {
        assert!(INDEX_HTML.len() > 500, "index page looks empty");
        assert!(
            INDEX_HTML.contains("<title>cospan</title>"),
            "not the cospan page"
        );
        assert!(INDEX_HTML.contains("/fold"), "page must fetch /fold");
        assert!(
            INDEX_HTML.contains("/stream"),
            "page must subscribe to /stream"
        );
        // AC-4: the page captures its own ?token= and reuses it (withTok wraps
        // the API + WS URLs) so a single saved link authenticates.
        assert!(
            INDEX_HTML.contains("token") && INDEX_HTML.contains("withTok"),
            "page must read and reuse a ?token="
        );
    }

    /// AC-5 (Slice C): the write posture — `with_writes` flips `allow_writes` and
    /// resolves the web author (default `web`, else the supplied id, always
    /// `who:"human"`); the default posture is read-only. And the page wires the
    /// capability probe + a POST write path.
    #[test]
    fn write_config_and_page_wiring() {
        let base = Shared::seed(PathBuf::from("/x"), Fold::default());
        assert!(!base.allow_writes, "read-only by default");

        let dflt = base.clone().with_writes(String::new());
        assert!(dflt.allow_writes);
        assert_eq!(dflt.web_author.who, "human");
        assert_eq!(dflt.web_author.id, "web", "empty id defaults to web");

        let named = base.with_writes("maxine".into());
        assert_eq!(named.web_author.id, "maxine", "supplied id is used");

        // The page probes /capabilities and offers a POST write path.
        assert!(
            INDEX_HTML.contains("/capabilities"),
            "page must probe capabilities"
        );
        assert!(
            INDEX_HTML.contains("caps.writes"),
            "write UI gated on the capability"
        );
        assert!(
            INDEX_HTML.contains("method: \"POST\""),
            "page must POST a write"
        );
        assert!(INDEX_HTML.contains("/resolve"), "page wires resolve");
    }

    /// AC-4 (Chat): the page wires the Chat tab over `/chat` with a collapse
    /// affordance for thinking/tool turns.
    #[test]
    fn index_html_wires_the_chat_tab() {
        assert!(
            INDEX_HTML.contains("data-view=\"chat\""),
            "chat tab present"
        );
        assert!(INDEX_HTML.contains("/chat"), "page fetches /chat");
        assert!(INDEX_HTML.contains("?session="), "session drill-in");
        assert!(
            INDEX_HTML.contains("COLLAPSES") && INDEX_HTML.contains("thinking"),
            "thinking/tool turns collapse"
        );
    }

    /// AC-3 (Slice A): the page wires the UX-pass behavior — a Comments tab over
    /// `/comments`+`/thread`, claim drill-in resolving cites through `by_cid`, a
    /// visibility-aware capped-backoff reconnect, and the render-once startup.
    #[test]
    fn index_html_wires_the_ux_pass() {
        for needle in [
            "data-view=\"comments\"", // the fourth tab
            "/thread",                // thread drill-in
            "by_cid",                 // cite resolution
            "document.hidden",        // visibility gating
            "visibilitychange",       // reconnect on return
            "backoff",                // capped backoff
            "started",                // render-once gate (REQ-5)
        ] {
            assert!(
                INDEX_HTML.contains(needle),
                "page missing UX-pass wiring: {needle}"
            );
        }
    }

    /// AC-5 (file-viewer slice): the page wires the file browser + viewer + the
    /// tap-a-line add path, and the add affordance is gated on the writes
    /// capability (not present in the read-only render path).
    #[test]
    fn index_html_wires_the_file_viewer() {
        for needle in [
            "/files",             // the browsable-file list endpoint
            "/file?path=",        // the file-content endpoint
            "renderFilesBrowser", // the all-files browser
            "openFileViewer",     // the highlighted viewer
            "segmented",          // the all files | commented toggle
            "codeview",           // the rendered code
            "startAddAt",         // tap-a-line to add the first comment
        ] {
            assert!(
                INDEX_HTML.contains(needle),
                "page missing file-viewer wiring: {needle}"
            );
        }
        // The add affordance (startAddAt) is reached only under caps.writes — the
        // tap handler guards it, so a read-only page never offers it.
        let tap = INDEX_HTML
            .split_once("startAddAt(file, lineNo)")
            .expect("tap handler present")
            .0;
        let guard = &tap[tap.len().saturating_sub(60)..];
        assert!(
            guard.contains("caps.writes"),
            "startAddAt must be gated on caps.writes, saw: {guard}"
        );
    }

    /// Responsive-desktop-layout slice: the page carries the desktop shell (a
    /// 900px breakpoint, the single `<nav>` restyled into a rail), the Comments
    /// and Chat master-detail panes with drill-ins targeting the detail pane, a
    /// readability cap, and leaves Now/Teloi/Browse single-pane — without dropping
    /// the mobile wiring.
    #[test]
    fn index_html_wires_the_responsive_layout() {
        // AC-1: desktop breakpoint, and the rail is the ONE restyled nav (not a 2nd).
        assert!(
            INDEX_HTML.contains("@media (min-width: 900px)"),
            "no desktop breakpoint"
        );
        assert_eq!(
            INDEX_HTML.matches("<nav").count(),
            1,
            "exactly one <nav> — the rail is the restyled bottom bar"
        );

        // AC-2: two master-detail views, each a list + detail pane; drill-ins
        // target the detail pane; the old single #comments/#chat replace-target is
        // gone (so drill-in no longer overwrites the whole view).
        // Comments + Chat + Teloi + Browse are master-detail (Browse joined in the browse-timeline slice).
        assert_eq!(
            INDEX_HTML.matches("class=\"view md\"").count(),
            4,
            "Comments+Chat+Teloi+Browse are master-detail"
        );
        assert_eq!(
            INDEX_HTML.matches("class=\"pane-list\"").count(),
            4,
            "four list panes (Comments, Chat, Teloi, Browse)"
        );
        assert_eq!(
            INDEX_HTML.matches("class=\"pane-detail\"").count(),
            4,
            "four detail panes (Comments, Chat, Teloi, Browse)"
        );
        assert!(
            INDEX_HTML.contains("detail-open"),
            "the mobile one-pane toggle"
        );
        assert!(
            INDEX_HTML.contains("paneDetail(\"comments\")")
                && INDEX_HTML.contains("paneDetail(\"chat\")"),
            "drill-ins must render into the detail pane"
        );
        assert!(
            !INDEX_HTML.contains("id=\"comments\"") && !INDEX_HTML.contains("id=\"chat\""),
            "the single #comments/#chat replace-target must be split into panes"
        );

        // AC-3: a readability cap bounds the master-detail reading width at desktop.
        // The detail pane fills its column (so it scales with the window — operator
        // feedback in the browse-timeline slice); the bound is the whole two-pane
        // area's max-width, so prose still can't stretch unbounded on ultra-wide.
        assert!(
            INDEX_HTML.contains(".view.md.active") && INDEX_HTML.contains("max-width: 1500px"),
            "the master-detail area must carry a max-width readability cap (REQ-6)"
        );

        // AC-4: the mobile/live/token wiring is intact (the refactor dropped none).
        for needle in [
            "function setView",
            "withTok",
            "/stream",
            "openFileViewer",
            "startAddAt",
            "--nav-h",
            "data-view=",
        ] {
            assert!(
                INDEX_HTML.contains(needle),
                "responsive refactor dropped: {needle}"
            );
        }

        // AC-5: Now/Teloi/Browse stay present and single-pane (not master-detail).
        for v in ["view-now", "view-telos", "view-browse"] {
            assert!(INDEX_HTML.contains(&format!("id=\"{v}\"")), "missing {v}");
        }
    }

    /// Teloi-grid-drilldown slice: the Teloi tab is master-detail with a tappable
    /// telos grid in the list pane and a drill-in detail (statement, witnesses +
    /// their probe descriptions, tensions naming it, and the telos's claims via the
    /// shared Browse renderer), with the tensions overview retained.
    #[test]
    fn index_html_wires_the_teloi_grid() {
        // AC-1: master-detail Teloi with a grid + drill-in.
        for needle in [
            "id=\"telos-list\"",
            "id=\"telos-detail\"",
            "telos-rows",
            "openTelos(",
        ] {
            assert!(
                INDEX_HTML.contains(needle),
                "missing teloi grid wiring: {needle}"
            );
        }
        // AC-4: renderTeloi fills the list pane, openTelos the detail pane.
        assert!(
            INDEX_HTML.contains("paneList(\"telos\")")
                && INDEX_HTML.contains("paneDetail(\"telos\")"),
            "renderTeloi must target the list pane and openTelos the detail pane"
        );
        // AC-2: the detail wires witness descriptions, tensions-for-slug, the
        // telos's claims, and REUSES the shared claimEl (no second renderer).
        assert!(
            INDEX_HTML.contains("process?.witnesses") || INDEX_HTML.contains("process.witnesses"),
            "witness description map"
        );
        assert!(
            INDEX_HTML.contains("fold.claims[\"telos/\" + slug]"),
            "reads the telos's claims"
        );
        assert!(
            INDEX_HTML.contains("claimEl(c)"),
            "reuses the Browse claim renderer"
        );
        // AC-3: the standalone tensions overview is retained on the list page.
        assert!(
            INDEX_HTML.contains("Tensions held"),
            "the tensions overview was kept"
        );
        // AC-4: no new external dependency.
        assert!(
            !INDEX_HTML.contains("<script src") && !INDEX_HTML.contains("<link href"),
            "the page must gain no external JS/CSS dependency"
        );
    }

    /// Browse-timeline-and-formatting slice: Browse is master-detail with a
    /// [Subjects | Timeline] toggle, per-subject state summaries, a flat capped
    /// timeline, striking per-operation formatting for all nine kinds, and a
    /// detail pane reusing the shared claimEl.
    #[test]
    fn index_html_wires_the_browse_view() {
        // AC-1: master-detail Browse shell (toggle + filter + content), still one filter.
        for needle in [
            "id=\"browse-list\"",
            "id=\"browse-detail\"",
            "id=\"browse-content\"",
            "id=\"browse-toggle\"",
            "data-mode=\"subjects\"",
            "data-mode=\"timeline\"",
            "id=\"filter\"",
            "renderBrowse",
        ] {
            assert!(
                INDEX_HTML.contains(needle),
                "missing browse wiring: {needle}"
            );
        }
        // AC-2: timeline — flatten/sort/cap + a "showing N of M" note + drill-in.
        assert!(INDEX_HTML.contains("BROWSE_TIMELINE_CAP"), "timeline cap");
        assert!(INDEX_HTML.contains("recorded_at"), "sorts by recorded_at");
        assert!(
            INDEX_HTML.contains("`showing ${") || INDEX_HTML.contains("showing "),
            "cap note"
        );
        assert!(
            INDEX_HTML.contains("openBrowseClaim("),
            "timeline claim drill-in"
        );
        // AC-3: per-subject state summary from Publication/Retraction/Status.
        for needle in [
            "subjectState",
            "\"Publication\"",
            "\"Retraction\"",
            "\"Status\"",
            "published",
            "retracted",
        ] {
            assert!(
                INDEX_HTML.contains(needle),
                "missing state-summary wiring: {needle}"
            );
        }
        // AC-4: per-op formatting — a KIND_GLYPH map + a .kind rule for each kind.
        assert!(INDEX_HTML.contains("KIND_GLYPH"), "kind glyph map");
        for kind in [
            "Subject",
            "Decision",
            "Publication",
            "Plan",
            "Observation",
            "Result",
            "Status",
            "Relation",
            "Retraction",
        ] {
            assert!(
                INDEX_HTML.contains(&format!(".kind.{kind}")),
                "missing kind color rule: {kind}"
            );
        }
        // AC-5: detail pane reuses claimEl; renderBrowse targets the list content.
        assert!(
            INDEX_HTML.contains("openBrowseSubject")
                && INDEX_HTML.contains("paneDetail(\"browse\")")
                && INDEX_HTML.contains("claimEl("),
            "detail pane must reuse claimEl via paneDetail(\"browse\")"
        );
        assert!(
            INDEX_HTML.contains("$(\"browse-content\")"),
            "renderBrowse targets the list-pane content"
        );
        // AC-5: no new external dependency.
        assert!(
            !INDEX_HTML.contains("<script src") && !INDEX_HTML.contains("<link href"),
            "the page must gain no external JS/CSS dependency"
        );
    }

    /// AC-1: the token minter yields a non-empty URL-safe token that differs each
    /// call, and `resolve_auth` honors `--no-auth` (open) vs an explicit token.
    #[test]
    fn token_minting_and_auth_resolution() {
        let t1 = mint_token();
        let t2 = mint_token();
        assert!(t1.len() >= 32, "token too short: {}", t1.len());
        assert!(
            t1.chars().all(|c| c.is_ascii_hexdigit()),
            "not url-safe: {t1}"
        );
        assert_ne!(t1, t2, "two mints must differ");

        assert!(
            matches!(resolve_auth(None, true), Auth::None),
            "--no-auth is open"
        );
        match resolve_auth(Some("pinned".into()), false) {
            Auth::Token(t) => assert_eq!(&*t, "pinned"),
            Auth::None => panic!("explicit --token must gate"),
        }
        // Empty --token falls through to a mint, not an empty (guessable) token.
        match resolve_auth(Some(String::new()), false) {
            Auth::Token(t) => assert!(t.len() >= 32, "empty --token must mint"),
            Auth::None => panic!("must still gate"),
        }
    }

    /// The constant-time compare accepts an exact match and rejects mismatches
    /// (including a length mismatch) — the gate's core predicate.
    #[test]
    fn constant_time_eq_matches_only_exact() {
        assert!(ct_eq(b"abc123", b"abc123"));
        assert!(!ct_eq(b"abc123", b"abc124"));
        assert!(!ct_eq(b"abc", b"abc123"));
        assert!(!ct_eq(b"", b"x"));
    }

    /// AC-6 (the isolated fold-loop-step half): call `fold_tick` directly, with no
    /// terminal and no socket, and assert that a changed `HEAD` mtime makes it
    /// rebuild and swap the shared latest-`Fold` (empty → populated) *and* broadcast
    /// the fresh fold to a subscriber. The `#[ignore]` is because it folds a real
    /// `.kan` log (CI has git but not kan) — run with
    /// `cargo test --lib -- --ignored`. The end-to-end socket path is covered
    /// separately by `tests/server_smoke.rs`.
    #[test]
    #[ignore = "folds a real kan log; run locally with --ignored"]
    fn fold_tick_rebuilds_and_swaps_shared_latest_and_broadcasts() {
        use std::process::Command;
        let repo = std::env::temp_dir().join(format!("cospan-foldtick-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).unwrap();
        let git = |args: &[&str]| {
            Command::new("git")
                .arg("-C")
                .arg(&repo)
                .args(args)
                .output()
                .unwrap();
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "t@t"]);
        git(&["config", "user.name", "t"]);
        std::fs::write(repo.join("f.txt"), "x").unwrap();
        git(&["add", "-A"]);
        git(&["commit", "-qm", "init"]);
        let kan = Command::new("kan")
            .current_dir(&repo)
            .args(["observe", "tick-subject", "a note the tick must fold in"])
            .output()
            .unwrap();
        assert!(
            kan.status.success(),
            "kan observe: {}",
            String::from_utf8_lossy(&kan.stderr)
        );

        // Seed with an EMPTY fold, so a successful tick is observable as a swap.
        let shared = Shared::seed(repo.clone(), Fold::default());
        let mut rx = shared.tx.subscribe();
        assert!(
            shared.latest.read().unwrap().subjects.is_empty(),
            "seeded empty"
        );

        // last=None → should_refold(None, Some(mtime)) is true → it must refold.
        let after = fold_tick(&shared, None);
        assert!(after.is_some(), "HEAD mtime is readable after a commit");

        // The shared latest was swapped to the freshly folded state.
        let subjects = shared.latest.read().unwrap().subjects.clone();
        assert!(
            subjects.iter().any(|s| s == "tick-subject"),
            "fold_tick must swap in the real fold: {subjects:?}"
        );

        // And the fresh fold was broadcast to the subscriber (REQ-5's feed).
        let pushed = rx.try_recv().expect("a fold was broadcast on refold");
        let v: serde_json::Value = serde_json::from_str(&pushed).unwrap();
        assert!(
            v["subjects"]
                .as_array()
                .unwrap()
                .iter()
                .any(|s| s == "tick-subject"),
            "the broadcast carries the swapped fold: {}",
            v["subjects"]
        );

        // A second tick with the SAME mtime must NOT refold or re-broadcast.
        let again = fold_tick(&shared, after);
        assert_eq!(again, after, "unchanged mtime carries forward");
        assert!(
            matches!(rx.try_recv(), Err(broadcast::error::TryRecvError::Empty)),
            "no refold ⇒ no broadcast"
        );

        std::fs::remove_dir_all(&repo).ok();
    }
}
