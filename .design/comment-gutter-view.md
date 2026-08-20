# Feature: the comment gutter view (P1)

## Summary
Surface the comment sidecar — which already works headless (`src/comments.rs`,
`cospan comment add` / `cospan comments`) — inside the TUI as a fourth view: a
picker of commented files, and for the selected file its content with an anchored
comment gutter showing each comment's live `Anchored` / `Drifted` /
`Unresolvable` state as the file changes. This is `telos/comment-roundtrip` made
visible — "the reason cospan exists beyond a nice viewer."

## Requirements
- REQ-1: A fourth top-level view `View::Comments` (`src/tui.rs`), reachable by
  `4` and the `Tab` cycle. `View::next`, `View::from_digit`, the digit-key match
  (`'1'..='4'`), the draw dispatch, and `view_header` all extend to it. Like the
  Atoms/Telos views it carries its own scroll/selection state and ignores `Focus`.
- REQ-2: The view discovers **commented files** by scanning the sidecar tree
  (`.cospan/comments/**/*.jsonl`, the scheme `comments::sidecar_path` defines)
  under `repo` and mapping each sidecar back to its source path. They render as a
  selectable left list, each with its comment count. An empty list shows an
  explicit "no comments yet" state, never a blank pane.
- REQ-3: Selecting a file renders its content in a scrollable, line-numbered pane
  with a **gutter marker** in the left margin of every line covered by a comment's
  `Localization.span`, colored by state (`Anchored`/`Drifted`). A comment cursor
  (`j`/`k`) moves between the file's comments, scrolling the pane to the anchored
  line; the selected comment's body, author, state, and confidence render in a
  detail strip.
- REQ-4: Comments are re-localized **live**. Once per poll tick the view stats the
  selected file's mtime (a per-file gate mirroring the `.kan/log/HEAD` gate — no
  second loop, one added `stat` per tick); on a change (or first open) it re-reads
  the file, runs `comments::localize_and_update` per comment (re-anchoring by
  last-seen tracking), and `comments::save`s the sidecar once — the same
  load→localize→save flow as `comments_cmd` in `src/main.rs`.
- REQ-5: `Unresolvable` comments (`span == None`) cannot be placed in the gutter
  and collect in a separate "unresolvable — replace by hand" list with their body
  and author, never silently dropped (`telos/honest-ambiguity`).
