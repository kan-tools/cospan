# Feature: One cached --all fold (performance)

## Summary
Replace cospan's many kan process spawns — `kan status` for the subject list plus
a `kan show <subject>` per selection plus separate `--all` folds for cites and
the process snapshot — with a **single `kan show --all --json` fold per tick**.
Subjects, each subject's claims, the cid index, and the atom/telos/tension
structure all derive from that one fold, held in memory and rebuilt only when the
log changes. This removes the per-keystroke shell-out that makes the browser feel
sluggish.

## Requirements
- REQ-1: A `substrate::Fold` holds everything one `kan show --all --json` yields:
  the sorted subject names, a `subject -> Vec<Claim>` map (newest-first), a
  `cid -> Claim` index, and a `ProcessSnapshot` — plus `day_status` and any
  errors. `substrate::fold(repo)` builds it with exactly **one** `kan show --all`
  spawn and one `day status` spawn.
- REQ-2: `Fold` exposes the reads the views need — `claims_for(subject) -> &[Claim]`,
  `namespace_counts()`, and `sessions()` — as pure in-memory lookups, so moving
  the selection never spawns a process.
- REQ-3: The TUI `AppState` holds a single `Fold` (rebuilt on `refold`) in place
  of the separate `dash`, per-subject `claims` cache, `cite_index`, and `process`
  fields; selecting a subject or a cite is a map lookup, not a fetch. The
  lazy-load methods (`ensure_selected_loaded`/`ensure_cite_index`/`ensure_process`)
  are gone.
- REQ-4: The claim detail resolves cites against the fold's `by_cid`, the Atoms/
  Telos views read the fold's `process`, and the process pane reads the fold's
  `day_status` — all from the one fold, none re-spawning kan.
- REQ-5: `watch-repo --once` renders the plain dashboard from the fold and exits
  0; the `subject` CLI subcommand keeps using the per-subject `subject_claims`
  (one subject, one spawn, unchanged); `demo`/`watch`/`comment(s)` are unaffected.
- REQ-6: The fold is rebuilt only when `.kan/log/HEAD`'s mtime changes (the
  existing poll gate), so a steady-state UI does no kan work at all between edits.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A unit test builds a `Fold` from an
  `--all`-shaped JSON payload and asserts the subject list, `claims_for` for a
  subject (newest-first), `by_cid` lookup, and the parsed `process` are all
  present from the one payload.
- [ ] AC-2: (covers REQ-2) A unit test asserts `namespace_counts` and `sessions`
  computed from a `Fold` match the subjects it was built from.
- [ ] AC-3: (covers REQ-3, REQ-4) A unit test builds an `AppState` from a `Fold`
  and asserts the selected subject's claims and a cite lookup resolve without any
  fetch closure — i.e. the state exposes them directly from the fold.
- [ ] AC-4: (covers REQ-5) `cospan watch-repo . --once` exits 0 with the plain
  dashboard, and `cospan subject . telos/p0-spine` still lists that subject's
  claims.

## Architecture
Today `src/substrate.rs` has `Dashboard` (from `kan status` + `day status`),
`subject_claims` (a `kan show <subject>` per call), `claim_index` (an `--all`
fold for cites), and `process_snapshot` (another `--all` fold). The TUI folds
`Dashboard` on each tick and then lazily shells `subject_claims` on every
selection move (cached per fold), `claim_index` when a detail opens, and
`process_snapshot` when a process view opens — several spawns, some on the hot
key path.

`Fold` subsumes all of it. `fold(repo)` runs `kan show --all --json` once and
walks the nested `subjects[].claims[]` (the shape `claim_index`/`process_snapshot`
already read): each subject's claims parse and sort newest-first into a
`HashMap`, every claim also lands in `by_cid`, and the `atom/`/`telos/`/`tension/`
subjects fold into a `ProcessSnapshot` — reusing the existing
`claim_from_value`, ordering, `parse_atom`/`parse_telos`/`parse_tension`, and the
`Dashboard::namespace_counts`/`sessions` logic, now methods on `Fold`. `day status`
is the one remaining second spawn (day exposes no `--json`), kept for the process
pane. `Dashboard` is removed; `collect` becomes `fold`.

`src/tui.rs` `AppState` drops `dash`, `claims`, `cite_index`, `process`,
`ensure_selected_loaded`, `ensure_cite_index`, and `ensure_process`, holding one
`fold: Fold`. `selected_claims`/`selected_claim`/`claim_count` read
`fold.claims_for(subject)`; `detail_view` takes `Some(&state.fold.by_cid)`; the
Atoms/Telos views read `state.fold.process`; the process pane reads
`state.fold.day_status`. `refold` becomes `self.fold = substrate::fold(&repo)` on
an mtime change — the poll gate is unchanged, so a quiet UI spawns nothing between
edits. `plain_frame` renders from the `Fold`. The `subject` CLI path keeps
`subject_claims` (a single subject is already one spawn; folding the whole log for
one subject would be slower, not faster).

Everything stays a read-only projection of kan's fold (`telos/kan-is-truth`); this
is purely how many times cospan asks kan for it.

## Resolved Questions
- RQ-1: The whole app derives from one `kan show --all --json` per tick (kan#123
  makes this one spawn cheaper than N per-subject spawns); `day status` remains a
  second spawn only because day has no machine-readable output.
- RQ-2: The `subject` CLI subcommand keeps `subject_claims` (one subject, one
  spawn); only the TUI moves to the whole-log fold, where the amortization pays off.

## Open Questions

_None outstanding — the two choices above were resolved during design._

## Out of Scope
- Linking the `kan` crate as a library to avoid the spawn entirely (a later
  hot-path upgrade, `.dropbox/02-kan-day-integration.md`).
- Incremental/partial folds (diffing the log); the whole-log fold is fast enough
  at this scale (176 claims in ~72ms).
- A machine-readable `day status` to fold day's live position in (blocked
  upstream, `day-summary-in-cospan`).
