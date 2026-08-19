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
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

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
    /// Artifact anchors as kan renders them, e.g. `Commit("…")`.
    pub artifacts: Vec<String>,
    /// CIDs this claim cites (empty when it cites nothing).
    pub cites: Vec<String>,
    /// The CID this claim supersedes/retracts, if any (e.g. a `Retraction`).
    pub supersedes: Option<String>,
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
        // A retraction has no text of its own; show what it acted on.
        if self.kind == "Retraction" {
            if let Some(target) = &self.supersedes {
                return format!("retracts {}", short_cid(target));
            }
        }
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
    sort_newest_first(&mut claims);
    claims
}

/// Newest first; undated claims sort last (they cannot be placed in time); ties
/// broken by cid so the order is fully deterministic (kan emits no `rev`).
fn sort_newest_first(claims: &mut [Claim]) {
    claims.sort_by(|a, b| match (a.recorded_at, b.recorded_at) {
        (Some(at), Some(bt)) => bt.cmp(&at).then_with(|| a.cid.cmp(&b.cid)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cid.cmp(&b.cid),
    });
}

/// The top-level namespace of a subject name, e.g. `atom/foo` -> "atom", with the
/// `agents/handoff/*` registry collapsed to one group.
pub fn namespace(name: &str) -> &str {
    if name.starts_with("agents/handoff/") {
        return "agents/handoff";
    }
    name.split('/').next().unwrap_or(name)
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
        artifacts: str_array_at(v, "artifacts"),
        cites: str_array_at(v, "cites"),
        supersedes: opt_str_at(v, "supersedes"),
    }
}

