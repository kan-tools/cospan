# 07 — First steps (fresh-session handoff)

**START HERE if you're picking up cospan cold.** This gets you oriented and gives
you the next concrete, ordered development steps with acceptance criteria.

## What cospan is (in one breath)

A CLI-native "mission control" for AI-driven dev: one apex view gluing agent
sessions, files, and [`kan`](../../kan) claims, plus a drift-tracking **comment
layer** for talking back to a running agent. It rides on the `kan` (memory) and
`day` (process) substrate in `~/code/kan-tools/`. Read [`00-vision.md`](00-vision.md)
for the why.

## Orient in 5 minutes

1. Read, in order: [`README.md`](README.md) → [`01-architecture.md`](01-architecture.md)
   → [`02-kan-day-integration.md`](02-kan-day-integration.md). Skim 03–06.
2. See it run:
   ```bash
   cd ~/code/kan-tools/cospan
   cargo run -- demo                              # the re-localizer, scripted
   cargo run -- watch-repo ~/code/kan-tools/day --once   # the P0 substrate dashboard
   cargo test                                      # 6/6, all in src/lib.rs
   ```
3. Code map:
   - `src/lib.rs` — the **re-localizer** (comment anchoring; the load-bearing algo) + `pub mod substrate`.
   - `src/substrate.rs` — P0 kan/day reads → `Dashboard` (shells to the binaries).
   - `src/main.rs` — subcommands `demo`, `watch`, `watch-repo`.

## Three rules that are easy to get wrong

- **Poll, don't subscribe.** The substrate has no push channel. Everything is a
  debounced watch loop that re-folds. Don't go looking for an event stream — there
  isn't one.
- **Mirror honest ambiguity.** `day` reports an ambiguous process position as a
  *list of candidates*; the re-localizer returns `Unresolvable` rather than
  guessing. Never collapse these into a single fake answer in the UI.
- **Ground every kan/day claim in the source.** The facts in 02 came from reading
  `~/code/kan-tools/{kan,day}`, which move fast. Before relying on a specific
  flag/subject/JSON field, verify it (e.g. run `kan status --json`, `day status`).

## The development arc — do these in order

Each step is small, shippable, and leaves the tool runnable. Commit after each.

### Step 1 — Per-subject claim drill-in (still plain-text, still shelling)
The P0 dashboard shows *subjects*; now show a subject's *claims*.
- Add `fn subject_claims(repo, subject) -> Vec<Claim>` in `substrate.rs`, folding
  `kan show <subject> --json` (shape: `{v, subjects:[{claims:[{cid, kind, author,
  recorded_at, text, ...}]}]}` — see 02).
- Add subcommand `cospan subject <repo> <subject>` that prints each claim: kind,
  short author, `recorded_at`, first line of `text`. Sort by `(rev, cid)` if
  present, else `recorded_at`.
- **Done when:** `cospan subject ~/code/kan-tools/day telos/spine` lists that
  subject's folded claims.
- *Why first:* zero new deps, extends the proven spine, and it's the core of the
  claims browser you'll need in the TUI.

### Step 2 — Go interactive: ratatui + crossterm
Turn the print-loop into a real TUI (the [`01-architecture.md`](01-architecture.md)
runtime decision).
- Add `ratatui` + `crossterm`. Build an app loop: poll `.kan/log/HEAD` on a tick,
  re-fold on change, redraw; handle keys (`q` quit; `j`/`k` move a selection).
- Port the P0 dashboard into a single ratatui widget first (no new panes yet).
- **Done when:** `cospan watch-repo <path>` is an interactive, flicker-free TUI you
  quit with `q`.

### Step 3 — Two panes: subject list + claim detail
The first real view split (seed of the harness + claims browser).
- Left pane: grouped subject list (reuse `namespace_counts` / group headers).
  Right pane: the selected subject's claims (Step 1's fold). Selection updates the
  right pane live.
- Add the responsive breakpoint skeleton from [`05-views-ux.md`](05-views-ux.md)
  (narrow → one pane + switcher; wide → both).
- **Done when:** you can arrow through subjects and read their claims in-pane.

### Step 4 — Comment sidecar store (P1 begins)
Bring in the reason cospan exists.
- Implement the sidecar comment store from [`03-comments.md`](03-comments.md).
  **First decide the open question:** sidecar location — recommend the
  `.cospan/comments/<path>.jsonl` tree (keeps working dirs clean). Record the
  decision in [`06-roadmap.md`](06-roadmap.md).
- Wire the **re-localizer** (already built) to compute each comment's live
  `{state, span, confidence}` against the current file contents.
- Add `cospan comments <file>` that lists each comment with its live state
  (`Anchored`/`Drifted`/`Unresolvable`).
- **Done when:** you can drop a comment in a sidecar, edit the file, and watch the
  comment re-localize — the doc-comment round trip, headless.

> After Step 4 you're into the P1 body: the editor view (tree-sitter, live), the
> comment gutter that expands the text column, and the `cospan mcp` server
> (read-only first). See [`06-roadmap.md`](06-roadmap.md) for P1–P3.

## Decisions that gate work (don't guess — ask the human)

- **Sidecar location/format** — needed before Step 4. (Recommendation above.)
- **Dispatch hierarchy** (P2) — ship the *inferred* agent tree, or hold for kan
  ADR-75 / kan#117 and show only the flat `stream_list`? Not needed until P2.
- Comment durability default is **already decided**: sidecar-only, with an explicit
  human "persist to kan" shortcut. Don't relitigate.

## Working conventions

- After changing code, run `cargo test` and one `cargo run -- watch-repo
  ~/code/kan-tools/day --once` sanity render. Commit per step.
- To update the installed binary: `cargo install --path . --force`. For iteration,
  prefer `cargo run --` from the repo.
- Keep the plain-text/shell-out path working until ratatui + library-linking fully
  replace it; don't rip out the spine mid-flight.
