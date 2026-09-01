//! The comment MCP surface (S5): the operations agents can perform on the
//! comment layer, transport-agnostic.
//!
//! This module is the *core* — pure dispatch over the sidecar store
//! (`crate::comments`), returning JSON — plus the MCP tool schemas. The stdio
//! JSON-RPC transport (see `run`) wraps it, but the core is fully testable
//! without any transport. Agents read and write cospan's own owned comment state
//! (`telos/kan-is-truth`'s sole exception); this never touches the agent-control
//! command bus (`telos/observe-now-control-later`).

use crate::comments::{self, Author, Comment};
use crate::{Localization, State};
use serde_json::{json, Value};
use std::path::Path;

/// The author stamped on a comment written over MCP: `who: "agent"`, with the id
/// from a harness-set `KAN_AGENT` tag (else a generic `agent`), so an agent write
/// is attributed but cannot spoof an arbitrary identity per call.
pub fn agent_author() -> Author {
    Author {
        who: "agent".into(),
        id: std::env::var("KAN_AGENT")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "agent".into()),
    }
}

fn state_str(s: State) -> &'static str {
    match s {
        State::Anchored => "anchored",
        State::Drifted => "drifted",
        State::Unresolvable => "unresolvable",
    }
}

fn read_content(repo: &Path, file: &str) -> String {
    std::fs::read_to_string(repo.join(file)).unwrap_or_default()
}

fn sidecar(repo: &Path, file: &str) -> std::path::PathBuf {
    repo.join(comments::sidecar_path(file))
}

/// Reject a `file` that is absolute, escapes the repo via a `..` component, or
/// whose *resolved real path* leaves the repo through a symlink — so an agent
/// over MCP (and, now, a client over HTTP) can only read/write files inside the
/// watched repo. Returns the error `Value` to return as-is.
///
/// The lexical check (absolute / `..`) is cheap and catches the common case. The
/// symlink check canonicalizes the repo root and the target's real path — for a
/// not-yet-existing target, the nearest existing ancestor plus the remaining
/// tail — and requires containment, closing the hole where an in-repo symlink
/// (`link -> /etc`) points outside the repo.
fn guard(repo: &Path, file: &str) -> Result<(), Value> {
    let err = || json!({ "error": format!("file must be a path inside the repo: {file}") });
    let p = Path::new(file);
    if p.is_absolute() || p.components().any(|c| c == std::path::Component::ParentDir) {
        return Err(err());
    }
    // A repo that itself can't be canonicalized (does not exist) can't be escaped
    // through a symlink; fall back to the lexical check already passed above.
    let Ok(root) = repo.canonicalize() else {
        return Ok(());
    };
    if real_path_escapes(&root, &repo.join(file)) {
        return Err(err());
    }
    Ok(())
}

/// Whether `target`'s real path lands outside `root`, resolving symlinks. Walks
/// up to the deepest existing ancestor of `target`, canonicalizes it (following
/// links), rejoins the non-existent tail, and checks containment. A target that
/// resolves to nothing under `root` is treated as escaping (fail closed).
fn real_path_escapes(root: &Path, target: &Path) -> bool {
    let mut cur = target.to_path_buf();
    let mut tail = std::path::PathBuf::new();
    loop {
        if let Ok(real) = cur.canonicalize() {
            let full = if tail.as_os_str().is_empty() {
                real
            } else {
                real.join(&tail)
            };
            return !full.starts_with(root);
        }
        match (cur.file_name(), cur.parent()) {
            (Some(name), Some(parent)) => {
                tail = Path::new(name).join(&tail);
                cur = parent.to_path_buf();
            }
            // Nothing along the path exists — can't confirm containment.
            _ => return true,
        }
    }
}

fn comment_json(c: &Comment, loc: &Localization) -> Value {
    json!({
        "id": c.id,
        "body": c.body,
        "author": { "who": c.author.who, "id": c.author.id },
        "resolved": c.resolved,
        "state": state_str(loc.state),
        "line": loc.span.map(|(s, _)| s + 1), // 1-based, null when unresolvable
        "replies": c.thread.iter().map(|r| json!({
            "author": { "who": r.author.who, "id": r.author.id },
            "body": r.body,
        })).collect::<Vec<_>>(),
    })
}