- REQ-6: The view is read-only with respect to kan and the source file: its only
  write is the sidecar re-anchor (cospan's own ephemeral state), and only on a
  file change. It adds no per-keystroke spawns and exactly one file `stat` per
  tick (`telos/poll-dont-subscribe`).

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts `View::from_digit('4') ==
  Some(View::Comments)`, that `View::next` cycles through `Comments` back to
  `Browser`, and that `view_header(View::Comments)` names the Comments tab.
- [ ] AC-2: (covers REQ-2) A unit test over a temp repo with two sidecar files
  and one un-commented file asserts the discovery function returns exactly the two
  commented source paths, sorted, each paired with its comment count.
- [ ] AC-3: (covers REQ-3, REQ-5) A pure `gutter_lines(content, &[(Comment,
  Localization)])`-style function test: given file content and a mix of an
  `Anchored` comment (span `Some`) and an `Unresolvable` one (span `None`), the
  produced gutter rows carry a marker on the anchored comment's line and none
  elsewhere, and the unresolvable comment is returned in the separate list, not
  placed on a line.
- [ ] AC-4: (covers REQ-4) A unit test drives the per-file re-localize step: build
  a comment on line N of content A, then feed content B where the anchored text
  moved, and assert the recomputed `Localization` reports the new span (state
  `Anchored`/`Drifted`), proving the view re-localizes rather than trusting the
  stored line.
- [ ] AC-5: (covers REQ-6) `cospan watch-repo . --once` still exits 0, and a unit
  test asserts the per-file mtime gate (`should_refold`-style pure check) fires
  only when the file's mtime changes.

## Architecture
The TUI is one poll-and-fold loop (`run`, `src/tui.rs`): one `event::poll(tick)`
is both the key wait and the re-fold tick, and the model is rebuilt only when
`.kan/log/HEAD`'s mtime changes (`head_mtime` + `should_refold`). Views are
projections of that model; the only owned state is the selection cursor. The
comment sidecar is cospan's one sanctioned mutable exception
(`src/comments.rs`), ephemeral and gitignored, never touching kan.

**New view.** `View` gains a `Comments` variant; `View::next`,
`View::from_digit`, the `'1'..='4'` digit match and per-view `j/k` arms in `run`,
the `draw` dispatch, and `view_header` extend to it (the change mirrors how
Atoms/Telos were added). `AppState` (`src/tui.rs`) gains comment-view fields:
the discovered `comment_files: Vec<(PathBuf, usize)>`, `comment_file_selected`,
the loaded `comment_content: String`, the localized set `Vec<(Comment,
Localization)>`, a per-file `comment_mtime: Option<SystemTime>`, a
`comment_selected` cursor, and a `comment_scroll` offset — initialized in
`AppState::new`.

**Discovery.** A pure `commented_files(repo) -> Vec<(PathBuf, usize)>` walks
`repo/.cospan/comments`, and for each `*.jsonl` reconstructs the source path
(inverse of `comments::sidecar_path`) and counts records via `comments::load`.
Sorted, deduped, so the picker is deterministic.

**Live re-localization.** Inside the existing tick, after the HEAD gate, a second
gate stats the selected file's mtime (`std::fs::metadata(..).modified()`) and
compares with `comment_mtime` via the existing pure `should_refold`. On a change
(or first open of a file), the view reads the file, runs
`comments::localize_and_update(&mut c, &content)` for each comment — the same
call `comments_cmd` uses, which fast-paths on the content hash and re-anchors on
`should_reanchor` — collects `(Comment, Localization)`, and `comments::save`s the
sidecar once. This is one added `stat` per tick and one write per file change,
not per tick — the `poll-don't-subscribe` and owned-state disciplines hold.

**Rendering.** A pure `gutter_lines(content, &[(Comment, Localization)]) ->
(Vec<Line>, Vec<&Comment>)` builds the content pane: each source line, prefixed
by a gutter `Span` (a marker for any comment whose `span` covers that line,
styled by `State` via a small `state_style`), and returns the `Unresolvable`
comments (`span == None`) as the separate list. It renders like
`draw_claim_detail` — a `Vec<Line>` `Paragraph` with the existing scroll — and
reuses `layout_mode`/`WIDE_COLS` for the wide (files-list + content) vs narrow
split, mirroring `draw_browser`. Splitting the gutter build out as a pure
function keeps it unit-testable without a terminal, as the tree and markdown work
did.

Nothing new is read from or written to kan. The gutter is a projection of (file
content + sidecar), and the sidecar re-anchor is cospan's designed behavior.

## Resolved Questions
- RQ-1: The view re-localizes and **re-anchors** (via `localize_and_update` +
  `save`, gated by the per-file mtime), matching `cospan comments` and the
  sidecar's last-seen tracking, rather than a pure read from the frozen anchor —
  so drift does not accumulate as an agent rewrites the file, which is the whole
  point of the feature.
- RQ-2: File selection for P1 is a picker over files that already have a sidecar
  (discovered from `.cospan/comments`), not the P2 session picker or an arbitrary
  file browser; the flagship demo is viewing existing comments live.
- RQ-3: Live re-read uses a second per-file mtime gate inside the single existing
  tick, not a new watch loop or thread (`telos/poll-dont-subscribe`).
- RQ-4: The comment cursor orders comments by their anchored line (top to bottom),
  not sidecar/creation order, so `j`/`k` (down/up) follow the file's vertical
  layout; `Unresolvable` comments (no line) sort last, stably.

## Open Questions

_None outstanding — scope, re-anchor behavior, and the poll model were resolved
during design._

## Out of Scope
- Writing comments from the TUI (add/reply/resolve) and the persist-to-kan
  shortcut; writes are the P3 command bus, and the MCP write path is separate.
  The TUI view is read + re-anchor only.
- The multi-line-comment column reflow and inline thread expansion from
  `.dropbox/05-views-ux.md`; P1 uses a fixed gutter marker plus a detail strip.
- tree-sitter syntax highlighting of the file pane; content renders plain.
- The session picker, harness view, and the comment MCP server (later P1/P2).
- Editing or reloading arbitrary files not already carrying a sidecar.
