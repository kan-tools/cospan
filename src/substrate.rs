//! P0 substrate reads — the watch-and-fold spine.
//!
//! Shells out to the real `kan` and `day` binaries and folds their output into a
//! small in-memory model (L2) for rendering. Shelling (not linking the `kan`
//! crate) is the deliberate P0 choice: it proves the spine against real repos with
//! zero build-coupling. The library upgrade for the hot read path comes later —
//! see `.dropbox/02-kan-day-integration.md`.
//!
//! Everything here is poll-driven: the caller watches `.kan/log/HEAD` and asks for
//! a fresh `Dashboard` only when it changes.

use serde_json::Value;
use std::path::Path;
use std::process::Command;

/// One kan subject as it appears in the cheap `kan status --json` manifest.
#[derive(Clone, Debug)]
pub struct Subject {
    pub name: String,
    pub state: String,
    pub durability: String,
}

impl Subject {
    /// The top-level namespace, e.g. `atom/foo` -> "atom", used for grouping.
    pub fn namespace(&self) -> &str {
        // agents/handoff/* is really one logical group; collapse it.
        if self.name.starts_with("agents/handoff/") {
            return "agents/handoff";
        }
        self.name.split('/').next().unwrap_or(&self.name)
    }
}

/// A folded snapshot of a repo's substrate at one tick.
#[derive(Clone, Debug, Default)]
pub struct Dashboard {
    pub subjects: Vec<Subject>,
    /// The `day status` process-position text (rendered verbatim — it already
    /// expresses ambiguity honestly, and re-implementing its inference would be a
    /// mistake). `None` if `day` is unavailable.
    pub day_status: Option<String>,
    pub errors: Vec<String>,
}

impl Dashboard {
    /// Sessions = the flat `agents/handoff/*` registry (day's real multi-agent
    /// surface; there is no parent/child hierarchy yet — see 02).
    pub fn sessions(&self) -> Vec<&Subject> {
        self.subjects
            .iter()
            .filter(|s| s.name.starts_with("agents/handoff/"))
            .collect()
    }

    /// (namespace, count) pairs, most-populous first.
    pub fn namespace_counts(&self) -> Vec<(String, usize)> {
        let mut map: std::collections::BTreeMap<String, usize> = Default::default();
        for s in &self.subjects {
            *map.entry(s.namespace().to_string()).or_default() += 1;
        }
        let mut v: Vec<_> = map.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v
    }
}

/// Collect a fresh dashboard by querying kan + day in `repo`.
pub fn collect(repo: &Path) -> Dashboard {
    let mut dash = Dashboard::default();

    match kan_status(repo) {
        Ok(subjects) => dash.subjects = subjects,
        Err(e) => dash.errors.push(format!("kan status: {e}")),
    }
    match day_status(repo) {
        Ok(text) => dash.day_status = Some(text),
        Err(e) => dash.errors.push(format!("day status: {e}")),
    }
    dash
}

fn kan_status(repo: &Path) -> Result<Vec<Subject>, String> {
    let out = Command::new("kan")
        .args(["status", "--json"])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("spawn failed: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    let json: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    let arr = json
        .get("subjects")
        .and_then(Value::as_array)
        .ok_or("no `subjects` array")?;
    let mut subjects: Vec<Subject> = arr
        .iter()
        .map(|s| Subject {
            name: str_at(s, "subject"),
            state: str_at(s, "state"),
            durability: str_at(s, "durability"),
        })
        .collect();
    subjects.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(subjects)
}

fn day_status(repo: &Path) -> Result<String, String> {
    let out = Command::new("day")
        .arg("status")
        .current_dir(repo)
        .output()
        .map_err(|e| format!("spawn failed: {e}"))?;
    // `day status` always exits zero by design; take stdout regardless.
    Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
}

fn str_at(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}