fn str_array_at(v: &Value, key: &str) -> Vec<String> {
    v.get(key)
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// A kan CID in the project's compact display form: `@` + the seven characters
/// after the shared `bafyrei` prefix + `…` (e.g. `@ctf6g6f…`). A CID without the
/// prefix falls back to `@` + its first seven characters + `…`
/// (subject `cid-shortcut-notation`).
pub fn short_cid(cid: &str) -> String {
    let rest = cid.strip_prefix("bafyrei").unwrap_or(cid);
    let take: String = rest.chars().take(7).collect();
    format!("@{take}…")
}

// --- The declared process structure: atoms, teloi, tensions ------------------

/// Return the JSON body of a fenced ```` ```<name> ```` block in `text`, or None.
/// The fence must start a line and the name must end that line, so `day-atom`
/// never matches a longer `day-atomx` or a ```` ``` ```` appearing mid-prose.
pub fn extract_fenced<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let fence = format!("```{name}");
    let mut from = 0;
    loop {
        let idx = from + text[from..].find(&fence)?;
        let at_line_start = idx == 0 || text.as_bytes()[idx - 1] == b'\n';
        let after = &text[idx + fence.len()..];
        let name_ends_line = after.starts_with('\n') || after.starts_with('\r');
        if at_line_start && name_ends_line {
            let body_start = after.find('\n')? + 1;
            let body = &after[body_start..];
            let end = body.find("```")?;
            return Some(body[..end].trim());
        }
        from = idx + fence.len();
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Atom {
    pub slug: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub next: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TelosView {
    pub slug: String,
    pub title: String,
    pub statement: String,
    pub witnesses: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ProcessSnapshot {
    pub atoms: Vec<Atom>,
    pub teloi: Vec<TelosView>,
    pub tensions: Vec<String>,
}

/// The body of the newest claim carrying a `name` fenced block, if any.
fn newest_block(claims: &[Claim], name: &str) -> Option<String> {
    claims
        .iter()
        .filter_map(|c| {
            c.text
                .as_deref()
                .and_then(|t| extract_fenced(t, name))
                .map(|b| (c.recorded_at.unwrap_or(0), b.to_string()))
        })
        .max_by_key(|(ts, _)| *ts)
        .map(|(_, b)| b)
}

fn parse_atom(slug: &str, claims: &[Claim]) -> Option<Atom> {
    let j: Value = serde_json::from_str(&newest_block(claims, "day-atom")?).ok()?;
    Some(Atom {
        slug: slug.to_string(),
        inputs: str_array_at(&j, "in"),
        outputs: str_array_at(&j, "out"),
        next: str_array_at(&j, "next"),
    })
}

fn parse_telos(slug: &str, claims: &[Claim]) -> TelosView {
    let title = claims
        .iter()
        .find(|c| c.kind == "Subject")
        .and_then(|c| c.title.clone())
        .unwrap_or_default();
    let statement = claims
        .iter()
        .filter(|c| c.kind == "Decision")
        .max_by_key(|c| c.recorded_at.unwrap_or(0))
        .and_then(|c| c.text.as_deref())
        .and_then(|t| t.lines().find(|l| !l.trim().is_empty()))
        .unwrap_or("")
        .to_string();
    let witnesses = newest_block(claims, "day-telos")
        .and_then(|b| serde_json::from_str::<Value>(&b).ok())
        .map(|j| flatten_witnesses(&j))
        .unwrap_or_default();
    TelosView {
        slug: slug.to_string(),
        title,
        statement,
        witnesses,
    }
}

/// Flatten a `day-telos` witnesses list; an alternative group (a nested array)
/// joins with `|`.
fn flatten_witnesses(j: &Value) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(ws) = j.get("witnesses").and_then(Value::as_array) {
        for w in ws {
            if let Some(s) = w.as_str() {
                out.push(s.to_string());
            } else if let Some(a) = w.as_array() {
                let any: Vec<String> = a
                    .iter()
                    .filter_map(|x| x.as_str().map(str::to_string))
                    .collect();
                out.push(any.join("|"));
            }
        }
    }
    out
}

fn parse_tension(claims: &[Claim]) -> Option<String> {
    let j: Value = serde_json::from_str(&newest_block(claims, "day-tension")?).ok()?;
    let between = str_array_at(&j, "between");
    (between.len() == 2).then(|| format!("{} <-> {}", between[0], between[1]))
}

// --- The unified fold: everything from one `kan show --all --json` -----------

/// Everything one `kan show --all --json` yields, folded once and read from
/// memory: the subjects, each subject's newest-first claims, a `cid -> Claim`
/// index for cite resolution, and the declared process structure — plus the
/// `day status` text and any errors. Rebuilt only when the log changes.
#[derive(Clone, Debug, Default)]
pub struct Fold {
    pub subjects: Vec<String>,
    pub claims: HashMap<String, Vec<Claim>>,
    pub by_cid: HashMap<String, Claim>,
    pub process: ProcessSnapshot,
    pub day_status: Option<String>,
    pub errors: Vec<String>,
}

impl Fold {
    /// The selected subject's claims (newest first), or an empty slice.
    pub fn claims_for(&self, subject: &str) -> &[Claim] {
        self.claims.get(subject).map(Vec::as_slice).unwrap_or(&[])
    }

    /// (namespace, count) pairs over the folded subjects, most-populous first.
    pub fn namespace_counts(&self) -> Vec<(String, usize)> {
        let mut map: std::collections::BTreeMap<String, usize> = Default::default();
        for name in &self.subjects {
            *map.entry(namespace(name).to_string()).or_default() += 1;
        }
        let mut v: Vec<_> = map.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v
    }

    /// The flat `agents/handoff/*` session registry.
    pub fn sessions(&self) -> Vec<&str> {
        self.subjects
            .iter()
            .filter(|n| n.starts_with("agents/handoff/"))
            .map(String::as_str)
            .collect()
    }
}

/// Build the whole in-memory model with one `kan show --all --json` spawn (plus
/// one `day status` for the process pane — day has no machine-readable output).
pub fn fold(repo: &Path) -> Fold {
    let mut f = Fold::default();
    match all_json(repo) {
        Ok(json) => populate_fold(&mut f, &json),
        Err(e) => f.errors.push(format!("kan show --all: {e}")),
    }
    match day_status(repo) {
        Ok(text) => f.day_status = Some(text),
        Err(e) => f.errors.push(format!("day status: {e}")),
    }
    f
}

fn all_json(repo: &Path) -> Result<Value, String> {
    let out = Command::new("kan")
        .args(["show", "--all", "--json"])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("spawn failed: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
}

/// Fold an `--all`-shaped payload into `f`. Split out so it is unit-testable
/// without shelling out.
fn populate_fold(f: &mut Fold, json: &Value) {
    let Some(subjects) = json.get("subjects").and_then(Value::as_array) else {
        return;
    };
    for s in subjects {
        let name = str_at(s, "subject");
        if name.is_empty() {
            continue;
        }
        let mut claims: Vec<Claim> = s
            .get("claims")
            .and_then(Value::as_array)
            .map(|a| a.iter().map(claim_from_value).collect())
            .unwrap_or_default();
        sort_newest_first(&mut claims);

        for c in &claims {
            f.by_cid.insert(c.cid.clone(), c.clone());
        }
        if let Some(slug) = name.strip_prefix("atom/") {
            if let Some(atom) = parse_atom(slug, &claims) {
                f.process.atoms.push(atom);
            }
        } else if let Some(slug) = name.strip_prefix("telos/") {
            f.process.teloi.push(parse_telos(slug, &claims));
        } else if name.starts_with("tension/") {
            if let Some(t) = parse_tension(&claims) {
                f.process.tensions.push(t);
            }
        }
        f.subjects.push(name.clone());
        f.claims.insert(name, claims);
    }
    f.subjects.sort();
    f.process.atoms.sort_by(|a, b| a.slug.cmp(&b.slug));
    f.process.teloi.sort_by(|a, b| a.slug.cmp(&b.slug));
    f.process.tensions.sort();
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
            artifacts: vec![],
            cites: vec![],
            supersedes: None,
        };
        let line = c.display_line();
        assert!(line.contains("Decision"));
        assert!(line.contains("zABCDEFG")); // short author, did:key: stripped
        assert!(line.contains("2026-08-18 22:13"));
        assert!(line.contains("hello world")); // first line of text
    }

    #[test]
    fn claim_parses_artifacts_and_cites() {
        let json: Value = serde_json::from_str(
            r#"{"claims":[
              {"cid":"bafyA","kind":"Decision","subject":"s","author":"did:key:z",
               "recorded_at":2,"text":"cites something",
               "artifacts":["Commit(\"abc\")"],"cites":["bafyOLD"]},
              {"cid":"bafyB","kind":"Subject","subject":"s","author":"did:key:z",
               "recorded_at":1,"title":"t","artifacts":["Commit(\"abc\")"]}
            ]}"#,
        )
        .unwrap();
        let cs = claims_from_json(&json);
        let a = cs.iter().find(|c| c.cid == "bafyA").unwrap();
        assert_eq!(a.artifacts, vec!["Commit(\"abc\")"]);
        assert_eq!(a.cites, vec!["bafyOLD"]);
        let b = cs.iter().find(|c| c.cid == "bafyB").unwrap();
        assert!(b.cites.is_empty(), "a claim without cites has an empty vec");
    }

    #[test]
    fn retraction_summary_names_what_it_retracts() {
        let json: Value = serde_json::from_str(
            r#"{"claims":[
              {"cid":"bafyR","kind":"Retraction","subject":"s","author":"did:key:z",
               "recorded_at":9,"supersedes":"bafyreiTARGET0"}
            ]}"#,
        )
        .unwrap();
        let c = &claims_from_json(&json)[0];
        assert_eq!(c.supersedes.as_deref(), Some("bafyreiTARGET0"));
        assert!(
            c.summary().starts_with("retracts @TARGET0"),
            "{}",
            c.summary()
        );
        assert!(c.display_line().contains("retracts @TARGET0"));
    }

    #[test]
    fn short_cid_strips_bafyrei_prefix() {
        assert_eq!(
            short_cid("bafyreictf6g6fq4covvtzxwdxahadplft4wu2fx5ohtv7gtnb37jsful3y"),
            "@ctf6g6f…"
        );
        // No bafyrei prefix: fall back to the first seven characters.
        assert_eq!(short_cid("zXYZ12345678"), "@zXYZ123…");
    }

    #[test]
    fn extract_fenced_pulls_a_block_body() {
        let text = "prose above\n\n```day-atom\n{\"in\":[\"x\"],\"out\":[\"y\"]}\n```\nprose below";
        assert_eq!(
            extract_fenced(text, "day-atom"),
            Some("{\"in\":[\"x\"],\"out\":[\"y\"]}")
        );
        assert_eq!(extract_fenced(text, "day-telos"), None);
        // A longer fence name is not matched by a shorter prefix.
        assert_eq!(
            extract_fenced("```day-atomx\n{\"z\":1}\n```", "day-atom"),
            None
        );
        // The real block is still found next to a prefix-colliding one.
        let both = "```day-atomx\n{\"z\":1}\n```\n```day-atom\n{\"in\":[\"a\"]}\n```";
        assert_eq!(extract_fenced(both, "day-atom"), Some("{\"in\":[\"a\"]}"));
    }

    #[test]
    fn snapshot_folds_atoms_teloi_and_witnesses() {
        let json: Value = serde_json::from_str(
            r#"{"subjects":[
              {"subject":"atom/build","claims":[
                {"cid":"a1","kind":"Decision","subject":"atom/build","author":"z","recorded_at":2,
                 "text":"builds\n\n```day-atom\n{\"in\":[\"design-doc\"],\"out\":[\"code-change\"],\"next\":[\"review\"]}\n```"}
              ]},
              {"subject":"telos/x","claims":[
                {"cid":"t1","kind":"Subject","subject":"telos/x","author":"z","recorded_at":1,"title":"Telos X"},
                {"cid":"t2","kind":"Decision","subject":"telos/x","author":"z","recorded_at":2,
                 "text":"x holds\n\n```day-telos\n{\"witnesses\":[\"code-change\",[\"a\",\"b\"]]}\n```"}
              ]},
              {"subject":"tension/x--y","claims":[
                {"cid":"n1","kind":"Observation","subject":"tension/x--y","author":"z","recorded_at":1,
                 "text":"```day-tension\n{\"between\":[\"x\",\"y\"]}\n```"}
              ]}
            ]}"#,
        )
        .unwrap();
        let mut snap = Fold::default();
        populate_fold(&mut snap, &json);
        let snap = snap.process;
        assert_eq!(snap.atoms.len(), 1);
        assert_eq!(snap.atoms[0].slug, "build");
        assert_eq!(snap.atoms[0].inputs, vec!["design-doc"]);
        assert_eq!(snap.atoms[0].next, vec!["review"]);
        assert_eq!(snap.teloi.len(), 1);
        assert_eq!(snap.teloi[0].title, "Telos X");
        assert_eq!(snap.teloi[0].witnesses, vec!["code-change", "a|b"]);
        assert_eq!(snap.tensions, vec!["x <-> y"]);
    }

    #[test]
    fn fold_indexes_every_claim_by_cid_across_subjects() {
        // `kan show --all --json` nests claims under subjects[].claims[].
        let json: Value = serde_json::from_str(
            r#"{"v":1,"subjects":[
              {"subject":"telos/a","claims":[
                {"cid":"bafy1","kind":"Decision","subject":"telos/a","author":"did:key:z","recorded_at":1,"text":"one"}
              ]},
              {"subject":"telos/b","claims":[
                {"cid":"bafy2","kind":"Result","subject":"telos/b","author":"did:key:z","recorded_at":2,"text":"two"},
                {"cid":"bafy3","kind":"Subject","subject":"telos/b","author":"did:key:z","recorded_at":3,"title":"B"}
              ]}
            ]}"#,
        )
        .unwrap();
        let mut f = Fold::default();
        populate_fold(&mut f, &json);
        assert_eq!(f.by_cid.len(), 3);
        assert_eq!(f.by_cid.get("bafy2").unwrap().kind, "Result");
        assert_eq!(f.by_cid.get("bafy1").unwrap().subject, "telos/a");
        // And the subjects + per-subject claims are present from the same fold.
        assert_eq!(f.subjects, vec!["telos/a", "telos/b"]);
        // Newest-first within a subject: bafy3 (recorded_at 3) before bafy2 (2).
        let b = f.claims_for("telos/b");
        assert_eq!(b.len(), 2);
        assert_eq!(b[0].cid, "bafy3");
        assert_eq!(b[1].cid, "bafy2");
    }

    #[test]
    fn fold_namespace_counts_and_sessions() {
        let mut f = Fold::default();
        for n in ["telos/a", "telos/b", "atom/x", "agents/handoff/thread-1"] {
            f.subjects.push(n.to_string());
        }
        let counts = f.namespace_counts();
        // Most-populous first, then alphabetical: telos(2), then agents/handoff(1), atom(1).
        assert_eq!(counts[0], ("telos".to_string(), 2));
        assert!(counts.contains(&("atom".to_string(), 1)));
        assert!(counts.contains(&("agents/handoff".to_string(), 1)));
        assert_eq!(f.sessions(), vec!["agents/handoff/thread-1"]);
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
