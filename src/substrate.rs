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
use std::cmp::Ordering;
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

fn opt_str_at(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::to_string)
}

// --- Per-subject claim drill-in ---------------------------------------------

/// One live claim on a subject, as `kan show <subject> --json` emits it.
///
/// The narrative payload is kind-dependent: Decision/Observation/Plan/Result
/// carry `text`, a Subject claim carries `title`, and some (e.g. Publication)
/// carry neither — hence both are optional.
#[derive(Clone, Debug)]
pub struct Claim {
    pub cid: String,
    pub kind: String,
    pub subject: String,
    pub author: String,
    /// Microseconds since the Unix epoch (kan's `recorded_at`); `None` when the
    /// claim carries no timestamp, so an undated claim is never given a
    /// real-looking date it does not have.
    pub recorded_at: Option<i64>,
    pub text: Option<String>,
    pub title: Option<String>,
}

impl Claim {
    /// The author with kan's `did:key:` prefix stripped and truncated — enough
    /// to tell signers apart without spending a line on a full key.
    pub fn short_author(&self) -> String {
        let a = self.author.strip_prefix("did:key:").unwrap_or(&self.author);
        a.chars().take(8).collect()
    }

    /// A one-line human summary: the first non-empty line of `text`, else the
    /// quoted `title`, else a kind label — so a payload-less claim is never
    /// rendered blank.
    pub fn summary(&self) -> String {
        if let Some(line) = self
            .text
            .as_deref()
            .and_then(|t| t.lines().find(|l| !l.trim().is_empty()))
        {
            return line.trim().to_string();
        }
        if let Some(title) = &self.title {
            return format!("\"{title}\"");
        }
        format!("({})", self.kind.to_lowercase())
    }

    /// `recorded_at` as a compact UTC stamp `YYYY-MM-DD HH:MM`, or `date unknown`
    /// when the claim carries no timestamp — an absent date is surfaced, never
    /// rendered as a real one (telos/honest-ambiguity).
    pub fn recorded_utc(&self) -> String {
        match self.recorded_at {
            Some(us) => fmt_utc(us),
            None => "date unknown".to_string(),
        }
    }

    /// One-line rendering: kind, short author, UTC stamp, summary. Shared by the
    /// `cospan subject` CLI and the TUI detail pane so the two never drift.
    pub fn display_line(&self) -> String {
        format!(
            "{:<11} {:<8}  {:<16}  {}",
            self.kind,
            self.short_author(),
            self.recorded_utc(),
            self.summary()
        )
    }
}

