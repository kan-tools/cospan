//! The ephemeral comment sidecar (P1).
//!
//! A comment is pinned to a text *fingerprint*, not a line number, and
//! re-resolved against the current file on every read into Anchored / Drifted /
//! Unresolvable by the re-localizer (`crate::relocalize`). This module is the
//! storage-and-sharing half of `.dropbox/03-comments.md`: a per-file JSONL
//! sidecar under a gitignored `.cospan/comments/` tree. It is cospan's only
//! owned, mutable state, ephemeral by default — nothing here ever touches kan
//! (`telos/kan-is-truth`'s sole exception; `telos/disposable`).

use crate::{relocalize, Anchor, Localization, State};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// A stored text fingerprint: the re-localizer's `Anchor` fields plus the
/// context width used and the file hash at last capture (for change detection
/// and incremental last-seen tracking).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredAnchor {
    pub target: String,
    pub before: String,
    pub after: String,
    pub line_hint: usize,
    pub ctx: usize,
    pub base_hash: u64,
}

impl StoredAnchor {
    /// Capture a fingerprint at `line` (0-based) of `content`, recording the
    /// context width and the content hash.
    pub fn capture(content: &str, line: usize, ctx: usize) -> StoredAnchor {
        let a = Anchor::from_file(content, line, ctx);
        StoredAnchor {
            target: a.target,
            before: a.before,
            after: a.after,
            line_hint: a.line_hint,
            ctx,
            base_hash: content_hash(content),
        }
    }

    /// The re-localizer `Anchor` for this fingerprint.
    pub fn as_anchor(&self) -> Anchor {
        Anchor {
            target: self.target.clone(),
            before: self.before.clone(),
            after: self.after.clone(),
            line_hint: self.line_hint,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Author {
    /// "human" or "agent".
    pub who: String,
    pub id: String,
}

/// One reply in a comment's thread. Same author model as the root comment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reply {
    pub author: Author,
    pub body: String,
    /// Microseconds since the Unix epoch.
    pub created_at: i64,
}

/// One sidecar comment. Ephemeral, re-localized every read; `resolved` and the
/// `thread` live here so persist-to-kan can map 1:1 later.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub anchor: StoredAnchor,
    pub body: String,
    pub author: Author,
    /// Microseconds since the Unix epoch.
    pub created_at: i64,
    pub resolved: bool,
    /// Reply thread, newest last. `default` so a sidecar written before threads
    /// existed loads unchanged (an absent field becomes an empty thread).
    #[serde(default)]
    pub thread: Vec<Reply>,
}

/// Append `r` to the thread of the comment with `id`; returns whether one matched.
pub fn add_reply(comments: &mut [Comment], id: &str, r: Reply) -> bool {
    match comments.iter_mut().find(|c| c.id == id) {
        Some(c) => {
            c.thread.push(r);
            true
        }
        None => false,
    }
}

/// Mark the comment with `id` resolved; returns whether one matched.
pub fn resolve(comments: &mut [Comment], id: &str) -> bool {
    match comments.iter_mut().find(|c| c.id == id) {
        Some(c) => {
            c.resolved = true;
            true
        }
        None => false,
    }
}

/// Set the resolved flag of the comment with `id` to `val`; returns whether one
/// matched. Unlike [`resolve`], this can also un-resolve (`val = false`) — the
/// toggle the interactive authoring surface needs.
pub fn set_resolved(comments: &mut [Comment], id: &str, val: bool) -> bool {
    match comments.iter_mut().find(|c| c.id == id) {
        Some(c) => {
            c.resolved = val;
            true
        }
        None => false,
    }
}

/// The outcome of an author-gated mutation ([`edit_body`] / [`delete`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutation {
    /// Applied to the matched comment.
    Applied,
    /// No comment carried that id.
    NotFound,
    /// The comment exists but is authored by someone else — refused, so one
    /// author can never rewrite or remove another's comment.
    Forbidden,
}

