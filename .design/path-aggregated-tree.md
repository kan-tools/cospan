# Feature: Path-aggregated left-pane tree

## Summary
Replace the left pane's fixed `Section → Group → Subject` model with a recursive
path trie: every subject is split on `/` and rendered at an indent equal to its
path depth, so `agents/handoff/main` nests three levels instead of showing flat.
The hardcoded namespace whitelist retires; grouping falls out of the paths
themselves. Leaves show their full path with the redundant prefix faded and the
final segment bright; intermediate branch nodes read at full weight; the two
top sections get colors.

## Requirements
- REQ-1: `Row` (`src/tui.rs:17`) becomes a depth-carrying trie node model —
  `Section`, an intermediate `Branch { path, depth }` (a pure structural path
  node), and a `Leaf { subject, depth }` (a recorded subject). `rebuild_rows`
  (`src/tui.rs:206`) builds the visible `Vec<Row>` by splitting each subject in
  `self.fold.subjects` on `/` within each of the two top sections, emitting a
  `Branch` for every distinct non-terminal prefix and a `Leaf` for every
  subject, ordered so a branch immediately precedes its descendants.
- REQ-2: The hardcoded namespace whitelist (`telos | atom | bridge | tension |
  schema`) in `rebuild_rows` and the `bare_day` fallback are retired. Every day
  subject aggregates by its path uniformly, so `agents/handoff/main` renders as
  `agents` → `handoff` → `main` (a `Leaf` at depth 3) rather than one flat row.
- REQ-3: In `draw_list` (`src/tui.rs:868`) indent is computed from a row's depth
  (not the fixed 0/2/4-space literals). A `Section` header is colored (a distinct
  color per section) and bold with a collapse marker; a `Branch` shows only its
  last path segment at full weight (no `DIM`) with a collapse marker; a `Leaf`
  shows its full subject path with the prefix up to and including the last `/`
  rendered `DIM` and the final segment at full weight.
- REQ-4: A subject that is also a path-prefix of another subject is emitted as
  **both** a `Branch` header (grouping its descendants) **and** a `Leaf` child
  carrying its full path one indent deeper, directly beneath that header — a
  sibling of its own children. This keeps `activate` (`src/tui.rs:313`)
  unambiguous: `Enter` on a `Branch` toggles its subtree, `Enter` on a `Leaf`
  descends into that subject's claims.
- REQ-5: Collapse state (`AppState::collapsed`, `src/tui.rs:46`) keys every
  collapsible node — `sec:<label>` for sections, `path:<prefix>` for branches —
  and a collapsed node hides its entire subtree. `row_key`/`index_of_key`
  (`src/tui.rs:279`, `:287`) and `refold` (`src/tui.rs:342`) keep the cursor on
  the same node across a re-fold or a toggle, with a `Leaf` keyed `sub:<subject>`.
- REQ-6: The non-interactive path is unchanged — `watch-repo --once`, the
  `subject` CLI, and the flat namespace listing (`src/tui.rs:588`, built from
  `Fold::namespace_counts`) keep their current output. This is an interactive
  left-pane change only.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A unit test folds the subjects `cospan`
  (my work), `telos/a`, `telos/b`, `agents/handoff/main` and asserts
  `rebuild_rows` yields, in order: `Section(my work)`, `Leaf(cospan, 1)`,
  `Section(day)`, `Branch(agents, 1)`, `Branch(agents/handoff, 2)`,
  `Leaf(agents/handoff/main, 3)`, `Branch(telos, 1)`, `Leaf(telos/a, 2)`,
  `Leaf(telos/b, 2)` — proving path depth and that `agents/handoff/main` nests
  three deep with no whitelist.
- [ ] AC-2: (covers REQ-3) A unit test over the rendered `ListItem`/`Line`s
  asserts a `Leaf` for `telos/readable-claim-browser` produces a line whose
  `telos/` prefix span carries `Modifier::DIM` and whose `readable-claim-browser`
  span does not; a `Branch(telos)` line carries no `DIM`; and each `Section`
  header carries its assigned color.
- [ ] AC-3: (covers REQ-4) A unit test folds `foo` and `foo/bar` and asserts the
  rows contain `Branch(foo, 1)` then `Leaf(foo, 2)` (its own full path `foo`,
  rendered as a child one indent deeper) then `Leaf(foo/bar, 2)`, that `activate`
  on the `Branch` toggles `path:foo` in `collapsed`, and that `activate` on the
  `Leaf(foo)` descends focus into claims.
- [ ] AC-4: (covers REQ-5) A unit test collapses `Branch(agents, 1)` and asserts
  the `agents/handoff` and `agents/handoff/main` rows disappear while the cursor
  stays on `agents`; re-expanding restores them; and a `refold` over the same
  subject set keeps the cursor on the same `Leaf` by key.
- [ ] AC-5: (covers REQ-6) `cospan watch-repo . --once` exits 0 and the existing
  flat-namespace-listing test still passes unchanged.

