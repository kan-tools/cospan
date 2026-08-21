//! Cross-harness transcript reads — the Chat tab's read layer.
//!
//! External substrate, deliberately NOT part of `substrate::Fold` (the kan
//! projection). cospan projects each harness's own on-disk session log; it
//! synthesizes nothing and persists nothing — the conversation lives in the
//! harness's files, cospan only reads it (`telos/kan-is-truth`). Poll-driven
//! like everything else: each source exposes a cheap `change_signal` (the newest
//! relevant mtime) so the watch loop re-reads only on change
//! (`telos/poll-dont-subscribe`).
//!
//! Three harnesses, three real storage shapes, one trait:
//!   * Claude Code — per-session JSONL under `~/.claude/projects/<escaped-cwd>/`.
//!   * Codex       — date-partitioned rollout JSONL under `~/.codex/sessions/`.
//!   * opencode    — a WAL-mode SQLite DB at `~/.local/share/opencode/`.
//!
//! Discovery is scoped to the repo cospan watches: each source keys sessions to
//! the repo by its own mechanism (escaped-cwd dir, `session_meta.cwd`,
//! `session.directory`). The parse functions are pure so they are unit-testable
//! without touching a real store.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

/// Which harness a session came from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Harness {
    ClaudeCode,
    Codex,
    Opencode,
}

impl Harness {
    pub fn label(self) -> &'static str {
        match self {
            Harness::ClaudeCode => "claude",
            Harness::Codex => "codex",
            Harness::Opencode => "opencode",
        }
    }
}

/// Who authored a turn.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl Role {
    pub fn label(self) -> &'static str {
        match self {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
            Role::Tool => "tool",
        }
    }
}

/// What kind of turn an event is. `Thinking` and `ToolCall`/`ToolResult` are the
/// ones the Chat view collapses to a drill-down; `Message` is the readable body.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventKind {
    Message,
    Thinking,
    ToolCall,
    ToolResult,
    Meta,
}

impl EventKind {
    /// Whether the Chat view collapses this behind a one-line drill-down by
    /// default (thinking + tool traffic), keeping the conversation readable.
    pub fn collapses(self) -> bool {
        matches!(
            self,
            EventKind::Thinking | EventKind::ToolCall | EventKind::ToolResult
        )
    }
}

/// One turn in a session, normalized across harnesses. `parent`/`is_sidechain`
/// carry the in-session threading; nothing here is written back anywhere.
#[derive(Clone, Debug)]
pub struct Event {
    pub role: Role,
    pub kind: EventKind,
    pub ts: Option<String>,
    pub id: Option<String>,
    pub parent: Option<String>,
    pub is_sidechain: bool,
    pub text: String,
}

/// A discovered session, listable before its body is read.
#[derive(Clone, Debug)]
pub struct SessionHandle {
    pub harness: Harness,
    pub id: String,
    pub title: String,
    pub git_branch: Option<String>,
    pub last_active: Option<SystemTime>,
    pub locator: Locator,
    /// False when the harness's body schema is not yet decoded (opencode, Q1):
    /// the session is listed but its turns read as unavailable rather than
    /// guessed (`telos/honest-ambiguity`).
    pub body_available: bool,
}

/// How to read a handle's body back.
#[derive(Clone, Debug)]
pub enum Locator {
    /// A single JSONL transcript file (Claude Code, Codex).
    File(PathBuf),
    /// A row set in the opencode SQLite DB.
    OpencodeRow { db: PathBuf, session_id: String },
}

/// A fully-read session: its handle metadata plus the ordered turns.
#[derive(Clone, Debug)]
pub struct Session {
    pub harness: Harness,
    pub id: String,
    pub title: String,
    pub git_branch: Option<String>,
    pub events: Vec<Event>,
}

/// A harness-specific transcript store. `discover` is cheap (metadata only),
/// `read` pulls one session's body, and `change_signal` is the poll gate.
pub trait TranscriptSource {
    fn harness(&self) -> Harness;
    fn discover(&self, repo: &Path) -> Vec<SessionHandle>;
    fn read(&self, handle: &SessionHandle) -> Session;
    fn change_signal(&self, repo: &Path) -> Option<SystemTime>;
}

// --- Aggregate over all sources ----------------------------------------------

fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// The three live sources, rooted under `$HOME`. Empty if `$HOME` is unset.
pub fn sources() -> Vec<Box<dyn TranscriptSource>> {
    let Some(h) = home() else {
        return Vec::new();
    };
    vec![
        Box::new(ClaudeCodeSource {
            root: h.join(".claude/projects"),
        }),
        Box::new(CodexSource {
            root: h.join(".codex/sessions"),
        }),
        Box::new(OpencodeSource {
            db: h.join(".local/share/opencode/opencode.db"),
        }),
    ]
}

/// Newest relevant mtime across every source — the Chat view's re-read gate.
pub fn change_signal(repo: &Path) -> Option<SystemTime> {
    aggregate_signal(sources().iter().map(|s| s.change_signal(repo)))
}

/// The aggregate change signal: the newest of the per-source signals (`None`
/// signals ignored). Split out so the max-across-sources rule is testable.
pub fn aggregate_signal(signals: impl Iterator<Item = Option<SystemTime>>) -> Option<SystemTime> {
    signals.flatten().max()
}

/// Every session for `repo` across all harnesses, most-recently-active first.
pub fn discover_all(repo: &Path) -> Vec<SessionHandle> {
    let mut v: Vec<SessionHandle> = sources().iter().flat_map(|s| s.discover(repo)).collect();
    // Newest-active first; unknown mtimes (`None`) sort last (stable).
    v.sort_by_key(|h| std::cmp::Reverse(h.last_active));
    v
}

/// Read one handle's body via whichever source owns its harness.
pub fn read(handle: &SessionHandle) -> Session {
    for s in sources() {
        if s.harness() == handle.harness {
            return s.read(handle);
        }
    }
    Session {
        harness: handle.harness,
        id: handle.id.clone(),
        title: handle.title.clone(),
        git_branch: handle.git_branch.clone(),
        events: Vec::new(),
    }
}

// --- shared helpers ----------------------------------------------------------

fn newest_mtime<'a>(paths: impl Iterator<Item = &'a Path>) -> Option<SystemTime> {
    paths
        .filter_map(|p| std::fs::metadata(p).ok())
        .filter_map(|m| m.modified().ok())
        .max()
}

/// A short one-line preview of a value for a tool-call summary.
fn brief(v: &Value) -> String {
    let s = match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    };
    let s = s.replace('\n', " ");
    if s.chars().count() > 60 {
        format!("{}…", s.chars().take(60).collect::<String>())
    } else {
        s
    }
}

// --- Claude Code -------------------------------------------------------------

/// `~/.claude/projects/<escaped-cwd>/<session-uuid>.jsonl`, one JSONL file per
/// session. The escaped-cwd dir is the repo path with non-alphanumeric runs
/// turned into `-`.
pub struct ClaudeCodeSource {
    pub root: PathBuf,
}

/// Claude Code's project-dir escaping: every non-alphanumeric character becomes
/// `-` (so `/Users/m/.claude` → `-Users-m--claude`). Matches the on-disk names.
pub fn claude_escaped_dir(repo: &Path) -> String {
    repo.to_string_lossy()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

impl ClaudeCodeSource {
    fn project_dir(&self, repo: &Path) -> PathBuf {
        self.root.join(claude_escaped_dir(repo))
    }

    /// Top-level `*.jsonl` transcript files in the project dir (not the `memory/`
    /// or per-session subdirectories).
    fn session_files(&self, repo: &Path) -> Vec<PathBuf> {
        let dir = self.project_dir(repo);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return Vec::new();
        };
        let mut files: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("jsonl"))
            .collect();
        files.sort();
        files
    }
}

impl TranscriptSource for ClaudeCodeSource {
    fn harness(&self) -> Harness {
        Harness::ClaudeCode
    }

    fn discover(&self, repo: &Path) -> Vec<SessionHandle> {
        self.session_files(repo)
            .into_iter()
            .map(|path| {
                let last_active = std::fs::metadata(&path)
                    .ok()
                    .and_then(|m| m.modified().ok());
                let id = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?")
                    .to_string();
                // Stream a bounded prefix for the title/branch — never the whole
                // (multi-MB) file at discovery time (perf: this runs per re-fold).
                let (title, branch) = claude_meta(&path, &id);
                SessionHandle {
                    harness: Harness::ClaudeCode,
                    id,
                    title,
                    git_branch: branch,
                    last_active,
                    locator: Locator::File(path),
                    body_available: true,
                }
            })
            .collect()
    }

