# Feature: right-anchored comment column with reflow

## Summary
Render comments as notes in a right column beside the lines they reference, and
reflow the code column — a multi-line note pushes the following code lines down
so notes never overlap code or each other. This is the comment view's flagship
layout from `.dropbox/05-views-ux.md` (d): "a multi-line comment expands and
reflows the text column to make room (the requirement that rules out tmux panes —
cospan draws this itself)." It replaces reading comments one-at-a-time in the
bottom strip with seeing them all in context.

## Requirements
- REQ-1: In the **wide** layout, the Comments content area splits into a code
  column and a right comment column (`draw_comments`, `src/tui.rs`). Each comment
  whose localization has a span renders as a compact note anchored at its span's
  start line — a header (`● @author · STATE`, `●` colored by state, `[resolved]`
  when resolved), the body word-wrapped to the note width and capped (a trailing
  `…` when longer), and a `+N replies` line when the thread is non-empty.
- REQ-2: The code column **reflows**: a note taller than one line inserts extra
  rows whose code cell is blank, so subsequent code lines shift down and no note
  overlaps code or another note. Nothing is hidden — a note that would run past
  the pane scrolls with it (`telos/honest-ambiguity`).
- REQ-3: The selected comment's note is emphasized and kept in view: `draw_comments`
  scrolls the paired columns so the selected comment's note row is visible; `j`/`k`
  moves the selection and `←`/`→` still switches files (unchanged).
- REQ-4: The bottom strip is kept — it still shows the selected comment's full
  thread (`thread_lines`) and the `Unresolvable` list (comments with no span),
  which have no line to anchor a note to.
- REQ-5: The **narrow** layout keeps today's single code-pane + strip view (no
  note column — too little width); the `--once`/CLI paths are unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test on `wrap_text(s, w)` asserts a long string
  wraps to lines each within `w`, an empty string yields one empty line, and a
  word longer than `w` is left intact on its own line.
- [ ] AC-2: (covers REQ-1) A unit test on `note_block` for a resolved comment with
  a two-line-wrapping body and one reply asserts the produced lines contain the
  author, the state, a `[resolved]` marker, wrapped body text, and a `+1 reply`
  line.
- [ ] AC-3: (covers REQ-2, REQ-3) A unit test on `reflow_rows`: given code lines
  and one note of height 3 anchored at code line 1, the paired rows place the
  note's first line beside code line 1, insert two blank-code rows for the note's
  other lines, and report code line 2 two rows later than without the note; and
  the returned row index for that note points at the row beside code line 1.
- [ ] AC-4: (covers REQ-4) A unit test asserts an `Unresolvable` comment (span
  `None`) produces no note in `reflow_rows` and is returned in the unresolvable
  list (via the existing `gutter_lines`), not placed in the column.

## Architecture
`draw_comments` (`src/tui.rs`) currently renders a single code pane (from
`gutter_lines`) plus a 6-row strip. This change, in the wide branch only, splits
the content area horizontally into a code column and a note column and renders
them from a reflowed row grid; the strip and the narrow branch are untouched.

Three pure functions are added beside `gutter_lines`, all unit-testable without a
terminal:
- `wrap_text(s: &str, w: usize) -> Vec<String>` — greedy word wrap; splits on the
  comment body's own newlines first, never breaks a single over-long word (ratatui
  clips it), always returns at least one line.
- `note_block(c: &Comment, loc: &Localization, w: usize, selected: bool) ->
  Vec<Line<'static>>` — the compact note: a `state_style`-colored header, the
  wrapped body capped to a few lines with a `…` when truncated (the full body and
  thread stay in the strip), and a `+N replies` line from `c.thread`. The selected
  note's header is emphasized (`REVERSED`).
- `reflow_rows(code_lines: Vec<Line>, notes: &[(usize, usize, Vec<Line>)]) ->
  (Vec<(Line, Line)>, Vec<(usize, usize)>)` — `notes` is `(localized_index,
  start_line, note_lines)` sorted by `start_line`. It walks `code_lines`, pairing
  each with a blank right cell, and where a note starts it places the note's first
  line on that code row and its remaining lines on following blank-code rows
  (the reflow). It returns the paired `(left, right)` rows and, per note, its
  `(localized_index, row)` so the caller can scroll to the selected comment.

`draw_comments` builds `code_lines`/`unresolved` from the existing `gutter_lines`
(so the left column keeps its markers and the unresolvable list is unchanged),
builds `notes` from `comment_localized` entries whose `span` is `Some` (anchored
at `span.start`, in the already-line-sorted order), calls `reflow_rows`, computes
the scroll offset from the selected comment's returned row (clamped, kept in
view), and renders the left and right cell vectors as two `Paragraph`s sharing
that offset so the columns stay aligned. Nothing new is read from or written to
kan; the grid is a pure projection of the file content plus the sidecar.

## Resolved Questions
- RQ-1: The note column carries a **compact** note (capped body, reply count),
  not the full thread; the full thread and the unresolvable list stay in the
  bottom strip, so the column reads at a glance and long threads do not blow up
  the reflow height.
- RQ-2: Reflow pushes code **down** (blank code cells) rather than overlaying
  notes or truncating them, matching the design's "expands the text column to make
  room" and avoiding overlap between nearby comments.
- RQ-3: The column is wide-layout only; narrow keeps the strip-based view rather
  than cramming a column into too little width.
- RQ-4: Scroll follows the selected comment (its note row is kept in view) rather
  than a free code-line offset, so `j`/`k` always reveals the note it selects.

## Open Questions

_None — the layout, the reflow direction, the compact-note scope, and the
wide-only decision were resolved during design._

## Out of Scope
- tree-sitter syntax highlighting of the code column (separate polish).
- Authoring/replying from the TUI (writes stay CLI/MCP; `observe-now-control-later`).
- A resizable / draggable column split; the split is a fixed proportion.
- Rendering the full thread inline in the column (it stays in the strip).
