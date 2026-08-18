# 03 — Comments

The comment layer is the reason cospan exists beyond "a nice viewer." Two halves:
the **anchoring model** (keep a comment on the right text as the file moves — built,
see [`04-relocalizer.md`](04-relocalizer.md)) and the **storage + sharing model**
(this doc).

## The core split: live sidecar vs. frozen kan claim

Files are in motion (agents rewrite them); kan is append-only immutable. Each side
does the one thing it's good at:

| | **sidecar comment** | **kan comment claim** |
|---|---|---|
| role | live, mutable localization | immutable point-in-time snapshot |
| lifetime | ephemeral (human↔agent scratch) | durable, signed, publishable |
| tracks file? | yes — re-localizes every tick | no — frozen at persist time |
| default | ✅ this is the default | opt-in, via explicit human action |

This resolves the tension: the sidecar chases the moving file; the kan claim
records "this is what I said, on this text, at this SHA," forever.

## Sidecar format (default, ephemeral)

Per-file sidecar, JSONL, next to the file (e.g. `<file>.cospan.jsonl` or a
`.cospan/comments/<path>.jsonl` tree — **decide in P1**). Auto-synced on every edit.

Each comment record (sketch):
```jsonc
{
  "id": "c_01H...",
  "anchor": {
    "target": "    let token = fetch_token(user);",   // exact line(s)
    "before": "fn login(user: &str) -> bool {",         // context above
    "after":  "    validate(token)",                     // context below
    "line_hint": 1,                                       // 0-based, tiebreak only
    "base_hash": "…"                                      // file hash at last anchor
  },
  "body": "is this call cached? it's hot.",
  "author": { "who": "human" | "agent", "id": "claude-code|role:prover|did:key:…" },
  "thread": [ { "author": …, "body": …, "at": … } ],
  "resolved": false,
  "created_at": 1734500000000000
}
```
Runtime attaches a transient `localization: { state, span, confidence }` (from the
re-localizer) — **not stored**; recomputed each tick.

Anchoring is by **text fingerprint**, never bare line number (see 04). As the file
changes, cospan tracks incrementally against the *last-seen* contents, not the
original, so many small edits don't accumulate drift.

## kan persistence (opt-in, on explicit human action)

A shortcut persists one comment — or a file's whole comment set — to kan. Maps
1:1 onto kan's real model:

- **Subject**: `Local("comment/<file-path-slug>")` — one claim log per file (your
  "per-file subjects"). The fold gives the file's live comment set.
- **workspace Anchor**: `LineRangeAt(path, commit_sha, span)` — kan already carries
  "commit SHA + line span." Free.
- **Body**: `Observation { text }`, where `text` carries a fenced **`cospan-comment`**
  JSON block — following day's house convention (its `day-atom`/`day-telos` blocks
  are fenced JSON in claim prose). The block holds what the anchor doesn't: the
  surrounding-text fingerprint ("adjacent context state"), thread/reply state,
  resolved flag.
- **`recorded_at`**: microsecond timestamp = "time of addition to kan."
- **`cites`**: optionally cite the prior claim it replies to (threading provenance).

Then **`kan publish comment/<path>`** → signed Markdown in `.claims/`, travels in
`git diff`, verifiable by CID. This is the canonical publish → git-tree path.

The persisted claim is an **immutable snapshot** — it never re-localizes. The
sidecar keeps tracking; kan freezes.

> Status note: this whole persistence path is explicitly a *temporary* solution —
> good enough to get durable, attributable, shareable comments without inventing a
> new store. Revisit once kan/day grow richer per-file/claim conventions.

## The comment MCP server (`cospan mcp`)

Registered beside `kan mcp` and `day mcp` in each harness config. Makes comments a
first-class thing agents can participate in.

- **Read (always on):** `list_comments(file)`, `get_thread(id)` — every harness can
  read what's been said on a file, anchored to context.
- **Write (behind a user setting):** `add_comment`, `reply`, `resolve` — enables
  **agent-to-agent doc commenting**: agent B drops a comment on a file agent A
  wrote, attributed to B's kan identity / harness tag; it surfaces live in the
  human's comment view.
- When "persist-to-kan" is *also* enabled, a written comment additionally lands as
  the `Observation` claim above (durable + attributable). Default stays
  sidecar-only.

### Anchoring for agent writes
An agent writing a comment supplies the same fingerprint (target + context) it's
commenting on; cospan re-localizes it into the human's view like any other comment.
Ambiguous → `Unresolvable` list, same as everything else.
