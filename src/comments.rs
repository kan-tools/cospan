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

/// One sidecar comment. Ephemeral, re-localized every read; `resolved` and the
/// (currently unused) thread live here so persist-to-kan can map 1:1 later.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub anchor: StoredAnchor,
    pub body: String,
    pub author: Author,
    /// Microseconds since the Unix epoch.
    pub created_at: i64,
    pub resolved: bool,
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