/// `comment_files()` — the index of files that have a comment sidecar, each with
/// its comment `total` and `unresolved` counts. A cheap read: it walks the
/// `.cospan/comments/` tree and reads each sidecar's records for the counts only,
/// never re-localizing (no source file is opened), so it does not need file
/// content. The `file` paths returned are repo-relative, recovered from the
/// sidecar layout (`comments::sidecar_path`), so nothing outside the tree is
/// listed. Missing tree → an empty list.
pub fn comment_files(repo: &Path) -> Value {
    let root = repo.join(".cospan/comments");
    let mut files: Vec<Value> = Vec::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in entries.flatten() {
            // `file_type()` reads the dir entry without following symlinks, so a
            // symlinked dir is skipped rather than traversed — no walk cycle and no
            // escape out of the tree via a symlink.
            let Ok(ft) = e.file_type() else {
                continue;
            };
            if ft.is_symlink() {
                continue;
            }
            let p = e.path();
            if ft.is_dir() {
                stack.push(p);
                continue;
            }
            if !ft.is_file() {
                continue;
            }
            // Recover the repo-relative source path: strip the tree prefix and the
            // `.jsonl` suffix. A file that is not `<rel>.jsonl` is skipped.
            let Ok(rel_jsonl) = p.strip_prefix(&root) else {
                continue;
            };
            let Some(rel) = rel_jsonl.to_str().and_then(|s| s.strip_suffix(".jsonl")) else {
                continue;
            };
            let cs = comments::load(&p).unwrap_or_default();
            if cs.is_empty() {
                continue;
            }
            let total = cs.len();
            let unresolved = cs.iter().filter(|c| !c.resolved).count();
            files.push(json!({ "file": rel, "total": total, "unresolved": unresolved }));
        }
    }
    // Stable order for a stable render.
    files.sort_by(|a, b| a["file"].as_str().cmp(&b["file"].as_str()));
    json!({ "files": files })
}

/// The cap on how many lines `file_view` highlights and returns. A phone viewer
/// wants a readable head of a file, not a multi-megabyte payload; a longer file
/// comes back `truncated` with the real `total`.
pub const FILE_VIEW_MAX_LINES: usize = 2000;

/// The cap on how many *bytes* `file_view` reads before decoding/highlighting.
/// The line cap alone does not bound memory: a minified or low-newline file is
/// few lines but huge, so without a byte cap the whole thing is read into memory
/// and highlighted (a review finding). Bounding the read first keeps a single
/// long line from defeating the "truncated head" contract.
pub const FILE_VIEW_MAX_BYTES: usize = 512 * 1024;

/// `file_view(file)` — one file's content, syntax-highlighted for the web viewer.
/// Path-guarded like every file op (an escaping path returns the guard error).
/// Returns `{ path, lines, truncated, total }` where `lines` is one array of
/// `{t, c}` runs per source line (`c` = `#rrggbb` or `""`), highlighted via
/// `highlight::styled_web`. The read is bounded by `FILE_VIEW_MAX_BYTES` and the
/// returned window by `FILE_VIEW_MAX_LINES`; `truncated` is set when either bound
/// clips the file. `total` is the real line count when the whole file was read,
/// else the line count of the head that was (the byte cap means the true total is
/// not known without reading past the bound). A path that is not a readable UTF-8
/// file (a directory, a binary, a missing file) returns `{error}`. A pure read:
/// no source file is written, no sidecar touched.
pub fn file_view(repo: &Path, file: &str) -> Value {
    use std::io::Read;
    if let Err(e) = guard(repo, file) {
        return e;
    }
    let err = || json!({ "error": format!("not a readable file: {file}") });
    let mut f = match std::fs::File::open(repo.join(file)) {
        Ok(f) => f,
        Err(_) => return err(),
    };
    // A directory opens on Unix but is not a file to view; reject it explicitly.
    if f.metadata().map(|m| m.is_dir()).unwrap_or(true) {
        return err();
    }
    // Read at most one byte past the cap, so a file exactly at the cap is not
    // falsely marked truncated, and bound memory before any decode/highlight.
    let mut buf = Vec::new();
    if f.by_ref()
        .take(FILE_VIEW_MAX_BYTES as u64 + 1)
        .read_to_end(&mut buf)
        .is_err()
    {
        return err();
    }
    let over_bytes = buf.len() > FILE_VIEW_MAX_BYTES;
    if over_bytes {
        buf.truncate(FILE_VIEW_MAX_BYTES);
    }
    // Decode as UTF-8. A byte-capped read can slice a multibyte char at the tail;
    // accept the valid prefix in that case (the invalid bytes are within one
    // char-width of the end). A file that is invalid UTF-8 well before the cap is
    // binary — reject it rather than emit mojibake (`telos/honest-ambiguity`).
    let content = match std::str::from_utf8(&buf) {
        Ok(s) => s.to_string(),
        Err(e) => {
            let valid = e.valid_up_to();
            if over_bytes && buf.len() - valid < 4 {
                String::from_utf8_lossy(&buf[..valid]).into_owned()
            } else {
                return err();
            }
        }
    };
    let head_lines = content.lines().count();
    let over_lines = head_lines > FILE_VIEW_MAX_LINES;
    let truncated = over_bytes || over_lines;
    // The highlighted window: the head we read, or its first N lines. Re-join with
    // '\n' (line endings were stripped by `.lines()`) so the grammar sees text.
    let shown = if over_lines {
        let mut s = content
            .lines()
            .take(FILE_VIEW_MAX_LINES)
            .collect::<Vec<_>>()
            .join("\n");
        s.push('\n');
        s
    } else {
        content
    };
    let total = head_lines;
    let ext = Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let lines: Vec<Value> = crate::highlight::styled_web(&shown, ext)
        .into_iter()
        .map(|runs| {
            Value::Array(
                runs.into_iter()
                    .map(|(c, t)| json!({ "t": t, "c": c }))
                    .collect(),
            )
        })
        .collect();
    json!({ "path": file, "lines": lines, "truncated": truncated, "total": total })
}