/// Rewrite the body of the comment with `id`, but only if it is authored by
/// `by_id`; then re-capture its anchor against the current `content` so the
/// edited comment is anchored to the file as it now stands. A comment authored
/// by someone else is [`Mutation::Forbidden`].
pub fn edit_body(
    comments: &mut [Comment],
    id: &str,
    by_id: &str,
    new_body: &str,
    content: &str,
) -> Mutation {
    match comments.iter_mut().find(|c| c.id == id) {
        None => Mutation::NotFound,
        Some(c) if c.author.id != by_id => Mutation::Forbidden,
        Some(c) => {
            // Re-anchor to the current position first (refreshing `base_hash`),
            // then replace the prose.
            localize_and_update(c, content);
            c.body = new_body.to_string();
            Mutation::Applied
        }
    }
}

/// Remove the comment with `id`, but only if it is authored by `by_id`. Gated
/// like [`edit_body`]: another author's comment is [`Mutation::Forbidden`].
pub fn delete(comments: &mut Vec<Comment>, id: &str, by_id: &str) -> Mutation {
    match comments.iter().position(|c| c.id == id) {
        None => Mutation::NotFound,
        Some(i) if comments[i].author.id != by_id => Mutation::Forbidden,
        Some(i) => {
            comments.remove(i);
            Mutation::Applied
        }
    }
}

/// Microseconds since the Unix epoch (0 if the clock reads before the epoch) —
/// the timestamp stamped on a freshly authored comment or reply.
pub fn now_micros() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

// --- Promote-to-kan (S4): the explicit human action that snapshots a sidecar
// --- comment into a durable, signed kan claim. Pure argv/text builders here; the
// --- TUI shells `kan` with them so the round trip is testable without kan.

/// The kan subject a file's promoted comments live on: `comment/<repo-rel path>`.
pub fn comment_subject(repo_rel: &str) -> String {
    format!(
        "comment/{}",
        repo_rel.strip_prefix("./").unwrap_or(repo_rel)
    )
}

/// The claim body for promoting `c`: the human text, then a fenced
/// `cospan-comment` JSON block carrying the full record (fingerprint, author,
/// thread, resolved) — everything the kan line anchor does not, for a 1:1 round
/// trip back to a `Comment`.
pub fn promote_text(c: &Comment) -> String {
    let block = serde_json::to_string(c).unwrap_or_default();
    format!("{}\n\n```cospan-comment\n{block}\n```", c.body)
}

/// The `--file` anchor for a localized span: `path:start-end` in 1-based inclusive
/// lines, or the bare path when the comment is unplaced (`Unresolvable`).
pub fn promote_anchor(repo_rel: &str, span: Option<(usize, usize)>) -> String {
    match span {
        Some((s, e)) => format!("{repo_rel}:{}-{}", s + 1, e + 1),
        None => repo_rel.to_string(),
    }
}

/// The `kan observe` arguments (after the program name) that promote `c` at
/// `span` in `rel`, citing `prior` (the previous promoted claim's CID) when a
/// re-promote. Pure, so the command is unit-testable without running kan.
pub fn promote_argv(
    rel: &str,
    c: &Comment,
    span: Option<(usize, usize)>,
    prior: Option<&str>,
) -> Vec<String> {
    // All flags first, then `--`, then the text as the sole trailing positional:
    // a comment body that begins with `-`/`--` (a markdown bullet, `--help`, …)
    // must NOT be parsed as a flag by kan's argument parser — that silently
    // recorded nothing while looking like success.
    let mut v = vec![
        "observe".to_string(),
        "--subject".to_string(),
        comment_subject(rel),
        "--file".to_string(),
        promote_anchor(rel, span),
    ];
    if let Some(p) = prior {
        v.push("--cites".to_string());
        v.push(p.to_string());
    }
    v.push("--".to_string());
    v.push(promote_text(c));
    v
}

/// A one-line summary suffix for a comment: reply count and resolved state, or
/// empty when it has neither.
pub fn thread_summary(c: &Comment) -> String {
    let mut parts = Vec::new();
    match c.thread.len() {
        0 => {}
        1 => parts.push("(1 reply)".to_string()),
        n => parts.push(format!("({n} replies)")),
    }
    if c.resolved {
        parts.push("[resolved]".to_string());
    }
    parts.join(" ")
}

