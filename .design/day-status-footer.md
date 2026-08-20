# Feature: the day status-line footer

## Summary
Replace the top `day status` process panel with a thin, always-on **footer** at
the bottom of the TUI that renders day's own status line — the exact compact text
Claude Code shows — sourced from day's status-line cache and width-matched to the
footer. This fixes the mis-sourced top panel (it currently scrapes the verbose
`day status`) and frees the top of the screen for the active view.

## Requirements
- REQ-1: A pure `pick_variant(cache: &str, width: u16, emoji: bool) ->
  Option<Vec<String>>` parses day's variants cache — blocks introduced by a
  `#day-footer <style> <width>` header line (`style` is `emoji` or `plain`,
  `width` a number) followed by that variant's text lines — and returns the lines
  of the best fit: the preferred style (`emoji` when `emoji`, else `plain`) whose
  declared width is the largest that is `<= width`, falling back to the narrowest
  of that style, then any variant, then `None`.
- REQ-2: `status_footer(repo, width, emoji) -> Vec<String>` reads
  `<repo>/.day/statusline.variants` and returns `pick_variant`'s lines; if the
  cache is missing or yields nothing it shells `day status-line` (the stable
  cache-only CLI) and returns its lines; if that also fails it returns a single
  explicit `"(day status-line unavailable)"` line — never empty
  (`telos/honest-ambiguity`).
- REQ-3: `draw` (`src/tui.rs`) renders the footer as a thin bottom band (the
  variant's lines, no verbose panel) and **removes the top process panel**; the
  reclaimed vertical space goes to the active view. The footer shows under every
  tab.
- REQ-4: The footer refreshes on a `.day/statusline.variants` mtime gate inside
  the single poll tick (`should_refold`-style, one added `stat` per tick), so it
  updates as day rewrites the cache without a second loop
  (`telos/poll-dont-subscribe`).
- REQ-5: The non-interactive `plain_frame`/`--once` output and the `subject` CLI
  are unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test on `pick_variant` over a cache holding an
  `emoji 43` and a `plain 57` block asserts: `emoji` requested at width 50 returns
  the emoji block's lines; `plain` requested returns the plain block; a width
  below every variant still returns the narrowest of the preferred style; and an
  empty/garbage cache returns `None`.
- [ ] AC-2: (covers REQ-2) A unit test in a temp repo with a `.day/statusline.
  variants` file asserts `status_footer` returns the expected variant's lines, and
  that with no cache file it returns a non-empty fallback (the explicit unavailable
  line, since `day status-line` is not guaranteed in the test environment).
- [ ] AC-3: (covers REQ-4) A unit test asserts the footer mtime gate
  (`should_refold`) fires only when the cache file's mtime changes.

## Architecture
`substrate` (`src/tui.rs` / `src/substrate.rs`) currently folds `day status` text
into `Fold::day_status`, and `draw` (`src/tui.rs`) renders it in a bordered
process panel above the body via a three-row vertical layout
(header / process / body). This change moves that status to a bottom footer and
changes its source.

day's status line is **cache-only**: `day status-line` reads a pre-rendered cache
(written by `day hook session-start`), never kan or git, because Claude Code
cancels a status line at 300 ms. The cache file `.day/statusline.variants` holds
several renders, each a `#day-footer <style> <width>` header followed by its
lines — e.g. `#day-footer emoji 43` and `#day-footer plain 57`. `pick_variant`
parses these blocks and selects by style then width, so cospan shows the widest
variant that fits its footer and matches its emoji support; it is a pure function
of the file text, unit-tested without touching day. `status_footer` wraps it with
the file read and the `day status-line` / explicit-line fallbacks.

`AppState` gains a `footer: Vec<String>` and a `footer_mtime: Option<SystemTime>`;
the `run` loop, after the HEAD gate, stats `.day/statusline.variants` once and
refreshes `footer` only when its mtime changes (reusing `should_refold`) — the
same single-tick, poll-don't-subscribe discipline the Comments view uses. `draw`'s
vertical layout becomes header / body / footer (the footer a fixed few rows), and
the old process-panel block is removed; the `day status` fold field may stay for
`plain_frame` but is no longer drawn at the top. Nothing new is written anywhere;
the footer is a projection of day's published cache.

## Resolved Questions
- RQ-1: The footer reads day's **variants cache file** (for width-matching)
  rather than only shelling `day status-line` (which emits one variant), with
  `day status-line` as the fallback — so cospan picks the right width without
  losing a stable CLI backstop. The cache is an artifact day publishes for
  consumers, like `.kan/log` (`telos/kan-is-truth`'s substrate-read pattern).
- RQ-2: The status moves to the **bottom** as a thin footer (not a top panel and
  not a tab), matching a status bar's conventional place and freeing the top.
- RQ-3: Emoji vs plain: cospan requests `emoji` by default (its other views use
  Unicode already); a future setting can select `plain`. Width comes from the
  footer's own width at render time.

## Open Questions

_None — the source, the cache-vs-CLI split, and the placement were resolved
during design._

## Out of Scope
- The Process tab reshape (atoms-as-flowchart, telos drill-down); this cycle only
  moves and re-sources the status line.
- Triggering a status-line **recompute** (that is `day hook session-start`'s job
  and carries session semantics); cospan reads the cache day maintains.
- Any tab rename (Browser→Ledger) or the Chat tab.
