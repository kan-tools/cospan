# Feature: Process tab — telos drill-down

## Summary
Turn the Process tab's Telos sub-pane into a **two-column** view: a selectable
telos list beside the selected telos's detail — statement, witnesses (each with
its probe description from `schema/witness`), and the tensions naming it.
`Enter`/`Esc` move focus between the list and the detail (for scrolling);
narrow terminals show only the focused pane. Mirrors the atom pane's
`process_detail` drill flag.

## Requirements
- REQ-1: `AppState` gains `telos_selected` (the highlighted telos in the list),
  initialized to 0, alongside the existing `telos_scroll` (reused as the detail
  scroll when drilled in).
- REQ-2: In the Telos pane, `process_move` (`j`/`k`) moves `telos_selected` over
  `fold.process.teloi` when not drilled, and scrolls `telos_scroll` over the
  selected telos's detail lines when drilled (`src/tui.rs`).
- REQ-3: `process_drill` (`Enter`/`Esc`) toggles `process_detail` for the Telos
  pane as well as Atoms, resetting `atom_scroll`/`telos_scroll` on entry.
- REQ-4: A pure `telos_detail(&TelosView, &[String], &BTreeMap<String,String>)`
  renders the detail: `telos/<slug>`, title, statement, each witness with its
  probe description (from `schema/witness`, or the bare name when unknown), and
  the tensions whose text names the slug ("(none)" when empty). `ProcessSnapshot`
  gains a `witnesses` map (witness type -> probe description) parsed from the
  `schema/witness` day-witness block.
- REQ-5: `draw_telos` renders two columns when wide — a selection-highlighted
  `List` of teloi (title + dim slug) beside the `telos_detail` of the selection —
  and only the focused pane when narrow; `pane_block` marks which pane has focus.
- REQ-6: `render_scrolled` wraps long lines (`Wrap`), so a long telos statement
  or atom field wraps instead of clipping. The non-interactive `plain_frame`
  (`process_view_lines`) keeps the flat telos list unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A unit test asserts `telos_select` moves
  `telos_selected` by ±1 and clamps at both ends over a fixture of teloi.
- [ ] AC-2: (covers REQ-3) A unit test asserts `process_drill(true)` sets
  `process_detail` in the Telos pane and `process_drill(false)` clears it.
- [ ] AC-3: (covers REQ-4) A unit test asserts `telos_detail` includes the slug,
  statement, each witness, and only the tensions whose text contains the slug.
- [ ] AC-4: (covers REQ-5) A unit test asserts `view_header(View::Process)` still
  names the process keys, and a render smoke over a fixture produces the list
  rows (title + slug) when not drilled.

## Architecture
The Process tab already has this shape for Atoms: `process_pane` selects
Atoms/Telos, `process_detail` is the shared drilled-in flag, `atom_selected`
picks a box and `atom_detail` renders its fields via `render_scrolled`, with
`process_drill` toggling detail and `process_move` dispatching by
`(pane, detail)` (`src/tui.rs`). The Telos pane currently only scrolls a flat
list built by `process_view_lines(_, ProcessPane::Telos)`.

This change gives Telos the same two-level model. `process_move`'s
`(ProcessPane::Telos, false)` arm calls a new `telos_select`; its
`(ProcessPane::Telos, true)` arm scrolls `telos_scroll` over
`telos_detail(&teloi[telos_selected], &tensions)`. `process_drill` drops its
`== Atoms` guard so it toggles `process_detail` for either pane.
`draw_telos` branches on `process_detail`: a `List` (like the Ledger panes) with
`telos_selected` highlighted, or the drilled detail. `TelosView` already carries
`slug`/`title`/`statement`/`witnesses`; tensions are `ProcessSnapshot.tensions`
(global strings) filtered to those naming the slug. `render_scrolled` gains
`.wrap(Wrap { trim: false })` so long statements wrap. Nothing touches the fold,
kan, or the non-interactive `plain_frame`.

## Resolved Questions
- RQ-1: Tensions are global (`ProcessSnapshot.tensions`), not per-telos, so the
  drill-down shows the tensions whose text contains the telos slug — the same
  slugs `day telos tension <a> <b>` records — rather than inventing a link.
- RQ-2: `telos_scroll` is reused for the detail scroll (Telos has no list scroll
  beyond selection); the list is a `List` widget whose `ListState` follows
  `telos_selected`, matching the Ledger/rail idiom.

## Open Questions

_None — the drill-down mirrors the atom pane, and the tension-by-slug link is
resolved above._

## Out of Scope
- Live per-witness state / whether a telos is currently met — still blocked on
  machine-readable `day` (kan-tools/day#240); this shows declared structure only.
- Any change to the Atoms pane beyond the shared `process_drill`/`render_scrolled`.
- The non-interactive `plain_frame` telos rendering.
