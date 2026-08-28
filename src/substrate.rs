//! Substrate reads — the watch-and-fold spine.
//!
//! Shells out to the real `kan` and `day` binaries and folds their output into a
//! small in-memory model (L2) for rendering. Shelling (not linking the `kan`
//! crate) is the deliberate P0 choice: it proves the spine against real repos with
//! zero build-coupling. The library upgrade for the hot read path comes later —
//! see `.dropbox/02-kan-day-integration.md`.
//!
//! Everything here is poll-driven: the caller watches `.kan/log/HEAD` and asks for
//! a fresh `Fold` (one `kan show --all --json` spawn) only when it changes.

use serde::Serialize;
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

/// day's status-line cache path — the width/style variants day writes for its
/// harness footer (`.day/statusline.variants`).
pub fn footer_cache_path(repo: &Path) -> std::path::PathBuf {
    repo.join(".day/statusline.variants")
}

/// Pick the best footer variant from day's `.day/statusline.variants` cache: each
/// variant is a `#day-footer <style> <width>` header followed by its lines
/// (`style` = `emoji`/`plain`). Prefers the requested style, then the widest
/// variant that fits `width` (else the narrowest of that style, else any). `None`
/// when the cache holds no variant.
pub fn pick_variant(cache: &str, width: u16, emoji: bool) -> Option<Vec<String>> {
    struct Variant {
        style: String,
        w: u16,
        lines: Vec<String>,
    }
    let mut variants: Vec<Variant> = Vec::new();
    for line in cache.lines() {
        if let Some(rest) = line.strip_prefix("#day-footer ") {
            let mut it = rest.split_whitespace();
            let style = it.next().unwrap_or("").to_string();
            let w = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            variants.push(Variant {
                style,
                w,
                lines: Vec::new(),
            });
        } else if let Some(v) = variants.last_mut() {
            v.lines.push(line.to_string());
        }
    }
    if variants.is_empty() {
        return None;
    }
    let want = if emoji { "emoji" } else { "plain" };
    let styled: Vec<&Variant> = variants.iter().filter(|v| v.style == want).collect();
    let pool = if styled.is_empty() {
        variants.iter().collect::<Vec<_>>()
    } else {
        styled
    };
    pool.iter()
        .filter(|v| v.w <= width)
        .max_by_key(|v| v.w)
        .or_else(|| pool.iter().min_by_key(|v| v.w))
        .map(|v| v.lines.clone())
}

/// The footer lines: day's width-matched status-line variant from the cache, or
/// the `day status-line` CLI (also cache-only), or an explicit unavailable line —
/// never empty (`telos/honest-ambiguity`).
pub fn status_footer(repo: &Path, width: u16, emoji: bool) -> Vec<String> {
    if let Ok(s) = std::fs::read_to_string(footer_cache_path(repo)) {
        if let Some(lines) = pick_variant(&s, width, emoji) {
            if !lines.is_empty() {
                return lines;
            }
        }
    }
    if let Ok(out) = Command::new("day")
        .arg("status-line")
        .current_dir(repo)
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<String> = text.lines().map(str::to_string).collect();
            if !lines.is_empty() {
                return lines;
            }
        }
    }
    vec!["(day status-line unavailable)".to_string()]
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
#[derive(Clone, Debug, Serialize)]
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
        let chars: Vec<char> = a.chars().collect();
        // Front truncation alone collides once two signers share a key prefix
        // (roles, cross-repo authors). Show head + tail so both ends distinguish.
        if chars.len() > 11 {
            let head: String = chars[..6].iter().collect();
            let tail: String = chars[chars.len() - 4..].iter().collect();
            format!("{head}…{tail}")
        } else {
            a.to_string()
        }
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
            "{:<11} {:<12}  {:<16}  {}",
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

