# cospan

A CLI-native "mission control" for AI-driven development: one apex view that glues
many local agent sessions, files, and [`kan`](../kan) claims together — plus a
drift-tracking comment layer for talking back to a running agent, from the
terminal or from another agent.

Named for the categorical **cospan** (`X → A ← Y`): open pieces joined along a
shared interface. That is exactly what [`day`](../day) does — atoms are open units
with typed in/out interfaces, composed along matching boundaries — and what this
tool does with sessions, files, and comments.

## The comment layer

Files move under an agent's edits, so a comment can't be pinned to a line number —
it's pinned to a text *fingerprint* and re-resolved on every change into one of
three honest states (never a silent guess):

| state | meaning |
|-------|---------|
| `Anchored` | found uniquely; high confidence |
| `Drifted` | text changed, best-guess match with a confidence score |
| `Unresolvable` | lost, or ambiguous → resolve-by-hand list |

Around that re-localizer (`src/lib.rs`) is a full round trip:

- **Browse & comment** — the `watch-repo` TUI's **Comments** tab is a collapsible
  file tree (git status per file) beside a syntax-highlighted code pane
  (~150 languages via syntect/onig, incl. Lean). Move the cursor to preview any
  file; the anchored line carries a background band. Author interactively: `a` add,
  `r` reply, `e`/`d` edit/delete your own, `x` resolve.
- **Promote to kan** — `p` snapshots a comment (or `P` a file's set) into a durable,
  signed [`kan`](../kan) claim on `comment/<file>`, anchored to the line span, with
  a fenced `cospan-comment` block for a lossless round trip. The sidecar stays
  ephemeral and keeps re-localizing; the kan claim is frozen. Promoted comments show
  a ◆ (vs ● for ephemeral), read back from the fold.
- **Agents too** — `cospan mcp` serves the comment layer to a harness as MCP tools,
  so an agent reads and writes the same anchored comments a human does (see below).

## Commands

```
cargo run -- watch-repo <path>          # the TUI: Chat · Comments · Ledger · Process
cargo run -- comment add <file> --line <N> <body>   # headless: add a comment
cargo run -- comment reply <file> <id> <body>       #           reply
cargo run -- comment resolve <file> <id>            #           resolve
cargo run -- comments <file>            # list a file's comments, re-localized
cargo run -- mcp [repo]                 # serve the comment layer over MCP (stdio)
cargo run -- demo                       # scripted edit sequence (re-localizer)
cargo run -- watch <file> --line <N>    # live re-localize a single pinned comment
cargo test
```

## Install & the MCP server

```sh
cargo install --path .        # puts `cospan` on your PATH
```

`cospan mcp [repo]` is a stdio MCP server (repo defaults to the working dir). It
exposes five tools over the ephemeral sidecar — read: `list_comments` (each
re-localized), `get_thread`; write: `add_comment`, `reply`, `resolve`. Agent
writes are attributed `who:"agent"` with the id from a `KAN_AGENT` env tag. It
writes only cospan's own sidecar state, never `kan` or the (future) command bus.

This repo is also a **Claude Code plugin** (`.claude-plugin/` + `.mcp.json`): once
`cospan` is on PATH, the plugin registers the `cospan-comments` server. Portable
config for any MCP client:

```json
{ "mcpServers": { "cospan-comments": {
  "command": "cospan", "args": ["mcp", "/abs/path/to/repo"],
  "env": { "KAN_AGENT": "claude-code:me" } } } }
```

## Design

Everything is **poll-and-fold**: the kan/day substrate has no push channel, so one
debounced watch loop folds `kan show --all --json` + `day status` per tick and
drives every view. The comment sidecar (`.cospan/comments/`, gitignored) is cospan's
only owned, mutable state; everything else is a projection of the kan log.

The vision corpus lives in [`.dropbox/`](.dropbox/) (architecture, the comment
model, the re-localizer, views/UX, roadmap, and a recorded mobile-frontend idea).
Still ahead: an editor-view redesign for the Comments tab, and a possible mobile
frontend over the same fold.
