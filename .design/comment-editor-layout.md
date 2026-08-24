# Feature: Comments-tab editor-view layout redesign

## Summary
Reshape the Comments tab's layout so the code and comments own the full frame: make
the existing file-tree rail a **toggleable tray** (auto-collapsed while reading a
file), **remove the fixed bottom strip**, rehome the two jobs the strip did — the
selected comment's full thread moves into an **Enter-triggered popup**, and the
`Unresolvable` comments become a **pinned group at the top of the note column** so
they stay visible (`telos/honest-ambiguity`). This is Slice A of the
`.dropbox/05-views-ux.md` section (2) vision; live git-diff rendering in the editor
pane is deliberately split into a follow-up slice.

Serves `telos/comment-roundtrip`: the human browse-and-author surface gets more room
and a cleaner read path without changing what the round trip is.

## Requirements
- REQ-1: The file-tree rail becomes a **toggleable tray** backed by a new
  `tray_open` flag on `AppState` (declared in the Comments block at `src/tui.rs`
  around the fields at `src/tui.rs:220`). Today rail visibility is width-only
  (`layout_mode` → `Fit::Wide`/`Fit::Narrow` at `src/tui.rs:3666`) with no toggle;
  the redesign gates the rail split (`src/tui.rs:3666`) on `tray_open` instead.
  Default behavior: the tray is **shown when no file is open** (`open_file` is
  `None`, `src/tui.rs:233`) because it is how a file is picked, and **auto-collapses
  when a file opens** (`open_path` at `src/tui.rs:877`), so code + notes take the
  full width while reading. The existing tree stays collapsible with directories
  collapsed by default (`build_file_rows`, `src/tui.rs:79`) — that behavior is
  unchanged.
- REQ-2: A key toggles the tray open/closed in either focus. Bind **`t`** in the
  Comments-tab key handler (the read-navigate arm near `src/tui.rs:3049`); `t` is
  currently unbound there (existing keys: `a`/`i`/`r`/`e`/`d`/`x`/`p`/`P`/`j`/`k`,
  `src/tui.rs:3049-3092`). Toggling the tray open does not change `comment_focus`;
  it only re-includes the rail rectangle.
- REQ-3: The fixed **bottom strip is removed**. Delete the
  `Layout::vertical([Min(3), Length(6)])` split that reserves `strip_area`
  (`src/tui.rs:3699`) and its render block (`src/tui.rs:3827-3865`); the content
  region (code column + note column) uses the full height of `main_area`.
- REQ-4: The `Unresolvable` comments (localization span `None`, already returned
  separately by `gutter_lines` at `src/tui.rs:1791`) render as a **pinned
  "unresolvable (N)" group at the top of the note column** (`note_area`, built at
  `src/tui.rs:3767`), above the line-anchored notes and not scrolled or capped away.
  Each is cursor-selectable and opens its thread in the popup (REQ-5). This replaces
  the strip's unresolvable list (`src/tui.rs:3832`) and preserves
  `telos/honest-ambiguity`'s "resolve-by-hand list stays visible" contract.
- REQ-5: Pressing **Enter** on the selected comment in `CommentFocus::Comments`
  (`src/tui.rs:63`) opens a **modal popup overlay** showing the full untruncated
  thread — the same content `thread_lines` builds today (`src/tui.rs:1862`): header
  (state · line · confidence · author · `◆ kan` · `[resolved]`), full body, and each
  reply. **Esc** closes it. Enter is currently unbound in `Comments` focus (it only
  drives `file_activate` in `Tree` focus, `src/tui.rs:3036`), so there is no
  conflict. The popup is backed by a new `Option` overlay field on `AppState`
  (alongside `editing` at `src/tui.rs:255`); while it is open, keys route to an
  overlay handler ahead of the normal arms the way `editing` intercepts keys at
  `src/tui.rs:3007`. The popup is **actionable**: `r`/`e`/`d`/`x` inside it invoke
  the same `begin_reply`/`begin_edit`/`delete_selected`/`toggle_resolve_selected`
  (`src/tui.rs:3055-3078`) against the comment being read, so the reader can reply
  or resolve the thread in place; `Esc` closes.
