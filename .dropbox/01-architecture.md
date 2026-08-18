# 01 — Architecture

## Three ground truths that shape everything

1. **Nothing pushes.** `day` is stateless — stores nothing, no daemon, no event
   bus; it recomputes from `kan` on every read. `kan` has no notification channel.
   → cospan is **one debounced watch loop** on `.kan/log/HEAD` (+ git HEAD) per
   tracked root; on change it re-folds via kan and re-renders. A single tick drives
   the harness view, editor live-reload, and process view together — no four-way
   sync problem.

2. **The dispatch *hierarchy* is not a shipped primitive.** `day` tracks concurrent
   agents as a **flat** registry (`agents/handoff/<thread>` subjects). Parent→child
   is roadmap (kan ADR-75 vouching claims + kan#117 trust frames + day Frames). →
   cospan **constructs** a best-effort tree now from claim authorship + `KAN_AGENT`
   harness tag + `cites` provenance + handoff lineage, shaped to drop onto ADR-75
   when it lands. See [`02-kan-day-integration.md`](02-kan-day-integration.md).

3. **kan is the single source of truth.** Everything rendered is a projection of
   the kan log — *except* ephemeral comments, the only state cospan owns. Invariant:
   **cospan invents no state except comments.**

## Runtime decision

**A single self-contained Rust + `ratatui` TUI that draws its own panes.** tmux is
just one way to background it, not the substrate.

Why self-drawn over tmux-as-layout:
- The comment gutter needs to *expand and reflow the text column* for multi-line
  comments (a UX requirement). tmux's fixed pane model can't do that.
- Link `kan` as a **library** for the hot read path (native fold, no per-read
  process spawn) instead of shelling out.
- One backgroundable static binary; happy over ssh / tmux / `disown`.

## The five layers

```
L4  Command bus ............ one channel. Today: navigate/read only.
                            Seam for "control later" (spawn/kill/redirect/claim-write).
L3  Views (ratatui) ........ session picker · harness view · editor · comment view
                            + responsive layout engine        → 05-views-ux.md
L2  Domain model (memory) .. Agents(constructed hierarchy) · Process(atoms/bridge/telos)
                            · Claims(folded) · Docs+Comments
L1  Ingest / watch ......... file-watch .kan/log/HEAD + git HEAD (debounced) →
                            kan lib re-fold;  slow tick → day status/session_context;
                            comment-store watcher
L0  Substrate .............. kan log (.car + index.sqlite) · git · day(invoked) · N worktrees
```

Plus a **sibling MCP server for comments** (`cospan mcp`), registered alongside
`kan mcp` and `day mcp` in each harness. See [`03-comments.md`](03-comments.md).

### L1 — Ingest / watch

- One watcher per tracked root. Watch `.kan/log/HEAD` and git `HEAD`. Debounce
  (~100–250ms). On change → re-query kan.
- Read path: link the `kan` crate. `Workspace::open_read_only(cwd)` +
  `actions::*_json` (versioned JSON, SCHEMA_VERSION 1) or `fold::fold` for
  structured `SubjectView`s. Sort claims by `(rev, cid)` — the canonical fold order.
- Slower tick (seconds): shell `day` (`day status`, `day session_context` via MCP,
  `day stream list`) — it's stateless and ~30ms/call, so keep it off the hot path.
- Comment-store watcher: watch the sidecar files; re-localize on change.

### L2 — Domain model

In-memory, rebuilt from L1. Four sub-models:
- **Agents**: from `agents/handoff/*` + per-claim authorship → agent nodes
  (identity, harness tag, handoff thread, inferred parent). Ambiguous edges kept
  as a list, not forced into a tree.
- **Process**: from `day` → current atom candidate(s), bridge, telos, drift
  warnings. Position is inferred + possibly ambiguous *by design* — model it as a
  set of candidates.
- **Claims**: folded `SubjectView`s per subject.
- **Docs + Comments**: file tree + the comment store (sidecar; see 03).

### L3 — Views
See [`05-views-ux.md`](05-views-ux.md).

### L4 — Command bus (stubbed)

A single command channel. Today it only carries navigation/read intents. The
"observe now, control later" seam: spawn/kill/redirect agents and write kan claims
slot in here later without re-architecting. Keep write-authority out of P0–P2.
