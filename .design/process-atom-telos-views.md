# Feature: Atom-DAG and telos/witness views

## Summary
Add two top-level views beside the claim browser: an **Atoms** view showing the
day atom graph (each atom's in/out types and next edges) and a **Telos** view
showing the teloi with their declared witnesses and recorded tensions. Both are
parsed from kan claims (the fenced `day-atom`/`day-telos` blocks) — the declared
structure. Live position and witness state are deferred, since they need
machine-readable day, so cospan does not fake day's inference.

## Requirements
- REQ-1: `substrate` parses fenced day blocks — `extract_fenced(text, name)`
  returns the JSON body of a ```` ```<name> ```` block — and folds a
  `ProcessSnapshot` from `kan show --all --json`: the `atom/*` subjects into atoms
  (slug, `in`, `out`, `next`), the `telos/*` subjects into teloi (slug, title,
  statement, witnesses), and the `tension/*` subjects into tension pairs.
- REQ-2: `AppState` gains a top-level `View` (Browser, Atoms, Telos); the number
  keys `1`/`2`/`3` select a view and `Tab` cycles them. The three-level browser
  navigation (subjects/claims/detail) applies only within the Browser view.
- REQ-3: The Atoms view renders each atom as `slug  in[…] → out[…]  next[…]`, from
  cospan's own parse of the `day-atom` blocks — the declared composition, not
  day's position inference.
- REQ-4: The Telos view renders each telos's title, statement, and declared
  witnesses, and lists the recorded tensions between teloi.
- REQ-5: These views show the declared structure only. The live inferred atom
  position and the per-witness met/unmet state are **not** shown — they need
  machine-readable day (`day-summary-in-cospan`), and re-deriving day's inference
  in cospan would fake a cursor day reports as a candidate set
  (`telos/honest-ambiguity`). The views say so rather than guessing.
- REQ-6: The snapshot is folded once and cached on `AppState`, cleared on
  `refold`; the Atoms and Telos views scroll with `j`/`k`. `watch-repo --once` and
  the `demo`/`watch`/`subject`/`comment(s)` subcommands are unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts `extract_fenced` returns the JSON
  body of a `day-atom` block embedded in claim prose and `None` when the block is
  absent.
- [ ] AC-2: (covers REQ-1, REQ-3, REQ-4) A unit test folds a small `--all`-shaped
  payload with one `atom/*` and one `telos/*` subject into a `ProcessSnapshot`,
  asserting the atom's `in`/`out`/`next` and the telos's witnesses are parsed.
- [ ] AC-3: (covers REQ-2) A unit test asserts the view selector cycles
  Browser → Atoms → Telos → Browser and that `1`/`2`/`3` select the matching view.
- [ ] AC-4: (covers REQ-6) `cospan watch-repo . --once` exits 0 with the plain
  dashboard and `cospan subject . telos/p0-spine` still works, confirming the new
  views left the scriptable path and other subcommands intact.

## Architecture
`src/substrate.rs` gains `extract_fenced(text: &str, name: &str) -> Option<&str>`
(finds the line ```` ```<name> ````, returns the text up to the closing ```` ``` ````),
`Atom` and `TelosView` structs, a `ProcessSnapshot { atoms, teloi, tensions }`,
and `process_snapshot(repo)` which shells `kan show --all --json` (the same nested
`subjects[].claims[]` shape `claim_index` reads) and, for each subject whose name
starts `atom/`, `telos/`, or `tension/`, parses the relevant fenced block from its
newest declaring claim's text with `serde_json`. This is the declared vocabulary,
read straight from the log — the same blocks `day doctor` reads, parsed
independently rather than by shelling day.

`src/tui.rs` gains `View { Browser, Atoms, Telos }` on `AppState`, plus a lazily
folded `process: Option<ProcessSnapshot>` cleared on `refold` and an
`atom_scroll`/`telos_scroll` offset. The key handler switches on `view` first:
`1`/`2`/`3` and `Tab` change it (the pure `View::next` and `View::from_digit` are
what AC-3 tests); within Browser the existing focus keys apply; within Atoms/Telos,
`j`/`k` scroll. `draw` dispatches on `view` to the existing browser body,
`draw_atoms`, or `draw_telos`. A one-line header names the active view and the
`1/2/3 · Tab` switch. Both new views open with a dim line stating that live
position and witness state need machine-readable day and are not shown here — an
honest gap, not a fake cursor.

Nothing is written and nothing beyond kan reads is added; the views are a
projection of the declared blocks (`telos/kan-is-truth`). `day status`/`day assess`
would give the live state, but neither offers `--json`, so this step stops at the
structure and records the rest as blocked (`day-summary-in-cospan`).

## Resolved Questions
- RQ-1: Views switch with `1`/`2`/`3` (direct) and `Tab` (cycle); the browser keeps
  its subjects/claims/detail navigation within the Browser view.
- RQ-2: The atom and telos views render the declared structure parsed from the
  `day-atom`/`day-telos` blocks in kan; live position and witness state are
  deferred and labeled as needing machine-readable day, not re-derived in cospan.

## Open Questions

_None outstanding — the two choices above were resolved during design._

## Out of Scope
- Live atom position, off-sequence findings, and per-witness met/unmet state —
  blocked on machine-readable day (`day-summary-in-cospan`).
- Rendering the atom graph as 2-D boxes/arrows; this step lists atoms and their
  edges textually.
- Bridges and their plans (`bridge/*`); this step covers atoms, teloi, and
  tensions.
- Editing or asserting anything; the views are read-only.
