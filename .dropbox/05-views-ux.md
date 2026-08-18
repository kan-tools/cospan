# 05 — Views & UX

Four views over one shared domain model (L2). The user moves between them
dynamically; layout adapts to terminal size and to which views are active.

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
