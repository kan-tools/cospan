# Feature: comment threads (reply + resolve)

## Summary
Give sidecar comments a reply thread and a resolve action. A comment gains a
`thread: Vec<Reply>` (the field the `Comment` doc comment already anticipates),
the CLI gains `cospan comment reply` and `cospan comment resolve`, and the TUI
Comments view renders the selected comment's full thread — root plus indented,
attributed replies, with a resolved marker. Writes stay on the CLI/MCP path; the
TUI still only reads and re-anchors.

## Requirements
- REQ-1: `src/comments.rs` gains `pub struct Reply { author: Author, body: String,
  created_at: i64 }` and `Comment` gains `#[serde(default)] pub thread:
  Vec<Reply>`. The `serde(default)` keeps every existing sidecar record loadable
  unchanged — an old comment with no `thread` field loads with an empty thread.
- REQ-2: `cospan comment reply <file> <id> <body>` loads the file's sidecar,
  finds the comment by `id`, appends a `Reply` (author `who:"human"`,
  `id` from `$USER`, `created_at` now), and saves. An unknown id exits non-zero
  with a message. `cospan comment resolve <file> <id>` sets `resolved = true` and
  saves; both mirror `comment add`'s load→mutate→save shape in `src/main.rs`.
- REQ-3: `cospan comments <file>` lists each comment's reply count and resolved
  state alongside its live localization (e.g. `… (2 replies) [resolved]`).
- REQ-4: The TUI Comments view (`src/tui.rs`) renders the selected comment's
  thread in the detail strip via a pure `thread_lines(comment, loc)`: the root
  (state · line · author · body), then each reply as an indented `└ @author:
  body`, and a `[resolved]` tag when resolved. The gutter/detail also shows the
  reply count so a threaded comment is visible as such.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test deserializes a sidecar JSONL line written
  without a `thread` field (the pre-threads shape) into a `Comment` with an empty
  `thread`, and round-trips a `Comment` carrying two replies through
  `save`/`load`.
- [ ] AC-2: (covers REQ-2) A unit test appends a reply to a comment by id and
  asserts the reloaded comment has the reply with the right author/body; and a
  test asserts `resolve` sets `resolved`. A reply to an unknown id is a no-op that
  reports an error (returns an `Err`/`false` from the helper the CLI calls).
- [ ] AC-3: (covers REQ-4) A unit test on `thread_lines` for a resolved comment
  with one reply asserts the produced lines contain the root body, an indented
  reply line naming its author and body, and a `[resolved]` marker.
- [ ] AC-4: (covers REQ-3) A unit test on the list-summary helper asserts a
  comment with two replies and `resolved = true` renders a suffix naming both
  the reply count and the resolved state (e.g. `(2 replies) [resolved]`).

## Architecture
The sidecar (`src/comments.rs`) is cospan's one owned mutable state, ephemeral
JSONL at `.cospan/comments/<path>.jsonl`, one `Comment` per line. `Comment`
already carries `resolved: bool` and its doc comment names a thread that was never
added; this feature adds it. `Reply` reuses the existing `Author { who, id }`.
`#[serde(default)]` on `thread` is the whole back-compat story — serde fills an
empty `Vec` for records written before the field existed, so `load` (which
already skips malformed lines) needs no change. A small
`reply(comments, id, reply) -> bool` / `resolve(comments, id) -> bool` helper
finds the comment by id and mutates in place, returning whether it matched, so
the CLI can report an unknown id without duplicating the search.

The CLI (`src/main.rs`) `comment_cmd` currently accepts only `add`; it grows
`reply` and `resolve` arms that load the sidecar, call the helper, and save —
the same read→mutate→`comments::save` flow `add` uses, with the `$USER`/human
author `add` already builds. `comments_cmd`'s per-line print gains the reply
count and a `[resolved]` suffix.

The TUI (`src/tui.rs`) `draw_comments` detail strip currently prints the selected
comment's one-line state + body. It calls a new pure `thread_lines(&Comment,
&Localization) -> Vec<Line<'static>>` that emits the root line, the body, each
reply indented and attributed, and a `[resolved]` tag — unit-testable without a
terminal, like `gutter_lines`. The gutter marker/detail notes the reply count.
The view still never writes a reply — authoring stays on the CLI (and the later
MCP path); the TUI reads threads and re-anchors, holding the read-only-view line
(`telos/observe-now-control-later`).

Nothing here touches kan; threads live only in the ephemeral sidecar until the
opt-in persist-to-kan path (out of scope) maps them onto a `cospan-comment`
block's thread state.

## Resolved Questions
- RQ-1: Replies are authored only via the CLI (and later MCP), never from the
  TUI, keeping the observation view read-only; the TUI displays threads. This
  matches how `comment add` already works and `telos/observe-now-control-later`.
- RQ-2: Back-compat is handled by `#[serde(default)]` on the new field rather
  than a schema version bump — a pre-threads sidecar loads unchanged.
- RQ-3: `resolve` toggles the existing `resolved` flag on (no un-resolve in this
  cut); the resolved state renders but does not hide or reorder the comment.

## Open Questions

_None — the model, the CLI surface, and the read-only-view boundary were resolved
during design._

## Out of Scope
- Agent-authored replies via the comment MCP server (the write path is a later
  P1/P2 piece); this cycle is the model + human CLI + display.
- Authoring or resolving comments from inside the TUI (P3 command bus).
- Persisting threads to kan (the opt-in `cospan-comment` block path).
- Un-resolving, editing, or deleting replies.