/// A deterministic content digest (`DefaultHasher`, no extra dependency) used to
/// detect whether a file changed since a comment was last anchored.
pub fn content_hash(content: &str) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut h);
    h.finish()
}

/// The sidecar path for a repo-relative file path: `.cospan/comments/<path>.jsonl`.
pub fn sidecar_path(repo_rel: &str) -> PathBuf {
    let rel = repo_rel.strip_prefix("./").unwrap_or(repo_rel);
    Path::new(".cospan/comments").join(format!("{rel}.jsonl"))
}

/// Read a file's sidecar; a missing sidecar is an empty list, not an error.
pub fn load(path: &Path) -> Result<Vec<Comment>, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.to_string()),
    };
    // A single torn line (e.g. an interrupted write) must not brick the whole
    // sidecar: skip it with a warning and keep the readable comments.
    let mut out = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Comment>(line) {
            Ok(c) => out.push(c),
            Err(e) => eprintln!(
                "cospan: skipping malformed comment {}:{}: {e}",
                path.display(),
                i + 1
            ),
        }
    }
    Ok(out)
}

/// Write a file's sidecar as JSONL, creating the tree on first write. Written to
/// a temp file and renamed into place, so a crash mid-write cannot leave a torn
/// trailing line behind.
pub fn save(path: &Path, comments: &[Comment]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut out = String::new();
    for c in comments {
        out.push_str(&serde_json::to_string(c).map_err(|e| e.to_string())?);
        out.push('\n');
    }
    let tmp = path.with_extension("jsonl.tmp");
    std::fs::write(&tmp, out).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())
}

/// Re-localize a comment against the current content and, on an Anchored or
/// Drifted match, re-capture its anchor at the new position (last-seen
/// tracking): the next comparison is against this state, not the original, so a
/// run of small edits does not accumulate drift. Unresolvable leaves the anchor
/// untouched — there is no reliable position to move it to.
pub fn localize_and_update(comment: &mut Comment, content: &str) -> Localization {
    // Unchanged since the last capture — the target is exactly where it was, no
    // search needed. This is what makes `base_hash` load-bearing rather than
    // speculative provenance.
    if content_hash(content) == comment.anchor.base_hash {
        let line = comment.anchor.line_hint;
        return Localization {
            state: State::Anchored,
            span: Some((line, line)),
            confidence: 1.0,
        };
    }
    let loc = relocalize(&comment.anchor.as_anchor(), content);
    if should_reanchor(&loc) {
        if let Some((start, _end)) = loc.span {
            comment.anchor = StoredAnchor::capture(content, start, comment.anchor.ctx);
        }
    }
    loc
}

/// The minimum confidence at which a Drifted match is trusted enough to become
/// the new anchor. Below it the drift is *reported* but the anchor is not moved,
/// so a run of weak matches can never walk the anchor onto unrelated text while
/// still reporting Anchored (`telos/honest-ambiguity`).
pub const RE_ANCHOR_FLOOR: f64 = 0.85;

fn should_reanchor(loc: &Localization) -> bool {
    matches!(loc.state, State::Anchored | State::Drifted) && loc.confidence >= RE_ANCHOR_FLOOR
}

#[cfg(test)]
mod tests {
    use super::*;

    const V0: &str = "fn login(user: &str) -> bool {\n    let token = fetch_token(user);\n    validate(token)\n}";

    fn comment_at(content: &str, line: usize) -> Comment {
        Comment {
            id: "c_1".into(),
            anchor: StoredAnchor::capture(content, line, 1),
            body: "is this cached?".into(),
            author: Author {
                who: "human".into(),
                id: "local".into(),
            },
            created_at: 1_700_000_000_000_000,
            resolved: false,
            thread: Vec::new(),
        }
    }

    #[test]
    fn comment_round_trips_through_jsonl() {
        let c = comment_at(V0, 1);
        let line = serde_json::to_string(&c).unwrap();
        assert!(!line.contains('\n'), "a record must be one JSONL line");
        let back: Comment = serde_json::from_str(&line).unwrap();
        assert_eq!(back, c);
    }

