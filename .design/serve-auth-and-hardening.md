# Feature: `cospan serve` auth + remote-channel hardening (Slice B)

## Summary
Now that `tailscale serve` makes `cospan serve` reachable from a phone, the
read API needs to be safe to expose. Add app-level authentication (a generated
bearer token gating every route, including the `/stream` WebSocket upgrade) and
close the two hardening follow-ups recorded on `mobile-api-server`: a
symlink-resolving path guard so `/comments`/`/thread` cannot escape the repo via
an in-repo symlink, and a configurable cap on concurrent `/stream` connections.
This is the honest prerequisite for Slice C (comment *writes* over the API):
authenticate and harden the observe channel before anything crosses it to write
(`telos/observe-now-control-later`), while keeping `serve` a throwaway
localhost-first process with no user store (`telos/disposable`).

## Requirements
- REQ-1: **Token lifecycle.** `cospan serve` mints a URL-safe random token at
  startup (from OS randomness via `getrandom`, hex-encoded, ≥32 hex chars),
  prints it **once to stderr** as a ready-to-open URL, and holds it **in memory
  only** — never written to disk (consistent with `transcripts.rs`'s recorded
  "cospan never reads a secret store"). `--token <t>` or the `COSPAN_SERVE_TOKEN`
  env var overrides the generated one (for a stable URL); `--no-auth` disables
  auth entirely. Auth is **on by default**.
- REQ-2: **Every route is gated** when auth is on — `GET /`, `GET /fold`,
  `GET /comments`, `GET /thread`, and the `WS /stream` upgrade. The token is
  accepted **either** as a `?token=<t>` query parameter **or** an
  `Authorization: Bearer <t>` header. A missing/incorrect token yields `401` for
  HTTP routes and a rejected upgrade (no `101`) for `/stream`. The comparison is
  **constant-time** (a fixed hand-rolled byte compare, no new crate) so the gate
  leaks no timing signal.
- REQ-3: **The embedded page carries its own token.** `src/web/index.html` reads
  `token` from its own `location` query on load and reuses it on every `/fold`
  and `/comments` fetch (as `?token=` or a Bearer header) and on the
  `/stream?token=<t>` WebSocket URL — so a single saved link
  `https://<host>/?token=<t>` works on mobile with no in-app token entry. With
  `--no-auth`, the page and API work token-free (no `?token=` needed).
- REQ-4: **Symlink-resolving path guard.** `mcp::guard` (`src/mcp.rs`) rejects a
  `file` whose *resolved real path* escapes the repo root, not merely a lexical
  `..`/absolute path. It canonicalizes the repo root and the target — or, when
  the target does not yet exist, its **nearest existing ancestor** — and requires
  the result to be contained in the repo root. This closes the recorded follow-up
  (an in-repo symlink pointing outside the repo, now reachable over HTTP).
- REQ-5: **`/stream` connection cap.** A configurable maximum of concurrent
  `/stream` clients (`--max-stream <N>`, default `64`) is enforced: an upgrade
  beyond the cap is cleanly rejected (`503`, no `101`); the live count is
  decremented when a client disconnects (via an RAII guard so an early return or
  panic still releases the slot). This closes the recorded follow-up.
- REQ-6: **Stays disposable and loopback-first.** Auth is an `axum` middleware
  layer over the existing router; the `127.0.0.1`-only bind
  (`server.rs::bind_addr`) is unchanged; no user database, no session store, no
  on-disk state is introduced — the token and the connection counter live only in
  the in-memory `Shared` state for the life of the process. When `--no-auth` is
  set, `serve` prints a one-line warning that the channel is unauthenticated
  (since a proxy like `tailscale serve` can expose a loopback bind).

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts the token minter returns a
  non-empty URL-safe token (hex, ≥32 chars) and that two mints differ; and that
  config resolution yields `None` (no gate) under `--no-auth` and the supplied
  value under `--token`/env. No server or kan needed.
- [ ] AC-2: (covers REQ-2, REQ-6) A CI test (no kan needed — a `401` short-
  circuits before any fold) starts the server with a known token on an ephemeral
  loopback port and asserts: `GET /fold` with no token → `401`; with
  `?token=<t>` → `200`; with `Authorization: Bearer <t>` → `200`; with a wrong
  token → `401`.
- [ ] AC-3: (covers REQ-2, REQ-3 WS) An `#[ignore]`d smoke asserts a `/stream`
  upgrade **without** a token is rejected (no `101`) and **with** `?token=<t>`
  connects and receives the snapshot frame.
- [ ] AC-4: (covers REQ-3) A unit test asserts `INDEX_HTML` reads `token` from
  the location query and references both `?token=` and a Bearer usage, plus the
  smoke opening `/?token=<t>` returns `200 text/html` while `/` without the token
  returns `401`.
- [ ] AC-5: (covers REQ-4) A unit test creates a temp repo containing a symlink
  that points outside it (e.g. `link -> /etc`), calls
  `mcp::list_comments(repo, "link/passwd")`, and asserts it returns the guard
  error — the real path escaped the repo — while a normal in-repo relative file
  is still accepted. No kan needed (pure fs + guard).
