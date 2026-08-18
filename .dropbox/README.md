# cospan — design notes

Working design corpus for **cospan**: a CLI-native "mission control" for AI-driven
development — one apex view that glues many agent sessions, harnesses, files, and
[`kan`](../../kan) claims together, plus a drift-tracking **comment layer** for
talking back to a running agent. It sits on the [`kan`](../../kan) (memory) and
[`day`](../../day) (process) substrate.

> **cospan** (category theory): `X → A ← Y` — open pieces joined along a shared
> interface. Exactly what `day` does (atoms are open units with typed in/out
> interfaces, composed along matching boundaries), and what this tool does with
> sessions, files, and comments.

## Status (2026-08-18)

- ✅ **Re-localizer core built** — the one load-bearing algorithm. 6/6 tests,
  working `demo` + `watch` modes. See [`04-relocalizer.md`](04-relocalizer.md).
- ✅ **P0 watch-and-fold spine built** — `cospan watch-repo <path>` polls
  `.kan/log/HEAD` and redraws a dashboard (day process position + agents/handoff
  sessions + claims-by-subject) from the real `kan`/`day` binaries. Runs against
  `~/code/kan-tools/day` today. Shells out (library upgrade later); plain-text
  render (ratatui later).
- ⬜ Still designed-not-built: the ratatui TUI + four real views, comment sidecar
  store + MCP server, editor, constructed dispatch hierarchy.
- Reserved sibling crate names published: `lan`, `yoneda`.

## Read in this order

| doc | what |
|-----|------|
| [`00-vision.md`](00-vision.md) | the problem, what cospan is, guiding principles |
| [`01-architecture.md`](01-architecture.md) | the 5 layers, runtime, the poll-and-fold spine, 3 ground truths |
| [`02-kan-day-integration.md`](02-kan-day-integration.md) | how to read kan + day; the dispatch-hierarchy situation |
| [`03-comments.md`](03-comments.md) | anchoring, sidecar format, kan-persistence schema, the comment MCP server |
| [`04-relocalizer.md`](04-relocalizer.md) | the built algorithm: states, thresholds, tests, roadmap |
| [`05-views-ux.md`](05-views-ux.md) | the four views + responsive layout + navigation |
| [`06-roadmap.md`](06-roadmap.md) | phasing P0–P3, open decisions, naming lineage |
| [`07-first-steps.md`](07-first-steps.md) | **fresh-session handoff** — orient fast + the ordered next dev steps |

## The load-bearing principle

Everything the substrate exposes is **poll, not push** — `day` is stateless and
`kan` has no notification channel. So cospan is one debounced watch loop that
re-folds and re-renders. This shapes the whole architecture; see
[`01-architecture.md`](01-architecture.md).
