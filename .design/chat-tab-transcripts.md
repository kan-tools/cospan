# Feature: Chat tab — watched transcripts (read) + a write seam

## Summary
Add the **Chat** tab: cross-harness agent session buffers rendered from the
harnesses' own on-disk transcripts, scoped to the repo cospan watches. The
transcript is the **highest-fidelity read** surface (the semantic conversation,
not a scraped terminal frame); it drops onto the existing poll-and-fold tick as
one more change-gated source. A separate `WriteChannel` trait is defined as the
**write seam** — designed now, implemented next — so input arrives later as the
first slice of the P3 "redirect" verb without re-architecting. This build ships
read-only; the write build is a follow-on.

## Requirements
- REQ-1: `View` (`src/tui.rs`) gains `Chat` as the first tab. `View::from_digit`
  maps `1`/`2`/`3`/`4` to `Chat`/`Comments`/`Ledger`/`Process` and `5` to `None`;
  `View::next` cycles `Chat → Comments → Ledger → Process → Chat`. The digit arm
  in `run` (today `'1'..='3'`) widens to `'1'..='4'`, and `view_header` renders
  Chat first with its own key legend.
- REQ-2: A new `transcripts` module defines a normalized session model —
  `Session { harness, id, repo_path, git_branch, title, last_active, events }` and
  `Event { role, kind, ts, id, parent, is_sidechain, text }` (`Role` =
  User/Assistant/System/Tool; `EventKind` = Message/ToolCall/ToolResult/Meta) —
  and a `TranscriptSource` trait with `discover(repo)`, `read(handle)`, and
  `change_signal(repo)`. The module is independent of `substrate::Fold`:
  transcripts are external substrate, never merged into the kan projection
  (`telos/kan-is-truth`).
- REQ-3: A `ClaudeCode` `TranscriptSource` reads
  `~/.claude/projects/<escaped-cwd>/*.jsonl`, where `<escaped-cwd>` is the watched
  repo path with `/` replaced by `-`. It parses per-line events, threads them by
  `parentUuid`, and sets `is_sidechain` from the event's `isSidechain` flag.
- REQ-4: A `Codex` `TranscriptSource` walks `~/.codex/sessions/**/rollout-*.jsonl`,
  keeps only rollouts whose `session_meta.payload.cwd` equals the watched repo,
  and maps the `{type, timestamp, payload}` envelope — `response_item` message
  items and `event_msg` — into the normalized model, taking `git.branch` from
  `session_meta`.
- REQ-5: An `Opencode` `TranscriptSource` opens
  `~/.local/share/opencode/opencode.db` **read-only**, selects `session` rows
  whose `directory` equals the watched repo and their `message`/`part` rows, and
  derives `change_signal` from the `opencode.db-wal` mtime. It reads **only** the
  `session`, `message`, `part`, and `project` tables — never the `account`,
  `credential`, or `control_account` tables in that same database.
- REQ-6: The Chat view renders the selected session's events as a readable
  conversation: User/Assistant `Message` text in full, `ToolCall` as a one-line
  summary, and `is_sidechain` (subagent) turns and thinking collapsed to a
  one-line drill-down that `Enter` expands. A session list enumerates the repo's
  sessions across all three harnesses, each labeled by harness, title, and
  last-active, newest-active first.
- REQ-7: The `run` tick (`src/tui.rs`) gains a Chat refresh gate mirroring the
  Comments gate: when the Chat view is active it re-reads transcripts only when
  `transcripts::change_signal(repo)` — the newest mtime across the three stores —
  advances. No push channel is introduced (`telos/poll-dont-subscribe`).
- REQ-8: A `WriteChannel` trait (`fn send(&self, session, text) -> Result<(),
  String>`) is defined as the write seam mounted on the L4 command bus, with **no
  implementation** this build. Its doc names the primary target (Claude Code
  Remote Control message bus). No read-path code constructs or calls it.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts `View::from_digit` maps `1/2/3/4`
  to `Chat/Comments/Ledger/Process` and `5` to `None`, and `View::next` cycles
  `Chat → Comments → Ledger → Process → Chat`.
- [ ] AC-2: (covers REQ-2) A unit test builds a `Session` from an in-memory
  `TranscriptSource` fixture and asserts the normalized `Event` fields
  (role, kind, parent, is_sidechain); the `transcripts` parsing path takes no
  `Fold` and shells out to neither `kan` nor `day`.