- [ ] AC-6: (covers REQ-5) A test drives `--max-stream 1`: the first `/stream`
  client connects, a second is rejected (`503`/no `101`), and after the first
  disconnects a new one succeeds again (the slot was released). Runs as the
  `#[ignore]`d smoke (real WS client).

## Architecture
The whole change lands in the existing `serve` module and its two collaborators;
no new long-lived machinery.

**Auth (`src/server.rs`).** Add an `Auth` value to the server config — either the
resolved token (`Arc<str>`) or `None` for `--no-auth`. Gate with an `axum`
middleware layer (`axum::middleware::from_fn_with_state`) wrapping the whole
`Router` in `app(...)`, so the single check covers `/`, `/fold`, `/comments`,
`/thread`, and the `/stream` upgrade request (the middleware runs on the HTTP
request that *initiates* the upgrade, before `on_upgrade`). The middleware pulls
the presented token from the `Authorization: Bearer` header or the `token` query
param, compares it constant-time to the configured token, and returns `401` on
mismatch. When `Auth::None`, the layer is a pass-through. This is the first
middleware in the module — `app`'s doc comment ("no middleware") updates.

**Token minting (`src/main.rs` / `src/server.rs`).** `serve_cmd` parses
`--token`, `--no-auth`, and `--max-stream` alongside the existing `--port`.
`server::run` resolves the effective auth: `--no-auth` → none; else
`--token`/`COSPAN_SERVE_TOKEN` if present; else a fresh mint from `getrandom`
(promoted to a direct dependency — it is already in the tree). It prints the
ready URL (`http://127.0.0.1:<port>/?token=<t>`) once to stderr, mirroring the
existing `eprintln!` startup line, and the `--no-auth` warning when applicable.

**Connection cap (`src/server.rs`).** `Shared` gains an
`Arc<std::sync::atomic::AtomicUsize>` live-stream counter and a `max_stream:
usize`. `get_stream` checks-and-increments before `on_upgrade`; over the cap it
returns `503` without upgrading. `stream_folds` holds an RAII guard whose `Drop`
decrements the counter, so a normal close, an error, or a panic all release the
slot.

**Symlink guard (`src/mcp.rs`).** `guard` keeps its cheap lexical reject
(absolute / `..`) and adds a canonicalized containment check: canonicalize the
repo root once, resolve the target's real path (or its nearest existing ancestor
joined with the remaining tail), and require `starts_with(repo_root)`. `guard`
gains the repo root as a parameter (today it takes only `file`); its callers
(`list_comments`, `get_thread`, `add_comment`, `reply`, `resolve`) already hold
`repo`. This is a projection-side read guard — it reads and rejects, it never
mutates a subject (`telos/kan-is-truth`).

**The embedded page (`src/web/index.html`).** On load, read
`new URLSearchParams(location.search).get("token")`; keep it in a module
variable; append `?token=` to the `/fold` and `/comments` fetch URLs (or set the
`Authorization` header) and build the WS URL as `/stream?token=<t>`. Absent (the
`--no-auth` case), the URLs are unchanged. Rendering stays `textContent`-only.

Nothing here writes a kan claim, touches `command_bus::WriteChannel`, or changes
the loopback bind; the token and counter are in-memory only
(`telos/disposable`).

## Open Questions

<!-- OPEN: Q1 -->
### Q1: Query-token log exposure — accept, or also support a `#token=` fragment?
A `?token=` query param is visible in the server's own logs, browser history,
and any proxy access log (`tailscale serve`'s included) — the well-known Jupyter
tradeoff. An alternative keeps the token in the URL **fragment** (`#token=`),
which browsers never send to the server: the page reads `location.hash`, uses it
as a Bearer header for fetches and `?token=` only for the WS URL (which has no
header option). The cost: the `GET /` HTML itself then cannot be gated by the
fragment (the server never receives it), so `/` would have to be served
**unauthenticated** — conflicting with the chosen "page is gated" model, and
leaving the static page (no fold data) publicly readable.

Decision taken for this slice: **`?token=` query**, page gated, token accepted as
query or Bearer — matching the chosen transport, at the cost of the token
appearing in logs (mitigated by the tailnet being private and the token being
rotatable by restart). **To resolve later**: decide whether the fragment variant
(ungated `/`, header-only fetch) is worth adding as an option, or whether query
exposure on a private tailnet is acceptable indefinitely.
<!-- /OPEN -->

## Out of Scope
- **Comment/agent writes over the API** — Slice C; this slice authenticates and
  hardens the read channel only. `command_bus::WriteChannel` stays untouched.
- **mTLS / per-device identity / a user database** — the vision's "app-level
  auth too (token / mTLS)"; a single shared bearer token is the disposable floor,
  not a multi-user auth system.
- **The overlay-network transport itself** — provided by `tailscale serve`
  externally; cospan stays loopback-bound and does not manage the mesh.
- **Rate limiting / brute-force lockout beyond the constant-time compare** — a
  32-byte token over a private tailnet does not warrant it in this slice.
- **Rotating the token without a restart** — restart re-mints; a live-rotation
  endpoint is not needed for a disposable process.
- **Chat/transcripts over the API** — a later slice (needs `Serialize` on the
  `transcripts` types).