/// Whether a subject belongs to day's own vocabulary (shown under the `[day]`
/// tree section) rather than the operator's own work.
pub fn is_day_subject(name: &str) -> bool {
    matches!(
        namespace(name),
        "telos" | "atom" | "bridge" | "tension" | "schema" | "agents/handoff"
    ) || name == "practice"
        || name == "general"
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Atom {
    pub slug: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub next: Vec<String>,
    /// Witness types this atom's completion produces (the `done` block key).
    pub done: Vec<String>,
    /// Atoms this one revisits (the `revisits` block key), i.e. its back-edges.
    pub revisits: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct TelosView {
    pub slug: String,
    pub title: String,
    pub statement: String,
    pub witnesses: Vec<String>,
}

/// A recorded telos tension: the two teloi it holds between, and the rationale
/// text (the claim body, minus its `day-tension` fence) explaining the trade-off.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Tension {
    pub between: (String, String),
    pub why: String,
}

impl Tension {
    /// Whether this tension names `slug` as one of its two teloi.
    pub fn names(&self, slug: &str) -> bool {
        self.between.0 == slug || self.between.1 == slug
    }
    /// `a <-> b`.
    pub fn pair(&self) -> String {
        format!("{} <-> {}", self.between.0, self.between.1)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ProcessSnapshot {
    pub atoms: Vec<Atom>,
    pub teloi: Vec<TelosView>,
    pub tensions: Vec<Tension>,
    /// Witness type -> a human description of how it is probed, from the
    /// `schema/witness` day-witness map. Lets a telos show what each of its
    /// witnesses actually means, not just its type name.
    pub witnesses: std::collections::BTreeMap<String, String>,
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
        done: str_array_at(&j, "done"),
        revisits: str_array_at(&j, "revisits"),
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

/// Parse the `schema/witness` day-witness map (witness name -> probe) into a
/// name -> human probe description map. A lone-probe block (not a name map) or a
/// missing/invalid block yields an empty map.
fn parse_witness_probes(claims: &[Claim]) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    let Some(block) = newest_block(claims, "day-witness") else {
        return out;
    };
    let Ok(j) = serde_json::from_str::<Value>(&block) else {
        return out;
    };
    if is_probe(&j) {
        return out; // a single probe, not a witness-name -> probe map
    }
    if let Some(map) = j.as_object() {
        for (name, probe) in map {
            out.insert(name.clone(), describe_probe_rich(probe));
        }
    }
    out
}

fn parse_tension(claims: &[Claim]) -> Option<Tension> {
    // The newest claim carrying a day-tension block; its prose (before the fence)
    // is the rationale for the trade-off, which the flat list used to drop.
    let text = claims
        .iter()
        .filter_map(|c| c.text.as_deref())
        .find(|t| extract_fenced(t, "day-tension").is_some())?;
    let j: Value = serde_json::from_str(extract_fenced(text, "day-tension")?).ok()?;
    let between = str_array_at(&j, "between");
    if between.len() != 2 {
        return None;
    }
    let why = text
        .split("```day-tension")
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    Some(Tension {
        between: (between[0].clone(), between[1].clone()),
        why,
    })
}

/// Append every top-level key of `j` not already `covered` (and not a `_version`
/// marker) as `key: value`, so a recognized block never hides author-provided
/// fields the summary does not know about (`telos/honest-ambiguity`).
fn append_extra_keys(out: &mut Vec<String>, j: &Value, covered: &[&str]) {
    let Some(map) = j.as_object() else { return };
    let mut keys: Vec<&String> = map
        .keys()
        .filter(|k| !covered.contains(&k.as_str()) && !k.starts_with('_'))
        .collect();
    keys.sort();
    for k in keys {
        let v = &map[k];
        let rendered = match v {
            Value::String(s) => s.clone(),
            Value::Array(_) => str_array_at(j, k).join(", "),
            other => other.to_string(),
        };
        out.push(format!("{k}: {rendered}"));
    }
}

/// Flatten a `day-bridge` plan tree into one line using day's composition
/// operators (day's `Node` enum, serde `rename_all = "lowercase"`): a `seq`
/// joins with ` > `, an `all` (concurrent) with ` & `, an `any` (alternatives)
/// with ` | `, and a leaf `{ "atom": name }` is its atom name. Composites nest,
/// so an `all` inside a `seq` renders inline. Anything unrecognized falls back to
/// compact JSON rather than vanishing (`telos/honest-ambiguity`).
fn flatten_plan(node: &Value) -> String {
    for (key, sep) in [("seq", " > "), ("all", " & "), ("any", " | ")] {
        if let Some(items) = node.get(key).and_then(Value::as_array) {
            return items.iter().map(flatten_plan).collect::<Vec<_>>().join(sep);
        }
    }
    if let Some(atom) = node.get("atom").and_then(Value::as_str) {
        return atom.to_string();
    }
    node.to_string()
}

/// The probe-kind keywords day's untagged witness-probe union uses. A `day-witness`
/// block whose top-level keys are all in this set is a single probe, not a
/// witness-name -> probe map. day emits no discriminator, so cospan shape-sniffs
/// (see the `kan-tools/day` issue on `day-witness` overloading).
const PROBE_KINDS: [&str; 7] = [
    "path", "command", "claim", "tag", "material", "record", "every",
];

/// True when `j` is a non-empty object whose every key names a probe kind — i.e.
/// the whole block is one probe rather than a witness-name -> probe map.
fn is_probe(j: &Value) -> bool {
    j.as_object()
        .is_some_and(|o| !o.is_empty() && o.keys().all(|k| PROBE_KINDS.contains(&k.as_str())))
}

/// The kind(s) of a probe value for a schema-map line: an object's keys joined
/// with `+` (a nested `{material, record}` is `material+record`), a bare string is
/// `(inline)`, else compact JSON — so a probe never renders as `?`.
fn probe_kind(v: &Value) -> String {
    match v {
        Value::Object(o) if !o.is_empty() => o.keys().cloned().collect::<Vec<_>>().join("+"),
        Value::String(_) => "(inline)".to_string(),
        other => other.to_string(),
    }
}

/// A lone probe as one line: `kind: value` when it is a single string-valued key
/// (e.g. `command: cargo test`), else its `probe_kind`.
fn describe_probe(j: &Value) -> String {
    if let Value::Object(o) = j {
        if o.len() == 1 {
            if let Some((k, Value::String(s))) = o.iter().next() {
                return format!("{k}: {s}");
            }
        }
    }
    probe_kind(j)
}

/// A richer, one-line probe description that unfolds one level of nesting — so a
/// compound witness reads `material path src/*.rs, record claim` instead of the
/// opaque `material+record`. Used for the telos detail view (block summaries keep
/// the terser `describe_probe`).
fn describe_probe_rich(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Object(o) if o.is_empty() => "?".to_string(),
        Value::Object(o) => {
            let sep = if o.len() == 1 { "" } else { ", " };
            let parts: Vec<String> = o
                .iter()
                .map(|(k, val)| {
                    let inner = match val {
                        Value::String(s) => s.clone(),
                        Value::Object(inner) if inner.len() == 1 => {
                            let (ik, iv) = inner.iter().next().unwrap();
                            match iv {
                                Value::String(s) => format!("{ik} {s}"),
                                _ => ik.clone(),
                            }
                        }
                        Value::Object(inner) => inner.keys().cloned().collect::<Vec<_>>().join("+"),
                        other => other.to_string(),
                    };
                    format!("{k} {inner}")
                })
                .collect();
            parts.join(sep)
        }
        other => other.to_string(),
    }
}

/// A human-readable view of a supported fenced block's parsed JSON, or `None`
/// for a block type this does not know (which the caller shows as raw code).
pub fn block_summary(fence: &str, j: &Value) -> Option<Vec<String>> {
    let arr = |k: &str| str_array_at(j, k).join(", ");
    match fence {
        "day-atom" => {
            let mut out = vec![
                format!("in:    {}", arr("in")),
                format!("out:   {}", arr("out")),
                format!("next:  {}", arr("next")),
            ];
            append_extra_keys(&mut out, j, &["in", "out", "next"]);
            Some(out)
        }
        "day-telos" => {
            let mut out = vec![format!("witnesses: {}", flatten_witnesses(j).join(", "))];
            append_extra_keys(&mut out, j, &["witnesses"]);
            Some(out)
        }
        "day-tension" => {
            let b = str_array_at(j, "between");
            let mut out = vec![format!("between: {}", b.join(" <-> "))];
            append_extra_keys(&mut out, j, &["between"]);
            Some(out)
        }
        "day-witness" => {
            // The fence carries two shapes: a single probe (`{"command":"…"}`),
            // or a witness-name -> probe map (`schema/witness`). Render both.
            let mut out = Vec::new();
            if is_probe(j) {
                out.push(describe_probe(j));
            } else if let Some(map) = j.as_object() {
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                for k in keys {
                    out.push(format!("{k}: {}", probe_kind(&map[k])));
                }
            }
            Some(out)
        }
        "cospan-comment" => {
            let mut out = vec![format!(
                "comment: {}",
                j.get("body").and_then(Value::as_str).unwrap_or("")
            )];
            append_extra_keys(&mut out, j, &["body"]);
            Some(out)
        }
        "day-bridge" => {
            let mut out = vec![
                format!("telos: {}", str_at(j, "telos")),
                format!("have:  {}", arr("have")),
                format!(
                    "plan:  {}",
                    j.get("plan").map(flatten_plan).unwrap_or_default()
                ),
            ];
            append_extra_keys(&mut out, j, &["telos", "have", "plan"]);
            Some(out)
        }
        "day-schema" | "day-docs" | "day-injection" => {
            // Every field rendered generically, so the view never falls behind
            // the block's vocabulary — a new key shows up without a code change.
            let mut out = Vec::new();
            append_extra_keys(&mut out, j, &[]);
            Some(out)
        }
        _ => None,
    }
}

// --- The unified fold: everything from one `kan show --all --json` -----------

/// Everything one `kan show --all --json` yields, folded once and read from
/// memory: the subjects, each subject's newest-first claims, a `cid -> Claim`
/// index for cite resolution, and the declared process structure — plus the
/// `day status` text and any errors. Rebuilt only when the log changes.
#[derive(Clone, Debug, Default, Serialize)]
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
        } else if name == "schema/witness" {
            f.process.witnesses = parse_witness_probes(&claims);
        }
        f.subjects.push(name.clone());
        f.claims.insert(name, claims);
    }
    f.subjects.sort();
    f.process.atoms.sort_by(|a, b| a.slug.cmp(&b.slug));
    f.process.teloi.sort_by(|a, b| a.slug.cmp(&b.slug));
    f.process.tensions.sort_by(|a, b| a.between.cmp(&b.between));
}