- REQ-6: Column notes keep their compact form — `BODY_CAP = 3` line cap
  (`src/tui.rs:3276`) and `reflow_rows`/`side_by_side_rows` layout
  (`src/tui.rs:3324`, `src/tui.rs:3398`) are unchanged; the popup (REQ-5), not
  inline expansion, is how full text is read.
- REQ-7: The transient status line `comment_msg` (`src/tui.rs:258`), previously
  surfaced in the strip's title (`src/tui.rs:3855`), is rehomed to the **footer**
  (drawn at `src/tui.rs:3180` region) since the strip is gone, so authoring errors
  like "not your comment" still show.
- REQ-8: **Narrow** layout (`Fit::Narrow`, no rail and no note column,
  `src/tui.rs:3674`, `src/tui.rs:3815`) drops the strip too: it shows the code pane
  with gutter markers, and the Enter popup (REQ-5) is the read path for a comment's
  thread. `comment_scroll` (`src/tui.rs:252`) still scrolls the code pane. Because
  narrow has neither note column nor strip, the pinned **unresolvable band** (REQ-4)
  is rendered below the code pane there too — `telos/honest-ambiguity` requires the
  resolve-by-hand list to stay visible at every width, not only wide.
- REQ-9: A comment-covered code line gets a **full-row background band**: the band
  colour (`gutter_lines`, `src/tui.rs:1871`) is filled across the whole pane width,
  not just the character cells, so the anchored line reads as a solid bar. A helper
  pads each banded line with a trailing coloured space span to the pane's inner
  width at render time (the width is only known in `draw_comments`).
- REQ-10: Stepping the comment cursor (`select_comment`, `src/tui.rs:1309`) follows
  the selection **stickily**: the note/code viewport scrolls only when the selected
  comment leaves the visible window — up to it when above, just far enough when
  below — instead of snapping it to the top every keypress. The viewport top is a
  render-space cache reset when the open file changes.
- REQ-11: The **line picker** for a new comment (`Editing::PickLine`,
  `src/tui.rs:398`) uses the same sticky scroll: it followed the pick cursor by
  re-centering it every step (`cursor.saturating_sub(3)`); it now scrolls only when
  the cursor leaves the viewport, sharing the code-pane viewport-top cache.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A unit test on the tray-visibility helper asserts
  the rail rectangle is included when `tray_open` is true and omitted when false, and
  that opening a file sets `tray_open` to false while an unopened state leaves it
  true; a key-handler test asserts `t` flips `tray_open` without changing
  `comment_focus`.
- [ ] AC-2: (covers REQ-3) A test on the Comments layout function asserts the content
  area height equals the body height (no 6-row strip reserved) and that no
  `strip_area` rectangle is produced.
- [ ] AC-3: (covers REQ-4) A unit test on the note-column builder asserts that, given
  at least one `Unresolvable` comment, the produced rows begin with an
  "unresolvable (N)" group header followed by one selectable row per unplaceable
  comment, ahead of the line-anchored notes.
- [ ] AC-4: (covers REQ-5, REQ-6) A state-transition test asserts Enter in
  `CommentFocus::Comments` opens the popup overlay, the popup's text equals
  `thread_lines(...)` for the selected comment, Esc clears the overlay, and the
  column note for that comment is still capped at `BODY_CAP` lines.
- [ ] AC-5: (covers REQ-7) A test asserts that when `comment_msg` is set it appears
  in the footer render output (not in any strip), and is cleared as it is today.
- [ ] AC-6: (covers REQ-8) A test on the narrow-layout path asserts no note column and
  no strip are produced, the Enter popup still opens over the code pane, and an
  `Unresolvable` comment still renders in a pinned "unresolvable (N)" band.
- [ ] AC-7: (covers REQ-5) A key-handler test asserts that with the popup overlay
  open, `r` enters reply-compose for the popup's comment, `x` toggles its resolved
  state, and `Esc` closes the overlay without mutating the comment.
- [ ] AC-8: (covers REQ-1) A test asserts that after the tray is toggled open,
  opening a file re-collapses it (`tray_open` back to false), so it does not stay
  stuck open across files.
- [ ] AC-9: (covers REQ-9) A unit test on the line-fill helper asserts a banded line
  (leading span carries a background) is padded with a trailing same-colour span to
  the full width, and an unbanded line is returned unchanged.
