# Feature: discoverable comment-view navigation

## Summary
File switching in the Comments view is bound only to `[` / `]` and hinted only in
a pane title, so it is undiscoverable — a user could not find how to change files.
Add `←` / `→` as file-switch keys (spatially natural beside the left files rail)
and a per-view key legend in the header, so the navigation is visible without
reading source.

## Requirements
- REQ-1: In the Comments view, `←` (Left) and `→` (Right) switch the selected
  commented file (previous / next), as aliases for the existing `[` / `]`, by
  calling `select_comment_file(-1)` / `select_comment_file(1)` (`src/tui.rs`).
  Arrow keys are otherwise unused in this view.
- REQ-2: The keys are shown, not just bound. `view_header` (`src/tui.rs`) gains a
  per-view key legend — for the Comments view it names the file-switch (`←→`) and
  comment-move (`j/k`) keys — and the content-pane and files-rail titles name the
  arrow keys (e.g. `←/→ file`), so the affordance is visible without reading
  source, in both wide and narrow layouts. Other views keep their current header.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-2) A unit test asserts `view_header(View::Comments)`
  contains the file-switch and comment-move key hints (the `←→` glyphs and
  `j/k`), and that `view_header(View::Browser)` does not carry the comment-move
  hint.
- [ ] AC-2: (covers REQ-1) A unit test asserts `select_comment_file` moves the
  selection by ±1 and clamps at both ends over a multi-file set — the action the
  `←`/`→` (and `[`/`]`) keys invoke.

## Architecture
The `run` loop (`src/tui.rs`) dispatches keys and, for `j`/`k`/arrows, branches
by `state.view`. File switching already exists as `[` / `]` arms calling
`select_comment_file` (`src/tui.rs`), a pure clamped move already covered by
`switching_files_reloads_content`-style tests. This change adds `KeyCode::Left`
/ `KeyCode::Right` arms gated to `View::Comments` calling the same method — arrow
keys are not otherwise bound in the Comments view, so there is no conflict.

`view_header(view)` currently renders the tab strip plus `· Tab switch · q quit`.
It gains a per-view hint segment: for `View::Comments`, `· ←→ file · j/k comment`
before the shared `· Tab switch · q quit`. The pure header function is
unit-testable, so the legend is asserted directly. The content-pane title
(`draw_comments`) and the files-rail title show `←/→` so the file affordance
reads in both layout modes. No state, model, or kan interaction changes; this is
input-binding and labeling only.

## Resolved Questions
- RQ-1: `←`/`→` are added as aliases rather than replacing `[`/`]`, so existing
  muscle memory and the bracket keys keep working; the arrows are the discoverable
  default named in the header.
- RQ-2: Navigation stays a flat two-axis model (`↑↓`/`j`/`k` = comments within a
  file, `←→`/`[`/`]` = files) rather than a focus-toggle between panes, keeping
  the keymap simple; the legend makes the two axes legible.

## Open Questions

_None — this is a small, contained input/labeling change._

## Out of Scope
- A focus model that moves a cursor into the files rail (Tab-to-focus); the flat
  two-axis keymap is kept.
- A full help overlay / leader menu (a later cross-view affordance).
- Any change to what the Comments view renders or how comments localize.
