//! End-to-end promote-to-kan smoke (S4). Shells real `git` and `kan`, so it is
//! `#[ignore]`d — CI has git but not kan. Run locally with:
//!   cargo test --test promote_smoke -- --ignored --nocapture

use cospan::comments::{self, Author, Comment, StoredAnchor};
use cospan::substrate;
use cospan::tui::AppState;
use std::path::PathBuf;
use std::process::Command;

fn git(dir: &PathBuf, args: &[&str]) {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .unwrap();
}

fn kan_json(dir: &PathBuf, subject: &str) -> serde_json::Value {
    let out = Command::new("kan")
        .current_dir(dir)
        .args(["show", subject, "--json"])
        .output()
        .unwrap();
    serde_json::from_slice(&out.stdout).unwrap()
}

#[test]
#[ignore = "shells real kan; run locally with --ignored"]
fn promote_records_a_claim_and_re_promote_cites_prior() {
    let dir = std::env::temp_dir().join(format!("cospan-promote-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    let content = "l0\nl1\nl2\nl3\n";
    std::fs::write(dir.join("src/a.rs"), content).unwrap();
    git(&dir, &["init", "-q"]);
    git(&dir, &["config", "user.email", "t@t"]);
    git(&dir, &["config", "user.name", "t"]);
    git(&dir, &["add", "-A"]);
    git(&dir, &["commit", "-qm", "init"]); // kan anchors to the root commit

    // A sidecar comment on line 1.
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

    let mut a = AppState::new(dir.clone(), substrate::fold(&dir), None);
    a.open_path(PathBuf::from("src/a.rs"));
    assert_eq!(a.comment_localized.len(), 1, "the comment loaded");

    a.promote_selected();
    let v = kan_json(&dir, "comment/src/a.rs");
    let claims = v["claims"].as_array().unwrap();
    assert_eq!(claims.len(), 1, "one promoted claim");
    let text = claims[0]["text"].as_str().unwrap();
    assert!(text.contains("is this hot?"), "body: {text}");
    assert!(text.contains("cospan-comment"), "block present: {text}");
    let arts = format!("{:?}", claims[0]["artifacts"]);
    assert!(
        arts.contains("src/a.rs") && arts.contains("2"),
        "line anchor: {arts}"
    );

    let first_cid = claims[0]["cid"].as_str().unwrap().to_string();

    // Re-promote: a second claim that CITES the first.
    a.promote_selected();
    let v = kan_json(&dir, "comment/src/a.rs");
    let claims = v["claims"].as_array().unwrap();
    assert_eq!(claims.len(), 2, "re-promote appended a second snapshot");
    let cites: Vec<&str> = claims[1]["cites"]
        .as_array()
        .map(|a| a.iter().filter_map(|c| c.as_str()).collect())
        .unwrap_or_default();
    assert!(
        cites.contains(&first_cid.as_str()),
        "the re-promote must cite the prior snapshot: {cites:?} vs {first_cid}"
    );

    std::fs::remove_dir_all(&dir).ok();
}