/// Format microseconds-since-epoch as a compact UTC stamp `YYYY-MM-DD HH:MM`.
///
/// Integer-only (Howard Hinnant's civil-from-days) so the P0 spine stays
/// dependency-free — a calendar crate is not worth pulling in for one column.
/// A compact UTC stamp `MM-DD HH:MM` from a `SystemTime` — for faded recency
/// labels (session last-active, message time). Pre-epoch times clamp to 0.
pub fn stamp_short(t: std::time::SystemTime) -> String {
    let us = t
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0);
    let full = fmt_utc(us); // YYYY-MM-DD HH:MM
    full.get(5..).unwrap_or(&full).to_string()
}

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
    fn short_author_strips_and_abbreviates_head_and_tail() {
        // A short key (<= 11 chars) is shown whole, prefix stripped.
        assert_eq!(parsed()[0].short_author(), "zAAAAAAAAA");
        // A real-length key abbreviates to head6…tail4, so both ends distinguish.
        let long = Claim {
            author: "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY".into(),
            ..parsed()[0].clone()
        };
        assert_eq!(long.short_author(), "zDnaeS…r8XY");
        // Two keys sharing the first 8 chars but differing later no longer collide.
        let a = Claim {
            author: "did:key:zSAMEPREFIXaaaaaaaaaaaaaaaaEND1".into(),
            ..parsed()[0].clone()
        };
        let b = Claim {
            author: "did:key:zSAMEPREFIXbbbbbbbbbbbbbbbbEND2".into(),
            ..parsed()[0].clone()
        };
        assert_ne!(a.short_author(), b.short_author());
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
        assert_eq!(snap.tensions.len(), 1);
        assert_eq!(snap.tensions[0].pair(), "x <-> y");
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
    fn is_day_subject_classifies_vocabulary() {
        assert!(is_day_subject("telos/x"));
        assert!(is_day_subject("schema/witness"));
        assert!(is_day_subject("practice"));
        assert!(is_day_subject("agents/handoff/main")); // day's own thread registry
        assert!(!is_day_subject("claim-detail-view"));
        assert!(!is_day_subject("release"));
    }

    #[test]
    fn block_summary_supported_and_unknown() {
        let atom: Value = serde_json::from_str(
            r#"{"in":["design-doc"],"out":["code-change"],"next":["review"],"done":["passing-tests"]}"#,
        )
        .unwrap();
        let joined = block_summary("day-atom", &atom).unwrap().join("\n");
        assert!(joined.contains("in:") && joined.contains("design-doc"));
        assert!(joined.contains("out:") && joined.contains("code-change"));
        assert!(joined.contains("next:") && joined.contains("review"));
        // An extra key the summary doesn't special-case is surfaced, not hidden.
        assert!(
            joined.contains("done: passing-tests"),
            "extra key hidden: {joined}"
        );
        // An unknown fence has no human view (the caller shows it as code).
        assert!(block_summary("something-else", &atom).is_none());
    }

    #[test]
    fn block_summary_day_bridge_renders_telos_have_and_plan() {
        let bridge: Value = serde_json::from_str(
            r#"{"telos":"readable-claim-browser","have":["design-doc"],
                "plan":{"seq":[{"atom":"generative-build"},{"atom":"adversarial-review"}]}}"#,
        )
        .unwrap();
        let joined = block_summary("day-bridge", &bridge).unwrap().join("\n");
        assert!(joined.contains("telos: readable-claim-browser"), "{joined}");
        assert!(joined.contains("have:") && joined.contains("design-doc"));
        assert!(
            joined.contains("plan:") && joined.contains("generative-build > adversarial-review"),
            "plan not flattened with seq operator: {joined}"
        );
    }

    #[test]
    fn flatten_plan_composes_seq_all_and_any() {
        // day's real plan grammar: Node::{Atom,Seq,All,Any}, lowercase serde.
        let plan: Value = serde_json::from_str(
            r#"{"seq":[{"atom":"a"},{"all":[{"atom":"b"},{"atom":"c"}]},
                {"any":[{"atom":"d"},{"atom":"e"}]}]}"#,
        )
        .unwrap();
        assert_eq!(flatten_plan(&plan), "a > b & c > d | e");
    }

    #[test]
    fn flatten_plan_renders_a_real_any_branch_not_raw_json() {
        // A real day corpus bridge: seq with an `any` branch. Must not leak JSON.
        let plan: Value = serde_json::from_str(
            r#"{"seq":[{"atom":"design"},{"any":[{"atom":"build"},{"atom":"ship"}]}]}"#,
        )
        .unwrap();
        assert_eq!(flatten_plan(&plan), "design > build | ship");
    }

    #[test]
    fn block_summary_day_schema_renders_every_field() {
        let schema: Value = serde_json::from_str(
            r#"{"requirement_prefix":"REQ-","min_requirements":2,
                "sections":["Summary","Requirements"]}"#,
        )
        .unwrap();
        let joined = block_summary("day-schema", &schema).unwrap().join("\n");
        assert!(joined.contains("requirement_prefix: REQ-"), "{joined}");
        assert!(joined.contains("min_requirements: 2"), "{joined}");
        assert!(
            joined.contains("sections: Summary, Requirements"),
            "array field not comma-joined: {joined}"
        );
        // Still None for a fence with no arm.
        assert!(block_summary("day-unknown", &schema).is_none());
    }

    #[test]
    fn block_summary_day_witness_single_probe_shows_value_not_question_mark() {
        // The bug: a lone probe's key IS the kind; its value is a string.
        let cmd: Value = serde_json::from_str(r#"{"command":"cargo test"}"#).unwrap();
        assert_eq!(
            block_summary("day-witness", &cmd).unwrap(),
            vec!["command: cargo test"]
        );
        let path: Value = serde_json::from_str(r#"{"path":"src/*.rs"}"#).unwrap();
        assert_eq!(
            block_summary("day-witness", &path).unwrap(),
            vec!["path: src/*.rs"]
        );
        // A lone composite probe (day's `every`) names its kind, not `?`.
        let every: Value = serde_json::from_str(r#"{"every":{"witness":"telos"}}"#).unwrap();
        assert_eq!(block_summary("day-witness", &every).unwrap(), vec!["every"]);
    }

    #[test]
    fn block_summary_day_witness_schema_map_names_each_probe_kind() {
        let map: Value = serde_json::from_str(
            r#"{"design-doc":{"path":"src/*.rs"},
                "code-change":{"material":{"path":"x"},"record":{"claim":{}}},
                "passing-tests":{"command":"cargo test"}}"#,
        )
        .unwrap();
        let out = block_summary("day-witness", &map).unwrap();
        assert_eq!(
            out,
            vec![
                "code-change: material+record",
                "design-doc: path",
                "passing-tests: command",
            ]
        );
    }

    #[test]
    fn block_summary_day_docs_and_injection_render_generically() {
        let docs: Value =
            serde_json::from_str(r#"{"doc_files":["README.md"],"version_key":"version"}"#).unwrap();
        let joined = block_summary("day-docs", &docs).unwrap().join("\n");
        assert!(joined.contains("doc_files: README.md"), "{joined}");
        assert!(joined.contains("version_key: version"), "{joined}");

        let inj: Value =
            serde_json::from_str(r#"{"cadence":"turn","max_practice_items":3}"#).unwrap();
        let joined = block_summary("day-injection", &inj).unwrap().join("\n");
        assert!(joined.contains("cadence: turn"), "{joined}");
        assert!(joined.contains("max_practice_items: 3"), "{joined}");
    }

    #[test]
    fn stamp_short_is_month_day_time() {
        assert_eq!(stamp_short(std::time::UNIX_EPOCH), "01-01 00:00");
    }

    #[test]
    fn fmt_utc_handles_epoch_leapday_and_negative() {
        assert_eq!(fmt_utc(0), "1970-01-01 00:00");
        // 2024-02-29T00:00:00Z — a leap day — is 1_709_164_800 s.
        assert_eq!(fmt_utc(1_709_164_800_000_000), "2024-02-29 00:00");
        // One microsecond before the epoch floors into the previous day.
        assert_eq!(fmt_utc(-1), "1969-12-31 23:59");
    }

    #[test]
    fn pick_variant_selects_by_style_and_width() {
        // (AC-1)
        let cache = "#day-footer emoji 43\ne1\ne2\ne3\n#day-footer plain 57\np1\np2\np3\n";
        let e = vec!["e1".to_string(), "e2".to_string(), "e3".to_string()];
        let p = vec!["p1".to_string(), "p2".to_string(), "p3".to_string()];
        assert_eq!(pick_variant(cache, 50, true), Some(e.clone())); // emoji fits 50
        assert_eq!(pick_variant(cache, 60, false), Some(p.clone())); // plain fits 60
                                                                     // Below every variant's width: the narrowest of the preferred style.
        assert_eq!(pick_variant(cache, 10, true), Some(e));
        // A style with no variants falls back to any variant.
        assert!(pick_variant("#day-footer emoji 43\nonly\n", 50, false).is_some());
        assert_eq!(pick_variant("", 50, true), None);
        assert_eq!(pick_variant("no header here", 50, true), None);
    }

    #[test]
    fn status_footer_reads_cache_then_falls_back() {
        // (AC-2)
        let dir = std::env::temp_dir().join(format!("cospan-footer-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join(".day")).unwrap();
        std::fs::write(footer_cache_path(&dir), "#day-footer emoji 43\nx1\nx2\n").unwrap();
        assert_eq!(
            status_footer(&dir, 50, true),
            vec!["x1".to_string(), "x2".to_string()]
        );
        // No cache file -> a non-empty fallback, never blank.
        let empty =
            std::env::temp_dir().join(format!("cospan-footer-empty-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&empty);
        std::fs::create_dir_all(&empty).unwrap();
        assert!(!status_footer(&empty, 50, true).is_empty());
    }
}
