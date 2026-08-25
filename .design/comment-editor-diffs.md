# Feature: Live git working-tree diffs in the editor pane (Slice B)

## Summary
Show the working-tree diff (vs `HEAD`) inline in the Comments-tab code pane as
agents rewrite files: added and changed lines carry a diff **sign** (`+`/`~`) and
a subtle tint, and deleted content is marked at its boundary. Slice B of the
Comments-tab redesign, split out of `comment-editor-layout` (Slice A). It keeps
the current-file-line-indexed render model Slice A established — no interleaved
removed lines — so the comment gutter, band, note reflow, sticky scroll, and the
pinned unresolvable band all keep working. Serves `telos/comment-roundtrip` (the
review surface shows what changed) and `telos/poll-dont-subscribe` (the diff is
computed on the fold loop, mtime-gated, never per keystroke).

## Requirements
- REQ-1: A new diff-parsing module (a sibling of `src/filetree.rs`, reusing its
  `git()`-runner pattern) parses `git diff HEAD -- <path>` into a `FileDiff` keyed
  on **current-file (new-file) 0-based line indices**: `added` (lines in
  additive-only hunks), `changed` (added-side lines in a hunk that also removed
  lines), and `deletions` (a map from the current-line index that follows a removed
  block to the count removed). Working-tree vs `HEAD` captures every uncommitted
  edit — what agents have written since the last commit.
- REQ-2: An **untracked** file (`GitStatus::Untracked`, `src/filetree.rs`) has no
  `HEAD` blob, so every line is treated as `added`. A file with no changes, a repo
  with no `HEAD`, or a non-git directory yields an empty `FileDiff` (no tint, no
  signs) rather than an error — mirroring how `filetree::list` degrades.
- REQ-3: The diff is recomputed on the poll-and-fold loop, **gated on the open
  file's content mtime** — the same gate `refresh_comments` (`src/tui.rs`) already
  uses to avoid re-reading — so `git diff` never runs per keystroke
  (`telos/poll-dont-subscribe`). The `FileDiff` is cached on `AppState`
  (`src/tui.rs:188`) beside `comment_localized`.
- REQ-4: `gutter_lines` (`src/tui.rs:1835`) gains a **diff sign column** after the
  `●`/`◆` comment marker: `+` (green) on an added line, `~` (yellow) on a changed
  line, and a deletion **framed on both sides** — `▁` (red) on the line just above
  a removed block and `▔` (red) on the line just below it, the two bars pointing at
  the gap — blank otherwise. The column is present only when the diff toggle is on,
  and its fixed one-cell width does not shift the existing marker/number/code layout.
- REQ-5: Added/changed lines, and the two lines **bracketing a deletion**, get a
  **subtle background tint that yields to the comment band**: the tint is applied
  only when the line carries no comment band (the `hit` is `None` in `gutter_lines`,
  `src/tui.rs:1874`), so a commented line keeps its comment band as the row
  background while the diff still shows via the sign column. Added/changed lines
  tint green/yellow; the lines above and below a deletion tint **red**, so a removal
  is highlighted on both sides of the gap. No rows are inserted — a deletion is
  marked on the bracketing lines, never a separate row — so the current-line-index
  model (comment anchoring, `reflow_rows` at `src/tui.rs:3324`, sticky scroll, the
  pinned unresolvable band) is untouched.
- REQ-6: A **toggle** shows/hides the diff, **default on**, backed by a
  `diff_on: bool` on `AppState`. It is bound to `D` in the Comments read-navigate
  key match (`src/tui.rs`, beside the `t` tray toggle), and the Comments legend
  (`view_header`, `src/tui.rs:3216`) advertises it.
- REQ-7: Coexistence with Slice A is verified, not assumed: the full-row comment
  band (`fill_line_bg`, `src/tui.rs`) still fills commented lines, the note-column
  reflow and the pinned unresolvable band render unchanged, and the diff palette
  reuses the ANSI colours `state_style` (`src/tui.rs:1822`) already uses, so the two
  layers read as one surface.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test on the diff parser feeds a hand-written
  unified `git diff` covering an add-only hunk, a modify hunk (removes + adds), and
  a pure-deletion hunk, and asserts `FileDiff.added`, `FileDiff.changed`, and
  `FileDiff.deletions` hold exactly the expected current-file line indices/counts.