/// `list_comments(file)` — every comment on `file`, each re-localized against the
/// current content so its `state`/`line` are honest. A pure read: it does not
/// persist re-anchoring.
pub fn list_comments(repo: &Path, file: &str) -> Value {
    if let Err(e) = guard(repo, file) {
        return e;
    }
    let content = read_content(repo, file);
    let mut cs = comments::load(&sidecar(repo, file)).unwrap_or_default();
    let items: Vec<Value> = cs
        .iter_mut()
        .map(|c| {
            let loc = comments::localize_and_update(c, &content);
            comment_json(c, &loc)
        })
        .collect();
    json!({ "file": file, "comments": items })
}

/// `get_thread(file, id)` — one comment with its full reply thread, re-localized.
pub fn get_thread(repo: &Path, file: &str, id: &str) -> Value {
    if let Err(e) = guard(repo, file) {
        return e;
    }
    let content = read_content(repo, file);
    let mut cs = comments::load(&sidecar(repo, file)).unwrap_or_default();
    match cs.iter_mut().find(|c| c.id == id) {
        Some(c) => {
            let loc = comments::localize_and_update(c, &content);
            comment_json(c, &loc)
        }
        None => json!({ "error": format!("no comment {id} on {file}") }),
    }
}

/// `add_comment(file, line, body)` — add an agent-authored comment (the MCP
/// path). Thin wrapper over `add_comment_as` with `agent_author()`.
pub fn add_comment(repo: &Path, file: &str, line: usize, body: &str) -> Value {
    add_comment_as(repo, file, line, body, agent_author())
}

/// `add_comment_as(file, line, body, author)` — capture a fingerprint at `line`
/// (1-based) of the current file and append a comment stamped `author`. Same
/// sidecar the human TUI and CLI write, so it re-localizes and surfaces like any
/// other comment. The author is passed in so the MCP path stamps `who:"agent"`
/// while a web write stamps `who:"human"`.
pub fn add_comment_as(repo: &Path, file: &str, line: usize, body: &str, author: Author) -> Value {
    if let Err(e) = guard(repo, file) {
        return e;
    }
    let content = read_content(repo, file);
    let path = sidecar(repo, file);
    let mut cs = comments::load(&path).unwrap_or_default();
    let created_at = comments::now_micros();
    let line0 = line.saturating_sub(1);
    let c = Comment {
        id: format!("c_{created_at}_{}", cs.len()),
        anchor: comments::StoredAnchor::capture(&content, line0, 2),
        body: body.to_string(),
        author,
        created_at,
        resolved: false,
        thread: Vec::new(),
    };
    let loc = comments::localize_and_update(&mut c.clone(), &content);
    let id = c.id.clone();
    cs.push(c);
    match comments::save(&path, &cs) {
        Ok(()) => {
            json!({ "id": id, "state": state_str(loc.state), "line": loc.span.map(|(s,_)| s+1) })
        }
        Err(e) => json!({ "error": e }),
    }
}

