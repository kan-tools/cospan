# 05 — Views & UX

> **Revised structure — 2026-08-20.** The section below (## Revised tab & footer
> structure) is the current north star; it supersedes the original four-views
> framing that follows it, which is kept as design context. As of v0.0.1-alpha.12
> the shipped shape is: a Browser (kan claims), Atoms and Telos views, a Comments
> view (anchored gutter + right comment column with reflow + threads), and a top
> `day status` process panel. The revision below renames and reshapes these.

## Revised tab & footer structure

**Four tabs plus an always-on footer**, over the one folded model.

```
┌───────────────────────────────────────────────┐
│ [Chat] [Comments] [Ledger] [Process]           │  ← tab bar
│                                                 │
│                  active view                    │
│                                                 │
├───────────────────────────────────────────────┤
│ ☀️ atom: release · 0/1 done · 🌿 main · ✔ …     │  ← footer (status bar)
└───────────────────────────────────────────────┘
```

### The footer (status bar) — not a tab

A thin, always-on, constantly-updating status bar at the **bottom**, replacing the
current top `day status` panel. Its content is day's own status line — the exact
text Claude Code renders — read from **`day status-line`** (a cache-only command:
it reads a pre-rendered cache written by `day hook session-start`, never touching
kan or git, because Claude Code cancels a status line at 300 ms). The cache lives
at `.day/statusline` / `.day/statusline.variants`, the latter carrying width/style
**variants** tagged `#day-footer <emoji|plain> <width>` (e.g. `emoji 43`,
`plain 57`). cospan polls the cache each tick (or on its mtime — poll-don't-
subscribe) and picks the variant matching its footer width and emoji support.
This is *not* `day status` (a terse single-atom report for people) and *not* the
`day hook session-start` banner (advisory session context) — it is the compact
footer line.

### (1) Chat

Cross-harness agent chats with session hierarchy — the live **session buffers**
themselves, not a summary. This is an open architecture question (a "tmux-ey"
capture): candidate sources are `tmux capture-pane` against known sessions,
cospan owning the PTYs of sessions it spawns (ties into the P3 command bus), or
reading harness transcript files. **Decide the source in its own design pass
before building.** Until then it is a declared-but-empty tab.

### (2) Comments — the editor view

The Comments view we are building, evolving toward a real editor surface:
- The file list becomes a **collapsible tray** (toggled open/closed), showing a
  **full filesystem tree with git state**, not just files that already carry a
  comment. Closed by default while reading/commenting a file.
- The editor pane shows **visual diffs** (git working-tree changes rendered),
  live as agents rewrite files.
- The **bottom strip is removed**; comments live only in the right column, and
  comment overflow opens an **expanded popup** on a shortcut key rather than a
  permanent tray.

### (3) Ledger

The kan claim browser — today's Browser view, renamed. Every subject's claims,
each opening to full detail (`telos/readable-claim-browser`).

### (4) Process

A synthesized view of day's process structure — today's Atoms and Telos views,
reimagined:
- **Atoms** as a **flowchart** (the atom DAG with `next` edges) rather than a
  flat content list, with **drill-down** into each atom's structure.
- **Teloi** as a **list** with **drill-down** to the full statement, witnesses,
  and tensions.

The synthesized day *position* (which atom, per-witness state) still awaits
machine-readable `day status` (kan-tools/day#240); the footer covers the compact
live status meanwhile.

---

_Original framing (design context; superseded by the section above):_

## (a) Session picker

A render of L1 discovery: scan for `.kan` roots across worktrees, read
`agents/handoff/*` to enumerate live sessions. Lists sessions/agents you're
working on; selecting one focuses the harness/editor/comment views on it. This is
the "one place I can see everything" entry point.

## (b) Harness view

The live state of a selected agent + the process it's in:
- **Process flow** from `day`: the atom **DAG** (atoms = typed in/out units, `next`
  edges), the active bridge, the telos, drift warnings. Render current-atom
  **candidate set** (day reports ambiguity as a list — do not fake a cursor).
- **Dispatch context**: where this agent sits in the (constructed) hierarchy —
  parent/children if resolvable, else the flat `stream_list` registry. Unresolved
  edges shown as a list.
- **kan claims** stream for the session's subjects (folded views).

## (c) Editor view

File-type-agnostic, **tree-sitter** syntax highlighting, **live** (auto-reload from
the L1 watch as agents rewrite files). Read-first (this is a review/nav surface,
not an authoring IDE). Shows comment markers inline in the gutter.

## (d) Comment view

Comments in a **right gutter, anchored to the lines they reference**. A **multi-line
comment expands and reflows the text column** to make room (the requirement that
rules out tmux panes — cospan draws this itself). Each comment shows its
re-localizer state (`Anchored`/`Drifted`/`Unresolvable` + confidence). Unresolvable
comments collect in a list to re-place by hand. Threads expand inline. Both human
and (gated) agent comments appear here, attributed by identity.

## Responsive layout engine

Breakpoint-driven; recompute on `SIGWINCH`:

| width | layout |
|-------|--------|
| narrow | one view full-screen + a switcher; comment gutter collapses to markers |
| medium | editor + comment gutter (gutter overlays/expands on focus) |
| wide | picker rail + harness + editor + comments simultaneously |

Because it's one self-drawn TUI, a view can *grow* (comment expansion) rather than
being locked to a fixed pane. Views are composable panes over the shared model, so
any subset can be shown at once and the screen-fill re-solves accordingly.

## Navigation & interaction (sketch — decide during build)

- A leader/space menu for discoverability (Helix-style), not memorized arcana.
- Fast switching between the four views; focus follows selection in the picker.
- Comment actions: add (pin to current line/selection), reply, resolve, and the
  **persist-to-kan** shortcut (03).
- Everything is read-only through P2; the command bus (L4) gains write actions in
  P3.
