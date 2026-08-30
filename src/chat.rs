//! Chat reads over the API: the transcript-store projection the `GET /chat`
//! endpoint and the mobile Chat tab consume.
//!
//! This is a *read* of the same agent transcript stores the TUI Chat view reads
//! (`crate::transcripts`), projected to JSON by hand — an explicit allow-list of
//! fields that **never** includes a session's `Locator` or any local `$HOME`
//! file path. That omission is the point: unlike the TUI, the API must not put
//! the machine's paths on the wire (`telos/disposable`, and the recorded rule
//! that cospan does not surface local paths). It writes nothing.

use crate::transcripts::{self, Event, Session, SessionHandle};
use serde_json::{json, Value};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// `Option<SystemTime>` → optional epoch-millis, so a timestamp crosses the wire
/// as a plain number the client can render ("2h ago") without a path or a
/// platform-specific encoding.
fn millis(t: Option<SystemTime>) -> Value {
    match t.and_then(|t| t.duration_since(UNIX_EPOCH).ok()) {
        Some(d) => json!(d.as_millis() as u64),
        None => Value::Null,
    }
}

/// Project a discovered session handle to its safe wire fields. The `Locator`
/// (a file path or SQLite row) is deliberately omitted — a client references a
/// session by `id`, and the server re-discovers it to read.
fn handle_json(h: &SessionHandle) -> Value {
    json!({
        "harness": h.harness.label(),
        "id": h.id,
        "title": h.title,
        "git_branch": h.git_branch,
        "last_active": millis(h.last_active),
        "body_available": h.body_available,
        "group": h.group,
        "is_subagent": h.is_subagent,
    })
}

/// Project one turn — every event, tagged by `kind`, so the client decides what
/// to collapse (thinking/tool) versus show (messages).
fn event_json(e: &Event) -> Value {
    json!({
        "role": e.role.label(),
        "kind": e.kind.label(),
        "ts": e.ts,
        "is_sidechain": e.is_sidechain,
        "text": e.text,
    })
}

/// Project a fully-read session to `{ harness, id, title, git_branch, events }`.
fn session_json(s: &Session) -> Value {
    json!({
        "harness": s.harness.label(),
        "id": s.id,
        "title": s.title,
        "git_branch": s.git_branch,
        "events": s.events.iter().map(event_json).collect::<Vec<_>>(),
    })
}

/// `GET /chat` — the index of the repo's chat sessions, most-recently-active
/// first, each projected without any local path.
pub fn chat_index(repo: &Path) -> Value {
    let sessions: Vec<Value> = transcripts::discover_all(repo)
        .iter()
        .map(handle_json)
        .collect();
    json!({ "sessions": sessions })
}

/// `GET /chat?session=<id>` — one session's turns. Finds the handle with that id
/// across the harnesses, reads it, and projects it; an unknown id is an error.
pub fn chat_session(repo: &Path, id: &str) -> Value {
    match transcripts::discover_all(repo)
        .into_iter()
        .find(|h| h.id == id)
    {
        Some(handle) => session_json(&transcripts::read(&handle)),
        None => json!({ "error": format!("no chat session {id}") }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcripts::{EventKind, Harness, Locator, Role};
    use std::path::PathBuf;

    fn sample_handle() -> SessionHandle {
        SessionHandle {
            harness: Harness::ClaudeCode,
            id: "sess-1".into(),
            title: "a session".into(),
            git_branch: Some("main".into()),
            last_active: Some(UNIX_EPOCH + std::time::Duration::from_millis(1_700_000_000_000)),
            locator: Locator::File(PathBuf::from(
                "/Users/m/.claude/projects/-Users-m-code/sess-1.jsonl",
            )),
            body_available: true,
            group: None,
            is_subagent: false,
        }
    }

    fn sample_session() -> Session {
        let ev = |kind: EventKind, text: &str| Event {
            role: Role::Assistant,
            kind,
            ts: Some("2026-08-30T00:00:00Z".into()),
            id: None,
            parent: None,
            is_sidechain: false,
            text: text.into(),
        };
        Session {
            harness: Harness::ClaudeCode,
            id: "sess-1".into(),
            title: "a session".into(),
            git_branch: Some("main".into()),
            events: vec![
                Event {
                    role: Role::User,
                    ..ev(EventKind::Message, "hello")
                },
                ev(EventKind::Thinking, "hmm"),
                ev(EventKind::ToolCall, "grep foo"),
            ],
        }
    }

    /// AC-1: the handle projection carries the safe fields and NO path/locator.
    #[test]
    fn handle_projection_has_safe_fields_and_no_path() {
        let v = handle_json(&sample_handle());
        assert_eq!(v["harness"], "claude");
        assert_eq!(v["id"], "sess-1");
        assert_eq!(v["title"], "a session");
        assert_eq!(v["last_active"], 1_700_000_000_000u64);
        assert!(v.get("locator").is_none(), "locator must not be serialized");
        let s = v.to_string();
        assert!(!s.contains(".jsonl"), "no transcript path: {s}");
        assert!(!s.contains("/Users/"), "no home path: {s}");
        assert!(!s.contains(".claude"), "no store path: {s}");
    }

    /// AC-2: the session projection keeps every event, kind-tagged, no path.
    #[test]
    fn session_projection_keeps_all_kinds_tagged() {
        let v = session_json(&sample_session());
        let events = v["events"].as_array().unwrap();
        assert_eq!(events.len(), 3, "all three events kept");
        assert_eq!(events[0]["kind"], "message");
        assert_eq!(events[0]["role"], "user");
        assert_eq!(events[0]["text"], "hello");
        assert_eq!(events[1]["kind"], "thinking");
        assert_eq!(events[2]["kind"], "toolcall");
        assert!(
            !v.to_string().contains("/Users/"),
            "no path in session json"
        );
    }

    /// AC-5: the path-leak guard, isolated — neither projection emits a `locator`
    /// or any string containing a local path, over real-looking inputs.
    #[test]
    fn projections_never_leak_a_local_path() {
        for s in [
            handle_json(&sample_handle()).to_string(),
            session_json(&sample_session()).to_string(),
        ] {
            assert!(!s.contains("locator"), "no locator key: {s}");
            assert!(!s.contains("/Users/"), "no /Users/ path: {s}");
            assert!(!s.contains(".jsonl"), "no .jsonl path: {s}");
        }
    }
}
