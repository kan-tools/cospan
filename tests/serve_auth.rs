//! Integration tests for `cospan serve` auth + the `/stream` connection cap
//! (Slice B). These need a running server and a WS client but **not** kan: the
//! server is seeded with a default `Fold` against a nonexistent repo, so the
//! headless fold loop never shells out. They therefore run in CI (unlike the
//! kan-requiring `server_smoke`), which is where the auth gate must be proven.

use cospan::server::{self, Auth, Shared};
use cospan::substrate::Fold;
use futures_util::StreamExt;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;

const TOKEN: &str = "cafef00dcafef00dcafef00dcafef00dcafef00dcafef00dcafef00dcafef00d0";

/// Start a server on an ephemeral loopback port, seeded with a default fold
/// against a nonexistent repo (the fold loop's mtime gate never fires, so kan is
/// never invoked). Returns the port.
async fn spawn(auth: Auth, max_stream: usize) -> u16 {
    spawn_repo(
        Path::new("/nonexistent-cospan-serveauth-repo"),
        auth,
        max_stream,
    )
    .await
}

/// Like `spawn` but against a specific repo (with no `.kan/`, so the fold loop's
/// mtime gate never fires and kan is never invoked) — used to serve real comment
/// sidecars written under the repo.
async fn spawn_repo(repo: &Path, auth: Auth, max_stream: usize) -> u16 {
    let listener = server::bind(0).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let shared = Shared::seed(repo.to_path_buf(), Fold::default())
        .with_auth(auth)
        .with_max_stream(max_stream);
    tokio::spawn(server::serve_on(listener, shared));
    tokio::time::sleep(Duration::from_millis(50)).await;
    port
}

/// Like `spawn_repo` but with comment writes enabled (`--allow-writes`), stamping
/// the given author id.
async fn spawn_writes(repo: &Path, auth: Auth, author: &str) -> u16 {
    let listener = server::bind(0).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let shared = Shared::seed(repo.to_path_buf(), Fold::default())
        .with_auth(auth)
        .with_writes(author.to_string());
    tokio::spawn(server::serve_on(listener, shared));
    tokio::time::sleep(Duration::from_millis(50)).await;
    port
}

/// POST a JSON body (optionally with one header line, e.g. Authorization),
/// returning (status, response-body).
async fn http_post(port: u16, path: &str, body: &str, header: Option<&str>) -> (u16, String) {
    let mut s = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let extra = header.map(|h| format!("{h}\r\n")).unwrap_or_default();
    let req = format!(
        "POST {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{extra}content-type: application/json\r\ncontent-length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    s.write_all(req.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    s.read_to_end(&mut buf).await.unwrap();
    let text = String::from_utf8_lossy(&buf);
    let status = text
        .split_whitespace()
        .nth(1)
        .and_then(|c| c.parse().ok())
        .unwrap_or(0);
    let rbody = text
        .split_once("\r\n\r\n")
        .map(|x| x.1.to_string())
        .unwrap_or_default();
    (status, rbody)
}

/// GET the response body (everything after the header terminator).
async fn http_body(port: u16, path: &str) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let req = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    s.write_all(req.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    s.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf)
        .split_once("\r\n\r\n")
        .map(|x| x.1.to_string())
        .unwrap_or_default()
}

/// Issue an HTTP/1.1 GET (optionally with one extra header line, e.g.
/// `Authorization: Bearer …`), returning the numeric status.
async fn http_status(port: u16, path: &str, header: Option<&str>) -> u16 {
    let mut s = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let extra = header.map(|h| format!("{h}\r\n")).unwrap_or_default();
    let req = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{extra}Connection: close\r\n\r\n");
    s.write_all(req.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    s.read_to_end(&mut buf).await.unwrap();
    let text = String::from_utf8_lossy(&buf);
    // "HTTP/1.1 NNN ..."
    text.split_whitespace()
        .nth(1)
        .and_then(|c| c.parse().ok())
        .unwrap_or(0)
}

async fn ws_connect(
    port: u16,
    path: &str,
) -> Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
    tokio_tungstenite::tungstenite::Error,
> {
    tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{port}{path}"))
        .await
        .map(|(ws, _resp)| ws)
}

#[test]
fn auth_gates_http_routes() {
    // AC-2: with a token, /fold needs it; the page (/) is gated too (AC-4).
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn(Auth::Token(Arc::from(TOKEN)), 64).await;

        assert_eq!(
            http_status(port, "/fold", None).await,
            401,
            "no token → 401"
        );
        assert_eq!(
            http_status(port, &format!("/fold?token={TOKEN}"), None).await,
            200,
            "?token= → 200"
        );
        let bearer = format!("Authorization: Bearer {TOKEN}");
        assert_eq!(
            http_status(port, "/fold", Some(&bearer)).await,
            200,
            "Bearer → 200"
        );
        assert_eq!(
            http_status(port, "/fold?token=wrong", None).await,
            401,
            "wrong token → 401"
        );

        // AC-4: the HTML page is gated by the same token in the query.
        assert_eq!(http_status(port, "/", None).await, 401, "/ no token → 401");
        assert_eq!(
            http_status(port, &format!("/?token={TOKEN}"), None).await,
            200,
            "/?token= → 200 (the page)"
        );
    });
}

