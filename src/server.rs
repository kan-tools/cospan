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
//!   * `GET /comments?file=`  — the S5 comment read core (`mcp::list_comments`).
//!   * `GET /thread?file=&id=`— one comment + thread (`mcp::get_thread`).
//!
//! This is the same projection the TUI renders — a read of the kan/day log,
//! nothing persisted, no claim written (`telos/kan-is-truth`,
//! `telos/observe-now-control-later`; the write seam `command_bus::WriteChannel`
//! stays untouched). It stays disposable (`telos/disposable`): a foreground
//! process bound to 127.0.0.1 with no daemon and no on-disk state.
//!
//! No second async runtime is introduced — `run` builds one `tokio` runtime and
//! `block_on`s the server, exactly as `mcp::run` does, so `fn main` stays sync.

use crate::substrate::{self, Fold};
use crate::{mcp, tui};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse, Json, Response};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
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

/// The shared state every handler and the fold loop read: the repo root, the
/// latest folded `Fold`, and the broadcast sender that fans a fresh serialized
/// fold out to `/stream` subscribers. Cheap to `clone` — everything is `Arc`.
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
}

impl Shared {
    /// Build shared state seeded with an initial fold, so `/fold` answers before
    /// the loop's first tick.
    pub fn seed(repo: PathBuf, initial: Fold) -> Self {
        let (tx, _rx) = broadcast::channel(STREAM_BUFFER);
        Self {
            repo: Arc::new(repo),
            latest: Arc::new(RwLock::new(initial)),
            tx,
        }
    }

    /// The current fold serialized to JSON. A poisoned lock is recovered rather
    /// than propagated — a read never needs a consistent *writer*, just the last
    /// whole value written (the writer swaps a fully-built `Fold` in one store).
    fn snapshot_json(&self) -> String {
        let f = self.latest.read().unwrap_or_else(|e| e.into_inner());
        serde_json::to_string(&*f).unwrap_or_else(|_| "{}".to_string())
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
/// every refold.
async fn get_stream(ws: WebSocketUpgrade, State(shared): State<Shared>) -> Response {
    ws.on_upgrade(move |socket| stream_folds(socket, shared))
}

/// Send the current fold immediately, then forward each broadcast refold until
/// the client disconnects. Client→server frames are ignored (the API is
/// read-only) except a Close, which ends the loop.
async fn stream_folds(mut socket: WebSocket, shared: Shared) {
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
    file: String,
}

/// `GET /comments?file=<rel>` — the S5 comment read core over HTTP. The blocking
/// sidecar I/O runs under `spawn_blocking`; `mcp::list_comments` is already
/// path-traversal-guarded, so a `file=../x` returns its error JSON, never a read
/// outside the repo.
async fn get_comments(
    State(shared): State<Shared>,
    Query(q): Query<CommentsQuery>,
) -> Json<serde_json::Value> {
    let repo = shared.repo.clone();
    let v = tokio::task::spawn_blocking(move || mcp::list_comments(&repo, &q.file))
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

/// The router over the shared state — the four read endpoints, no middleware.
/// Split out so a test can drive the exact app the server serves.
pub fn app(shared: Shared) -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/fold", get(get_fold))
        .route("/stream", get(get_stream))
        .route("/comments", get(get_comments))
        .route("/thread", get(get_thread))
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

/// `cospan serve <repo> [--port N]` — run the read API in the foreground on
/// 127.0.0.1. Builds one tokio runtime and blocks on it (like `mcp::run`), so
/// `fn main` stays synchronous and no `#[tokio::main]` is introduced.
pub fn run(repo: PathBuf, port: u16) -> Result<(), String> {
    if !repo.join(".kan").is_dir() {
        eprintln!(
            "warning: {} has no .kan/ — is this a kan repo?",
            repo.display()
        );
    }
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async move {
        let listener = bind(port).await.map_err(|e| e.to_string())?;
        let addr = listener.local_addr().map_err(|e| e.to_string())?;
        // Seed with an initial fold so the first request does not race the loop.
        let initial = substrate::fold(&repo);
        let shared = Shared::seed(repo, initial);
        eprintln!("cospan serve: http://{addr}  (read-only; Ctrl-C to stop)");
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
