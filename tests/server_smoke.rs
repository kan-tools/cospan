//! End-to-end smoke for the mobile read-API server (mobile Phase 1). Shells real
//! `git` and `kan` and folds a real `.kan` log, so it is `#[ignore]`d — CI has
//! git but not kan. Run locally with:
//!   cargo test --test server_smoke -- --ignored --nocapture
//!
//! Covers the acceptance criteria a hand-built `Fold` can't: `GET /fold` and
//! `GET /comments` over HTTP against a real log (AC-2, AC-4-live), the `/stream`
//! snapshot-then-push on an actual refold (AC-3, AC-6), and that starting and
//! stopping the server writes no state under the repo (AC-5, no-disk half).

use cospan::comments::{self, Author, Comment, StoredAnchor};
use cospan::server::{self, Shared};
use cospan::substrate;
use futures_util::{SinkExt, StreamExt};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;

fn git(dir: &Path, args: &[&str]) {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .unwrap();
}

fn kan(dir: &Path, args: &[&str]) {
    let out = Command::new("kan")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "kan {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A temp git+kan repo carrying one claim on `smoke-subject` and one sidecar
/// comment on `src/a.rs`, so the fold has a subject and a comment to read.
fn setup_repo(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cospan-serve-smoke-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    let content = "l0\nl1\nl2\nl3\n";
    std::fs::write(dir.join("src/a.rs"), content).unwrap();
    git(&dir, &["init", "-q"]);
    git(&dir, &["config", "user.email", "t@t"]);
    git(&dir, &["config", "user.name", "t"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-qm", "init"]); // kan anchors to the root commit
    kan(&dir, &["observe", "smoke-subject", "a folded note"]);

    let c = Comment {
        id: "c_smoke".into(),
        anchor: StoredAnchor::capture(content, 1, 2),
        body: "is this hot?".into(),
        author: Author {
            who: "human".into(),
            id: "tester".into(),
        },
        created_at: 1,
        resolved: false,
        thread: Vec::new(),
    };
    comments::save(&dir.join(comments::sidecar_path("src/a.rs")), &[c]).unwrap();
    dir
}

/// Every path under `dir`, relative — a set to diff before/after the server runs.
fn tree(dir: &Path) -> BTreeSet<PathBuf> {
    fn walk(dir: &Path, base: &Path, out: &mut BTreeSet<PathBuf>) {
        let Ok(rd) = std::fs::read_dir(dir) else {
            return;
        };
        for e in rd.flatten() {
            let p = e.path();
            out.insert(p.strip_prefix(base).unwrap().to_path_buf());
            if p.is_dir() {
                walk(&p, base, out);
            }
        }
    }
    let mut out = BTreeSet::new();
    walk(dir, dir, &mut out);
    out
}

/// A minimal HTTP/1.1 GET over a fresh loopback connection; returns the response
/// body (everything after the header terminator). `Connection: close` so the read
/// ends at EOF without parsing content-length.
async fn http_get(port: u16, path: &str) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let req = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
    s.write_all(req.as_bytes()).await.unwrap();
    let mut buf = Vec::new();
    s.read_to_end(&mut buf).await.unwrap();
    let text = String::from_utf8_lossy(&buf);
    text.split_once("\r\n\r\n")
        .map(|x| x.1)
        .unwrap_or("")
        .to_string()
}

#[test]
#[ignore = "shells real kan; run locally with --ignored"]
fn serve_fold_and_comments_over_http_and_writes_no_state() {
    let dir = setup_repo("http");
    let before = tree(&dir);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let listener = server::bind(0).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let shared = Shared::seed(dir.clone(), substrate::fold(&dir));
        let handle = tokio::spawn(server::serve_on(listener, shared));

        // AC-2: GET /fold carries the folded subject and its claim.
        let fold_body = http_get(port, "/fold").await;
        let fold: serde_json::Value = serde_json::from_str(&fold_body)
            .unwrap_or_else(|e| panic!("/fold body was not JSON ({e}): {fold_body:.200}"));
        let subjects: Vec<&str> = fold["subjects"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|s| s.as_str())
            .collect();
        assert!(
            subjects.contains(&"smoke-subject"),
            "/fold subjects: {subjects:?}"
        );
        assert!(
            fold["claims"]["smoke-subject"].is_array(),
            "/fold claims: {}",
            fold["claims"]
        );

        // AC-4 (live): GET /comments returns the sidecar comment; a traversal is refused.
        let comments_body = http_get(port, "/comments?file=src/a.rs").await;
        let cs: serde_json::Value = serde_json::from_str(&comments_body).unwrap();
        assert_eq!(cs["comments"][0]["body"], "is this hot?", "/comments: {cs}");
        let escape = http_get(port, "/comments?file=../../etc/passwd").await;
        let ev: serde_json::Value = serde_json::from_str(&escape).unwrap();
        assert!(ev.get("error").is_some(), "traversal must error: {ev}");

        handle.abort();
    });

    // AC-5 (no-disk half): the server wrote nothing under the repo. (No claim was
    // recorded in this test, so the log is untouched too.)
    let after = tree(&dir);
    assert_eq!(before, after, "server must write no state under the repo");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
#[ignore = "shells real kan; run locally with --ignored"]
fn serve_stream_snapshots_then_pushes_on_refold() {
    let dir = setup_repo("stream");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let listener = server::bind(0).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let shared = Shared::seed(dir.clone(), substrate::fold(&dir));
        let handle = tokio::spawn(server::serve_on(listener, shared));

        let (mut ws, _resp) =
            tokio_tungstenite::connect_async(format!("ws://127.0.0.1:{port}/stream"))
                .await
                .expect("ws connect");

        // AC-3: the current Fold arrives immediately on connect.
        let snap = next_fold(&mut ws).await;
        let subj0: Vec<&str> = snap["subjects"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|s| s.as_str())
            .collect();
        assert!(
            subj0.contains(&"smoke-subject"),
            "connect snapshot: {subj0:?}"
        );
        assert!(
            !subj0.contains(&"pushed-subject"),
            "not recorded yet: {subj0:?}"
        );

        // Trigger a real refold: a new claim touches .kan/log/HEAD (AC-6 — the
        // headless loop rebuilds and re-broadcasts with no terminal involved).
        kan(
            &dir,
            &["observe", "pushed-subject", "arrived after connect"],
        );

        // AC-3/AC-6: a pushed Fold carrying the new subject arrives. Poll a few
        // frames within a timeout — the loop ticks every ~250ms.
        let deadline = Duration::from_secs(10);
        let got = tokio::time::timeout(deadline, async {
            loop {
                let f = next_fold(&mut ws).await;
                let subs: Vec<String> = f["subjects"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|s| s.as_str().map(String::from))
                    .collect();
                if subs.iter().any(|s| s == "pushed-subject") {
                    return true;
                }
            }
        })
        .await;
        assert!(
            got.is_ok(),
            "no pushed fold with the new subject within {deadline:?}"
        );

        handle.abort();
    });
    std::fs::remove_dir_all(&dir).ok();
}

/// Read WebSocket messages until a text frame parses as a Fold JSON object,
/// returning it. Panics on close/error — the stream should stay open.
async fn next_fold(
    ws: &mut tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
) -> serde_json::Value {
    loop {
        match ws.next().await {
            Some(Ok(Message::Text(t))) => {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&t) {
                    if v.get("subjects").is_some() {
                        return v;
                    }
                }
            }
            Some(Ok(Message::Ping(p))) => {
                let _ = ws.send(Message::Pong(p)).await;
            }
            Some(Ok(_)) => {}
            other => panic!("stream ended before a fold frame: {other:?}"),
        }
    }
}