/// `reply(file, id, body)` — append an agent-authored reply (the MCP path). Thin
/// wrapper over `reply_as` with `agent_author()`.
pub fn reply(repo: &Path, file: &str, id: &str, body: &str) -> Value {
    reply_as(repo, file, id, body, agent_author())
}

/// `reply_as(file, id, body, author)` — append a reply stamped `author` to a
/// comment's thread.
pub fn reply_as(repo: &Path, file: &str, id: &str, body: &str, author: Author) -> Value {
    if let Err(e) = guard(repo, file) {
        return e;
    }
    let path = sidecar(repo, file);
    let mut cs = comments::load(&path).unwrap_or_default();
    let r = comments::Reply {
        author,
        body: body.to_string(),
        created_at: comments::now_micros(),
    };
    if !comments::add_reply(&mut cs, id, r) {
        return json!({ "error": format!("no comment {id} on {file}") });
    }
    match comments::save(&path, &cs) {
        Ok(()) => json!({ "ok": true, "id": id }),
        Err(e) => json!({ "error": e }),
    }
}

/// `resolve(file, id, [value])` — set (default) or clear a comment's resolved flag.
pub fn resolve(repo: &Path, file: &str, id: &str, value: bool) -> Value {
    if let Err(e) = guard(repo, file) {
        return e;
    }
    let path = sidecar(repo, file);
    let mut cs = comments::load(&path).unwrap_or_default();
    if !comments::set_resolved(&mut cs, id, value) {
        return json!({ "error": format!("no comment {id} on {file}") });
    }
    match comments::save(&path, &cs) {
        Ok(()) => json!({ "ok": true, "id": id, "resolved": value }),
        Err(e) => json!({ "error": e }),
    }
}

fn str_arg<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("missing string argument `{key}`"))
}

fn uint_arg(args: &Value, key: &str) -> Result<usize, String> {
    args.get(key)
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .ok_or_else(|| format!("missing integer argument `{key}`"))
}

/// Dispatch a tool call by name with JSON `args`, returning the tool result JSON.
/// Transport-agnostic: the stdio server and any test both go through here.
pub fn call_tool(repo: &Path, name: &str, args: &Value) -> Result<Value, String> {
    match name {
        "list_comments" => Ok(list_comments(repo, str_arg(args, "file")?)),
        "get_thread" => Ok(get_thread(
            repo,
            str_arg(args, "file")?,
            str_arg(args, "id")?,
        )),
        "add_comment" => Ok(add_comment(
            repo,
            str_arg(args, "file")?,
            uint_arg(args, "line")?,
            str_arg(args, "body")?,
        )),
        "reply" => Ok(reply(
            repo,
            str_arg(args, "file")?,
            str_arg(args, "id")?,
            str_arg(args, "body")?,
        )),
        "resolve" => Ok(resolve(
            repo,
            str_arg(args, "file")?,
            str_arg(args, "id")?,
            args.get("value").and_then(|v| v.as_bool()).unwrap_or(true),
        )),
        other => Err(format!("unknown tool `{other}`")),
    }
}

/// The MCP tool schemas, for a `tools/list` response.
pub fn tool_definitions() -> Value {
    let file = json!({ "type": "string", "description": "repo-relative file path" });
    let id = json!({ "type": "string", "description": "comment id" });
    let body = json!({ "type": "string", "description": "comment text" });
    json!([
        {
            "name": "list_comments",
            "description": "List the comments on a file, each re-localized (anchored/drifted/unresolvable) against the current content.",
            "inputSchema": { "type": "object", "properties": { "file": file }, "required": ["file"] }
        },
        {
            "name": "get_thread",
            "description": "Get one comment and its full reply thread.",
            "inputSchema": { "type": "object", "properties": { "file": file, "id": id }, "required": ["file", "id"] }
        },
        {
            "name": "add_comment",
            "description": "Add an agent-authored comment anchored at a 1-based line of a file.",
            "inputSchema": { "type": "object", "properties": {
                "file": file, "line": { "type": "integer", "description": "1-based line" }, "body": body
            }, "required": ["file", "line", "body"] }
        },
        {
            "name": "reply",
            "description": "Append an agent-authored reply to a comment's thread.",
            "inputSchema": { "type": "object", "properties": { "file": file, "id": id, "body": body }, "required": ["file", "id", "body"] }
        },
        {
            "name": "resolve",
            "description": "Set (default) or clear a comment's resolved flag.",
            "inputSchema": { "type": "object", "properties": {
                "file": file, "id": id, "value": { "type": "boolean", "description": "resolved state (default true)" }
            }, "required": ["file", "id"] }
        }
    ])
}

