# Feature: Collapsible tree left pane

## Summary
Turn the flat namespace-grouped subject list into a collapsible tree with two
top sections — `[my work]` (the bare, no-namespace subjects: designs, ideas,
release) and `[day]` (all of day's vocabulary: telos/atom/bridge/tension/schema
grouped by namespace, plus practice) — each node expandable and collapsible.
Selecting a section or group toggles it; selecting a subject drills into its
claims as before.

## Requirements
- REQ-1: The left pane is a tree of three row kinds — `Row::Section(label)` (the
  top-level `[my work]` / `[day]`), `Row::Group(namespace)` (a namespace under
  `[day]`), and `Row::Subject(name)`. `rebuild_rows` emits only the *visible*
  rows: a collapsed Section or Group hides its descendants.
- REQ-2: Subjects split into two sections by a `substrate::is_day_subject`
  predicate: `[day]` holds subjects whose namespace is telos/atom/bridge/tension/
  schema, plus `practice` and `general`; `[my work]` holds every other (bare)
  subject. Within `[day]`, namespaced subjects are grouped under a `Row::Group`
  per namespace; bare day subjects (practice, general) sit directly under the
  section.
- REQ-3: Collapse state is a `HashSet<String>` of collapsed node keys on
  `AppState`; a node is expanded by default. Toggling a node adds/removes its key
  and rebuilds the rows, keeping the selection on the toggled node.
- REQ-4: In the Subjects focus, `j`/`k` move the selection over *all* visible
  rows (sections, groups, subjects). `Enter` on a Section or Group toggles its
  collapse; `Enter` on a Subject descends into its claims (the existing
  Subjects→Claims focus step). `Esc` ascends as before.
- REQ-5: A re-fold preserves the selection by the selected row's identity (its
  label/name), not a raw index, so a background log change does not jump the
  cursor; if that row is gone, the index clamps into range.
- REQ-6: The tree renders with indentation and a collapse marker (`▸`/`▾`) on
  Section/Group rows; the selected row is highlighted. `watch-repo --once` keeps
  its flat plain-text listing (scriptable), and the other subcommands are
  unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A unit test builds an `AppState` from a fold
  with a bare subject, two `telos/*`, one `atom/*`, and `practice`, and asserts
  the visible rows are `Section([my work])`, its bare Subject, `Section([day])`,
  `Group(telos)` + its two subjects, `Group(atom)` + its subject, and the
  `practice` Subject.
- [ ] AC-2: (covers REQ-3, REQ-4) A unit test collapses `[day]` (via the toggle
  on the selected Section) and asserts its descendants disappear from the rows
  while `[my work]` and its subject remain; toggling again restores them.
- [ ] AC-3: (covers REQ-2) A unit test asserts `substrate::is_day_subject` is
  true for `telos/x`, `schema/witness`, and `practice`, and false for a bare
  design subject like `claim-detail-view`.
- [ ] AC-4: (covers REQ-4) A unit test asserts that with the selection on a Group
  row, `Enter` (activate) toggles that group's collapse rather than changing
  focus, while with the selection on a Subject row it descends focus to Claims.
- [ ] AC-5: (covers REQ-5) A unit test moves the selection to a Group row,
  re-folds, and asserts the selection stays on that same Group by identity.

## Architecture
Step "unified-fold" left `src/tui.rs` with `AppState` holding a `Fold`, a flat
`Row { Header, Subject }`, and navigation that treats only Subject rows as
selectable. This step generalizes the row model and navigation; the fold and the
claim/detail/process layers are untouched.

`Row` becomes `Section(String)`, `Group(String)`, `Subject(String)`.
`substrate::is_day_subject(name)` classifies a subject. `rebuild_rows` walks the
fold's subjects into the two sections: `[my work]` (non-day subjects, sorted) and
`[day]` (day subjects, its namespaced ones grouped under a `Row::Group` per
`namespace`, bare ones listed after) — emitting a Section/Group row only when not
in `AppState::collapsed`, and skipping a collapsed node's children. Selection is
now over every visible row: `select_next`/`select_prev` move within
`0..rows.len()`, `selected_subject` returns `Some` only on a `Row::Subject`, and a
new `activate()` (bound to `Enter` in Subjects focus) toggles a Section/Group or
descends a Subject. `toggle(key)` flips the collapsed set, rebuilds rows, and
restores the cursor to the toggled node by a `row_key` identity (`sec:<label>` /
`grp:<ns>` / `sub:<name>`), the same identity `refold` now preserves instead of
the old subject-name-only logic.

`draw_list` renders each row with indentation by depth and a `▸`/`▾` marker on
Section/Group rows, keeping the existing highlight for the selected row. The key
handler's `Enter` in the Browser view + Subjects focus calls `activate()` rather
than always `descend()`. `plain_frame` (the `--once` path) is unchanged — it
still lists the flat grouped rows for scriptability, so this is an
interactive-only change (`telos/kan-is-truth` unaffected: the tree is a view over
the same fold, inventing only collapse state alongside the cursor).

## Resolved Questions
- RQ-1: The tree has two top sections, `[my work]` (bare subjects) and `[day]`
  (all day vocabulary, namespaced sub-groups plus practice/general), per the
  operator's categorization.
- RQ-2: `Enter` is context-sensitive in the Subjects focus — it toggles a
  Section/Group node and descends a Subject into its claims — rather than adding a
  separate collapse key; `j`/`k` navigate all visible rows.

## Open Questions

_None outstanding — the two choices above were resolved during design._

## Out of Scope
- Persisting collapse state across runs; it resets on each launch.
- A configurable section taxonomy; the `[my work]`/`[day]` split and the day
  namespace set are fixed here.
- Changing the `--once` plain listing to a tree; it stays flat and scriptable.
- Multi-level nesting deeper than Section → Group → Subject.
