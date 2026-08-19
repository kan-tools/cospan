# Feature: Two-pane subject list + claim detail

## Summary
Split the interactive `watch-repo` TUI into two panes: the grouped subject list
on the left (the selection already built in Step 2) and, on the right, the
selected subject's live claims folded via `subject_claims` (Step 1), updating as
the selection moves. A first responsive breakpoint shows both panes side by side
on wide terminals and one at a time on narrow ones. This is the seed of the
claims browser — you can arrow through subjects and read their claims in-pane.
Serves `telos/p0-spine`.

## Requirements
- REQ-1: `AppState` (src/tui.rs) gains a claims cache keyed by subject name and a
  narrow-mode active-pane flag. A loader fetches the selected subject's claims via
  `substrate::subject_claims` lazily — invoked from the event loop, never from
  `draw` — and stores the outcome in the cache. The loader is parameterized over
  the fetch function so its cache behavior is unit-testable without shelling out.
- REQ-2: The interactive view renders two panes: LEFT the existing grouped
  subject list with the selection highlighted; RIGHT the selected subject's
  claims from the cache, one line per claim. The right pane is a pure projection
  of `substrate::subject_claims` output — cospan re-derives nothing
  (`telos/kan-is-truth`).
- REQ-3: Fetch-on-select with a per-fold cache: the selected subject's claims are
  fetched at most once per fold generation; revisiting a subject is a cache hit
  (no new `kan` spawn); a re-fold (`.kan/log/HEAD` changed) clears the cache so
  stale claims are never shown. An empty subject and a failed fetch are rendered
  as distinct, explicit states — never a blank or fabricated pane
  (`telos/honest-ambiguity`).
- REQ-4: A responsive breakpoint at 100 columns: at width ≥ 100 both panes render
  side by side; below it, one pane at a time — the subject list by default,
  `Enter` shows the selected subject's claim detail full-screen, `Esc` returns to
  the list. `j`/`k`/Down/Up always move the subject selection and the detail
  follows it, in both layouts.
- REQ-5: The per-claim line format (kind, short author, UTC, summary) is a single
  shared function on `substrate::Claim`, used by both the `cospan subject`
  subcommand (`src/main.rs`) and the TUI detail pane, so the two renderings cannot
  drift.
- REQ-6: `watch-repo --once` still renders the existing plain-text dashboard and
  exits 0 — the scriptable, testable path is unchanged; the two-pane browser is
  interactive-only.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-3) A unit test drives the loader with a counting
  fake fetcher: selecting a subject fetches once; revisiting it is a cache hit
  (the fetcher is not called again); after a re-fold the cache is cleared and the
  fetcher is called again.
- [ ] AC-2: (covers REQ-4) A unit test asserts the layout mode is wide at width
  100 and above and narrow below, and that in narrow mode `Enter` sets the active
  pane to detail and `Esc` returns it to the list.
- [ ] AC-3: (covers REQ-2, REQ-5) A unit test asserts the shared
  `Claim` line formatter renders a claim as its kind, short author, UTC stamp, and
  summary; both `cospan subject` and the detail pane call this one function.
- [ ] AC-4: (covers REQ-2, REQ-6) `cospan watch-repo . --once` exits 0 and still
  prints the plain dashboard (the grouped subject list and verbatim `day status`),
  confirming the two-pane work did not disturb the scriptable path.
- [ ] AC-5: (covers REQ-3, REQ-6) A unit test of the detail render function shows
  a distinct "(no live claims)" line for an empty subject and a visible error line
  for a failed fetch, and the claim lines newest-first for a populated subject —
  never an empty render.

## Architecture
Step 2 left `src/tui.rs` with `AppState` (grouped `Row`s + a subject selection),
a poll-driven `run` loop, a `draw` function, and `plain_frame` for `--once`. Step
3 extends the interactive path only; `--once`/`plain_frame` and the `demo`,
`watch`, and `subject` subcommands are untouched (REQ-6).

`AppState` gains `claims: HashMap<String, Result<Vec<Claim>, String>>` (keyed by
subject name) and a `pane: Pane` flag (`List` or `Detail`) for narrow mode. A
method `ensure_selected_loaded(fetch)` — parameterized over a
`Fn(&Path, &str) -> Result<Vec<Claim>, String>` — populates the cache for the
selected subject if absent; the `run` loop calls it with
`substrate::subject_claims` after any selection change or re-fold, and tests call
it with a counting fake (AC-1). `refold` clears `claims` so a changed log never
shows stale detail (REQ-3). Loading in the loop rather than in `draw` keeps `draw`
free of I/O and keeps kan off the redraw path — the same reason
`.dropbox/01-architecture.md` puts the watch on a debounced tick.

`draw` chooses its layout from `frame.area().width` against a `WIDE_COLS = 100`
constant: `Layout::horizontal` with the list and detail side by side when wide,
or a single pane — list, or detail when `pane == Detail` — when narrow. A pure
`layout_mode(width)` and the `Enter`/`Esc` pane transitions are what AC-2 tests;
the ratatui draw itself stays thin glue over them. The detail pane renders
`claims.get(selected_subject)`: the ordered claim lines when present, a single
"(no live claims …)" line for an empty vector, or an error line when the fetch
returned `Err` — a pure `detail_lines(subject, &Result<…>)` function that AC-5
tests directly. day's process pane and the `clip_lines` overflow cue from Step 2
are retained.

The per-claim line currently formatted inline in `src/main.rs`'s `subject_cmd`
(kind, `short_author`, `recorded_utc`, `summary`) moves to a
`Claim::display_line` method on `substrate::Claim`; `subject_cmd` and the detail
pane both call it (REQ-5), so the CLI and the TUI can never disagree about how a
claim reads. This is the same fold — `subject_claims` — already proven in Step 1;
the right pane is one more consumer of it, re-deriving nothing.

The compact day process-summary header (`day-summary-in-cospan`) is deliberately
excluded — it is blocked on a machine-readable day status, since parsing day's
human `day status` text is brittle and the `day status-line` cache is stale.

## Resolved Questions
- RQ-1: The right pane fetches on selection change and caches per fold generation
  — at most one `kan show` spawn per subject until the log changes, so scrolling
  back is instant and a re-fold refreshes everything.
- RQ-2: The narrow switcher is `Enter` to open the detail and `Esc` to return; the
  breakpoint to side-by-side is 100 columns. `j`/`k` always move the subject
  selection in both layouts.
- RQ-3: The compact day-summary header is out of scope for this step — blocked on
  a machine-readable day status (recorded on `day-summary-in-cospan`); Step 3 is
  the two-pane split only.

## Open Questions

_None outstanding — the three scope choices above were resolved during design._

## Out of Scope
- The compact day process-summary header — blocked on machine-readable day status
  (`day-summary-in-cospan`).
- Independent scrolling of a very long claim list within the detail pane; claim
  counts per subject are small, so the pane renders the fold and defers a scroll
  offset to a later step.
- The medium breakpoint from `.dropbox/05-views-ux.md`; this step ships only the
  narrow/wide split.
- The tree-sitter editor view, the comment sidecar and gutter, and the session
  picker / harness multi-view (later phases).
- Linking the `kan` crate as a library; reads still shell out through
  `substrate::subject_claims`.
- Any write path; the tool stays read-only through P2.
