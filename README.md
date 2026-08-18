# cospan

A CLI-native "mission control" for AI-driven development: one apex view that glues
many local agent sessions, files, and [`kan`](../kan) claims together — plus a
drift-tracking comment layer for talking back to a running agent.

Named for the categorical **cospan** (`X → A ← Y`): open pieces joined along a
shared interface. That is exactly what [`day`](../day) does — atoms are open units
with typed in/out interfaces, composed along matching boundaries — and what this
tool does with sessions, files, and comments.

## Status: prototype

The first load-bearing piece is built: the **comment re-localizer** (`src/lib.rs`).
Files move under an agent's edits, so a comment can't be pinned to a line number —
it's pinned to a text *fingerprint* and re-resolved on every change into one of
three honest states:

| state | meaning |
|-------|---------|
| `Anchored` | found uniquely; high confidence |
| `Drifted` | text changed, best-guess match with a confidence score |
| `Unresolvable` | lost, or ambiguous → resolve-by-hand list |

```
cargo run -- demo                       # scripted edit sequence
cargo run -- watch <file> --line <N>    # live: poll + re-localize as you edit
cargo test
```

## Where this is going

- Views: session picker · harness view · editor (live, tree-sitter) · comment view
- Integration: link `kan` as a library for the fold; observe `day` process position
  and the (best-effort) agent dispatch hierarchy
- Comments: ephemeral sidecar by default, with a shortcut to persist a file's
  comments to `kan` (per-file subject, anchored to commit SHA + context + timestamp)
  and publish to the git tree via `kan publish`
- An MCP server exposing sidecar comments to every harness (read always; write
  behind a user setting, for agent-to-agent doc commenting)

Everything is **poll-and-fold**: the substrate has no push channel, so one debounced
watch loop drives every view.
