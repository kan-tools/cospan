# Feature: Chat reads over the API (Chat tab — mobile)

## Summary
Surface the agent chat/transcript sessions over `cospan serve`, read-only: a
`GET /chat` index of the repo's sessions and `GET /chat?session=<id>` for one
session's turns, feeding a **Chat** tab in the embedded page. It completes the
`B → A → C → Chat` roadmap and the mobile-frontend vision's four read tabs
(Chat · Comments · Ledger · Process). Serialization is a hand-built projection
that **never emits local `$HOME` file paths** (the `Locator`), honoring
`telos/disposable` and the operational rule that cospan does not leak the machine's
paths; it stays read-only (`telos/observe-now-control-later` — sending messages to
agents is the later control tier).

## Requirements
- REQ-1: **Chat index.** `chat::chat_index(repo)` (new `src/chat.rs`) returns
  `{ "sessions": [ … ] }` from `transcripts::discover_all(repo)` (`src/transcripts.rs`),
  most-recently-active first, each projected to only safe fields — `harness` (its
  `label()`), `id`, `title`, `git_branch`, `last_active` (epoch-millis, or null),
  `body_available`, `group`, `is_subagent`. The `Locator` and any filesystem path
  are **never** serialized.
- REQ-2: **Chat session.** `chat::chat_session(repo, id)` finds the handle in
  `discover_all` whose `id` matches, reads it via `transcripts::read`
  (`src/transcripts.rs`), and returns `{ harness, id, title, git_branch, events:
  [ { role, kind, ts, is_sidechain, text } ] }` — **every** event, tagged by its
  `kind` (`message`/`thinking`/`toolcall`/`toolresult`/`meta`) so the client
  decides what to collapse. An unknown id returns `{ "error": … }`. No path leaks.
- REQ-3: **Endpoint.** `GET /chat` with no `session` → the index; with
  `?session=<id>` → that session, via a `get_chat` handler (`src/server.rs`)
  running the core under `spawn_blocking` (discovery/read shell the filesystem).
  It sits behind the Slice-B auth middleware like every route, and follows the
  resource-collection grammar Slice A set (`GET /<collection>` lists,
  `?sel` drills in) — the `GET /chat` the Slice-A design explicitly named.
- REQ-4: **Chat tab.** A fifth bottom-nav tab in `src/web/index.html` lists the
  sessions (title, a harness badge, relative last-active) and on tapping one loads
  its turns: `message` events render as readable bodies; `thinking`/`toolcall`/
  `toolresult` events render collapsed as a one-line marker that expands on tap.
  All fetched with the page's `?token=` via `withTok`; rendering stays
  `textContent`/`createElement`.
- REQ-5: **Read-only, path-safe, disposable.** No chat write path exists (sending
  to an agent is the control tier, out of scope); `command_bus::WriteChannel`
  is untouched; no response includes a `Locator` or `$HOME` path; nothing is
  persisted. `chat_index`/`chat_session` are pure reads of the transcript stores.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-5) A unit test on the handle projection: given a
  `SessionHandle` carrying `Locator::File("/Users/x/.claude/projects/…/s.jsonl")`,
  the JSON has `harness`/`id`/`title`/`last_active` (millis) but contains **no**
  `.jsonl`, no `/Users/`, no `.claude`, and no `locator` key.
- [ ] AC-2: (covers REQ-2, REQ-5) A unit test on the session projection: a
  `Session` with a `message`, a `thinking`, and a `toolcall` event projects to an
  `events` array preserving all three with their `kind` labels and `text`, and the
  JSON contains no filesystem path.
- [ ] AC-3: (covers REQ-3) A CI integration test (no kan; a temp repo with no
  transcripts) asserts `GET /chat` returns `{"sessions":[…]}` (an array — empty is
  fine) and `GET /chat?session=nope` returns `{"error":…}`, both `200`, and that a
  request with auth on but no token is `401`.
