//! The repo file list with git working-tree status (S2).
//!
//! Enumerates the files worth browsing — tracked files plus untracked-not-ignored
//! ones — by shelling `git` (consistent with cospan's shell-out spine, and it
//! respects `.gitignore` for free) and tags each with its working-tree status.
//! No walker, no watch thread: the caller polls this on the same mtime gate as
//! the rest of the model (`telos/poll-dont-subscribe`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A file's working-tree status, as far as the browser cares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitStatus {
    Clean,
    Modified,
    Added,
    Untracked,
    Deleted,
}

/// One browsable file: its repo-relative path and working-tree status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileEntry {
    pub path: PathBuf,
    pub status: GitStatus,
}

/// A one-char marker for a status (blank for clean), for the rail gutter.
pub fn marker(s: GitStatus) -> char {
    match s {
        GitStatus::Clean => ' ',
        GitStatus::Modified => 'M',
        GitStatus::Added => 'A',
        GitStatus::Untracked => '?',
        GitStatus::Deleted => 'D',
    }
}

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout).ok()
}

/// Parse a `git status --porcelain` line into `(path, status)`. Handles the
/// rename form `R  old -> new` by taking the destination. Returns `None` for a
/// line too short to carry a path.
fn parse_status_line(line: &str) -> Option<(String, GitStatus)> {
    if line.len() < 4 {
        return None;
    }
    let code = &line[..2];
    let rest = &line[3..];
    // A rename/copy names `old -> new`; the browsable path is the destination.
    let path = rest.rsplit(" -> ").next().unwrap_or(rest).to_string();
    let status = if code == "??" {
        GitStatus::Untracked
    } else if code.contains('D') {
        GitStatus::Deleted
    } else if code.contains('A') || code.contains('R') || code.contains('C') {
        GitStatus::Added
    } else {
        GitStatus::Modified
    };
    Some((path, status))
}

/// Every browsable file in `repo` (tracked ∪ untracked-not-ignored), each tagged
/// with its working-tree status, sorted by path. Empty if `git` is unavailable or
/// this is not a git repo.
pub fn list(repo: &Path) -> Vec<FileEntry> {
    let mut status: BTreeMap<String, GitStatus> = BTreeMap::new();
    if let Some(porcelain) = git(repo, &["status", "--porcelain"]) {
        for line in porcelain.lines() {
            if let Some((path, st)) = parse_status_line(line) {
                status.insert(path, st);
            }
        }
    }
    // Tracked files are the browsable base; the porcelain map colors them and adds
    // the untracked ones. A tracked file absent from the map is Clean.
    let mut paths: BTreeMap<String, GitStatus> = BTreeMap::new();
    if let Some(tracked) = git(repo, &["ls-files"]) {
        for p in tracked.lines() {
            if !p.is_empty() {
                paths.insert(p.to_string(), GitStatus::Clean);
            }
        }
    }
    for (p, st) in status {
        // A `Deleted` file no longer exists on disk — drop it from the browsable
        // list rather than offering a file that cannot be opened.
        if st == GitStatus::Deleted {
            paths.remove(&p);
        } else {
            paths.insert(p, st);
        }
    }
    paths
        .into_iter()
        .map(|(p, status)| FileEntry {
            path: PathBuf::from(p),
            status,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_porcelain_codes_into_statuses() {
        assert_eq!(
            parse_status_line("?? new.rs"),
            Some(("new.rs".into(), GitStatus::Untracked))
        );
        assert_eq!(
            parse_status_line(" M src/a.rs"),
            Some(("src/a.rs".into(), GitStatus::Modified))
        );
        assert_eq!(
            parse_status_line("A  added.rs"),
            Some(("added.rs".into(), GitStatus::Added))
        );
        assert_eq!(
            parse_status_line(" D gone.rs"),
            Some(("gone.rs".into(), GitStatus::Deleted))
        );
        assert_eq!(
            parse_status_line("R  old.rs -> new.rs"),
            Some(("new.rs".into(), GitStatus::Added)),
            "a rename is browsed at its destination"
        );
        assert_eq!(parse_status_line("x"), None);
    }

    #[test]
    fn lists_tracked_and_untracked_with_status_over_a_real_repo() {
        // A throwaway git repo: one committed clean file, one modified, one
        // untracked; an ignored file must not appear.
        let dir = std::env::temp_dir().join(format!("cospan-ft-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let run = |args: &[&str]| {
            Command::new("git")
                .arg("-C")
                .arg(&dir)
                .args(args)
                .output()
                .unwrap();
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "t@t"]);
        run(&["config", "user.name", "t"]);
        std::fs::write(dir.join("clean.txt"), "hi\n").unwrap();
        std::fs::write(dir.join("mod.txt"), "one\n").unwrap();
        std::fs::write(dir.join(".gitignore"), "ignored.txt\n").unwrap();
        run(&["add", "clean.txt", "mod.txt", ".gitignore"]);
        run(&["commit", "-qm", "init"]);
        std::fs::write(dir.join("mod.txt"), "one\ntwo\n").unwrap(); // now modified
        std::fs::write(dir.join("new.txt"), "fresh\n").unwrap(); // untracked
        std::fs::write(dir.join("ignored.txt"), "nope\n").unwrap(); // ignored

        let entries = list(&dir);
        let by = |name: &str| entries.iter().find(|e| e.path.as_path() == Path::new(name));
        assert_eq!(by("clean.txt").map(|e| e.status), Some(GitStatus::Clean));
        assert_eq!(by("mod.txt").map(|e| e.status), Some(GitStatus::Modified));
        assert_eq!(by("new.txt").map(|e| e.status), Some(GitStatus::Untracked));
        assert!(
            by("ignored.txt").is_none(),
            "ignored files are not browsable"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
