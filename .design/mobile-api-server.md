# Feature: Read-only HTTP/WS API server over the fold (mobile Phase 1)

## Summary
Add a `cospan serve` subcommand that runs the existing watch-and-fold loop
headless and exposes it over a localhost HTTP/WebSocket API: `GET /fold` returns
the serialized `substrate::Fold` (the kan/day projection behind the Ledger and
Process tabs), `WS /stream` pushes it on every refold, and `GET /comments` /
`GET /thread` reuse the S5 MCP read core. This is Phase 1 of the mobile-frontend
vision (`.dropbox/08-mobile-frontend.md`) — the transport down payment. The web
client and secure remote transport are later, separate phases. Serves
`telos/observe-now-control-later` (read-only first; the write/control channel is
`command_bus::WriteChannel`, untouched here).

## Requirements
- REQ-1: A new **`serve` subcommand** — `cospan serve <repo> [--port N]` — added to
  the hand-rolled dispatch in `src/main.rs` (a `Some("serve") => …` arm plus the
  usage string), parsing the repo path the way `watch_repo` does (`src/main.rs`) and
  delegating to a new server module (a sibling of `src/mcp.rs`) whose `run` owns a
  tokio runtime via `block_on`, mirroring `mcp_cmd` (`src/main.rs`) → `mcp::run`
  (`src/mcp.rs`). `fn main` (`src/main.rs`) stays synchronous.
- REQ-2: `substrate::Fold` (`src/substrate.rs`) and its component types — `Claim`
  (`src/substrate.rs`), `ProcessSnapshot`, `Atom`, `TelosView`, `Tension` — gain
  `#[derive(Serialize)]` alongside their existing derives; they hold only
  serde-friendly fields (`String`/`Vec`/`Option`/`HashMap`/tuples), so
  `serde_json::to_string(&fold)` is the `/fold` body with no separately-maintained
  shape.
- REQ-3: The server runs the **headless fold loop** in a tokio task: the same
  `head_mtime` + `should_refold` (`src/tui.rs`) mtime gate that drives the TUI,
  calling `substrate::fold(&repo)` (`src/substrate.rs`) on change at a ~250ms tick,
  with no terminal or ratatui. It keeps the latest `Fold` in shared state and, on
  each refold, publishes it to subscribers. (`telos/poll-dont-subscribe` governs how
  cospan reads the *substrate*; pushing to cospan's own clients is a channel cospan
  owns — consistent with the vision.)
- REQ-4: **`GET /fold`** returns the current serialized `Fold` as a JSON snapshot,
  via an `axum` handler reading the shared latest-fold state.
- REQ-5: **`WS /stream`** pushes the serialized `Fold` to each connected client on
  every refold, over a `tokio::sync::broadcast` channel fed by the fold loop. A
  newly-connected client receives the current `Fold` immediately, then updates as
  the log changes.
- REQ-6: **`GET /comments?file=<rel>`** and **`GET /thread?file=<rel>&id=<id>`**
  return JSON by calling the S5 MCP read core directly — `mcp::list_comments`
  (`src/mcp.rs`) and `mcp::get_thread` (`src/mcp.rs`), already
  `serde_json::Value` and already path-traversal-guarded by `guard` (`src/mcp.rs`).
  The blocking file I/O runs under `tokio::task::spawn_blocking`.
- REQ-7: The server binds **127.0.0.1 only**, runs in the **foreground** (Ctrl-C
  stops it), and writes **no state to disk** — it stays a throwaway like
  `watch-repo`, holding the disposability the vision flags as in tension
  (`telos/disposable`). `--port` defaults to a fixed loopback port.
- REQ-8: `Cargo.toml` gains `axum` (with its `ws` feature) and adds the `net`/`sync`
  features to the existing `tokio` dependency; no second async runtime is
  introduced — the runtime is built in the `serve` handler exactly as `mcp::run`
  builds one today.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-2) A unit test builds a `Fold` with subjects, a `Claim`, and
  a `ProcessSnapshot` (atoms/teloi/tensions), calls `serde_json::to_string`, and
  asserts the JSON contains those subjects, the claim's fields, and the process
  keys — proving the derives cover every rendered type.
- [ ] AC-2: (covers REQ-1, REQ-4) An integration test starts the server on an
  ephemeral loopback port against a temp git repo carrying a real `.kan` log,
  issues `GET /fold`, and asserts the response JSON holds the repo's subjects and
  claims. Gated the way the real-substrate smokes are (it shells `kan`).