    fn read(&self, handle: &SessionHandle) -> Session {
        let Locator::File(path) = &handle.locator else {
            return empty_session(handle);
        };
        let content = std::fs::read_to_string(path).unwrap_or_default();
        Session {
            harness: Harness::ClaudeCode,
            id: handle.id.clone(),
            title: handle.title.clone(),
            git_branch: handle.git_branch.clone(),
            events: parse_claude(&content),
        }
    }

    fn change_signal(&self, repo: &Path) -> Option<SystemTime> {
        let files = self.session_files(repo);
        newest_mtime(files.iter().map(PathBuf::as_path))
    }
}

/// Title (the first `aiTitle`) and git branch (the first non-empty `gitBranch`,
/// present on every event line) for a Claude Code transcript. Streams a bounded
/// prefix and stops as soon as both are known, so discovery never reads a whole
/// multi-MB session file — the title may be an earlier one if it was later
/// updated, which is honest and cheap.
fn claude_meta(path: &Path, id: &str) -> (String, Option<String>) {
    use std::io::BufRead;
    let Ok(file) = std::fs::File::open(path) else {
        return (short_id(id), None);
    };
    let mut title: Option<String> = None;
    let mut branch: Option<String> = None;
    // Cap the scan: the title normally appears within the first few turns; if it
    // never does, fall back to the short id rather than scanning the whole file.
    for line in std::io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .take(500)
    {
        if let Ok(v) = serde_json::from_str::<Value>(&line) {
            if title.is_none() {
                if let Some(t) = v.get("aiTitle").and_then(Value::as_str) {
                    title = Some(t.to_string());
                }
            }
            if branch.is_none() {
                if let Some(b) = v.get("gitBranch").and_then(Value::as_str) {
                    if !b.is_empty() {
                        branch = Some(b.to_string());
                    }
                }
            }
        }
        if title.is_some() && branch.is_some() {
            break;
        }
    }
    (title.unwrap_or_else(|| short_id(id)), branch)
}

/// Parse a whole Claude Code JSONL transcript into normalized events.
pub fn parse_claude(content: &str) -> Vec<Event> {
    content.lines().flat_map(parse_claude_line).collect()
}

/// Parse one Claude Code JSONL line. A `user`/`assistant` line yields one event
/// per content block; every other line type is dropped from the chat render.
pub fn parse_claude_line(line: &str) -> Vec<Event> {
    let Ok(v) = serde_json::from_str::<Value>(line) else {
        return Vec::new();
    };
    let Some(ty) = v.get("type").and_then(Value::as_str) else {
        return Vec::new();
    };
    let role = match ty {
        "user" => Role::User,
        "assistant" => Role::Assistant,
        _ => return Vec::new(),
    };
    let parent = v
        .get("parentUuid")
        .and_then(Value::as_str)
        .map(str::to_string);
    let id = v.get("uuid").and_then(Value::as_str).map(str::to_string);
    let ts = v
        .get("timestamp")
        .and_then(Value::as_str)
        .map(str::to_string);
    let is_sidechain = v
        .get("isSidechain")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let content = v.pointer("/message/content").unwrap_or(&Value::Null);
    blocks_to_events(content, role, &parent, &id, &ts, is_sidechain)
}

/// Turn a Claude `message.content` (a string or an array of typed blocks) into
/// events. Text → Message, thinking → Thinking, tool_use → ToolCall (as a
/// one-line summary), tool_result → ToolResult.
fn blocks_to_events(
    content: &Value,
    role: Role,
    parent: &Option<String>,
    id: &Option<String>,
    ts: &Option<String>,
    is_sidechain: bool,
) -> Vec<Event> {
    let mk = |kind: EventKind, role: Role, text: String| Event {
        role,
        kind,
        ts: ts.clone(),
        id: id.clone(),
        parent: parent.clone(),
        is_sidechain,
        text,
    };
    match content {
        Value::String(s) => vec![mk(EventKind::Message, role, s.clone())],
        Value::Array(blocks) => blocks
            .iter()
            .filter_map(|b| {
                let bt = b.get("type").and_then(Value::as_str)?;
                match bt {
                    "text" => Some(mk(
                        EventKind::Message,
                        role,
                        b.get("text")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                    )),
                    "thinking" => Some(mk(
                        EventKind::Thinking,
                        role,
                        b.get("thinking")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string(),
                    )),
                    "tool_use" => {
                        let name = b.get("name").and_then(Value::as_str).unwrap_or("tool");
                        let input = b.get("input").map(brief).unwrap_or_default();
                        Some(mk(
                            EventKind::ToolCall,
                            Role::Tool,
                            format!("{name}({input})"),
                        ))
                    }
                    "tool_result" => Some(mk(
                        EventKind::ToolResult,
                        Role::Tool,
                        b.get("content").map(brief).unwrap_or_default(),
                    )),
                    _ => None,
                }
            })
            .collect(),
        _ => Vec::new(),
    }
}