- [ ] AC-4: (covers REQ-4) A unit test asserts `INDEX_HTML` wires the Chat tab —
  `data-view="chat"`, a `/chat` fetch, and a collapse affordance for
  thinking/tool events (a `collapses`/`kind`-driven marker).
- [ ] AC-5: (covers REQ-1, REQ-2, REQ-5) A unit test asserts neither projection
  emits a `locator` field nor any string containing `/Users/` or `.jsonl`, over a
  handle and a session constructed with real-looking local paths — the path-leak
  guard, isolated.

## Architecture
**Read core (`src/chat.rs`, new).** Two projection functions mirror
`mcp::comment_json`'s shape: `handle_json(&SessionHandle) -> Value` and
`session_json(&Session) -> Value`, each emitting an explicit allow-list of fields
and **omitting the `Locator`** — so a path cannot leak by accident. `chat_index`
maps `transcripts::discover_all` through `handle_json`; `chat_session` finds the
matching handle, `transcripts::read`s it, and runs it through `session_json`.
`last_active: Option<SystemTime>` becomes optional epoch-millis via
`duration_since(UNIX_EPOCH)`. Enum values use the existing `.label()` methods
(`Harness::label`, `Role::label`, `EventKind::label` — the last added if absent),
so the wire strings are the lowercase labels the TUI already uses. Registered as
`pub mod chat;` in `src/lib.rs`.

**Server (`src/server.rs`).** A `ChatQuery { session: Option<String> }` and a
`get_chat` handler: `None` → `chat::chat_index(&repo)`, `Some(id)` →
`chat::chat_session(&repo, &id)`, under `spawn_blocking` (discovery reads files
and, for opencode, a SQLite DB). Mounted `.route("/chat", get(get_chat))` inside
the auth-wrapped router. This is a read; it never writes.

**Page (`src/web/index.html`).** A `Chat` tab joins Now/Teloi/Browse/Comments in
the nav and `setView`; opening it fetches `/chat` and lists sessions; tapping one
fetches `/chat?session=<id>` and renders the turns — `message` bodies shown,
collapsing events shown as a dim expandable marker keyed on `kind`. Fetched on tab
entry (chat is not carried on `/stream`), reusing the `withTok`/`backBar` helpers
from the Comments tab.

This reads the transcript stores the TUI Chat view already reads; it writes
nothing and, unlike the TUI, deliberately never puts a store path on the wire
(`telos/disposable`, and the recorded rule that cospan does not surface the
machine's paths). It is orthogonal to the fold — `/chat` is its own collection,
not part of `/fold` or `/stream`.

## Resolved Questions
- RQ-1: **All events, kind-tagged.** `chat_session` returns every event with its
  `kind`, and the client collapses `thinking`/`toolcall`/`toolresult` — honest and
  matching the TUI — rather than the server dropping them to messages-only.
- RQ-2: **Hand-built projection, paths omitted.** A `chat_index`/`chat_session`
  projection (not blanket `#[derive(Serialize)]` on the transcript types) emits an
  explicit safe field set and never the `Locator`, so a local `$HOME` path cannot
  leak — worth the small extra code over coupling the wire to internal structs.
- RQ-3: **`GET /chat` resource-collection**, read-only. The collection Slice A
  named; sending a message to an agent is the control tier and out of scope.

## Out of Scope
- **Sending messages to agents / any chat write** — the control tier
  (`command_bus::WriteChannel`), a later phase with its own authz; this slice is
  read-only.
- **The in-session threading rail / subagent nesting** the TUI draws — the index
  carries `group`/`is_subagent`, but the mobile view renders a flat turn list for
  now, not the nested rail.
- **Live push of chat updates over `/stream`** — `/stream` carries only `Fold`
  refolds; the Chat tab re-fetches on entry, like the Comments tab.
- **Exposing transcript file paths / the `Locator`** — never serialized.
- **Decoding the opencode body schema** where `body_available` is false — such a
  session lists but its turns read as unavailable (`telos/honest-ambiguity`),
  unchanged by this slice.