- [ ] AC-10: (covers REQ-10) A unit test on the sticky-scroll helper asserts the
  viewport top does not move while the selection is within `[top, top+view_h)`,
  scrolls up to the selection when above, scrolls just far enough when below, and
  clamps to `max_top`.
- [ ] AC-11: (covers REQ-11) A render test enters `PickLine`, asserts the viewport
  top stays put while the cursor is visible, and scrolls just far enough (not to the
  top) once the cursor moves past the viewport bottom.

## Architecture
The change is confined to the Comments view in `src/tui.rs` and its render helpers;
it touches no substrate, no `kan`/`day` shell-outs, and no anchor/re-localizer logic
(`src/lib.rs`, `src/comments.rs` are unchanged). It reads the same
`comment_localized: Vec<(Comment, Localization)>` (`src/tui.rs:241`) the view already
folds each tick — no new data source, consistent with the poll-and-fold spine and
`telos/kan-is-truth` (comments remain cospan's owned sidecar state, nothing new is
persisted).

New `AppState` fields (the struct spans `src/filetree.rs:188` with the Comments block
at `src/tui.rs:220`): `tray_open: bool` (REQ-1/2) and a popup overlay
`Option<...>` (REQ-5) sitting beside `editing` (`src/tui.rs:255`). The focus enum
`CommentFocus { Tree, Comments }` (`src/tui.rs:63`) is **not** extended — the tray is
a layout flag, and the popup is a modal overlay that intercepts keys ahead of the
normal arms (mirroring how `editing.is_some()` short-circuits key routing at
`src/tui.rs:3007`), so the two-value focus model is preserved.

`draw_comments` (`src/tui.rs:3636`) is restructured: (1) the rail split
(`src/tui.rs:3666`) is gated on `tray_open` rather than `layout_mode` alone; (2) the
vertical strip split (`src/tui.rs:3699`) is removed so `content_area` is the whole
`main_area`; (3) the note-column builder (`src/tui.rs:3767`) prepends the pinned
unresolvable group; (4) a new `draw_thread_popup` renders a centered overlay from
`thread_lines` when the overlay field is set, drawn last so it sits atop the panes
(as `draw_compose` at `src/tui.rs:3873` does for authoring). The strip render block
(`src/tui.rs:3827-3865`) is deleted and its two jobs are rehomed per REQ-4/REQ-5;
`comment_msg` moves to the footer per REQ-7. No operation removes or rewrites a
comment or a claim — the append-only sidecar and fold invariants are untouched.

## Resolved Questions
- RQ-1: Scope is split. This design is **Slice A** — the toggleable tray, strip
  removal, overflow popup, and pinned unresolvable group. **Live git working-tree
  visual diffs** in the editor pane are a separate follow-up slice (Slice B) with
  their own design/build/review, because they are greenfield and independent of the
  layout work.
- RQ-2: The `Unresolvable` comments are rehomed to a **pinned "unresolvable (N)"
  group at the top of the note column**, always visible rather than behind a
  keypress, honoring `telos/honest-ambiguity`'s resolve-by-hand contract (REQ-4).
- RQ-3: Full-thread reading is done through the **Enter popup** (REQ-5); the column
  notes stay **compact** — the `BODY_CAP = 3` cap and reflow are unchanged (REQ-6) —
  rather than expanding inline.
- RQ-4: The file tray **auto-behaves**: shown when no file is open (needed to pick
  one), auto-collapsed once a file opens so code + notes get full width, and toggled
  anytime with **`t`** (REQ-1, REQ-2).
- RQ-5: The popup is **actionable**, not read-only: `r`/`e`/`d`/`x` act on the
  comment being read while it is open (REQ-5, AC-7).

## Out of Scope
- **Live git working-tree visual diffs** in the editor pane — the largest,
  independent piece of the section-(2) vision; its own design + build + review slice
  (Slice B). This design renders the current file content via
  `highlight::styled_upto` exactly as today.
- Any change to the re-localizer, the comment sidecar format, promote-to-kan, or the
  MCP server (`src/lib.rs`, `src/comments.rs`, `src/mcp.rs`).
- The responsive breakpoint engine beyond the existing `Fit::Wide`/`Fit::Narrow`
  split — no new width tiers are introduced.
- Mouse interactions for the tray/popup beyond the existing compose scroll
  (`src/tui.rs:3141`).
