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