#[test]
fn wrong_bearer_does_not_mask_a_correct_query_token() {
    // Review follow-up: a stale/wrong Authorization header must not shadow a
    // correct ?token= — any presented token that matches authenticates.
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn(Auth::Token(Arc::from(TOKEN)), 64).await;
        assert_eq!(
            http_status(
                port,
                &format!("/fold?token={TOKEN}"),
                Some("Authorization: Bearer wrong")
            )
            .await,
            200,
            "correct query token must win over a wrong Bearer header"
        );
    });
}

#[test]
fn empty_configured_token_authenticates_nothing() {
    // Review follow-up: defend the impossible OS-RNG-failure branch — an empty
    // configured token must never fail open, not even to an empty ?token=.
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn(Auth::Token(Arc::from("")), 64).await;
        assert_eq!(
            http_status(port, "/fold", None).await,
            401,
            "empty token: no creds → 401"
        );
        assert_eq!(
            http_status(port, "/fold?token=", None).await,
            401,
            "empty token: empty ?token= must NOT authenticate"
        );
    });
}

#[test]
fn comment_index_endpoint_and_single_file_and_traversal() {
    // AC-2 + AC-4 (Slice A): GET /comments (no file) returns the files index;
    // GET /comments?file= still returns that file's comments; a traversal on the
    // single-file path still hits the guard.
    let repo = std::env::temp_dir().join(format!("cospan-serveA-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/a.rs"), "l0\nl1\nl2\n").unwrap();
    // Seed a comment through the public core (writes a sidecar).
    cospan::mcp::add_comment(&repo, "src/a.rs", 1, "hot?");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn_repo(&repo, Auth::None, 64).await;

        let idx: serde_json::Value =
            serde_json::from_str(&http_body(port, "/comments").await).unwrap();
        let files = idx["files"].as_array().expect("files array");
        assert_eq!(files.len(), 1, "index lists the one commented file: {idx}");
        assert_eq!(files[0]["file"], "src/a.rs");
        assert_eq!(files[0]["total"], 1);

        let one: serde_json::Value =
            serde_json::from_str(&http_body(port, "/comments?file=src/a.rs").await).unwrap();
        assert_eq!(
            one["comments"][0]["body"], "hot?",
            "single-file path still works: {one}"
        );

        let esc: serde_json::Value =
            serde_json::from_str(&http_body(port, "/comments?file=../../etc/passwd").await)
                .unwrap();
        assert!(esc.get("error").is_some(), "traversal still guarded: {esc}");
    });
    std::fs::remove_dir_all(&repo).ok();
}

#[test]
fn writes_off_by_default_405s_and_capabilities_reports_false() {
    // AC-2 (Slice C): without --allow-writes, POST is a 405 and /capabilities says so.
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn(Auth::None, 64).await; // read-only server
        let (status, _b) = http_post(
            port,
            "/comments?file=src/a.rs",
            "{\"line\":1,\"body\":\"x\"}",
            None,
        )
        .await;
        assert_eq!(status, 405, "POST must be 405 when writes are off");
        let caps: serde_json::Value =
            serde_json::from_str(&http_body(port, "/capabilities").await).unwrap();
        assert_eq!(caps["writes"], false, "capabilities: {caps}");
    });
}