// --- Codex -------------------------------------------------------------------

/// `~/.codex/sessions/YYYY/MM/DD/rollout-<ts>-<id>.jsonl`. The first line is a
/// `session_meta` carrying `cwd` and `git.branch`; the body is `response_item`
/// (Responses-API message items) and `event_msg`.
pub struct CodexSource {
    pub root: PathBuf,
}

impl CodexSource {
    /// Every `rollout-*.jsonl` under the date tree.
    fn rollouts(&self) -> Vec<PathBuf> {
        let mut out = Vec::new();
        walk_jsonl(&self.root, &mut out);
        out.retain(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("rollout-"))
                .unwrap_or(false)
        });
        out.sort();
        out
    }

    /// Rollouts whose `session_meta.cwd` matches `repo`, with their first-line meta.
    fn for_repo(&self, repo: &Path) -> Vec<(PathBuf, CodexMeta)> {
        let want = repo.to_string_lossy();
        self.rollouts()
            .into_iter()
            .filter_map(|p| {
                let first = read_first_line(&p)?;
                let meta = parse_codex_meta(&first)?;
                (meta.cwd == want).then_some((p, meta))
            })
            .collect()
    }
}

impl TranscriptSource for CodexSource {
    fn harness(&self) -> Harness {
        Harness::Codex
    }

    fn discover(&self, repo: &Path) -> Vec<SessionHandle> {
        self.for_repo(repo)
            .into_iter()
            .map(|(path, meta)| {
                let last_active = std::fs::metadata(&path)
                    .ok()
                    .and_then(|m| m.modified().ok());
                SessionHandle {
                    harness: Harness::Codex,
                    id: meta.session_id.clone(),
                    title: short_id(&meta.session_id),
                    git_branch: meta.branch,
                    last_active,
                    locator: Locator::File(path),
                    body_available: true,
                }
            })
            .collect()
    }

    fn read(&self, handle: &SessionHandle) -> Session {
        let Locator::File(path) = &handle.locator else {
            return empty_session(handle);
        };
        let content = std::fs::read_to_string(path).unwrap_or_default();
        Session {
            harness: Harness::Codex,
            id: handle.id.clone(),
            title: handle.title.clone(),
            git_branch: handle.git_branch.clone(),
            events: parse_codex(&content),
        }
    }

    fn change_signal(&self, _repo: &Path) -> Option<SystemTime> {
        // Stat-only over all rollout files — deliberately NOT `for_repo`, which
        // reads each file's first line. This over-triggers (any codex session
        // changing re-discovers this repo's), but discovery is gated on the
        // signal advancing, so the expensive first-line scan runs only on change
        // — never per tick or per keystroke (`telos/poll-dont-subscribe`).
        newest_mtime(self.rollouts().iter().map(PathBuf::as_path))
    }
}

/// The fields cospan needs out of a Codex `session_meta` payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodexMeta {
    pub session_id: String,
    pub cwd: String,
    pub branch: Option<String>,
}

/// Parse a Codex `session_meta` line. Returns `None` for any other line type.
pub fn parse_codex_meta(line: &str) -> Option<CodexMeta> {
    let v: Value = serde_json::from_str(line).ok()?;
    if v.get("type").and_then(Value::as_str) != Some("session_meta") {
        return None;
    }
    let p = v.get("payload")?;
    Some(CodexMeta {
        session_id: p
            .get("session_id")
            .and_then(Value::as_str)
            .unwrap_or("?")
            .to_string(),
        cwd: p.get("cwd").and_then(Value::as_str)?.to_string(),
        branch: p
            .pointer("/git/branch")
            .and_then(Value::as_str)
            .map(str::to_string),
    })
}

/// Parse a whole Codex rollout into normalized events (meta/event_msg dropped).
pub fn parse_codex(content: &str) -> Vec<Event> {
    content.lines().flat_map(parse_codex_line).collect()
}

