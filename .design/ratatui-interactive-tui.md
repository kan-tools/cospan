# Feature: Interactive ratatui TUI for watch-repo

## Summary
Turn the `watch-repo` print-loop into a real interactive terminal UI built on
ratatui + crossterm: a poll-driven app loop that re-folds the substrate only when
`.kan/log/HEAD` changes, redraws without flicker, and lets you move a selection
over the repo's subjects with `j`/`k` and quit with `q`. The existing P0 dashboard
is ported into a single ratatui view — no new panes yet. It is Step 2 of the P0
arc and the foothold every later view builds on. Serves `telos/p0-spine`.

## Requirements
- REQ-1: Add `ratatui` (0.30) and `crossterm` (0.29) as dependencies in
  `Cargo.toml`; the interactive `watch-repo` path renders through ratatui rather
  than ANSI `print!` escapes.
- REQ-2: A new module (src/tui.rs) holds an `AppState` — the folded
  `substrate::Dashboard` plus a selection over the repo's subjects — and the
  event loop; `src/main.rs`'s `watch_repo` stays a thin dispatcher into it.
- REQ-3: The loop is poll-driven, honoring `telos/poll-dont-subscribe`: a single
  `crossterm::event::poll(tick)` call is both the input wait and the tick, and on
  a timeout it re-reads `.kan/log/HEAD`'s mtime and re-folds via
  `substrate::collect` **only when the mtime changed**. No async runtime and no
  notify/subscription is introduced. The change gate is a pure function so it can
  be tested without a filesystem.
- REQ-4: The dashboard is ported into one ratatui view showing, top to bottom:
  the process position (the `day status` text rendered verbatim), the
  `agents/handoff` sessions, and the CLAIMS section as a scrollable list of the
  repo's subjects grouped under namespace headers (built from
  `substrate::Dashboard::namespace_counts` and the subjects themselves), with the
  selected subject highlighted. The view is derived entirely from the folded
  `Dashboard`; cospan invents no state but the selection cursor
  (`telos/kan-is-truth`). An empty sessions or subjects set renders as an explicit
  empty state, never a fabricated row (`telos/honest-ambiguity`).
- REQ-5: Keys: `q` and `Ctrl-C` quit; `j`/`k` and Down/Up move the selection over
  subject rows only (namespace headers are skipped), clamped at both ends. The
  selection is remembered by subject name across a re-fold, and clamps back into
  range if that subject is gone — so a background edit to the log never jumps the
  cursor to an unrelated row.
- REQ-6: `watch-repo --once` renders a single non-interactive plain-text frame to
  stdout and exits 0 — scriptable, CI-friendly, and unit-testable via a pure
  render function. Interactive mode enters the alternate screen and raw mode and
  restores the terminal unconditionally on both normal exit and panic, so a crash
  never leaves the user's terminal broken (`telos/disposable`).

## Acceptance Criteria
- [ ] AC-1: (covers REQ-2, REQ-4) A unit test builds an `AppState` from a
  hand-made `Dashboard` and asserts the rows are namespace headers in
  `namespace_counts` order with each namespace's subjects beneath it, and that the
  number of selectable rows equals the subject count.
- [ ] AC-2: (covers REQ-5) A unit test drives `j`/`k` navigation and asserts it
  moves one subject at a time and clamps at the first and last subject rather than
  wrapping or going out of bounds.
- [ ] AC-3: (covers REQ-5) A unit test re-folds an `AppState` with a new
  `Dashboard`: when the selected subject still exists the selection stays on it by
  name; when it has been removed the selected index clamps into the new range.
- [ ] AC-4: (covers REQ-6, REQ-4) `cospan watch-repo . --once` exits 0 and its
  stdout contains the grouped subject list (e.g. a `telos/p0-spine` line under a
  `telos` group) and the verbatim `day status` text.
- [ ] AC-5: (covers REQ-3) A unit test of the mtime-change gate returns true only
  when the stored modified-time differs from the current one, and false when they
  match.