    fn reply_from(id: &str, body: &str) -> Reply {
        Reply {
            author: Author {
                who: "human".into(),
                id: id.into(),
            },
            body: body.into(),
            created_at: 1,
        }
    }

    #[test]
    fn pre_threads_record_loads_with_empty_thread() {
        // (AC-1) a JSONL line written before `thread` existed has no such field.
        let json = r#"{"id":"c_1","anchor":{"target":"x","before":"","after":"","line_hint":0,"ctx":1,"base_hash":0},"body":"hi","author":{"who":"human","id":"local"},"created_at":0,"resolved":false}"#;
        let c: Comment = serde_json::from_str(json).unwrap();
        assert!(c.thread.is_empty());
        // And a comment carrying replies round-trips.
        let mut with = comment_at(V0, 1);
        with.thread = vec![reply_from("a", "one"), reply_from("b", "two")];
        let back: Comment = serde_json::from_str(&serde_json::to_string(&with).unwrap()).unwrap();
        assert_eq!(back, with);
    }

    #[test]
    fn add_reply_and_resolve_match_by_id() {
        // (AC-2) reply/resolve mutate the right comment; an unknown id is a no-op.
        let mut cs = vec![comment_at(V0, 1)]; // id "c_1"
        assert!(add_reply(
            &mut cs,
            "c_1",
            reply_from("me", "cached upstream")
        ));
        assert_eq!(cs[0].thread, vec![reply_from("me", "cached upstream")]);
        assert!(!add_reply(&mut cs, "nope", reply_from("me", "x")));
        assert!(resolve(&mut cs, "c_1"));
        assert!(cs[0].resolved);
        assert!(!resolve(&mut cs, "nope"));
        assert_eq!(cs[0].thread.len(), 1, "a failed reply must not append");
    }

    #[test]
    fn promote_helpers_build_subject_anchor_and_round_trip_block() {
        let c = comment_at(V0, 1); // id "c_1", body "is this cached?"
        assert_eq!(comment_subject("src/tui.rs"), "comment/src/tui.rs");
        assert_eq!(comment_subject("./README.md"), "comment/README.md");
        // 1-based inclusive anchor; unplaced -> bare path.
        assert_eq!(promote_anchor("src/a.rs", Some((1, 2))), "src/a.rs:2-3");
        assert_eq!(promote_anchor("src/a.rs", None), "src/a.rs");
        // The fenced block round-trips back to the same Comment.
        let text = promote_text(&c);
        assert!(text.starts_with(&c.body), "body leads the claim: {text}");
        let json = text
            .split("```cospan-comment\n")
            .nth(1)
            .and_then(|s| s.split("\n```").next())
            .expect("a cospan-comment block");
        let back: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(back, c, "the block is a lossless snapshot");
    }

    #[test]
    fn promote_argv_carries_subject_anchor_and_optional_cite() {
        let c = comment_at(V0, 1);
        let argv = promote_argv("src/a.rs", &c, Some((0, 0)), None);
        let after = |flag: &str| {
            argv.iter()
                .position(|a| a == flag)
                .map(|i| argv[i + 1].clone())
        };
        assert_eq!(argv[0], "observe");
        assert_eq!(after("--subject").as_deref(), Some("comment/src/a.rs"));
        assert_eq!(after("--file").as_deref(), Some("src/a.rs:1-1"));
        assert!(
            !argv.contains(&"--cites".to_string()),
            "no cite on first promote"
        );
        // The text is the trailing positional, guarded by `--` so a dash-led body
        // is never read as a flag.
        assert_eq!(
            argv[argv.len() - 2],
            "--",
            "text is guarded by a -- separator"
        );
        assert!(argv[argv.len() - 1].contains("cospan-comment"));
        // A re-promote cites the prior claim, still before the `--`.
        let re = promote_argv("src/a.rs", &c, Some((0, 0)), Some("bafyPRIOR"));
        let i = re
            .iter()
            .position(|a| a == "--cites")
            .expect("cites on re-promote");
        assert_eq!(re[i + 1], "bafyPRIOR");
        assert!(
            re.iter().position(|a| a == "--").unwrap() > i,
            "the cite flag is before the -- separator"
        );
    }