// --- The rmcp stdio server (S5 transport) -----------------------------------
// Thin: each tool destructures its typed args and calls the tested core above.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{tool, tool_handler, tool_router, transport::stdio, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, JsonSchema)]
struct FileArg {
    /// repo-relative file path
    file: String,
}
#[derive(Debug, Deserialize, JsonSchema)]
struct ThreadArg {
    /// repo-relative file path
    file: String,
    /// comment id
    id: String,
}
#[derive(Debug, Deserialize, JsonSchema)]
struct AddArg {
    /// repo-relative file path
    file: String,
    /// 1-based line the comment anchors to
    line: u32,
    /// comment text
    body: String,
}
#[derive(Debug, Deserialize, JsonSchema)]
struct ReplyArg {
    /// repo-relative file path
    file: String,
    /// comment id to reply to
    id: String,
    /// reply text
    body: String,
}
#[derive(Debug, Deserialize, JsonSchema)]
struct ResolveArg {
    /// repo-relative file path
    file: String,
    /// comment id
    id: String,
    /// resolved state (default true)
    value: Option<bool>,
}

fn text(v: Value) -> String {
    serde_json::to_string(&v).unwrap_or_else(|_| "{}".into())
}

/// The comment MCP server: the read+write comment tools over stdio, rooted at
/// `repo`. Agents read and write cospan's owned sidecar state only.
#[derive(Clone)]
pub struct CommentServer {
    repo: PathBuf,
    /// Serializes the load-modify-save writes so two concurrent tool calls (rmcp
    /// services requests in parallel) can't lose an update. `Arc` so every
    /// per-request clone of the handler shares the one lock. Reads don't take it:
    /// `comments::save` renames atomically, so a read sees a whole old-or-new file.
    writes: std::sync::Arc<std::sync::Mutex<()>>,
    #[allow(dead_code)] // read by the #[tool_handler]-generated request dispatch
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl CommentServer {
    pub fn new(repo: PathBuf) -> Self {
        Self {
            repo,
            writes: std::sync::Arc::new(std::sync::Mutex::new(())),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "List the comments on a file, each re-localized (anchored/drifted/unresolvable) against the current content."
    )]
    async fn list_comments(&self, Parameters(a): Parameters<FileArg>) -> String {
        text(list_comments(&self.repo, &a.file))
    }

    #[tool(description = "Get one comment and its full reply thread.")]
    async fn get_thread(&self, Parameters(a): Parameters<ThreadArg>) -> String {
        text(get_thread(&self.repo, &a.file, &a.id))
    }

    #[tool(description = "Add an agent-authored comment anchored at a 1-based line of a file.")]
    async fn add_comment(&self, Parameters(a): Parameters<AddArg>) -> String {
        let _w = self.writes.lock().unwrap_or_else(|e| e.into_inner());
        text(add_comment(&self.repo, &a.file, a.line as usize, &a.body))
    }

    #[tool(description = "Append an agent-authored reply to a comment's thread.")]
    async fn reply(&self, Parameters(a): Parameters<ReplyArg>) -> String {
        let _w = self.writes.lock().unwrap_or_else(|e| e.into_inner());
        text(reply(&self.repo, &a.file, &a.id, &a.body))
    }

    #[tool(description = "Set (default) or clear a comment's resolved flag.")]
    async fn resolve(&self, Parameters(a): Parameters<ResolveArg>) -> String {
        let _w = self.writes.lock().unwrap_or_else(|e| e.into_inner());
        text(resolve(&self.repo, &a.file, &a.id, a.value.unwrap_or(true)))
    }
}

#[tool_handler]
impl ServerHandler for CommentServer {
    fn get_info(&self) -> ServerInfo {
        // ServerInfo is #[non_exhaustive]; build from Default and set fields.
        let mut info = ServerInfo::default();
        info.instructions = Some(
            "cospan comment layer: read and write anchored comments on files. Comments \
             re-localize as files change (anchored/drifted/unresolvable)."
                .into(),
        );
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info
    }
}

/// Serve the comment MCP over stdio until the client disconnects. Blocks; builds
/// its own tokio runtime so the rest of cospan stays synchronous.
pub fn run(repo: PathBuf) -> Result<(), String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async move {
        let service = CommentServer::new(repo)
            .serve(stdio())
            .await
            .map_err(|e| e.to_string())?;
        service.waiting().await.map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("cospan-mcp-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("src")).unwrap();
        std::fs::write(d.join("src/a.rs"), "l0\nl1\nl2\nl3\n").unwrap();
        d
    }

    #[test]
    fn add_list_reply_resolve_round_trip() {
        let repo = tmp("roundtrip");
        // add_comment (agent-authored) at line 2.
        let r = call_tool(
            &repo,
            "add_comment",
            &json!({"file":"src/a.rs","line":2,"body":"hot?"}),
        )
        .unwrap();
        assert_eq!(r["state"], "anchored");
        assert_eq!(r["line"], 2);
        let id = r["id"].as_str().unwrap().to_string();

        // list_comments sees it, authored by an agent.
        let list = call_tool(&repo, "list_comments", &json!({"file":"src/a.rs"})).unwrap();
        let items = list["comments"].as_array().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0]["author"]["who"], "agent");
        assert_eq!(items[0]["body"], "hot?");

