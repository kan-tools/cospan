//! Working-tree diff of a single file (vs `HEAD`), for the editor pane's live
//! change highlighting. A projection of git, like the file list — nothing here is
//! persisted, and a non-git or clean file degrades to an empty diff rather than
//! an error (`telos/honest-ambiguity`, `telos/kan-is-truth`).

use crate::filetree::GitStatus;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

/// Which current-file (new-file) lines changed, keyed on 0-based line index so it
/// lines up with the `i` that `gutter_lines` iterates. `added` lines are pure
/// insertions; `changed` lines are the added side of a modification (a removal
/// paired with an insertion); `deletions` maps the line that *follows* a removed
/// block to how many lines were removed there — a boundary marker, since removed
/// content has no line in the current file.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FileDiff {
    pub added: BTreeSet<usize>,
    pub changed: BTreeSet<usize>,
    pub deletions: BTreeMap<usize, usize>,
}

impl FileDiff {
    pub fn empty() -> Self {
        Self::default()
    }

    /// Every line is an addition — the shape for an untracked file, which has no
    /// `HEAD` blob to diff against.
    pub fn all_added(line_count: usize) -> Self {
        FileDiff {
            added: (0..line_count).collect(),
            changed: BTreeSet::new(),
            deletions: BTreeMap::new(),
        }
    }

    /// True when there is nothing to show.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.changed.is_empty() && self.deletions.is_empty()
    }

    /// Compute the working-tree diff for `rel` in `repo`. An untracked file maps
    /// every line to `added`; anything git cannot answer (no HEAD, not a repo, a
    /// clean file) is an empty diff.
    pub fn compute(repo: &Path, rel: &Path, status: GitStatus, line_count: usize) -> FileDiff {
        if status == GitStatus::Untracked {
            return FileDiff::all_added(line_count);
        }
        match git_diff(repo, rel) {
            Some(text) => FileDiff::parse(&text),
            None => FileDiff::empty(),
        }
    }

    /// Parse unified `git diff` text into new-file line classifications. A simple
    /// state machine over the hunk bodies: a `+` with a pending removal is a
    /// modification (`changed`), a `+` with none is an insertion (`added`), and a
    /// run of `-` lines left unpaired at the next context line (or hunk end) is a
    /// deletion recorded at the following line.
    pub fn parse(diff: &str) -> FileDiff {
        let mut out = FileDiff::default();
        let mut new_line: usize = 0; // 0-based index of the next new-file line
        let mut pending_removed: usize = 0;

        let flush_deletion = |out: &mut FileDiff, at: usize, pending: &mut usize| {
            if *pending > 0 {
                *out.deletions.entry(at).or_insert(0) += *pending;
                *pending = 0;
            }
        };

        // Only dispatch on the leading byte *inside* a hunk. The file headers
        // (`diff --git`, `index`, `--- a/…`, `+++ b/…`) all precede the first `@@`;
        // guarding on them by prefix would mis-handle a hunk-body line whose own
        // content starts with `--` or `++` (a SQL comment, a YAML `---`, `++i`).
        let mut in_hunk = false;
        for line in diff.lines() {
            if line.starts_with("diff --git") {
                // A new file section (only reachable if a caller diffs >1 file).
                flush_deletion(&mut out, new_line, &mut pending_removed);
                in_hunk = false;
                continue;
            }
            if let Some(rest) = line.strip_prefix("@@") {
                // Any pending removals belonged to the previous hunk's tail.
                flush_deletion(&mut out, new_line, &mut pending_removed);
                if let Some(start) = parse_hunk_new_start(rest) {
                    new_line = start;
                }
                in_hunk = true;
                continue;
            }
            if !in_hunk {
                continue; // pre-hunk headers, including `--- a/…` / `+++ b/…`
            }
            match line.as_bytes().first() {
                Some(b'+') => {
                    if pending_removed > 0 {
                        out.changed.insert(new_line);
                        pending_removed -= 1;
                    } else {
                        out.added.insert(new_line);
                    }
                    new_line += 1;
                }
                Some(b'-') => {
                    pending_removed += 1;
                }
                Some(b'\\') => {} // "\ No newline at end of file"
                _ => {
                    // Context (or a blank body line): any unpaired removals end here.
                    flush_deletion(&mut out, new_line, &mut pending_removed);
                    new_line += 1;
                }
            }
        }
        flush_deletion(&mut out, new_line, &mut pending_removed);
        out
    }
}

/// New-file start line (0-based) from a hunk header tail like ` -a,b +c,d @@ ...`.
fn parse_hunk_new_start(rest: &str) -> Option<usize> {
    let plus = rest.split('+').nth(1)?; // "c,d @@ ..."
    let num: String = plus.chars().take_while(|c| c.is_ascii_digit()).collect();
    let start: usize = num.parse().ok()?;
    Some(start.saturating_sub(1)) // 1-based -> 0-based
}