/// Parse one Codex rollout line. Only `response_item` message/reasoning/function
/// items become events; `session_meta` and `event_msg` are dropped.
pub fn parse_codex_line(line: &str) -> Vec<Event> {
    let Ok(v) = serde_json::from_str::<Value>(line) else {
        return Vec::new();
    };
    if v.get("type").and_then(Value::as_str) != Some("response_item") {
        return Vec::new();
    }
    let ts = v
        .get("timestamp")
        .and_then(Value::as_str)
        .map(str::to_string);
    let Some(p) = v.get("payload") else {
        return Vec::new();
    };
    let mk = |kind: EventKind, role: Role, text: String| Event {
        role,
        kind,
        ts: ts.clone(),
        id: p.get("id").and_then(Value::as_str).map(str::to_string),
        parent: None,
        is_sidechain: false,
        text,
    };
    match p.get("type").and_then(Value::as_str) {
        Some("message") => {
            let role = match p.get("role").and_then(Value::as_str) {
                Some("assistant") => Role::Assistant,
                Some("user") => Role::User,
                _ => Role::System,
            };
            let text = codex_content_text(p.get("content"));
            if text.is_empty() {
                Vec::new()
            } else {
                vec![mk(EventKind::Message, role, text)]
            }
        }
        Some("reasoning") => {
            let text = codex_content_text(p.get("summary").or_else(|| p.get("content")));
            vec![mk(EventKind::Thinking, Role::Assistant, text)]
        }
        Some("function_call") => {
            let name = p.get("name").and_then(Value::as_str).unwrap_or("tool");
            let args = p.get("arguments").map(brief).unwrap_or_default();
            vec![mk(
                EventKind::ToolCall,
                Role::Tool,
                format!("{name}({args})"),
            )]
        }
        Some("function_call_output") => {
            vec![mk(
                EventKind::ToolResult,
                Role::Tool,
                p.get("output").map(brief).unwrap_or_default(),
            )]
        }
        _ => Vec::new(),
    }
}

