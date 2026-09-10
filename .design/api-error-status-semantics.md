# Feature: API error status semantics — map core errors to 4xx

## Summary
Today every cospan HTTP endpoint returns `200 OK` even when the core produced an
error, with the failure carried only in a `{"error": …}` JSON body (a convention
inherited from the S5 core / Slice A). Clients must inspect the body, not the
status. Map those core errors to real HTTP status codes (400/404/500) across all
read and write endpoints, keeping the `{error}` body backward-compatible, by
tagging each core error with an additive machine-readable `code` and mapping it
to a status in a single handler helper.

## Requirements
- REQ-1: **Error Values carry an additive `code`.** Each core error site tags its
  `{"error": …}` Value with a `code` string: `bad_request`, `not_found`,
  `forbidden`, or `internal`. This is additive — the shared functions are also
  called by the MCP server (`src/mcp.rs` tool handlers), which ignores the extra
  field, so the MCP surface is unaffected. Sites and their codes:
  `mcp::comment_files`/`list_comments` path guard (`src/mcp.rs:56`) → `bad_request`;
  `mcp::file_view` "not a readable file" (`src/mcp.rs:197`) → `bad_request`;
  `mcp::get_thread`/`resolve`/`reply` "no comment {id}" (`src/mcp.rs:300`, `:364`,
  `:380`) → `not_found`; the write `Err(e)` arms (`src/mcp.rs:340`, `:368`, `:384`)
  → `bad_request`; `chat::chat_session` "no chat session {id}" (`src/chat.rs:92`)
  → `not_found`.
- REQ-2: **A single handler helper maps `code` → status.** A `respond(v:
  serde_json::Value) -> Response` in `src/server.rs` returns `(StatusCode, Json(v))`:
  a Value with no `"error"` key → `200 OK` unchanged; otherwise the status is
  `not_found` → 404, `forbidden` → 403, `internal` → 500, and `bad_request` or any
  other/absent code → 400. The body is passed through verbatim.
- REQ-3: **Every endpoint routes through the helper.** `get_comments`,
  `get_thread`, `get_chat`, `get_files`, `get_file` (`src/server.rs`), and the
  write handlers `post_comment`, `post_reply`, `post_resolve` (via `write_blocking`)
  return `respond(v)` instead of `Json(v)`. `get_files` has no core error path but
  is routed too for uniformity, so that its `spawn_blocking` join-failure fallback
  maps to 500 like the others. Every `unwrap_or_else` join-failure fallback tags
  `code: "internal"` so a worker panic is a 500, not a 400.
- REQ-4: **The handler-level "missing ?file=" is a 400.** `post_comment`'s early
  `missing ?file=` return (`src/server.rs:543`) returns `respond` of a
  `bad_request`-coded body (400), not a 200.
- REQ-5: **The error body shape is preserved (backward compatible).** A 4xx
  response still carries the `{"error": …}` field (plus the new `code`). The web
  client (`src/web/index.html`) reads `body.error` and does not gate on
  `response.ok`/`response.status` (confirmed: no such check in any fetch path), so
  a 4xx status does not regress the page — `fetch` does not reject on 4xx and the
  JSON body still parses.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2, REQ-3) An integration test in
  `tests/serve_auth.rs` asserts real GET statuses via `http_status`: a valid
  `GET /comments?file=<f>` (or `/files`) → 200; `GET /file?path=../../etc/passwd`
  (path escape) → 400; `GET /file?path=src` (a directory / not a readable file) →
  400; `GET /thread?file=<f>&id=bogus` → 404; `GET /chat?session=bogus` → 404.
- [ ] AC-2: (covers REQ-3, REQ-4) A write integration test (via `spawn_writes`)
  asserts `http_post` statuses: `POST /resolve` with a bogus id → 404; `POST
  /comments` with no `?file=` → 400.
- [ ] AC-3: (covers REQ-2) A `server::tests` unit test on `respond`: an
  error-free Value → 200; `{error, code:"not_found"}` → 404; `{error,
  code:"bad_request"}` → 400; `{error, code:"internal"}` → 500; `{error}` with no
  code → 400.
- [ ] AC-4: (covers REQ-5) An integration test asserts a 4xx response body still
  contains a JSON `error` field (via `http_post`/`http_body`), so body-reading
  clients keep working.
- [ ] AC-5: (covers REQ-1..REQ-5) `cargo test`, `cargo clippy --all-targets --
  -D warnings`, and `cargo fmt --check` are green.

## Architecture
**HTTP-layer status mapping over an additively-coded core error** — the core
keeps returning a `serde_json::Value`, so nothing about the read/fold model or
the MCP callers changes structurally.

**Core (`src/mcp.rs`, `src/chat.rs`).** Each `json!({ "error": … })` construction
gains a sibling `"code"` field per REQ-1. These functions are shared with the MCP
tool handlers (`src/mcp.rs`), which return the Value as tool output; the extra
`code` field is inert there. No signature changes — the error stays in-band in the
Value, which is why the blast radius is a handful of `json!` literals rather than a
`Result` refactor rippling through every caller.

**HTTP layer (`src/server.rs`).** A new `respond(v) -> Response` centralizes the
mapping (REQ-2). Each handler currently ends `Json(v)`; it becomes `respond(v)`.
The `spawn_blocking` join-failure `unwrap_or_else` closures gain `code:
"internal"` so a worker panic maps to 500 (the one genuine server-fault path;
every other coded error is client-caused by construction, so an unmarked error
defaults to 400). `get_files` is routed through `respond` too — it has no core error path, but
uniformity means its worker-panic fallback maps to 500 rather than a lone 200.
`get_capabilities`/`get_fold`/`get_stream` are unchanged (they never carried an
error body).

**Client (`src/web/index.html`).** No change required (REQ-5): the fetch paths
read `body.error` and never branch on status, and `fetch` resolves (not rejects)
on 4xx. Verified by grep — the only `.status` uses are kan claim-status badges,
unrelated to HTTP.

**Invariants.** Read-only endpoints stay reads; writes stay gated on
`--allow-writes`. No fold-shape or route change; the wire change is additive
(`code` on error bodies) and a status upgrade from 200 to the correct 4xx.

## Resolved Questions
- RQ-1: **A typed `code` on the error Value**, mapped to status by one handler
  helper — chosen over the handler classifying by inspecting the error message
  string. Substring classification would make a reworded message silently change
  the HTTP contract; an explicit `code` is robust and is additive/harmless to the
  MCP surface that shares these functions.
- RQ-2: **Status mapping.** Path-escape guard → 400 (treated as malformed input;
  it does not confirm what exists outside the repo, unlike a 403); "not a readable
  file" → 400; no such comment / no such chat session → 404; missing `?file=` /
  bad body → 400; a `comments` write `Err` → 400; a `spawn_blocking` worker panic
  → 500. `forbidden` (403) is defined in the helper for completeness but no core
  site emits it in this slice.

## Out of Scope
- **A `Result`-typed core refactor** — the error stays in-band in the returned
  `Value`; converting the core readers/writers to `Result<_, ApiError>` (and
  updating the MCP callers) is a larger change this slice deliberately avoids.
- **Axum extractor rejections** — a missing *required* query param (`/thread`
  without `file`/`id`, `/file` without `path`) already returns axum's own 400;
  this slice does not reshape those into the `{error, code}` body.
- **Changing the MCP tool responses** — the MCP surface keeps returning the
  in-band error Value (now with a `code`); its behavior is unchanged.
- **New endpoints, auth changes, or a machine-readable error taxonomy beyond the
  four codes** — only the four codes needed to cover the existing error sites.