- [ ] AC-3: (covers REQ-3) A unit test parses a Claude Code JSONL fixture holding
  a user turn, an assistant turn, a `tool_use`, and an `isSidechain` turn, and
  asserts the roles, the `parentUuid` threading, and `is_sidechain` set only on
  the sidechain turn.
- [ ] AC-4: (covers REQ-4) A unit test parses a Codex rollout fixture: the
  `session_meta` cwd and `git.branch` are extracted, `response_item` message
  items become the right-role events, and a second fixture with a non-matching
  cwd is excluded by discovery.
- [ ] AC-5: (covers REQ-5) A unit test over an opencode-shaped SQLite fixture
  asserts sessions are selected by `directory` and messages mapped; and that the
  adapter's table set is a fixed allowlist of `session`/`message`/`part`/`project`
  (a test fails if `account`/`credential`/`control_account` appears in any query).
- [ ] AC-6: (covers REQ-6) A unit test over a `Session` asserts the render
  collapses thinking and `is_sidechain` events to summary lines by default and
  expands them when the event's `expanded` flag is set, and that `ToolCall`
  renders as a single summary line.
- [ ] AC-7: (covers REQ-7) A unit test asserts `transcripts::change_signal`
  returns the maximum mtime across sources and that the gate re-reads only when it
  advances (the `should_refold` pattern).
- [ ] AC-8: (covers REQ-8) A test asserts the `WriteChannel` trait exists with the
  `send` signature and that no implementor is constructed in the view/run path
  (the seam is present but inert).

## Architecture
Today `View` (`src/tui.rs:143`) is `Comments`/`Ledger`/`Process`, switched by
digit keys and `Tab`, dispatched in `draw` and the `run` loop; `View::from_digit`
and `View::next` encode the order. Adding `Chat` as the first tab is the same
shape as the recorded UI vision (`.dropbox/05-views-ux.md`): widen `from_digit`
to `1..=4`, extend `next`, widen the `'1'..='3'` digit arm to `'1'..='4'`, add a
`draw_chat` dispatch and a `view_header` entry.