## Architecture
The tree is a flattened `Vec<Row>` recomputed by `rebuild_rows` (`src/tui.rs:206`)
on every fold or collapse toggle; collapse state lives separately in
`AppState::collapsed: HashSet<String>` (`src/tui.rs:46`), and the cursor is a
stable string key (`row_key`/`index_of_key`, `src/tui.rs:279`/`:287`) rather than
an index, so `refold` (`src/tui.rs:342`) can restore it after a rebuild. This
change stays entirely within that shape — no new state, no new reads.

`Row` (`src/tui.rs:17`) currently has three depth-implicit variants
(`Section`/`Group`/`Subject`) whose indent is hardcoded as literal-space prefixes
in `draw_list` (`src/tui.rs:868`, lines 883–887). It becomes a trie model:
`Section(String)`, `Branch { path: String, depth: usize }`, and
`Leaf { subject: String, depth: usize }`. `rebuild_rows` keeps the two-section
partition — `[my work]` vs `[day]` via `is_day_subject` (`src/substrate.rs:172`) —
but within each section builds the trie by splitting each subject on `/`: for a
subject `a/b/c` it ensures a `Branch` exists for prefix `a` (depth 1) and `a/b`
(depth 2), then emits a `Leaf` for `a/b/c` (depth 3). Branches and their
descendants are ordered depth-first and alphabetically, so a branch row always
immediately precedes its subtree. The `namespace`-whitelist grouping and the
`bare_day` fallback (`src/tui.rs:226`–267) are deleted; `namespace`
(`src/substrate.rs:163`) is no longer used by the tree, though `is_day_subject`
still draws the section line and the flat `--once` listing (`src/tui.rs:588`)
keeps using `namespace_counts`.

A subject that is also a strict path-prefix of another (none exist in the current
log, but `agents/handoff/*` shows the shape) is emitted twice: once as the
`Branch` header that owns its descendants, and once as a `Leaf` carrying its own
full path immediately under that header. This is the whole reason `Branch` and
`Leaf` are distinct variants rather than one node that is sometimes both — it
lets `activate` (`src/tui.rs:313`) stay a clean match: `Branch → toggle(path:…)`,
`Leaf → descend`. `toggle` (`src/tui.rs:329`) and the collapse-skip in
`rebuild_rows` gain `path:<prefix>` keys alongside the existing `sec:<label>`.

`draw_list` (`src/tui.rs:868`) computes indent as `depth * 2` spaces (sections at
depth 0) instead of the fixed literals, and styles per variant: `Section` gets a
per-section color from a small helper (e.g. `[my work]` cyan, `[day]` magenta,
both from the ANSI-16 palette so they read on light and dark like `kind_style`,
`src/tui.rs:906`) plus `BOLD` and a collapse marker; `Branch` shows its last
path segment (the text after the final slash) at full weight with a marker; `Leaf` splits
its subject at the last `/` into a `DIM` prefix span and a full-weight tail span
(a subject with no `/`, e.g. a `[my work]` leaf, has an empty prefix and renders
whole at full weight). Selection highlight (`REVERSED`, `> `) is unchanged.

Nothing is read from or written to kan; the tree stays a pure projection of
`self.fold.subjects`, serving `telos/readable-claim-browser` (every subject
listed and reachable) with the deeper paths now legible rather than flattened.

## Resolved Questions
- RQ-1: A leaf shows its **full path** with the prefix faded (`Modifier::DIM`)
  and the final segment at full weight; intermediate branch nodes are **not**
  faded (reversing today's `Group` `DIM`); the two section headers are colored.
  Leaves are self-describing even when scrolled away from their branch header.
- RQ-2: A subject that is also a branch is emitted as a full-path `Leaf` child
  one indent deeper than its `Branch` header (a sibling of its own children), so
  `Enter` stays branch=toggle / leaf=descend and the dual node's own claims
  remain one keystroke away. Collapsing the branch hides this self-leaf too.
- RQ-3: The `[my work]` / `[day]` partition (`is_day_subject`) is kept as the top
  split; recursive path aggregation happens *within* each section. The
  `telos|atom|bridge|tension|schema` whitelist is retired.
- RQ-4: Section colors are `[my work]` cyan and `[day]` magenta from the ANSI-16
  palette (matching `kind_style`'s theme-safe choice); they are a one-line helper
  and trivially adjustable.

## Open Questions

_None outstanding — the render and interaction forks were resolved during design._

## Out of Scope
- The block-summary schema additions (`day-bridge`, `day-schema`) — that is the
  next cycle, explicitly sequenced after this one.
- The flat `--once` output and the `namespace_counts` listing (`src/tui.rs:588`);
  those stay grep-friendly and untouched (REQ-6).
- Horizontal-scroll or truncation handling for very deep/long paths; current
  paths are shallow and the pane already clips.
- Any ordering other than depth-first + alphabetical within a level.