    #[test]
    fn promote_argv_guards_a_dash_led_body_behind_the_separator() {
        // A markdown-bullet body (`- ...`) or a `--help`-shaped body must land as
        // the trailing positional after `--`, never parsed as a flag.
        for body in ["- should this be cached?", "--help", "-x"] {
            let mut c = comment_at(V0, 1);
            c.body = body.into();
            let argv = promote_argv("src/a.rs", &c, Some((0, 0)), None);
            let sep = argv.iter().position(|a| a == "--").expect("a -- separator");
            assert_eq!(sep, argv.len() - 2, "-- immediately precedes the text");
            assert!(
                argv[sep + 1].starts_with(body),
                "the dash-led body is the trailing positional: {:?}",
                argv[sep + 1]
            );
        }
    }

    #[test]
    fn set_resolved_toggles_both_ways() {
        // (AC-3) the interactive toggle can un-resolve, unlike `resolve`.
        let mut cs = vec![comment_at(V0, 1)];
        assert!(set_resolved(&mut cs, "c_1", true));
        assert!(cs[0].resolved);
        assert!(set_resolved(&mut cs, "c_1", false));
        assert!(!cs[0].resolved, "un-resolve must clear the flag");
        assert!(!set_resolved(&mut cs, "nope", true));
    }

    fn owned_by(id: &str) -> Comment {
        let mut c = comment_at(V0, 1);
        c.author = Author {
            who: "human".into(),
            id: id.into(),
        };
        c
    }

    #[test]
    fn edit_body_rewrites_and_reanchors_own_but_refuses_others() {
        // (AC-4) an author edits their own comment: the body changes and the
        // anchor re-captures against the current content; a foreign comment is
        // Forbidden and left untouched.
        let mut cs = vec![owned_by("alice")];
        // Edit against shifted content so the re-anchor is observable.
        let shifted = format!("// pushed down\n{V0}");
        assert_eq!(
            edit_body(&mut cs, "c_1", "alice", "now cached?", &shifted),
            Mutation::Applied
        );
        assert_eq!(cs[0].body, "now cached?");
        assert_eq!(
            cs[0].anchor.base_hash,
            content_hash(&shifted),
            "the anchor must re-capture against the edited-against content"
        );

        // Bob cannot rewrite Alice's comment.
        assert_eq!(
            edit_body(&mut cs, "c_1", "bob", "hijacked", V0),
            Mutation::Forbidden
        );
        assert_eq!(cs[0].body, "now cached?", "a forbidden edit must not apply");
        assert_eq!(
            edit_body(&mut cs, "nope", "alice", "x", V0),
            Mutation::NotFound
        );
    }

    #[test]
    fn delete_removes_own_but_refuses_others() {
        // (AC-5) deleting your own comment removes exactly it; another author's is
        // Forbidden and stays.
        let mut cs = vec![owned_by("alice"), {
            let mut b = owned_by("bob");
            b.id = "c_2".into();
            b
        }];
        assert_eq!(delete(&mut cs, "c_2", "alice"), Mutation::Forbidden);
        assert_eq!(cs.len(), 2, "a forbidden delete must not remove anything");
        assert_eq!(delete(&mut cs, "c_1", "alice"), Mutation::Applied);
        assert_eq!(cs.len(), 1);
        assert_eq!(
            cs[0].id, "c_2",
            "the surviving comment is the other author's"
        );
        assert_eq!(delete(&mut cs, "gone", "bob"), Mutation::NotFound);
    }

    #[test]
    fn thread_summary_names_reply_count_and_resolved() {
        // (AC-4) the list suffix reflects replies and resolved state.
        let mut c = comment_at(V0, 1);
        assert_eq!(thread_summary(&c), "");
        c.thread.push(reply_from("a", "one"));
        assert_eq!(thread_summary(&c), "(1 reply)");
        c.thread.push(reply_from("b", "two"));
        c.resolved = true;
        assert_eq!(thread_summary(&c), "(2 replies) [resolved]");
    }

