//! Integration tests for `cospan serve` auth + the `/stream` connection cap
//! (Slice B). These need a running server and a WS client but **not** kan: the
//! server is seeded with a default `Fold` against a nonexistent repo, so the
//! headless fold loop never shells out. They therefore run in CI (unlike the
//! kan-requiring `server_smoke`), which is where the auth gate must be proven.

use cospan::server::{self, Auth, Shared};
use cospan::substrate::Fold;
use futures_util::StreamExt;
use std::path::PathBuf;
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
    let listener = server::bind(0).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let shared = Shared::seed(
        PathBuf::from("/nonexistent-cospan-serveauth-repo"),
        Fold::default(),
    )
    .with_auth(auth)
    .with_max_stream(max_stream);
    tokio::spawn(server::serve_on(listener, shared));
    // Yield so the listener is accepting before the first client connects.
    tokio::time::sleep(Duration::from_millis(50)).await;
    port
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