/// Fold one subject's live claims, newest first.
///
/// Shells `kan show <subject> --json` in `repo` (mirroring `kan_status`) and
/// returns kan's live-claim set verbatim: retracted and trust-excluded claims
/// are already dropped by kan's fold, so cospan re-derives nothing — the direct
/// expression of `telos/kan-is-truth` at the fold layer. Ordered newest-first
/// by `recorded_at`, tie-broken by `cid` (kan 0.13 emits no `rev`).
pub fn subject_claims(repo: &Path, subject: &str) -> Result<Vec<Claim>, String> {
    let out = Command::new("kan")
        .args(["show", subject, "--json"])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("spawn failed: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    let json: Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    Ok(claims_from_json(&json))
}

/// Parse the **top-level** `claims` array of a `kan show --json` payload and
/// order it newest-first. Split out from `subject_claims` so it can be unit
/// tested against a captured payload without shelling out.
fn claims_from_json(json: &Value) -> Vec<Claim> {
    let mut claims: Vec<Claim> = json
        .get("claims")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().map(claim_from_value).collect())
        .unwrap_or_default();
    // Newest first; undated claims sort last (they cannot be placed in time);
    // ties broken by cid so the order is fully deterministic (kan emits no `rev`).
    claims.sort_by(|a, b| match (a.recorded_at, b.recorded_at) {
        (Some(at), Some(bt)) => bt.cmp(&at).then_with(|| a.cid.cmp(&b.cid)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cid.cmp(&b.cid),
    });
    claims
}

fn claim_from_value(v: &Value) -> Claim {
    Claim {
        cid: str_at(v, "cid"),
        kind: str_at(v, "kind"),
        subject: str_at(v, "subject"),
        author: str_at(v, "author"),
        recorded_at: v.get("recorded_at").and_then(Value::as_i64),
        text: opt_str_at(v, "text"),
        title: opt_str_at(v, "title"),
    }
}

/// Format microseconds-since-epoch as a compact UTC stamp `YYYY-MM-DD HH:MM`.
///
/// Integer-only (Howard Hinnant's civil-from-days) so the P0 spine stays
/// dependency-free — a calendar crate is not worth pulling in for one column.
fn fmt_utc(micros: i64) -> String {
    let secs = micros.div_euclid(1_000_000);
    let days = secs.div_euclid(86_400);
    let tod = secs.rem_euclid(86_400);
    let (hour, min) = (tod / 3600, (tod % 3600) / 60);

    // civil_from_days: days since 1970-01-01 -> (year, month, day), UTC.
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let day = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let month = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };

    format!("{year:04}-{month:02}-{day:02} {hour:02}:{min:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    // A trimmed `kan show <subject> --json` payload: three kinds, and
    // recorded_at deliberately out of order so the sort is exercised.
    const SHOW_JSON: &str = r#"{
      "v": 1,
      "subject": "telos/demo",
      "subjects": ["telos/demo"],
      "claims": [
        {"cid": "bafyOLD", "kind": "Decision", "subject": "telos/demo",
         "author": "did:key:zAAAAAAAAA", "recorded_at": 100,
         "text": "\nfirst declaration\nmore"},
        {"cid": "bafyNEW", "kind": "Decision", "subject": "telos/demo",
         "author": "did:key:zAAAAAAAAA", "recorded_at": 300,
         "text": "revised declaration"},
        {"cid": "bafyMID", "kind": "Subject", "subject": "telos/demo",
         "author": "did:key:zAAAAAAAAA", "recorded_at": 200,
         "title": "Demo telos"},
        {"cid": "bafyPUB", "kind": "Publication", "subject": "telos/demo",
         "author": "did:key:zAAAAAAAAA", "recorded_at": 250}
      ]
    }"#;

    fn parsed() -> Vec<Claim> {
        let json: Value = serde_json::from_str(SHOW_JSON).unwrap();
        claims_from_json(&json)
    }

    #[test]
    fn parses_and_orders_newest_first() {
        let claims = parsed();
        assert_eq!(claims.len(), 4);
        let order: Vec<&str> = claims.iter().map(|c| c.cid.as_str()).collect();
        assert_eq!(order, ["bafyNEW", "bafyPUB", "bafyMID", "bafyOLD"]);
        assert_eq!(claims[0].kind, "Decision");
        assert_eq!(claims[2].kind, "Subject");
    }

    #[test]
    fn summary_is_kind_aware_and_never_blank() {
        let claims = parsed();
        // Decision -> first non-empty line of text (leading blank line skipped).
        assert_eq!(claims[0].summary(), "revised declaration");
        let old = claims.iter().find(|c| c.cid == "bafyOLD").unwrap();
        assert_eq!(old.summary(), "first declaration");
        // Subject -> quoted title.
        let subj = claims.iter().find(|c| c.kind == "Subject").unwrap();
        assert_eq!(subj.summary(), "\"Demo telos\"");
        // Publication -> non-empty kind label.
        let pubc = claims.iter().find(|c| c.kind == "Publication").unwrap();
        assert_eq!(pubc.summary(), "(publication)");
    }

    #[test]
    fn short_author_strips_and_truncates() {
        assert_eq!(parsed()[0].short_author(), "zAAAAAAA");
    }

    #[test]
    fn utc_matches_kans_own_render() {
        // 1787091237989445 µs is what kan recorded for a claim it shows as
        // 2026-08-18T22:13:57Z.
        assert_eq!(fmt_utc(1_787_091_237_989_445), "2026-08-18 22:13");
    }

    #[test]
    fn equal_timestamps_break_by_cid_deterministically() {
        // Both at recorded_at 500: the tie-break, not the timestamp, decides —
        // this is the branch RQ-1's determinism guarantee rests on.
        let json: Value = serde_json::from_str(
            r#"{"claims":[
              {"cid":"bafyB","kind":"Decision","subject":"s","author":"did:key:z","recorded_at":500,"text":"b"},
              {"cid":"bafyA","kind":"Decision","subject":"s","author":"did:key:z","recorded_at":500,"text":"a"}
            ]}"#,
        )
        .unwrap();
        let order: Vec<String> = claims_from_json(&json).into_iter().map(|c| c.cid).collect();
        assert_eq!(order, ["bafyA", "bafyB"]);
    }

    #[test]
    fn undated_claim_sorts_last_and_renders_unknown() {
        // A claim with no recorded_at must not be handed a fake 1970 date; it
        // sorts after every dated claim and renders its date as unknown.
        let json: Value = serde_json::from_str(
            r#"{"claims":[
              {"cid":"bafyDATED","kind":"Decision","subject":"s","author":"did:key:z","recorded_at":100,"text":"dated"},
              {"cid":"bafyNODATE","kind":"Decision","subject":"s","author":"did:key:z","text":"undated"}
            ]}"#,
        )
        .unwrap();
        let claims = claims_from_json(&json);
        let last = claims.last().unwrap();
        assert_eq!(last.cid, "bafyNODATE");
        assert_eq!(last.recorded_at, None);
        assert_eq!(last.recorded_utc(), "date unknown");
    }

    #[test]
    fn display_line_has_kind_author_utc_and_summary() {
        let c = Claim {
            cid: "bafyX".into(),
            kind: "Decision".into(),
            subject: "telos/a".into(),
            author: "did:key:zABCDEFGH".into(),
            recorded_at: Some(1_787_091_237_989_445),
            text: Some("hello world\nmore".into()),
            title: None,
        };
        let line = c.display_line();
        assert!(line.contains("Decision"));
        assert!(line.contains("zABCDEFG")); // short author, did:key: stripped
        assert!(line.contains("2026-08-18 22:13"));
        assert!(line.contains("hello world")); // first line of text
    }

    #[test]
    fn fmt_utc_handles_epoch_leapday_and_negative() {
        assert_eq!(fmt_utc(0), "1970-01-01 00:00");
        // 2024-02-29T00:00:00Z — a leap day — is 1_709_164_800 s.
        assert_eq!(fmt_utc(1_709_164_800_000_000), "2024-02-29 00:00");
        // One microsecond before the epoch floors into the previous day.
        assert_eq!(fmt_utc(-1), "1969-12-31 23:59");
    }
}