    #[test]
    fn sidecar_path_maps_under_the_tree() {
        assert_eq!(
            sidecar_path("src/main.rs"),
            Path::new(".cospan/comments/src/main.rs.jsonl")
        );
        assert_eq!(
            sidecar_path("./README.md"),
            Path::new(".cospan/comments/README.md.jsonl")
        );
    }

    #[test]
    fn fresh_comment_anchors_on_unchanged_content() {
        let mut c = comment_at(V0, 1); // the `let token` line
        let loc = localize_and_update(&mut c, V0);
        assert_eq!(loc.state, State::Anchored);
        assert_eq!(loc.span, Some((1, 1)));
    }

    #[test]
    fn round_trip_line_shift_reanchors_and_delete_is_unresolvable() {
        // Pure line-shift: prepend two lines. The target moves down two lines and
        // re-anchors there.
        let mut c = comment_at(V0, 1);
        let shifted = format!("// docs\n// more\n{V0}");
        let loc = localize_and_update(&mut c, &shifted);
        assert_eq!(loc.state, State::Anchored);
        assert_eq!(loc.span, Some((3, 3)));
        assert_eq!(c.anchor.line_hint, 3); // anchor followed the target
        assert_eq!(c.anchor.base_hash, content_hash(&shifted));

        // Target deleted entirely: Unresolvable, and the anchor is left as-is.
        let mut c2 = comment_at(V0, 1);
        let before = c2.anchor.clone();
        let gone = "fn unrelated() {\n    println!(\"hi\");\n}";
        let loc = localize_and_update(&mut c2, gone);
        assert_eq!(loc.state, State::Unresolvable);
        assert_eq!(c2.anchor, before);
    }

    #[test]
    fn incremental_tracking_updates_last_seen_across_edits() {
        // Two successive shifts. After the first, the anchor tracks the new
        // contents, so the second is measured from there, not the original.
        let mut c = comment_at(V0, 1);
        let s1 = format!("// a\n{V0}");
        localize_and_update(&mut c, &s1);
        assert_eq!(c.anchor.base_hash, content_hash(&s1));
        assert_eq!(c.anchor.line_hint, 2);

        let s2 = format!("// a\n// b\n{V0}");
        let loc = localize_and_update(&mut c, &s2);
        assert_eq!(loc.state, State::Anchored);
        assert_eq!(loc.span, Some((3, 3)));
        assert_eq!(c.anchor.base_hash, content_hash(&s2));
    }

    #[test]
    fn reanchor_only_follows_strong_matches() {
        let mk = |state, confidence| Localization {
            state,
            span: Some((0, 0)),
            confidence,
        };
        assert!(should_reanchor(&mk(State::Anchored, 1.0)));
        assert!(should_reanchor(&mk(State::Drifted, 0.90)));
        // A weak drift is reported but must not become the new ground truth.
        assert!(!should_reanchor(&mk(State::Drifted, 0.64)));
        // Ambiguous/lost never re-anchors, whatever the score.
        assert!(!should_reanchor(&mk(State::Unresolvable, 0.99)));
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("cospan-{}-{}", std::process::id(), tag));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn load_skips_a_malformed_line_without_losing_the_rest() {
        let dir = temp_dir("load");
        let path = dir.join("s.jsonl");
        let good = serde_json::to_string(&comment_at(V0, 1)).unwrap();
        std::fs::write(&path, format!("{good}\nthis is not json\n")).unwrap();
        let loaded = load(&path).unwrap();
        assert_eq!(
            loaded.len(),
            1,
            "the readable comment must survive a torn line"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn save_then_load_round_trips_through_the_sidecar() {
        let dir = temp_dir("save");
        let path = dir.join("s.jsonl");
        let cs = vec![comment_at(V0, 1), comment_at(V0, 2)];
        save(&path, &cs).unwrap();
        assert_eq!(load(&path).unwrap(), cs);
        std::fs::remove_dir_all(&dir).ok();
    }
}
