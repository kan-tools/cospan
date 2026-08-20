# Feature: tab restructure — Comments · Ledger · Process

## Summary
Restructure the top-level tabs toward the recorded UI vision: rename Browser to
**Ledger**, and fold today's separate Atoms and Telos views into a single
**Process** tab with an atoms/telos sub-pane. The tab bar becomes `1 Comments ·
2 Ledger · 3 Process` (Chat deferred until its architecture is designed). This is
the scaffold the later Process content reshape (atoms-as-flowchart, telos
drill-down) hangs off; the pane contents are unchanged in this cycle.

## Requirements
- REQ-1: `View` (`src/tui.rs`) becomes `Comments`, `Ledger`, `Process`. `Ledger`
  is today's `Browser` renamed (its tree/claims/detail behavior and `Enter`/`Esc`
  navigation are unchanged). `View::next` cycles Comments → Ledger → Process →
  Comments; `View::from_digit` maps `1`/`2`/`3` to Comments/Ledger/Process and
  `4` to `None`.
- REQ-2: Atoms and Telos fold into the Process tab via a `ProcessPane` enum
  (`Atoms`, `Telos`) on `AppState`. The Process view renders the selected pane
  using today's `draw_atoms`/`draw_telos` + `process_view_lines`; `←`/`→` in the
  Process view toggles the sub-pane and `j`/`k` scrolls it (reusing `atom_scroll`
  / `telos_scroll`). Pane contents are unchanged this cycle.
- REQ-3: `view_header` (`src/tui.rs`) shows the new tabs and a per-view key legend
  — Process: `· ←→ atoms/telos · j/k scroll`; Comments keeps `· ←→ file · j/k
  comment`; Ledger has none. The Process view labels which sub-pane is active.
- REQ-4: The footer, `plain_frame`/`--once`, and the `subject` CLI are unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts `View::from_digit` maps `1/2/3` to
  `Comments/Ledger/Process` and `4` to `None`, and `View::next` cycles
  `Comments → Ledger → Process → Comments`.
- [ ] AC-2: (covers REQ-3) A unit test asserts `view_header(View::Ledger)` names
  the `ledger` tab, `view_header(View::Process)` names `process` and the
  `atoms/telos` sub-pane keys, and no header still says `browser`.
- [ ] AC-3: (covers REQ-2) A unit test asserts `process_view_lines(p,
  ProcessPane::Atoms)` yields the atom lines and `ProcessPane::Telos` the telos
  lines (the same content as today, keyed by `ProcessPane` instead of `View`).
- [ ] AC-4: (covers REQ-2) A unit test asserts a `toggle`/`set` of `AppState`'s
  `process_pane` flips Atoms↔Telos, the action `←`/`→` invoke in the Process view.

## Architecture
`View` (`src/tui.rs`) currently has `Browser`/`Atoms`/`Telos`/`Comments`, switched
by digit keys and `Tab`, dispatched in `draw` and in the `run` loop's `j`/`k`
arms; `process_view_lines(&ProcessSnapshot, View)` renders the Atoms/Telos content
and clamps their scroll. This change collapses Atoms/Telos into one `Process` tab.

`View` becomes `Comments`/`Ledger`/`Process`. `Browser` → `Ledger` is a rename
across its match sites (`draw` dispatch → `draw_browser`, the `Enter`/`Esc` guards,
the `move_down`/`move_up` arm, `process_view_lines`' never-routes arm). A new
`ProcessPane { Atoms, Telos }` lives on `AppState` (default `Atoms`);
`process_view_lines` is re-keyed to take a `ProcessPane`. `draw` gains a
`draw_process` that dispatches to `draw_atoms`/`draw_telos` by `process_pane` and
labels the active sub-pane; the `run` loop's Process arm scrolls the active pane
(`atom_scroll`/`telos_scroll`) on `j`/`k` and toggles `process_pane` on `←`/`→`
(the same key vocabulary the Comments view uses for files). `view_header` renders
the three tabs and the per-view legend. `atom_scroll`/`telos_scroll`,
`draw_atoms`, `draw_telos`, and the process content are reused unchanged.

No model, fold, or kan interaction changes; this is a view-layer reorganization.
The footer and the non-interactive paths are untouched.

## Resolved Questions
- RQ-1: Atoms and Telos become sub-panes of one Process tab (toggled `←`/`→`)
  rather than two tabs, matching the vision's single Process tab; their content is
  carried over unchanged now and reshaped (flowchart / drill-down) in a later
  cycle.
- RQ-2: Chat is left off the tab bar entirely until its source is designed, rather
  than shown as an empty stub — an honest tab bar over a fake one.
- RQ-3: Tab order is `Comments · Ledger · Process` (Comments first) for now; Chat
  takes the first slot when it lands, per the recorded vision.

## Open Questions

_None — the fold-into-Process, the sub-pane keys, and deferring Chat were resolved
during design._

## Out of Scope
- The Process content reshape (atoms-as-flowchart, telos drill-down to
  statement/witnesses/tensions) — a later cycle; this only re-homes the panes.
- The Chat tab and its buffer-capture architecture.
- Any change to Comments, the footer, or the Ledger (Browser) behavior.
