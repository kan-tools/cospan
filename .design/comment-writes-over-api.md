# Feature: Comment writes over the API (Slice C — observe → control)

## Summary
The first *write* over `cospan serve`: add / reply / resolve comments from the
phone, opt-in behind `--allow-writes`. Writes go only to cospan's owned sidecar
state (`.cospan/comments/`, the `telos/kan-is-truth` exception) — never to kan and
never through the agent-control command bus — so this crosses `observe → control`
by the mildest step available, reusing the tested S5 write cores. A
`GET /capabilities` probe and minimal write affordances in the Comments tab make
it usable from the phone.

## Requirements
- REQ-1: **Opt-in writes.** A `--allow-writes` flag (default off) enables the POST
  routes; without it `serve` is read-only and a `POST` to a comment route gets
  `405` (the route carries only `GET`). `serve_cmd` (`src/main.rs`) parses it and
  passes it to `server::run`; `app` (`src/server.rs`) mounts the POST handlers
  only when it is set. Writes are still gated by Slice B's auth middleware (it
  runs on every method), so a write also needs the token.
- REQ-2: **Three write endpoints**, reusing the S5 cores:
  `POST /comments?file=<rel>` body `{ "line": <n>, "body": <s> }` → add;
  `POST /thread?file=<rel>&id=<id>` body `{ "body": <s> }` → reply;
  `POST /resolve?file=<rel>&id=<id>` body `{ "value": <bool> }` (default `true`) →
  resolve. Each calls `mcp::add_comment_as` / `mcp::reply_as` / `mcp::resolve`
  (`src/mcp.rs`) under `spawn_blocking`, and is path-guarded by the same `guard`
  as the reads — a `file=../x` returns the guard error, never a write outside the
  repo.
- REQ-3: **Attribution.** A web-authored comment/reply is stamped
  `Author { who: "human", id: <author-id> }`, where `<author-id>` comes from
  `--author` / `COSPAN_WEB_AUTHOR`, defaulting to `"web"` — an honest provenance
  for the operator acting over the web channel. The write cores are parameterized
  to take an `Author`: new `add_comment_as` / `reply_as` (`src/mcp.rs`) carry the
  author; the existing `add_comment` / `reply` become thin wrappers passing
  `agent_author()`, so the MCP surface and its tests are unchanged.
- REQ-4: **Serialized writes.** Concurrent web writes cannot lose an update: the
  handlers take a `Shared` write mutex (an `Arc<std::sync::Mutex<()>>`, mirroring
  `CommentServer.writes` in `src/mcp.rs`) around the load-modify-save, held inside
  `spawn_blocking` and never across an `.await`.
- REQ-5: **Capabilities probe.** `GET /capabilities` (always present, behind auth)
  returns `{ "writes": <bool>, "author": { "who": "human", "id": <id> } }` when
  writes are on, and `{ "writes": false }` when off — so the page knows whether to
  show write UI without probing a `405`.
- REQ-6: **Minimal write UI + disposability.** When `/capabilities` reports
  `writes`, the Comments tab (`src/web/index.html`) shows: an add-comment form on
  a file's comment list, a reply box in the thread view, and a resolve toggle on a
  comment — each `POST`ing with the page's `?token=` and reloading that level on
  success. Writes touch only the sidecar tree: no kan claim is written
  (`telos/kan-is-truth`; promote-to-kan stays a human TUI action), the write seam
  `command_bus::WriteChannel` (`src/command_bus.rs`) is untouched, and
  `--allow-writes` prints a one-line "control enabled" notice at startup. Nothing
  is persisted beyond the sidecars cospan already owns (`telos/disposable`).

## Acceptance Criteria
- [ ] AC-1: (covers REQ-3) A unit test asserts `mcp::add_comment_as` /
  `mcp::reply_as` stamp the supplied author (`who:"human", id:"web"`) on the
  written record, while `mcp::add_comment` / `mcp::reply` still stamp
  `agent_author()` — the MCP path is unchanged.
- [ ] AC-2: (covers REQ-1, REQ-5) A CI integration test with writes **off**
  asserts `POST /comments?file=x` returns `405` and `GET /capabilities` returns
  `{"writes":false}`.
- [ ] AC-3: (covers REQ-2, REQ-3, REQ-4) A CI integration test with writes **on**:
  `POST /comments?file=<f>` `{line,body}` adds a comment that a subsequent
  `GET /comments?file=<f>` shows authored `who:"human", id:"web"`; `POST /thread`
  adds a reply; `POST /resolve` sets resolved; and a second add after the first
  leaves **both** comments present (no lost update).
