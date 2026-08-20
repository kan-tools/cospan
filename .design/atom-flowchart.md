# Feature: the atom flowchart (Process tab)

## Summary
Reshape the Process tab's atoms sub-pane from a flat list into an ASCII
box-and-arrow **flowchart**: each atom is a labelled box, laid out in columns by
its depth in the atom DAG, with `──▶` arrows along the `next` edges. A selected
box is highlighted (double border), and Enter drills into that atom's full detail
(in/out/next/done/revisits). Edges the 2D layout cannot cleanly route (back-edges,
row jumps) are listed textually rather than faked (`telos/honest-ambiguity`).

## Requirements
- REQ-1: A pure `layout_atoms(atoms: &[Atom]) -> Vec<Placed>` assigns each atom a
  `(col, row)`: `col` is its longest-path depth along `next` edges from a root
  (computed by bounded relaxation, so a back-edge/cycle cannot loop forever), and
  `row` is its position among atoms sharing a column (stable by input order).
- REQ-2: A pure `atom_flowchart(atoms: &[Atom], selected: usize) -> Vec<String>`
  renders the layout into a character grid: each atom a 3-line box (`┌─ slug ─┐`
  style), the `selected` atom's box drawn with a double border (`╔═╗`), and a
  `──▶` arrow painted for every `next` edge whose target sits one column right on
  the same row. `next` edges that are not same-row/next-column (back-edges, skips)
  are appended below the grid as `<from> ⇢ <to>` lines, never drawn as fake art.
- REQ-3: The Process atoms sub-pane renders `atom_flowchart` with the selected
  atom; `j`/`k` moves the selection among atoms (in layout order) and the graph
  re-highlights; `←`/`→` still toggles atoms/telos and `Tab`/digits still switch
  tabs. When atoms are empty the pane shows the existing "(no atoms declared)".
- REQ-4: Enter in the Process view (atoms sub-pane) drills into the selected
  atom's detail — slug, `in`, `out`, `next`, `done`, and any `revisits` — and Esc
  returns to the flowchart, mirroring the Ledger's descend/ascend focus. Atoms
  carry these fields already (`substrate::Atom`; `done`/`revisits` come from the
  block's extra keys — see below).
- REQ-5: The telos sub-pane, the footer, Ledger, Comments, and `--once`/`subject`
  are unchanged this cycle.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test on `layout_atoms` for a linear chain
  `a→b→c` asserts columns `0,1,2` all at row `0`; and for a fan (`a→b`, `a→c`)
  asserts `b` and `c` share column `1` at rows `0` and `1`.
- [ ] AC-2: (covers REQ-2) A unit test asserts `atom_flowchart` for `a→b` contains
  a box for each slug and a line joining them with `─`/`▶`; that the selected
  atom's box uses a double-border char (`╔` or `═`) while an unselected one uses
  `┌`; and that a back-edge (`b`'s `next` points to `a`) appears in the
  not-drawn list, not as a drawn arrow.
- [ ] AC-3: (covers REQ-3) A unit test asserts moving the atom selection is
  clamped to `[0, atoms.len())` and that `atom_flowchart` re-renders the new
  selection with the double border on the newly selected box.
- [ ] AC-4: (covers REQ-4) A unit test asserts the atom-detail lines for an atom
  with `in`/`out`/`next`/`done`/`revisits` contain each of those fields' values.

## Architecture
The Process atoms sub-pane is today `draw_atoms` → `render_scrolled` over
`process_view_lines(.., Atoms)`, a flat text list. This change replaces that pane
with the flowchart while leaving the telos pane, `render_scrolled`, and
`process_view_lines`' telos branch intact.

`substrate::Atom` carries `slug`, `inputs`, `outputs`, `next`. `done`/`revisits`
are today folded only into the atom block's extra keys and shown by
`block_summary`, not on `Atom`; this cycle extends `parse_atom`
(`src/substrate.rs`) to also capture `done: Vec<String>` and `revisits:
Vec<String>` onto `Atom` (additive struct fields, `Default`-friendly), so the
flowchart's drill-down can show them without re-reading the block.

Layout and render are pure functions in `src/tui.rs`, beside `process_view_lines`:
- `layout_atoms(&[Atom]) -> Vec<Placed{col,row}>`. Resolve `next` slugs to
  indices; `col[v] = max(col[v], col[u]+1)` relaxed `atoms.len()` times (bounded
  against cycles); group by `col`, assign `row` by input order.
- `atom_flowchart(&[Atom], selected) -> Vec<String>`. Compute per-column box
  widths, x/y offsets (box height 3 + a row gap), paint boxes into a
  `Vec<Vec<char>>` grid (double border for `selected`), paint `──▶` for same-row
  next-column edges, collect the rest as `from ⇢ to` annotation lines, then join
  the grid rows (trailing spaces trimmed) followed by the annotations.
- `atom_detail(&Atom) -> Vec<String>` for the drill-down.

`AppState` gains `atom_selected: usize` (the highlighted box) and reuses the
`Focus` idea for the drill-down: the Process view tracks whether it is showing
the graph or an atom's detail (a small `process_detail: bool`). `draw_process`'s
atoms branch renders the flowchart (or the detail when drilled in); the `run`
loop's Process `j`/`k` moves `atom_selected` (graph) or scrolls the detail, and
`Enter`/`Esc` toggle `process_detail`. No kan or fold change; the flowchart is a
pure projection of the declared atoms.

## Resolved Questions
- RQ-1: Layout is by longest-path column with bounded relaxation (cycle-safe)
  rather than a full graph-layout library; day's atom graph is small and mostly
  linear, so this renders cleanly and stays dependency-free.
- RQ-2: Edges the grid cannot route on a straight same-row arrow are listed as
  text (`from ⇢ to`) rather than drawn with approximate/curved ASCII, keeping the
  picture honest (`telos/honest-ambiguity`).
- RQ-3: The selected box is shown by a double border (a different box-drawing
  style) rather than a color span, so the char-grid render needs no per-cell
  styling.

## Open Questions

_None — the layout method, the honest edge-annotation, and the double-border
selection were resolved during design._

## Out of Scope
- The telos drill-down (statement/witnesses/tensions) — a separate cycle.
- Curved/multi-segment ASCII arrow routing for back-edges and row jumps.
- Live atom *position* / per-witness state (still needs machine-readable day,
  kan-tools/day#240); this renders the declared structure.
- Horizontal scrolling of a very wide graph beyond what the pane clips.