- [ ] AC-2: (covers REQ-2) A unit test asserts an untracked file maps every line to
  `added`, and that empty diff text yields an empty `FileDiff`.
- [ ] AC-3: (covers REQ-3, REQ-6) A state test asserts the cached `FileDiff` is
  populated for the open file and that toggling `diff_on` off clears the diff from
  the render (see AC-4) without recomputing.
- [ ] AC-4: (covers REQ-4) A `gutter_lines` test with a constructed `FileDiff`
  asserts the sign column shows `+` on an added line, `~` on a changed line, and the
  boundary glyph on the line after a deletion when `diff_on`, and shows none of them
  when `diff_on` is false.
- [ ] AC-5: (covers REQ-5, REQ-7) A `gutter_lines` test asserts a line that is both
  changed and comment-covered keeps its comment-band background (the diff tint does
  not overwrite it) while still carrying the `~` sign.
- [ ] AC-6: (covers REQ-6) A key-handler test asserts `D` flips `diff_on` without
  changing `comment_focus`.

## Architecture
A new diff module (beside `src/filetree.rs`) owns the parse: it shells
`git diff HEAD -- <path>` through the same `Command::new("git").arg("-C")…`
pattern `filetree::git` uses (`core.quotepath=false`, tolerant of a non-zero exit
→ empty diff), and walks the unified hunks. Per hunk it tracks removed count `R`
and the new-file line index of each `+` line; `+` lines in a hunk with `R > 0` are
`changed`, otherwise `added`; a net removal (`R` greater than the added count, or a
pure `-` hunk) records a `deletions[new_line] += R − added`. All indices are
new-file 0-based, matching the `i` that `gutter_lines` already iterates, so the two
never disagree.

`AppState` (`src/tui.rs:188`) gains `file_diff: FileDiff` and `diff_on: bool`
(default true). `refresh_comments` (`src/tui.rs`) — already the mtime-gated
re-read of the open file — also recomputes `file_diff` on the same gate, so the
diff and the content can never drift and no extra `stat`/`git` runs on an idle
tick. This is a projection of the git working tree, like the file list; nothing is
persisted and no claim is written, so `telos/kan-is-truth` and the disposable
sidecar model are untouched.

`gutter_lines` (`src/tui.rs:1835`) takes `&FileDiff` and `diff_on` and, in its
per-line loop, inserts one sign cell after the comment marker and applies the tint
**only when `hit` is `None`** (`src/tui.rs:1874`) so the comment band always wins
the row background. Because no row is inserted, `reflow_rows`/`side_by_side_rows`
and the sticky `note_scroll` viewport keep aligning notes to code by line index
exactly as in Slice A. The three call sites of `gutter_lines` (the wide/narrow read
render, and `draw_compose`) pass the cached `file_diff`; the pick-line and compose
overlays may pass `diff_on = false` to keep the authoring views clean. `D` in the
Comments key match toggles `diff_on`; the legend string gains `· D diff`.

## Resolved Questions
- RQ-1: Deletions are shown by marking **changed/added current-file lines** (sign +
  tint) and a **boundary marker** where content was removed — **no interleaved
  removed-line content and no inserted rows**, because every downstream index
  (comment anchoring, reflow, sticky scroll, the pinned band) is keyed on the
  current-file line and an inserted row would shift them all. A full interleaved
  diff was rejected as fighting Slice A's model.
- RQ-2: The diff is **toggleable with `D`, default on** — shown live as agents
  rewrite (the section-(2) vision), but hideable for a clean read of a
  heavily-edited file.
- RQ-3: When a line is both changed and comment-covered, the **comment band wins the
  row background** and the diff shows via the **sign column** (plus a tint that
  yields to the band), so neither layer hides the other.

## Out of Scope
- Interleaved removed-line content / a side-by-side or unified full-diff view — the
  rejected RQ-1 alternative.
- Diffing against any ref other than `HEAD` (staged-only, a chosen commit, or
  branch comparison), and a per-hunk deletion **count** shown beyond the boundary
  marker glyph.
- Editing, staging, or reverting hunks from the pane — this is a read/observe
  surface (`telos/observe-now-control-later`).
- Any change to the re-localizer, the comment sidecar, promote-to-kan, or the MCP
  server (`src/lib.rs`, `src/comments.rs`, `src/mcp.rs`).
