# Feature: The comment authoring surface — the writable comment round trip

## Summary
Turn cospan's comment layer from read-only-in-the-TUI into a full human
authoring surface and agent access surface: browse any file with syntax
highlighting, add/reply/resolve/edit/delete comments interactively, promote a
comment (or a file's whole set) into a durable kan claim on an explicit keypress,
and expose the same read+write operations to agents over a `cospan mcp` server.
This is a **milestone** decomposed into five sequenced build slices over the
already-mature sidecar store and re-localizer (`src/comments.rs`, `src/lib.rs`);
the lead slice is interactive human authoring on the existing gutter view.

The target telos is `telos/comment-roundtrip`, whose recorded statement is scoped
to the *headless* round trip — this milestone makes that round trip writable from
the TUI and reachable by agents, so the telos is revised in Phase 6. Comment
writes are cospan's own owned state (`telos/kan-is-truth`), not the agent
spawn/kill/redirect control that `telos/observe-now-control-later` defers, so no
tension is recorded against that telos; the promote-to-kan slice does touch the
existing `disposable ↔ kan-is-truth` tension and relates to it explicitly.

## Requirements

**S1 — Interactive authoring in the TUI (lead slice).**
- REQ-1: The Comments view (`View::Comments`, `src/tui.rs`) gains a compose input
  mode. A new `AppState` field holding an optional compose buffer (kind:
  new-comment | reply | edit, target line or comment id, and the in-progress text)
  drives it, alongside the existing comment fields at `src/tui.rs:118`.
- REQ-2: `a` (or `i`) on a source line in `comment_content` opens a new-comment
  compose; on submit it captures a fingerprint anchor from the file content at that
  line via `Anchor::from_file` / `StoredAnchor::capture` (`src/comments.rs:32`),
  appends a `Comment` (`src/comments.rs:73`) authored `who:"human"`, `id:$USER`,
  and persists it with `comments::save` (`src/comments.rs:168`).
- REQ-3: `r` on the selected comment opens a reply compose; on submit it calls
  `Comment::add_reply` (`src/comments.rs:89`) and persists.
- REQ-4: `x` toggles `resolved` on the selected comment via `Comment::resolve`
  (`src/comments.rs:100`), including un-resolve (toggle back off).
- REQ-5: `e` edits the selected comment's own body: it opens compose pre-filled,
  and on submit rewrites the body and re-captures the anchor via
  `StoredAnchor::capture`, then persists. Editing is gated to the caller's own
  comments (`author.id == $USER`); a foreign comment cannot be edited.
- REQ-6: `d` deletes the selected comment: it removes exactly that record from the
  sidecar and persists via `comments::save`. Deletion is gated to the caller's own
  comments; a foreign comment cannot be deleted.
- REQ-7: Compose is a hand-rolled **multi-line** editor (ratatui has no textarea):
  it accepts newlines and cursor movement, wraps for display, and stores the body
  verbatim including embedded newlines, so a paragraph of writing feedback is a
  single comment.
- REQ-8: All authoring goes through the same sidecar path the CLI uses
  (`comments::sidecar_path` at `src/comments.rs:134`), and the view re-reads and
  re-localizes after every write through the existing `refresh_comments`
  (`src/tui.rs:551`), so a TUI-authored comment is byte-identical in structure to a
  `cospan comment add` one and re-localizes on the next tick.
- REQ-9: This reverses the settled `comment-threads` RQ-1 ("replies authored only
  via CLI, never from the TUI") and `comment-threads` RQ-3 (resolve-only, no
  un-resolve); the design records both reversals and their reason (comments are
  cospan's owned state, not agent control) so the change is traceable rather than
  silent.

**S2 — File browser with git state.**
- REQ-10: A new filesystem-tree module walks the watched repo and produces a
  collapsible tree of files/dirs, replacing the commented-files-only rail built by
  `commented_files` / `collect_sidecars` (`src/tui.rs:1008`) so any file can be
  opened for commenting, not only files that already have a sidecar.
- REQ-11: Each entry carries its git working-tree status, obtained by shelling
  `git status --porcelain` (consistent with the shell-out spine used by
  `substrate::fold` at `src/substrate.rs:720`), mapped to per-path markers
  (modified / added / untracked / clean).
- REQ-12: The tree is polled on the same tick and mtime-gated like the rest of the
  model (`should_refold` / `head_mtime` pattern at `src/tui.rs:1164`), never a new
  watch thread (`telos/poll-dont-subscribe`).
- REQ-13: Selecting a file loads its content into `comment_content` and its sidecar
  (if any) into `comment_localized`, reusing `select_comment_file`
  (`src/tui.rs:992`); files with no sidecar open with an empty comment set that
  authoring (S1) can populate.

**S3 — Syntax highlighting (syntect), markdown + tex first.**
- REQ-14: Add `syntect` to `Cargo.toml` (currently zero highlighting deps —
  `crossterm`, `pulldown-cmark`, `ratatui`, `serde`, `serde_json`) and a new
  highlight module that maps a file's content + extension to styled lines.
- REQ-15: The gutter renderer `gutter_lines` (`src/tui.rs:1053`), which today emits
  `Span::raw(text)` per source line (`src/tui.rs:1099`), renders syntect-highlighted
  spans instead, preserving the leading state marker `●` and the selected-line
  reverse styling.
- REQ-16: Markdown (`.md`) and LaTeX (`.tex`) are highlighted correctly (both are in
  syntect's default syntax set); unknown extensions fall back to plain spans, never
  erroring.
- REQ-17: syntect colors are converted to ratatui `Style`, degrading to the
  256-color palette when truecolor is unavailable, so the render is legible in a
  standard terminal.

**S4 — Promote-to-kan (the explicit human action).**
- REQ-18: A keypress (`P`) in the Comments view promotes the selected comment — or,
  with a modifier, the current file's whole comment set — into kan by shelling
  `kan observe --subject comment/<file-slug> "<text>" --file <path>:<start>-<end>`
  (verbs confirmed: `kan observe` carries `--subject`, `--file path:start-end`,
  `--cites`). A file-set promote writes one claim per comment, each keeping its own
  anchor, not one aggregate claim.
- REQ-19: The claim `text` carries a fenced `cospan-comment` JSON block (the house
  convention mirrored from day's `day-atom` blocks) holding the fingerprint
  (`StoredAnchor` target/before/after/base_hash), thread/replies, resolved flag, and
  original author — i.e. everything the kan workspace anchor does not.
- REQ-20: The kan anchor is the current file span at the current HEAD:
  `<path>:<start>-<end>` for the comment's localized span, so the claim records
  "this text, at this SHA, forever" while the sidecar keeps re-localizing.
- REQ-21: Promotion is a durable snapshot and never mutates or deletes the sidecar
  comment; the sidecar remains the live copy.
- REQ-22: Re-promoting an already-promoted comment appends a fresh immutable
  `Observation` and `--cites` the prior promoted claim's CID (an explicit snapshot
  chain), rather than mutating the prior claim or writing an unlinked duplicate.
- REQ-23: The promote path relates to the recorded `disposable ↔ kan-is-truth`
  tension (persisting ephemeral comments into kan trades disposability for
  durability); the design notes this and it is recorded in Phase 6.

**S5 — Comment MCP server (read + write).**
- REQ-24: A new `cospan mcp` subcommand (dispatched from `main`, `src/main.rs:26`)
  runs an MCP stdio server built on the official `rmcp` SDK, exposing read tools
  `list_comments` and `get_thread` over the sidecar store (`comments::load` at
  `src/comments.rs:140`, `thread_summary` at `src/comments.rs:112`). Adding `rmcp`
  introduces a tokio async runtime the crate does not have today.
- REQ-25: The server exposes write tools `add_comment`, `reply`, and `resolve` that
  call the same `src/comments.rs` API S1 uses, so an agent can drop a comment on a
  file another agent wrote.
- REQ-26: An agent write is attributed `who:"agent"` with an `id` read from a
  harness-set environment tag (e.g. `KAN_AGENT`); when the tag is absent the id
  falls back to a generic `agent` marker rather than a spoofable per-call argument.
- REQ-27: An agent-written comment supplies the same fingerprint (target + context)
  a human does; cospan re-localizes it into the human's view like any other comment,
  and ambiguous anchors surface as `Unresolvable` (`telos/honest-ambiguity`).
- REQ-28: When promote-to-kan (S4) is enabled by setting, an MCP-written comment
  additionally lands as the S4 kan Observation; the default stays sidecar-only.
- REQ-29: The MCP write tools operate on cospan's owned comment state only, not on
  agent lifecycle; the `WriteChannel` seam (`src/command_bus.rs:25`) for the harness
  message bus is untouched by this milestone.

## Acceptance Criteria
- [ ] AC-1: A unit test drives compose state: opening a new-comment compose on a
  given line and submitting appends exactly one `Comment` to the sidecar with an
  anchor whose `target` equals that line's text (REQ-1, REQ-2).
- [ ] AC-2: A test submits a reply through the compose path and asserts the selected
  comment's `thread` grew by one `Reply` with `who:"human"` (REQ-3).
- [ ] AC-3: A test toggles resolve on then off and asserts `resolved` returns to
  `false` (un-resolve) (REQ-4).
- [ ] AC-4: A test edits an own comment's body through compose and asserts the body
  changed and the anchor was re-captured; a second test asserts editing a comment
  whose `author.id` differs from `$USER` is refused (REQ-5).
- [ ] AC-5: A test deletes an own comment and asserts exactly that record is gone
  from the sidecar; a second test asserts deleting a foreign comment is refused
  (REQ-6).
- [ ] AC-6: A test enters a multi-line body (with an embedded newline) through
  compose and asserts the stored `body` contains the newline verbatim (REQ-7).
- [ ] AC-7: A test asserts a TUI-authored comment reloaded via `comments::load` is
  structurally identical to one written by the `comment_add` CLI path
  (`src/main.rs:149`), and that `refresh_comments` runs after a write so the new
  comment appears in `comment_localized` with a `Localization` state (REQ-8, REQ-9).
- [ ] AC-8: A test builds the file tree over a temp repo fixture and asserts it
  lists a file that has no sidecar, proving the browser is not sidecar-gated, and
  that the tree is produced without spawning a watch thread (REQ-10, REQ-12).
- [ ] AC-9: A test maps a `git status --porcelain` fixture string to per-path status
  markers and asserts modified/untracked/clean are distinguished (REQ-11).
- [ ] AC-10: A test selects a sidecar-less file and asserts `comment_content` loads
  and `comment_localized` is empty, ready for authoring (REQ-13).
- [ ] AC-11: A test highlights a `.md` and a `.tex` sample and asserts more than one
  distinct style is produced — real highlighting, not all-plain (REQ-14, REQ-16).
- [ ] AC-12: A test highlights an unknown extension and asserts it falls back to
  plain spans without error, and a color-depth test asserts truecolor degrades to
  256-color (REQ-16, REQ-17).
- [ ] AC-13: A test asserts the highlighted gutter line still begins with the state
  marker and that the selected line carries the reverse modifier (REQ-15).
- [ ] AC-14: A test builds the promote command for a comment and asserts the
  `--subject comment/<slug>`, the `--file <path>:<start>-<end>` anchor, and the
  fenced `cospan-comment` block (parseable JSON with fingerprint + thread +
  resolved) are all present (REQ-18, REQ-19, REQ-20).
- [ ] AC-15: A test asserts promoting does not modify or remove the sidecar comment
  — sidecar byte-identical before and after the promote call is built — preserving
  the live copy the disposability side of the tension needs (REQ-21, REQ-23).
- [ ] AC-16: A test asserts a re-promote builds a command carrying `--cites <prior
  CID>`, and that a file-set promote produces one command per comment (REQ-22).
- [ ] AC-17: An integration smoke against a scratch kan repo records a promoted
  comment and reads it back via `kan show --all --json` on subject `comment/<slug>`,
  asserting the anchor and block round-trip (REQ-18, REQ-19).
- [ ] AC-18: A test starts the `rmcp` server in-process, calls `list_comments` on a
  fixture file, and asserts the returned set matches `comments::load` (REQ-24).
- [ ] AC-19: A test calls `add_comment` over the server and asserts a `who:"agent"`
  comment lands in the sidecar and is readable by `get_thread`; a setting-on variant
  asserts it also produces the S4 kan Observation (REQ-25, REQ-28).
- [ ] AC-20: A test sets the `KAN_AGENT` tag, calls `add_comment`, and asserts the
  stored `author.id` equals the tag; a no-tag variant asserts the generic `agent`
  fallback (REQ-26).
- [ ] AC-21: A test calls the write tools with an ambiguous anchor and asserts the
  resulting comment re-localizes to `State::Unresolvable` rather than a guessed span
  (REQ-27).
- [ ] AC-22: A test asserts no read/write path in the MCP server constructs or calls
  `WriteChannel` (`src/command_bus.rs`), keeping the harness-control seam untouched
  (REQ-29).

## Architecture

The milestone sits entirely on top of the mature store: `src/comments.rs` (the
`Comment`/`StoredAnchor`/`Reply` model, sidecar JSONL at
`.cospan/comments/<path>.jsonl`, `add_reply`/`resolve`/`localize_and_update`) and
`src/lib.rs` (the `relocalize` engine and `Anchored`/`Drifted`/`Unresolvable`
states). None of the five slices touches the re-localizer or the append-only
kan fold invariant; S4 only *appends* observation claims via the `kan` CLI, and
the sidecar remains cospan's sole owned mutable state.

**S1 (authoring)** adds a multi-line input mode to `src/tui.rs`. ratatui has no
textarea, so compose is a small hand-rolled buffer with newline + cursor handling,
rendered as an overlay in `draw_comments` (`src/tui.rs:2691`) and dispatched from
the Comments-view key handler (`src/tui.rs:2237`). It ships full CRUD — add,
reply, resolve/un-resolve, edit-own, delete-own — each gated to the caller's own
`author.id`. Every submit calls the existing `src/comments.rs` API and then
`refresh_comments`, so the write path and the read path stay the same code the CLI
already exercises. This is the smallest read→write flip and the reason it leads.

**S2 (browser)** adds a filesystem-tree module (proposed `src/filetree.rs`, new)
that walks the repo and shells `git status --porcelain`. It replaces the
sidecar-gated rail so authoring is possible on any file. It follows the
poll-and-mtime discipline of `src/substrate.rs` rather than adding a watcher.

**S3 (highlighting)** adds `syntect` and a highlight module (proposed
`src/highlight.rs`, new) converting syntect styles to ratatui styles, wired into
`gutter_lines`. It is independent of S2 but pairs with it to form the "editor
surface" (the `.dropbox/05-views-ux.md` editor-view vision); together S2+S3 let
you browse → see highlighted → comment.

**S4 (promote)** is the `telos/kan-is-truth` "explicit human action." It shells
`kan observe` with a `comment/<file-slug>` subject, a `path:start-end` workspace
anchor, and a fenced `cospan-comment` block, exactly as `.dropbox/03-comments.md`
specifies. It is an immutable snapshot; re-promoting appends a fresh claim that
`--cites` the prior, and the sidecar keeps tracking. It relates to the recorded
`disposable ↔ kan-is-truth` tension.

**S5 (MCP)** adds a `cospan mcp` server (proposed `src/mcp.rs`, new) on the `rmcp`
SDK, exposing the read+write comment operations to agents (bringing in a tokio
runtime the crate lacks today). Read is always on; write enables agent-to-agent
doc comments, attributed from a harness-set `KAN_AGENT` tag. It reuses the S1 write
API and, when enabled, the S4 promote path. It does not touch the `WriteChannel`
harness-control seam (`src/command_bus.rs`), which belongs to the separate
chat/command-bus milestone that `telos/observe-now-control-later` governs.

**Build sequence (the bridge plan):** `S1 > (S2 & S3) > S4 > S5`. S1 delivers
value on the existing gutter immediately; S2 and S3 are concurrent and together
open authoring on arbitrary highlighted files; S4 makes any comment durable; S5
opens the same surface to agents. Each slice is its own
design→build→adversarial-review→PR→release cycle under the recorded process
rhythm; this doc is the milestone plan they share.

## Resolved Questions
- RQ-1: S1 ships full CRUD — add, reply, resolve+unresolve, edit-own-body (which
  re-captures the anchor), and delete-own-comment — each gated to the caller's own
  `author.id` ($USER); and compose is a hand-rolled **multi-line** editor from the
  start, fitting paragraph-length writing feedback rather than a single line.
- RQ-2: The S5 MCP server uses the official `rmcp` SDK (accepting the tokio async
  runtime it pulls in, for spec-correctness and consistency with `kan mcp` /
  `day mcp`), and attributes an agent write from a harness-set environment tag
  (`KAN_AGENT`), defaulting to a generic `who:"agent"` id when the tag is absent —
  not a spoofable per-call author argument.
- RQ-3: Re-promoting an already-promoted comment appends a fresh immutable
  `Observation` and `--cites` the prior promoted claim's CID (an explicit snapshot
  chain); a file-set promote writes one claim per comment, each keeping its own
  anchor, rather than one aggregate claim.

## Out of Scope
- The chat/command-bus control plane (spawn/kill/redirect) and the `WriteChannel`
  implementation (`src/command_bus.rs`) — that is the separate milestone
  `telos/observe-now-control-later` governs; this milestone touches only cospan's
  own comment state.
- Visual git diffs rendered in the editor pane (the `.dropbox/05-views-ux.md`
  "visual diffs" vision) — S2 shows git *status* per file, not inline diffs.
- opencode message-body decoding and the Chat tab — unrelated milestone.
- A general-purpose text editor / IDE authoring of source files — cospan stays a
  review-and-comment surface; S1 authors comments, not code.
- Removing the bottom detail strip / comment-overflow popup redesign
  (`.dropbox/05-views-ux.md`) — a later comment-view polish, not this milestone.