- [ ] AC-6: (covers REQ-1, REQ-6) `cargo build` succeeds with the new
  dependencies, and interactive mode installs a restore hook (a panic hook plus a
  teardown on the normal path) so the terminal's raw mode and alternate screen are
  always undone; `cargo test` stays green.

## Architecture
Today `src/main.rs`'s `watch_repo` polls `.kan/log/HEAD`'s mtime every 500ms and,
on change, calls `substrate::collect` and prints `render_dashboard` with ANSI
clear-screen escapes. The fold layer (`src/substrate.rs`: `Dashboard`, `Subject`,
`collect`, `namespace_counts`, `sessions`) is already correct and untouched by
this step — this is purely an L3 (views) change over the existing L2 model, the
split `.dropbox/01-architecture.md` draws.

The interactive path moves into a new module, src/tui.rs, exposed as `pub mod tui`
from `src/lib.rs` beside `substrate` so its logic is unit-testable. It carries:

- `AppState { repo, dash, rows, selected, selected_subject, last_mtime }`, where
  `rows` is a flattened `Vec` of a `Row` enum — `Row::Header(namespace)` or
  `Row::Subject(name)` — and only `Subject` rows are selectable. Building `rows`
  from a `Dashboard` (grouping by `namespace_counts` order), the `j`/`k` clamp,
  the by-name selection preservation across `refold`, and the mtime-change gate
  are all pure functions with no terminal or filesystem dependency, which is what
  makes AC-1, AC-2, AC-3 and AC-5 real unit tests rather than manual checks.
- `run(repo)`: the event loop. It uses ratatui 0.30's terminal init/restore and a
  single `crossterm::event::poll(tick)` as both the key wait and the re-fold tick,
  so the whole thing stays poll-and-fold with no push channel — the invariant
  `telos/poll-dont-subscribe` names. A `Drop`-based guard (or ratatui's restore
  plus an installed panic hook) guarantees the terminal is returned to normal on
  every exit path.

Rendering is one shared source of truth: a function turns `(&Dashboard,
selected_subject)` into the ordered display sections, which the `--once` path
formats to a plain `String` for stdout and the interactive path draws as ratatui
`Line`s with the selected row styled. Both paths show identical content, and both
render `day status` verbatim — day already expresses process ambiguity as a
candidate list, and re-interpreting it here would violate `telos/honest-ambiguity`
(`.dropbox/05-views-ux.md` states this for the harness view). This step keeps the
kan read path shelling through `substrate` — linking the `kan` crate is a later
hot-path upgrade, not part of going interactive.

`src/main.rs`'s `watch_repo` becomes a thin dispatcher: `--once` calls the plain
single-frame render; otherwise it calls `tui::run(repo)`. The `demo`, `watch`, and
`subject` subcommands are unaffected. This step is the `generative-build` atom
consuming this design-doc, producing the `code-change` that `adversarial-review`
audits next.

## Resolved Questions
- RQ-1: `j`/`k` moves the selection over the grouped **subject** list, not the
  namespace count rows — a selected subject is exactly what Step 3's claim-detail
  pane will open, so the selection has a downstream meaning from the start.
- RQ-2: `--once` stays a non-interactive plain-text single frame (scriptable,
  CI-friendly, and the seam that keeps the dashboard render unit-testable);
  the interactive default enters the ratatui alternate screen.

## Open Questions

_None outstanding — the two scope choices above were resolved during design._

## Out of Scope
- The two-pane subject-list + claim-detail split and the claim-detail view
  (Step 3); this step ports the dashboard as a single view only.
- The comment sidecar, gutter, and re-localizer wiring (Step 4 / P1).
- The tree-sitter editor view, the session picker, and the harness multi-view
  (`.dropbox/05-views-ux.md`, later phases).
- The responsive breakpoint engine (narrow/medium/wide); this step is one view at
  any width.
- Linking the `kan` crate as a library; reads still shell out through
  `substrate` (a later hot-path upgrade, `.dropbox/02-kan-day-integration.md`).
- Any write path — comment writes, spawn/kill/redirect — the command bus stays
  read-only through P2.