/// Join the `text` fields out of a Codex content array (`input_text`/`output_text`).
fn codex_content_text(content: Option<&Value>) -> String {
    match content {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|i| i.get("text").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join(""),
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}

// --- opencode ----------------------------------------------------------------

/// opencode stores sessions in a WAL-mode SQLite DB. cospan opens it read-only
/// and touches ONLY the session/message/part/project tables — the same file also
/// holds `account`/`credential`/`control_account` rows with live tokens, which
/// this adapter's table allowlist keeps cospan from ever reading.
pub struct OpencodeSource {
    pub db: PathBuf,
}

/// The ONLY tables the opencode adapter may query. Credential/account tables are
/// deliberately absent (operational safety: cospan never reads a secret store).
pub const OPENCODE_TABLES: &[&str] = &["session", "message", "part", "project"];

/// The read-only `session` discovery query for a repo directory. Built only from
/// `OPENCODE_TABLES`; the directory is bound as a literal with quotes escaped.
pub fn opencode_session_query(repo: &Path) -> String {
    let dir = repo.to_string_lossy().replace('\'', "''");
    // `session` is the sole table named — asserted by tests against the allowlist.
    format!(
        "SELECT id, title, time_updated FROM session WHERE directory = '{dir}' ORDER BY time_updated DESC;"
    )
}

impl OpencodeSource {
    /// Run a read-only query via `sqlite3 -readonly`, returning stdout rows.
    fn query(&self, sql: &str) -> Option<String> {
        let out = Command::new("sqlite3")
            .arg("-readonly")
            .arg("-separator")
            .arg("\u{1f}") // unit separator: safe against titles containing commas/tabs
            .arg(&self.db)
            .arg(sql)
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        Some(String::from_utf8_lossy(&out.stdout).into_owned())
    }
}

impl TranscriptSource for OpencodeSource {
    fn harness(&self) -> Harness {
        Harness::Opencode
    }

    fn discover(&self, repo: &Path) -> Vec<SessionHandle> {
        if !self.db.exists() {
            return Vec::new();
        }
        let Some(rows) = self.query(&opencode_session_query(repo)) else {
            return Vec::new();
        };
        rows.lines()
            .filter_map(|line| {
                let mut f = line.split('\u{1f}');
                let id = f.next()?.to_string();
                let title = f.next().unwrap_or("").to_string();
                let time_updated = f.next().and_then(opencode_epoch);
                let title = if title.is_empty() {
                    short_id(&id)
                } else {
                    title
                };
                Some(SessionHandle {
                    harness: Harness::Opencode,
                    id: id.clone(),
                    title,
                    git_branch: None,
                    // Per-session `time_updated`, so opencode sessions sort by
                    // their own recency in the rail rather than all clustering at
                    // the shared DB mtime. The DB-wal mtime still drives the gate.
                    last_active: time_updated,
                    locator: Locator::OpencodeRow {
                        db: self.db.clone(),
                        session_id: id,
                    },
                    // Q1: the message/part `data` JSON shape is not yet decoded, so
                    // bodies read as unavailable rather than guessed.
                    body_available: false,
                })
            })
            .collect()
    }

    fn read(&self, handle: &SessionHandle) -> Session {
        // Honest-ambiguity: opencode's per-turn body schema (message/part `data`)
        // is undecoded (design Q1). List the session; mark the body unavailable
        // rather than fabricate a parse.
        Session {
            harness: Harness::Opencode,
            id: handle.id.clone(),
            title: handle.title.clone(),
            git_branch: None,
            events: vec![Event {
                role: Role::System,
                kind: EventKind::Meta,
                ts: None,
                id: None,
                parent: None,
                is_sidechain: false,
                text: "opencode message body not yet decoded (design Q1) — session \
                       listed, turns unavailable"
                    .to_string(),
            }],
        }
    }

    fn change_signal(&self, _repo: &Path) -> Option<SystemTime> {
        // The WAL file is what an append touches; fall back to the DB itself.
        // This is the whole-DB signal (not repo-scoped: opencode keys sessions by
        // directory only inside the DB). Cross-repo opencode activity therefore
        // advances the aggregate signal and re-discovers the session list — but
        // the Chat view re-reads a *body*, and resets scroll/expansion, only when
        // the *selected* session's own mtime changed (see `AppState::refresh_chat`
        // via `chat_reread_plan`), so an unrelated append never disturbs reading.
        let wal = self.db.with_extension("db-wal");
        newest_mtime([wal.as_path(), self.db.as_path()].into_iter())
    }
}

// --- small shared utilities --------------------------------------------------

fn empty_session(handle: &SessionHandle) -> Session {
    Session {
        harness: handle.harness,
        id: handle.id.clone(),
        title: handle.title.clone(),
        git_branch: handle.git_branch.clone(),
        events: Vec::new(),
    }
}

/// Short display id: the first segment of a UUID, or the whole thing if short.
fn short_id(id: &str) -> String {
    id.split('-').next().unwrap_or(id).to_string()
}

/// Convert an opencode `time_updated` integer to a `SystemTime`, tolerating both
/// seconds and milliseconds (the `>1e12` threshold detects ms) so sessions sort
/// by their own recency regardless of the column's unit.
fn opencode_epoch(s: &str) -> Option<SystemTime> {
    let n: i64 = s.trim().parse().ok()?;
    if n <= 0 {
        return None;
    }
    let secs = if n > 1_000_000_000_000 {
        (n / 1000) as u64
    } else {
        n as u64
    };
    Some(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs))
}

/// The first line of a file, read without loading the rest — the Codex
/// `session_meta` line sits at the top of a multi-MB rollout.
fn read_first_line(path: &Path) -> Option<String> {
    use std::io::BufRead;
    let file = std::fs::File::open(path).ok()?;
    std::io::BufReader::new(file).lines().next()?.ok()
}