The read layer is a **new `transcripts` module**, deliberately parallel to
`comments.rs` (cospan's other non-kan substrate) and deliberately *not* part of
`substrate::Fold` (`src/substrate.rs:585`), which is strictly the kan projection.
`telos/kan-is-truth` holds because transcripts are a projection of an external
append-only log the harness owns — cospan reads them, synthesizes nothing, and
persists nothing. `AppState` grows a `chat: ChatState` (session list + selection +
per-event expand flags) sitting beside, not inside, `fold`.

Three harnesses, three real storage shapes, one trait:

- **Claude Code** — per-session JSONL at `~/.claude/projects/<escaped-cwd>/*.jsonl`
  (`/`→`-`). Events carry `type` (`user`/`assistant`/`system`/…), `parentUuid`,
  `isSidechain`, `cwd`, `gitBranch`, `timestamp`. The whole in-session message
  tree and provenance fall out of the file directly.
- **Codex** — date-partitioned rollout JSONL at
  `~/.codex/sessions/YYYY/MM/DD/rollout-*.jsonl`. Envelope `{type, timestamp,
  payload}`: the first line is `session_meta` (carrying `cwd`, `git.branch`,
  `session_id`, `parent_thread_id`); the body is `response_item` (Responses-API
  message items with `role` + `content`) and `event_msg`. Discovery filters by
  `session_meta.cwd == repo`.
- **opencode** — a WAL-mode SQLite DB at `~/.local/share/opencode/opencode.db`.
  `session(id, directory, path, title, parent_id, time_updated, …)` keys sessions
  to a repo by `directory`; `message(id, session_id, data)` and `part(id,
  message_id, session_id, data)` hold the turns. The adapter opens the DB
  read-only, polls the `.db-wal` mtime for `change_signal`, and touches only the
  session/message/part/project tables. The same DB also holds `account` /
  `credential` / `control_account` rows with live access/refresh tokens; the
  adapter's table allowlist keeps cospan from ever reading them.

`TranscriptSource::change_signal(repo)` returns each source's newest relevant
mtime; `transcripts::change_signal` maxes them. The `run` tick (`src/tui.rs:1017`)
gets a Chat gate exactly like the Comments re-localize gate (`src/tui.rs:1025`)
and the footer gate — re-read on change, only while Chat is the active view. No
new watcher primitive; the same debounced poll drives it.

The **write seam** is a `WriteChannel` trait mounted on the L4 command bus
(`.dropbox/01-architecture.md`, "L4 — Command bus (stubbed)"). This build defines
the trait and documents the primary target — Claude Code Remote Control /
SendMessage, which injects a real structured user turn (higher fidelity than
scraped keystrokes and available with no workflow change) — but ships **no
implementor**. `send-keys` and PTY ownership are alternate impls the trait admits
later. Keeping the write build out of this cycle preserves the
`telos/observe-now-control-later` ordering: the read surface is observation
(P2, now); the write surface is control (P3, next), correlated to the transcript
cospan already reads.

The interactive render (session list, conversation, drill-down) gets a human
live-TTY eyeball before merge, per this repo's process rhythm; the parsing and
model layers are covered by the unit ACs above.

## Resolved Questions
- RQ-1: This build ships read-only Chat. `WriteChannel` is a defined-but-inert
  seam; its first implementation (the P3 "redirect" slice) is the next feature.
  This keeps `telos/observe-now-control-later` in order rather than pulling
  control into P2.
- RQ-2: Sessions are scoped to the **watched repo** but **cross-harness** —
  Claude Code, Codex, and opencode adapters ship together, each keyed to the repo
  by its own mechanism (escaped-cwd dir, `session_meta.cwd`, `session.directory`).
  "Cross-harness" is concrete this build, not aspirational.
- RQ-3: The primary write target is the **harness message bus** (Claude Code
  Remote Control), designed behind `WriteChannel`; it works today, needs no
  workflow change, and injects a structured turn rather than raw keystrokes.
  Multiplexer `send-keys` and PTY ownership are alternates the trait leaves open.
- RQ-4: Chat renders the **readable conversation** — User/Assistant text plus
  one-line tool-call summaries — with thinking and `is_sidechain` subagent turns
  as `Enter`-to-expand drill-downs, threaded within a session by `parentUuid`.
- RQ-5: Transcripts are external substrate, read in the new `transcripts` module
  entirely separate from `Fold`. `telos/kan-is-truth` is upheld: cospan projects
  an external log, invents no state, and persists nothing from it.
- RQ-6: The opencode adapter opens its DB **read-only** and reads only the
  session/message/part/project tables — never the account/credential tables in
  the same file (operational safety: cospan never reads a credential store).

## Open Questions

<!-- OPEN: Q1 -->
### Q1: opencode message/part body schema
`message.data` and `part.data` are opaque JSON blobs, and there are no local
opencode rows to sample (0 sessions on this machine). The `session` table is
legible enough to enumerate sessions, but the per-turn body shape (how role,
text, and tool parts are encoded inside `data`) is unconfirmed. Until it is
decoded from a live opencode session or from opencode's source, the opencode
adapter may **list** sessions and mark their bodies "unavailable" rather than
guess a parse — `telos/honest-ambiguity` over a fabricated read.
**To resolve**: sample a real opencode session in this repo (or read opencode's
message schema) and pin the `data` shape, then decide list-only vs full-body for
the first opencode ship.
<!-- /OPEN -->

<!-- OPEN: Q2 -->
### Q2: write-turn identity (deferred to the write build)
When the message-bus `WriteChannel` lands, does the injected turn appear in the
target harness's own log as the human operator, or as cospan acting on the
operator's behalf? Attribution writes into a log cospan does not own, so it is a
real identity decision, not cosmetics.
**To resolve**: decide during the write build (next feature); out of scope for
this read-only cycle, recorded here so it is not lost.
<!-- /OPEN -->

## Out of Scope
- The `WriteChannel` **implementation** (the message-bus send) — the next
  feature; this build defines only the seam.
- The **cross-session dispatch hierarchy** (parent/child *between* sessions via
  `bridgeSessionId` / `parent_thread_id` / `parent_id`) — that is the separate
  constructed-hierarchy effort (`.dropbox/02-kan-day-integration.md`); this build
  renders each session's own in-session message tree only.
- `send-keys` and PTY-ownership write implementations.
- Any harness beyond the three named.
- Any change to Comments, Ledger, Process, the footer, or the kan `Fold`.