/// `git diff HEAD -- <rel>`, tolerant of a non-zero exit (→ None). Mirrors the
/// `filetree::git` invocation (quotepath off so non-ASCII paths stay verbatim).
fn git_diff(repo: &Path, rel: &Path) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["-c", "core.quotepath=false"])
        .args(["diff", "HEAD", "--"])
        .arg(rel)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_add_modify_and_delete_hunks() {
        // Add-only: lines 1,2 (0-based) inserted after context line 0.
        let d = "@@ -1,1 +1,3 @@\n ctx\n+add1\n+add2\n";
        let fd = FileDiff::parse(d);
        assert_eq!(fd.added, [1, 2].into_iter().collect());
        assert!(fd.changed.is_empty());
        assert!(fd.deletions.is_empty());

        // Modify: one removal paired with one insertion at line 0 -> changed.
        let d = "@@ -1,2 +1,2 @@\n-old\n+new\n ctx\n";
        let fd = FileDiff::parse(d);
        assert_eq!(fd.changed, [0].into_iter().collect());
        assert!(fd.added.is_empty());
        assert!(fd.deletions.is_empty());

        // Pure deletion: two lines removed between new lines 0 and 1 -> marked at 1.
        let d = "@@ -1,3 +1,1 @@\n ctx\n-r1\n-r2\n ctx2\n";
        let fd = FileDiff::parse(d);
        assert!(fd.added.is_empty());
        assert!(fd.changed.is_empty());
        assert_eq!(fd.deletions.get(&1), Some(&2));
    }

    #[test]
    fn net_removal_marks_a_boundary_and_keeps_the_change() {
        // Two removed, one added at line 0: the add is a change; the extra removal
        // is a deletion boundary at the following line.
        let d = "@@ -1,3 +1,2 @@\n-old1\n-old2\n+new\n ctx\n";
        let fd = FileDiff::parse(d);
        assert_eq!(fd.changed, [0].into_iter().collect());
        assert_eq!(fd.deletions.get(&1), Some(&1));
    }

    #[test]
    fn untracked_is_all_added_and_empty_is_empty() {
        assert_eq!(
            FileDiff::all_added(3).added,
            [0, 1, 2].into_iter().collect()
        );
        assert!(FileDiff::parse("").is_empty());
        assert!(FileDiff::empty().is_empty());
    }

    #[test]
    fn hunk_body_content_starting_with_plus_or_minus_is_not_a_header() {
        // Regression: a hunk-body line whose content starts with `++`/`--` (a SQL
        // comment, a YAML `---`, `++i`) must not be swallowed by a +++/--- header
        // guard, which would drop it and cascade an off-by-one down the hunk.
        let fd = FileDiff::parse("@@ -1,1 +1,3 @@\n ctx\n+++b\n+x\n");
        assert_eq!(
            fd.added,
            [1, 2].into_iter().collect(),
            "`++`-content is an add"
        );

        let fd = FileDiff::parse("@@ -1,2 +1,2 @@\n--- drop table\n+kept\n ctx\n");
        assert_eq!(
            fd.changed,
            [0].into_iter().collect(),
            "`--`-content is a removal"
        );
        assert!(fd.added.is_empty());

        let fd = FileDiff::parse("@@ -1,3 +1,2 @@\n ctx\n----\n ctx2\n");
        assert_eq!(
            fd.deletions.get(&1),
            Some(&1),
            "a deleted `----` line still marks"
        );
    }

    #[test]
    fn compute_reads_a_real_git_working_tree() {
        use std::fs;
        let dir = std::env::temp_dir().join(format!("cospan-diff-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let git = |args: &[&str]| {
            Command::new("git")
                .arg("-C")
                .arg(&dir)
                .args([
                    "-c",
                    "commit.gpgsign=false",
                    "-c",
                    "user.email=t@t",
                    "-c",
                    "user.name=t",
                ])
                .args(args)
                .output()
                .unwrap()
        };
        git(&["init", "-q"]);
        fs::write(dir.join("f.txt"), "a\nb\nc\n").unwrap();
        git(&["add", "f.txt"]);
        git(&["commit", "-qm", "init"]);
        // Modify line 1 (b -> B) and append a line.
        fs::write(dir.join("f.txt"), "a\nB\nc\nd\n").unwrap();
        let fd = FileDiff::compute(&dir, Path::new("f.txt"), GitStatus::Modified, 4);
        assert!(fd.changed.contains(&1), "line 1 (b->B) is a change: {fd:?}");
        assert!(fd.added.contains(&3), "appended line 3 is an add: {fd:?}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn second_hunk_resets_the_line_counter() {
        let d = "@@ -1,1 +1,2 @@\n ctx\n+a\n@@ -10,1 +11,2 @@\n ctx\n+b\n";
        let fd = FileDiff::parse(d);
        // First hunk adds new line 1; second hunk starts at new line 10, ctx at 10,
        // add at 11.
        assert_eq!(fd.added, [1, 11].into_iter().collect());
    }
}