/// Recursively collect `*.jsonl` files under `dir`.
fn walk_jsonl(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk_jsonl(&p, out);
        } else if p.extension().and_then(|x| x.to_str()) == Some("jsonl") {
            out.push(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claude_line_parses_user_and_assistant_with_threading() {
        let user = r#"{"type":"user","uuid":"u1","parentUuid":null,"isSidechain":false,"message":{"role":"user","content":"fix the bug"}}"#;
        let asst = r#"{"type":"assistant","uuid":"a1","parentUuid":"u1","isSidechain":false,"message":{"role":"assistant","content":[{"type":"text","text":"on it"},{"type":"tool_use","name":"Edit","input":{"file":"x.rs"}}]}}"#;
        let ue = parse_claude_line(user);
        assert_eq!(ue.len(), 1);
        assert_eq!(ue[0].role, Role::User);
        assert_eq!(ue[0].text, "fix the bug");

        let ae = parse_claude_line(asst);
        assert_eq!(ae.len(), 2);
        assert_eq!(ae[0].role, Role::Assistant);
        assert_eq!(ae[0].kind, EventKind::Message);
        assert_eq!(ae[0].parent.as_deref(), Some("u1"));
        assert_eq!(ae[1].kind, EventKind::ToolCall);
        assert!(ae[1].text.starts_with("Edit("));
    }

    #[test]
    fn claude_sidechain_and_thinking_flagged_for_collapse() {
        let side = r#"{"type":"assistant","uuid":"s1","isSidechain":true,"message":{"role":"assistant","content":[{"type":"thinking","thinking":"hmm"}]}}"#;
        let e = parse_claude_line(side);
        assert_eq!(e.len(), 1);
        assert!(e[0].is_sidechain);
        assert_eq!(e[0].kind, EventKind::Thinking);
        assert!(e[0].kind.collapses());
        // A plain message does not collapse.
        assert!(!EventKind::Message.collapses());
    }

    #[test]
    fn non_message_claude_lines_drop() {
        assert!(parse_claude_line(r#"{"type":"system","content":"x"}"#).is_empty());
        assert!(parse_claude_line("not json").is_empty());
    }

    #[test]
    fn codex_meta_extracts_cwd_and_branch() {
        let line = r#"{"type":"session_meta","timestamp":"t","payload":{"session_id":"abc-123","cwd":"/repo","git":{"branch":"main"}}}"#;
        let m = parse_codex_meta(line).expect("meta");
        assert_eq!(m.cwd, "/repo");
        assert_eq!(m.branch.as_deref(), Some("main"));
        assert_eq!(m.session_id, "abc-123");
        // A non-meta line is not mistaken for meta.
        assert!(parse_codex_meta(r#"{"type":"response_item","payload":{}}"#).is_none());
    }

    #[test]
    fn codex_response_items_map_by_role() {
        let asst = r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"hello"}]}}"#;
        let e = parse_codex_line(asst);
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].role, Role::Assistant);
        assert_eq!(e[0].text, "hello");
        let call = r#"{"type":"response_item","payload":{"type":"function_call","name":"shell","arguments":{"cmd":"ls"}}}"#;
        let c = parse_codex_line(call);
        assert_eq!(c[0].kind, EventKind::ToolCall);
        // session_meta / event_msg lines are dropped from the body.
        assert!(
            parse_codex_line(r#"{"type":"event_msg","payload":{"type":"task_started"}}"#)
                .is_empty()
        );
    }

    #[test]
    fn opencode_query_filters_by_directory_and_touches_only_allowlisted_tables() {
        let q = opencode_session_query(Path::new("/repo/x"));
        assert!(q.contains("WHERE directory = '/repo/x'"));
        // The query names only the `session` table — never a credential store.
        assert!(q.contains("FROM session"));
        for forbidden in ["credential", "account", "control_account"] {
            assert!(
                !q.to_lowercase().contains(forbidden),
                "query must never name the {forbidden} table"
            );
        }
        // And `session` is in the allowlist.
        assert!(OPENCODE_TABLES.contains(&"session"));
        for forbidden in ["credential", "account", "control_account"] {
            assert!(!OPENCODE_TABLES.contains(&forbidden));
        }
    }

    #[test]
    fn opencode_directory_with_quote_is_escaped() {
        let q = opencode_session_query(Path::new("/o'brien/repo"));
        assert!(q.contains("directory = '/o''brien/repo'"));
    }

    #[test]
    fn claude_escaped_dir_matches_on_disk_form() {
        assert_eq!(
            claude_escaped_dir(Path::new("/Users/m/code/cospan")),
            "-Users-m-code-cospan"
        );
    }

    #[test]
    fn discover_and_read_over_a_claude_fixture() -> Result<(), Box<dyn std::error::Error>> {
        // A real project dir under a temp root; discovery finds it, read parses it.
        let tmp = std::env::temp_dir().join(format!("cospan-tx-{}", std::process::id()));
        let repo = Path::new("/tmp/demo-repo");
        let proj = tmp.join(claude_escaped_dir(repo));
        std::fs::create_dir_all(&proj)?;
        let jsonl = "\
{\"type\":\"user\",\"uuid\":\"u1\",\"message\":{\"role\":\"user\",\"content\":\"hi\"},\"gitBranch\":\"feat/x\",\"aiTitle\":\"my session\"}
{\"type\":\"assistant\",\"uuid\":\"a1\",\"parentUuid\":\"u1\",\"message\":{\"role\":\"assistant\",\"content\":[{\"type\":\"text\",\"text\":\"hey\"}]}}";
        std::fs::write(proj.join("sess-1.jsonl"), jsonl)?;

        let src = ClaudeCodeSource { root: tmp.clone() };
        let handles = src.discover(repo);
        assert_eq!(handles.len(), 1);
        assert_eq!(handles[0].title, "my session");
        assert_eq!(handles[0].git_branch.as_deref(), Some("feat/x"));
        assert!(src.change_signal(repo).is_some());

        let session = src.read(&handles[0]);
        assert_eq!(session.events.len(), 2);
        assert_eq!(session.events[0].text, "hi");

        std::fs::remove_dir_all(&tmp).ok();
        Ok(())
    }

    #[test]
    fn codex_discover_excludes_non_matching_cwd() -> Result<(), Box<dyn std::error::Error>> {
        // (AC-4) Two rollouts under the date tree; only the one whose
        // `session_meta.cwd` matches the repo is discovered.
        let tmp = std::env::temp_dir().join(format!("cospan-codex-{}", std::process::id()));
        let day = tmp.join("2026/08/20");
        std::fs::create_dir_all(&day)?;
        let meta = |sid: &str, cwd: &str| {
            format!(
                "{{\"type\":\"session_meta\",\"timestamp\":\"t\",\"payload\":{{\"session_id\":\"{sid}\",\"cwd\":\"{cwd}\",\"git\":{{\"branch\":\"main\"}}}}}}"
            )
        };
        std::fs::write(day.join("rollout-a.jsonl"), meta("s-a", "/my/repo"))?;
        std::fs::write(day.join("rollout-b.jsonl"), meta("s-b", "/other/repo"))?;

        let src = CodexSource { root: tmp.clone() };
        let got = src.discover(Path::new("/my/repo"));
        assert_eq!(got.len(), 1, "only the matching-cwd rollout is discovered");
        assert_eq!(got[0].harness, Harness::Codex);
        assert_eq!(got[0].id, "s-a");
        assert_eq!(got[0].git_branch.as_deref(), Some("main"));

        std::fs::remove_dir_all(&tmp).ok();
        Ok(())
    }

    #[test]
    fn opencode_discover_selects_by_directory_and_never_reads_credentials(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // (AC-5) Over a real SQLite DB shaped like opencode's: sessions are
        // selected by `directory`, and neither discovery nor read touches the
        // credential table that lives in the same file.
        if Command::new("sqlite3").arg("--version").output().is_err() {
            return Ok(()); // no sqlite3 on this host — nothing to exercise.
        }
        let tmp = std::env::temp_dir().join(format!("cospan-oc-{}", std::process::id()));
        std::fs::create_dir_all(&tmp)?;
        let db = tmp.join("opencode.db");
        let setup =
            "CREATE TABLE session(id TEXT, title TEXT, time_updated INTEGER, directory TEXT);\
             CREATE TABLE credential(email TEXT, access_token TEXT);\
             INSERT INTO session VALUES('s1','My chat',5,'/repo/x');\
             INSERT INTO session VALUES('s2','Elsewhere',4,'/other');\
             INSERT INTO credential VALUES('me','SECRET_TOKEN_VALUE');";
        let ok = Command::new("sqlite3")
            .arg(&db)
            .arg(setup)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        assert!(ok, "sqlite3 fixture setup failed");

        let src = OpencodeSource { db: db.clone() };
        let got = src.discover(Path::new("/repo/x"));
        assert_eq!(got.len(), 1, "only the /repo/x session is selected");
        assert_eq!(got[0].title, "My chat");
        assert_eq!(got[0].harness, Harness::Opencode);
        // Q1: body undecoded → listed but not available.
        assert!(!got[0].body_available);
        // The read is the honest "unavailable" stub — never the secret.
        let session = src.read(&got[0]);
        let dump = format!("{session:?}");
        assert!(
            !dump.contains("SECRET_TOKEN_VALUE"),
            "a credential must never surface in a session read"
        );

        std::fs::remove_dir_all(&tmp).ok();
        Ok(())
    }

    #[test]
    fn aggregate_signal_takes_the_max_and_ignores_none() {
        // (AC-7) the aggregate is the newest per-source signal; `None`s drop out.
        use std::time::Duration;
        let a = SystemTime::UNIX_EPOCH + Duration::from_secs(10);
        let b = SystemTime::UNIX_EPOCH + Duration::from_secs(20);
        assert_eq!(
            aggregate_signal([Some(a), None, Some(b)].into_iter()),
            Some(b)
        );
        assert_eq!(aggregate_signal([None, None].into_iter()), None);
        assert_eq!(aggregate_signal(std::iter::empty()), None);
    }
}