- [ ] AC-3: (covers REQ-3, REQ-5) A test connects to `WS /stream`, asserts the
  current `Fold` arrives immediately, then triggers a refold (a new claim / touched
  `.kan/log/HEAD`) and asserts a pushed `Fold` update is received.
- [ ] AC-4: (covers REQ-6) A test asserts `GET /comments?file=<f>` returns exactly
  what `mcp::list_comments(&repo, f)` produces, and that a traversal `file=../x` is
  rejected (the `guard` path), never reading outside the repo.
- [ ] AC-5: (covers REQ-7) A test asserts the server's bind address is loopback
  (127.0.0.1) — a non-loopback bind is never constructed — and that no file is
  written under the repo by starting and stopping the server.
- [ ] AC-6: (covers REQ-3, REQ-8) A unit test on the headless fold-loop step asserts
  that, given a changed HEAD mtime, it rebuilds and swaps the shared latest-`Fold`
  (via `should_refold`) with no terminal in the loop, and that the process builds
  and links `axum`/`tokio` (it compiles and the test runs).

## Architecture
The server is a new module beside `src/mcp.rs` — the same shape as the S5 MCP
server, which already proved cospan can host a transport over its operations core
without disturbing the synchronous TUI. `serve_cmd` in `src/main.rs` resolves the
repo (like `watch_repo`) and calls the module's `run`, which builds a
`tokio::runtime::Runtime` and `block_on`s an `axum` server — the identical pattern
`mcp::run` (`src/mcp.rs`) uses, so `fn main` stays sync and no `#[tokio::main]` is
introduced.

Inside `run`: a **shared latest-fold** state (an `Arc<Mutex<Fold>>` or an
`arc_swap`) plus a `tokio::sync::broadcast::Sender<…>` are created. A spawned task
runs the **headless fold loop** — `head_mtime` + `should_refold` (`src/tui.rs`,
both terminal-independent) gating `substrate::fold(&repo)` (`src/substrate.rs`) on a
250ms tick — and on each change stores the new `Fold` and broadcasts it. This is
the exact fold-rebuild core the TUI's `run` loop (`src/tui.rs`) uses, lifted out of
the ratatui event loop; nothing in it touches crossterm.

Handlers: `GET /fold` serializes the shared `Fold` (REQ-2 derives) via
`serde_json`. `WS /stream` subscribes to the broadcast channel, sends the current
`Fold` on connect, then forwards each broadcast. `GET /comments` / `GET /thread`
call `mcp::list_comments` / `mcp::get_thread` (`src/mcp.rs`) under `spawn_blocking`
and return their `serde_json::Value` directly — the same path-guarded core the S5
agent transport uses, now over HTTP.

This is a projection of the log, like the TUI — nothing is persisted and no claim
is written (`telos/kan-is-truth`, `telos/observe-now-control-later`). The write
seam `command_bus::WriteChannel` (`src/command_bus.rs`) is deliberately untouched.
The one telos this trades against is `telos/disposable`: a server is more than a
Ctrl-C sidecar, held in check by REQ-7 (foreground, loopback-only, no on-disk
state), and recorded as an explicit tension.

## Resolved Questions
- RQ-1: Phase-1 scope is **Fold + Comments** — `GET /fold` + `WS /stream` (the
  kan/day projection behind Ledger and Process) plus the comment reads
  (`GET /comments`, `GET /thread`) reusing the S5 MCP core. **Chat** (needs new
  `Serialize` derives on the `transcripts` types), the **web client**, and the
  **secure remote transport** are later phases, out of this slice.
- RQ-2: The `Fold` is serialized by **deriving `Serialize`** on `Fold`/`Claim`/the
  process types (a single source of truth for the shape), not a hand-written
  `serde_json::Value` projection.
- RQ-3: `serve` stays **disposable**: a foreground process bound to 127.0.0.1 with
  no daemon, no auto-start, and no on-disk state — the `telos/disposable` tension is
  named and held, not resolved away.

## Out of Scope
- The **web/PWA client** — a separate non-Rust codebase and its own design (vision
  Phase 2).
- **Secure remote transport** (overlay network, app-level auth, mTLS) and any
  non-loopback exposure — vision Phase 3; this slice is localhost-only with no auth.
- **Write/control** over the API (comment writes, spawn/kill/redirect) — vision
  Phases 4–5; `command_bus::WriteChannel` stays unimplemented here
  (`telos/observe-now-control-later`).
- The **Chat tab** over the API, and pushing comment/chat updates over the WS — the
  WS carries only `Fold` refolds in this slice.
- Any change to the TUI, the re-localizer, the comment sidecar, or the MCP stdio
  server.