        // reply, then get_thread shows it.
        call_tool(
            &repo,
            "reply",
            &json!({"file":"src/a.rs","id":id,"body":"cached upstream"}),
        )
        .unwrap();
        let thread = call_tool(&repo, "get_thread", &json!({"file":"src/a.rs","id":id})).unwrap();
        assert_eq!(thread["replies"][0]["body"], "cached upstream");
        assert_eq!(thread["replies"][0]["author"]["who"], "agent");

        // resolve toggles the flag.
        call_tool(&repo, "resolve", &json!({"file":"src/a.rs","id":id})).unwrap();
        let thread = call_tool(&repo, "get_thread", &json!({"file":"src/a.rs","id":id})).unwrap();
        assert_eq!(thread["resolved"], true);

        std::fs::remove_dir_all(&repo).ok();
    }

    #[test]
    fn add_comment_re_localizes_to_unresolvable_when_the_target_is_gone() {
        let repo = tmp("unresolvable");
        let r = call_tool(
            &repo,
            "add_comment",
            &json!({"file":"src/a.rs","line":2,"body":"note"}),
        )
        .unwrap();
        let id = r["id"].as_str().unwrap().to_string();
        // Rewrite the file so the anchored text no longer exists.
        std::fs::write(repo.join("src/a.rs"), "totally\ndifferent\ncontent\n").unwrap();
        let list = call_tool(&repo, "list_comments", &json!({"file":"src/a.rs"})).unwrap();
        assert_eq!(list["comments"][0]["state"], "unresolvable");
        assert!(
            list["comments"][0]["line"].is_null(),
            "unresolvable has no line"
        );
        // The comment is not silently dropped — honest ambiguity.
        assert_eq!(list["comments"][0]["id"], id);
        std::fs::remove_dir_all(&repo).ok();
    }

    #[test]
    fn agent_identity_comes_from_the_kan_agent_tag() {
        // Default is a generic "agent"; the env tag overrides it.
        std::env::remove_var("KAN_AGENT");
        assert_eq!(agent_author().id, "agent");
        std::env::set_var("KAN_AGENT", "claude-code:reviewer");
        assert_eq!(agent_author().id, "claude-code:reviewer");
        std::env::remove_var("KAN_AGENT");
    }

    #[test]
    fn unknown_tool_and_missing_args_error() {
        let repo = tmp("errors");
        assert!(call_tool(&repo, "nope", &json!({})).is_err());
        assert!(call_tool(&repo, "add_comment", &json!({"file":"src/a.rs"})).is_err()); // no line/body
        std::fs::remove_dir_all(&repo).ok();
    }

    #[test]
    fn path_traversal_is_refused() {
        let repo = tmp("traversal");
        // A `..`-escaping path and an absolute path are both refused, on read and
        // write, so an agent cannot touch files outside the repo.
        let r = call_tool(&repo, "list_comments", &json!({"file":"../../etc/passwd"})).unwrap();
        assert!(r.get("error").is_some(), "escaping read must error: {r}");
        let r = call_tool(
            &repo,
            "add_comment",
            &json!({"file":"/etc/passwd","line":1,"body":"x"}),
        )
        .unwrap();
        assert!(r.get("error").is_some(), "absolute write must error: {r}");
        // No sidecar was written outside the repo tree.
        assert!(!repo
            .join(".cospan/comments/../../etc/passwd.jsonl")
            .exists());
        std::fs::remove_dir_all(&repo).ok();
    }

    /// AC-1 (Slice C): the `_as` write cores stamp the supplied author on the
    /// record, while the plain `add_comment`/`reply` still stamp `agent_author()` —
    /// so a web write is `who:"human"` and the MCP path stays `who:"agent"`.
    #[test]
    fn write_cores_carry_the_supplied_author() {
        let repo = tmp("author");
        let web = Author {
            who: "human".into(),
            id: "web".into(),
        };
        // add_comment_as stamps the web author; reply_as too.
        let r = add_comment_as(&repo, "src/a.rs", 1, "from the phone", web.clone());
        let id = r["id"].as_str().unwrap().to_string();
        reply_as(&repo, "src/a.rs", &id, "and a reply", web.clone());
        let t = get_thread(&repo, "src/a.rs", &id);
        assert_eq!(t["author"]["who"], "human");
        assert_eq!(t["author"]["id"], "web");
        assert_eq!(t["replies"][0]["author"]["who"], "human");
        assert_eq!(t["replies"][0]["author"]["id"], "web");

        // The MCP path (plain add_comment) is unchanged: who:"agent".
        std::env::remove_var("KAN_AGENT");
        let r2 = add_comment(&repo, "src/a.rs", 2, "from an agent");
        let id2 = r2["id"].as_str().unwrap().to_string();
        let t2 = get_thread(&repo, "src/a.rs", &id2);
        assert_eq!(t2["author"]["who"], "agent");
        std::fs::remove_dir_all(&repo).ok();
    }

    /// AC-1 (Slice A): the comment index lists every file with a sidecar and its
    /// total/unresolved counts (no source read, no re-localization), an empty repo
    /// yields an empty list, and only repo-relative paths are surfaced.
    #[test]
    fn comment_files_indexes_sidecars_with_counts() {
        let repo = tmp("index");
        // No sidecars yet → empty index.
        let empty = comment_files(&repo);
        assert_eq!(
            empty["files"].as_array().unwrap().len(),
            0,
            "empty: {empty}"
        );

        // Two commented files: src/a.rs (1 open + 1 resolved), README.md (1 open).
        call_tool(
            &repo,
            "add_comment",
            &json!({"file":"src/a.rs","line":1,"body":"open one"}),
        )
        .unwrap();
        let r = call_tool(
            &repo,
            "add_comment",
            &json!({"file":"src/a.rs","line":2,"body":"to resolve"}),
        )
        .unwrap();
        let id = r["id"].as_str().unwrap().to_string();
        call_tool(&repo, "resolve", &json!({"file":"src/a.rs","id":id})).unwrap();
        std::fs::write(repo.join("README.md"), "hello\nworld\n").unwrap();
        call_tool(
            &repo,
            "add_comment",
            &json!({"file":"README.md","line":1,"body":"readme note"}),
        )
        .unwrap();

        let idx = comment_files(&repo);
        let files = idx["files"].as_array().unwrap();
        assert_eq!(files.len(), 2, "two commented files: {idx}");
        // Sorted by path: README.md before src/a.rs.
        assert_eq!(files[0]["file"], "README.md");
        assert_eq!(files[0]["total"], 1);
        assert_eq!(files[0]["unresolved"], 1);
        assert_eq!(files[1]["file"], "src/a.rs");
        assert_eq!(files[1]["total"], 2);
        assert_eq!(files[1]["unresolved"], 1, "one of two resolved");
        // Every listed path is repo-relative (no tree prefix leaks out).
        for f in files {
            let p = f["file"].as_str().unwrap();
            assert!(!p.contains(".cospan"), "path leaks the sidecar tree: {p}");
            assert!(!p.starts_with('/'), "path is absolute: {p}");
        }
        std::fs::remove_dir_all(&repo).ok();
    }

    /// AC-2 + review finding F1: `file_view` serves a normal file, rejects a
    /// directory and a binary, guards a traversal, and — the F1 fix — byte-caps a
    /// low-newline file without slicing a multibyte char mid-way or emitting
    /// mojibake.
    #[test]
    fn file_view_serves_guards_and_byte_caps() {
        let repo = tmp("fileview");

        // a normal small file: served, not truncated, with styled runs.
        let v = file_view(&repo, "src/a.rs");
        assert!(v.get("error").is_none(), "in-repo file served: {v}");
        assert_eq!(v["truncated"], false);
        assert_eq!(v["total"], 4, "l0..l3");
        assert!(v["lines"].as_array().is_some_and(|l| l.len() == 4));

        // a directory and a traversal both error, not panic.
        assert!(
            file_view(&repo, "src").get("error").is_some(),
            "directory errors"
        );
        assert!(
            file_view(&repo, "../../etc/passwd").get("error").is_some(),
            "traversal guarded"
        );

        // a binary file (invalid UTF-8 early) is rejected, not mojibake'd.
        std::fs::write(repo.join("bin"), [0x00u8, 0xff, 0xfe, 0x01, 0x02]).unwrap();
        assert!(
            file_view(&repo, "bin").get("error").is_some(),
            "binary rejected"
        );

        // F1: one line of 2× the byte cap is byte-capped; the shown text is bounded
        // by the cap, not the file size, and it is marked truncated.
        let huge = "x".repeat(FILE_VIEW_MAX_BYTES * 2);
        std::fs::write(repo.join("huge.txt"), &huge).unwrap();
        let hv = file_view(&repo, "huge.txt");
        assert_eq!(
            hv["truncated"], true,
            "byte-over file truncated: {}",
            hv["truncated"]
        );
        let shown: usize = hv["lines"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|l| l.as_array().unwrap())
            .map(|r| r["t"].as_str().unwrap().len())
            .sum();
        assert!(
            shown <= FILE_VIEW_MAX_BYTES,
            "shown text byte-capped: {shown}"
        );

        // A valid-UTF-8 file whose multibyte char straddles the cap must NOT be
        // rejected as binary: fill just under the cap with ASCII, then a 3-byte
        // char (\u{20AC} = €) that the cap slices. The valid prefix is accepted.
        let pad = FILE_VIEW_MAX_BYTES - 1; // 1 byte of the € lands inside the cap
        let mut straddle = "a".repeat(pad);
        straddle.push('\u{20AC}');
        straddle.push_str("bbbb");
        std::fs::write(repo.join("straddle.txt"), &straddle).unwrap();
        let sv = file_view(&repo, "straddle.txt");
        assert!(
            sv.get("error").is_none(),
            "boundary-cut UTF-8 must not be binary: {sv}"
        );
        assert_eq!(sv["truncated"], true);

        std::fs::remove_dir_all(&repo).ok();
    }

    /// AC-5: a repo-internal *symlink* that points outside the repo cannot be used
    /// to read past the guard — the lexical `..`/absolute check misses this, so the
    /// canonicalizing containment check must catch it, while a normal in-repo file
    /// is still accepted.
    #[test]
    #[cfg(unix)]
    fn symlink_escape_is_refused_but_in_repo_file_is_allowed() {
        use std::os::unix::fs::symlink;
        let repo = tmp("symlink");
        // `link` -> `/etc`, entirely inside the repo path-wise, resolves outside.
        let link = repo.join("link");
        let _ = std::fs::remove_file(&link);
        symlink("/etc", &link).unwrap();

        // Reading `link/hosts` has no `..` and is not absolute, so only the
        // symlink-resolving guard rejects it.
        let escaped = list_comments(&repo, "link/hosts");
        assert!(
            escaped.get("error").is_some(),
            "symlinked escape must error: {escaped}"
        );

        // A genuine in-repo file (no comments yet) is still served, not rejected.
        let ok = list_comments(&repo, "src/a.rs");
        assert!(ok.get("error").is_none(), "in-repo read must succeed: {ok}");
        assert_eq!(ok["file"], "src/a.rs");

        std::fs::remove_dir_all(&repo).ok();
    }
}