#[test]
fn writes_on_add_reply_resolve_round_trip_with_web_attribution() {
    // AC-3 (Slice C): add → reply → resolve over POST, attributed who:human id:web,
    // and a second add does not lose the first (the write lock).
    let repo = std::env::temp_dir().join(format!("cospan-serveC-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/a.rs"), "l0\nl1\nl2\n").unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn_writes(&repo, Auth::None, "web").await;

        // capabilities reports writes + the author.
        let caps: serde_json::Value =
            serde_json::from_str(&http_body(port, "/capabilities").await).unwrap();
        assert_eq!(caps["writes"], true);
        assert_eq!(caps["author"]["id"], "web");

        // add
        let (st, body) = http_post(
            port,
            "/comments?file=src/a.rs",
            "{\"line\":1,\"body\":\"from the phone\"}",
            None,
        )
        .await;
        assert_eq!(st, 200, "add: {body}");
        let add: serde_json::Value = serde_json::from_str(&body).unwrap();
        let id = add["id"].as_str().expect("new comment id").to_string();

        // a second add — the first must survive (no lost update)
        let (_st2, _b2) = http_post(
            port,
            "/comments?file=src/a.rs",
            "{\"line\":2,\"body\":\"second\"}",
            None,
        )
        .await;

        // GET shows both, the first authored who:human id:web
        let listed: serde_json::Value =
            serde_json::from_str(&http_body(port, "/comments?file=src/a.rs").await).unwrap();
        let items = listed["comments"].as_array().unwrap();
        assert_eq!(items.len(), 2, "both adds persisted: {listed}");
        assert_eq!(items[0]["author"]["who"], "human");
        assert_eq!(items[0]["author"]["id"], "web");

        // reply
        let (rst, rbody) = http_post(
            port,
            &format!("/thread?file=src/a.rs&id={id}"),
            "{\"body\":\"a reply\"}",
            None,
        )
        .await;
        assert_eq!(rst, 200, "reply: {rbody}");

        // resolve
        let (sst, sbody) = http_post(
            port,
            &format!("/resolve?file=src/a.rs&id={id}"),
            "{\"value\":true}",
            None,
        )
        .await;
        assert_eq!(sst, 200, "resolve: {sbody}");
        let thread: serde_json::Value =
            serde_json::from_str(&http_body(port, &format!("/thread?file=src/a.rs&id={id}")).await)
                .unwrap();
        assert_eq!(thread["resolved"], true);
        assert_eq!(thread["replies"][0]["body"], "a reply");
        assert_eq!(thread["replies"][0]["author"]["id"], "web");
    });
    std::fs::remove_dir_all(&repo).ok();
}

#[test]
fn writes_are_auth_gated_and_path_guarded() {
    // AC-4 (Slice C): a POST needs the token, and the guard fires on ?file=../
    let repo = std::env::temp_dir().join(format!("cospan-serveCg-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn_writes(&repo, Auth::Token(Arc::from(TOKEN)), "web").await;

        // no token → 401
        let (st, _b) = http_post(
            port,
            "/comments?file=x",
            "{\"line\":1,\"body\":\"x\"}",
            None,
        )
        .await;
        assert_eq!(st, 401, "write without token must be 401");

        // with token but a traversal path → guard error, no write outside repo
        let bearer = format!("Authorization: Bearer {TOKEN}");
        let (st2, b2) = http_post(
            port,
            "/comments?file=../../etc/passwd",
            "{\"line\":1,\"body\":\"x\"}",
            Some(&bearer),
        )
        .await;
        assert_eq!(st2, 200, "guard returns JSON error, not an HTTP error");
        let v: serde_json::Value = serde_json::from_str(&b2).unwrap();
        assert!(v.get("error").is_some(), "traversal must be guarded: {v}");
        assert!(
            !std::path::Path::new("/etc/passwd.jsonl").exists(),
            "no write escaped"
        );
    });
    std::fs::remove_dir_all(&repo).ok();
}

#[test]
fn no_auth_leaves_routes_open() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn(Auth::None, 64).await;
        assert_eq!(
            http_status(port, "/fold", None).await,
            200,
            "--no-auth: open"
        );
        assert_eq!(
            http_status(port, "/", None).await,
            200,
            "--no-auth: page open"
        );
    });
}

#[test]
fn auth_gates_the_stream_upgrade() {
    // AC-3: /stream without a token is rejected; with ?token= it connects and the
    // snapshot frame arrives.
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn(Auth::Token(Arc::from(TOKEN)), 64).await;

        assert!(
            ws_connect(port, "/stream").await.is_err(),
            "unauthenticated /stream upgrade must be rejected"
        );

        let mut ws = ws_connect(port, &format!("/stream?token={TOKEN}"))
            .await
            .expect("authenticated /stream connects");
        let msg = tokio::time::timeout(Duration::from_secs(3), ws.next())
            .await
            .expect("snapshot within timeout")
            .expect("a frame")
            .expect("ok frame");
        match msg {
            Message::Text(t) => {
                let v: serde_json::Value = serde_json::from_str(&t).unwrap();
                assert!(v.get("subjects").is_some(), "snapshot is a fold: {t}");
            }
            other => panic!("expected a text snapshot, got {other:?}"),
        }
    });
}

#[test]
fn stream_cap_rejects_then_releases() {
    // AC-6: with --max-stream 1, a second concurrent /stream is rejected; after
    // the first disconnects, the slot is released and a new one connects.
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let port = spawn(Auth::None, 1).await;

        let ws1 = ws_connect(port, "/stream").await.expect("first connects");
        assert!(
            ws_connect(port, "/stream").await.is_err(),
            "second over the cap must be rejected"
        );

        // Close the first; the server drops its slot when the task ends.
        drop(ws1);

        // Poll for the slot to free (bounded), then a new client must connect.
        let mut connected = false;
        for _ in 0..40 {
            tokio::time::sleep(Duration::from_millis(50)).await;
            if ws_connect(port, "/stream").await.is_ok() {
                connected = true;
                break;
            }
        }
        assert!(
            connected,
            "slot must release after the first client disconnects"
        );
    });
}