- [ ] AC-4: (covers REQ-1, REQ-2) A CI integration test asserts a write is
  gated and guarded: `POST /comments` with no token → `401`; with the token but
  `file=../../etc/passwd` → the guard error and no write outside the repo.
- [ ] AC-5: (covers REQ-5, REQ-6) A unit test asserts `GET /capabilities` shape
  for both postures, the web-author id resolves to `"web"` by default and to an
  override when supplied, and `INDEX_HTML` wires `/capabilities` and a `POST`
  write path (an add/reply/resolve affordance) gated on the writes capability.

## Architecture
**Cores (`src/mcp.rs`).** Extract `add_comment_as(repo, file, line, body, author)`
and `reply_as(repo, file, id, body, author)` — the current bodies with the
hardcoded `agent_author()` replaced by the passed `Author`. `add_comment` /
`reply` become one-line wrappers passing `agent_author()`, so the MCP tool
handlers, the stdio smoke, and `serve_auth`'s existing `add_comment` call are
untouched. `resolve` already takes no author. All three keep the `guard`.

**Server (`src/server.rs`).** `Shared` gains `allow_writes: bool`, a `web_author:
Author`, and `writes: Arc<Mutex<()>>`, set by a `with_writes(author)` builder
(default posture stays read-only, so every existing `seed(...)` call site — tests
and smokes — is unaffected). `app` mounts POST only when `allow_writes`:
`.route("/comments", get(get_comments).post(post_comment))`,
`.route("/thread", get(get_thread).post(post_reply))`, and
`.route("/resolve", post(post_resolve))`; otherwise the GET-only routes stand and
a POST is a `405`. `GET /capabilities` is always mounted. Each write handler
extracts its `Query` (file, id) and a `Json` body, locks `writes`, and runs the
core under `spawn_blocking`. The auth middleware and the loopback bind are
unchanged; writes ride the same gate.

**CLI (`src/main.rs`).** `serve_cmd` parses `--allow-writes` (bool) and
`--author <id>` alongside the Slice-B flags and threads them into `run`, which
resolves the author (`--author` → `COSPAN_WEB_AUTHOR` → `"web"`) and prints the
control-enabled notice when writes are on.

**Page (`src/web/index.html`).** On load, fetch `/capabilities`; store `writes`.
In the Comments views, when `writes` is true, render an add-comment form (line +
body) on the file view, a reply box in the thread view, and a resolve toggle on a
comment — each calling `fetch(withTok(...), {method:"POST", body: JSON})` and
re-loading that level on success. Read-only when `writes` is false (today's
behavior). Rendering stays `textContent`/`createElement`.

This is a projection-plus-owned-state write: the sidecar is cospan's one owned
store (`telos/kan-is-truth`'s stated exception), and nothing here writes a kan
claim or signs anything — the seed never enters the picture, because comment
writes never reach kan. The `observe-now-control-later ↔ disposable` tension
sharpens (a remote write channel is more than a Ctrl-C sidecar); it is held by
REQ-1 (opt-in, off by default), REQ-6 (sidecar-only, no daemon, no on-disk state
beyond the owned tree), and the existing loopback-only bind.

## Resolved Questions
- RQ-1: **Writes are opt-in** via `--allow-writes` (off by default), so `serve`
  stays read-only unless control is explicitly enabled — the
  `telos/observe-now-control-later` ordering, made a runtime switch rather than an
  always-on capability.
- RQ-2: **Web writes are attributed `who:"human"`** with a configurable id
  (default `"web"`), not reused `agent_author()` — a human operator's comment is
  not mislabeled as an agent's. The cores are parameterized so the MCP agent path
  keeps `who:"agent"`.
- RQ-3: **Three POST endpoints** (`/comments`, `/thread`, `/resolve`) mirroring the
  three S5 write tools 1:1, each beside its GET, rather than a PATCH state-change —
  the simplest shape consistent with the existing endpoints and the resource
  grammar Slice A set.

## Out of Scope
- **kan claim writes / promote-to-kan over the API.** Promoting a comment into the
  kan log is a signing action; it stays a human action in the TUI, on the laptop
  that holds the seed. This slice writes only sidecars.
- **Agent control** (spawn / kill / redirect) and any use of
  `command_bus::WriteChannel` — the sensitive control tier, a later phase with its
  own authz model and per-action confirmation.
- **Per-user identity / multi-writer auth.** One shared token and one configurable
  author id; distinguishing individual writers is not in this slice.
- **Per-write confirmation.** Comment writes to owned state are low-stakes and
  immediate; the confirmation model the vision reserves is for the control tier.
- **Editing or deleting existing comments** over the API — only add / reply /
  resolve, the three S5 write ops.
