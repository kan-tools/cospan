# Feature: Web view UX pass — comments, claim drill-in, resilient stream (Slice A)

## Summary
A UX pass on the embedded `cospan serve` web view (`src/web/index.html`, served
at `GET /`): a **Comments** tab (files-with-comments → a file's comments → a
comment's full thread) over a regularized read API, **claim drill-in** in the
Browse tab (tap a claim for its full text, resolved cites, artifacts), and two
resilience fixes recorded on `mobile-web-view` (a capped, visibility-aware
`/stream` reconnect and no redundant first render). It stays a single
self-contained embedded page (`telos/disposable`) and read-only
(`telos/observe-now-control-later`); the new server surface is one index
endpoint that establishes the resource-collection shape the Phase-2 web client
and later write verbs extend.

## Requirements
- REQ-1: **Comment index endpoint.** `GET /comments` with **no** `file` param
  returns a JSON index of files that have comments —
  `{ "files": [ { "file": <rel>, "total": <n>, "unresolved": <n> } ] }` — from a
  read-only walk of the `.cospan/comments/` sidecar tree (`comments::sidecar_path`
  maps `<rel>` → `.cospan/comments/<rel>.jsonl`). It reads each sidecar's records
  for the counts only (no re-localization, so it is cheap). `GET /comments?file=`
  is unchanged. This overloads the existing `get_comments` handler
  (`src/server.rs`) whose query `file` becomes optional; the walk lives in a new
  `mcp::comment_files(repo)` core beside `mcp::list_comments` (`src/mcp.rs`),
  path-guarded like its siblings.
- REQ-2: **Comments tab.** A fourth bottom-nav tab in `src/web/index.html`: it
  lists the index (REQ-1) — file path + a count badge + an unresolved marker —
  and on tapping a file loads that file's comments via `GET /comments?file=`
  (each rendered with its state badge anchored/drifted/unresolvable, body,
  1-based line, author), and on tapping a comment loads its full reply thread via
  `GET /thread?file=&id=`. All three requests carry the page's `?token=` through
  the existing `withTok` helper. Rendering stays `textContent`/`createElement`
  only (no `innerHTML` with log data).
- REQ-3: **Claim drill-in.** In the Browse tab, tapping a claim row expands it in
  place to show the **full untruncated `text`** (currently only the first line is
  shown by `summarize`), the `artifacts`, the author and UTC time, and each
  `cites` CID **resolved through the fold's `by_cid` map** to the cited claim's
  one-line summary (falling back to the short CID when the cite is not in
  `by_cid`). No server change — every field is already in `GET /fold`.
- REQ-4: **Resilient reconnect.** The `/stream` reconnect (`connect()` in
  `src/web/index.html`) uses **capped exponential backoff** (e.g. 1s → 2s → … →
  30s cap) instead of a fixed 1.5s retry, resets to the floor on a successful
  open, and **pauses while the tab is hidden** (`document.hidden`) — reconnecting
  immediately on `visibilitychange` back to visible. This closes the recorded
  battery-cost follow-up.
- REQ-5: **No redundant first render.** On load the page currently `fetch`es
  `/fold` and then the WS delivers the same snapshot on connect, causing two
  renders. The first render is gated so only the first-arriving of the two
  renders, and a subsequent identical snapshot does not re-render — closing the
  recorded redundant-render follow-up.
- REQ-6: **Disposable and read-only.** Everything stays in the one embedded
  `include_str!` page and the read-only server; no separate codebase, no build
  step, no write path, `command_bus::WriteChannel` untouched. The API keeps the
  resource-collection shape (`GET /<collection>` lists, `GET /<collection>?sel`
  drills in) so future collections (chat) and write verbs extend it uniformly.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test on `mcp::comment_files`: a temp repo with
  two sidecars (one with an unresolved comment, one all-resolved) returns a
  `files` array naming both with correct `total`/`unresolved`; a repo with no
  `.cospan/comments/` returns `{ "files": [] }`; and a traversal is impossible
  (the walk is rooted at the tree, and any `file` echoed back is repo-relative).
- [ ] AC-2: (covers REQ-1) A CI integration test (no kan; seed a default fold and
  write sidecars under the served repo): `GET /comments` (no `file`) returns the
  index JSON with the seeded files, and `GET /comments?file=<f>` still returns
  that file's comments — proving the overload does not break the single-file path.
- [ ] AC-3: (covers REQ-2, REQ-3, REQ-4, REQ-5) A unit test on `INDEX_HTML`
  asserts the page wires the new behavior: it references `/thread`, a
  `Comments`/fourth-tab marker, `by_cid` (cite resolution), `document.hidden`
  /`visibilitychange` (visibility gating), and a backoff variable — guarding the
  client logic is present and wired, the same way `index_html_is_embedded…` guards
  the existing wiring.
- [ ] AC-4: (covers REQ-1, REQ-6) A test asserts `GET /comments` with no `file`
  and `GET /comments?file=../../etc/passwd` both behave safely — the index never
  lists a path outside the repo, and the traversal still returns the guard error
  (the guard from Slice B still applies to the single-file branch).

## Architecture
**Server (`src/server.rs`, `src/mcp.rs`).** `get_comments` gains an optional
`file`: `Query<CommentsQuery>` where `file: Option<String>`. `Some(f)` →
`mcp::list_comments(repo, f)` (today's path, unchanged); `None` →
`mcp::comment_files(repo)`. The new core walks `repo/.cospan/comments/`
recursively, and for each `*.jsonl` recovers the repo-relative source path by
stripping the `.cospan/comments/` prefix and the `.jsonl` suffix, loads the
sidecar with `comments::load`, and counts `total` and `unresolved` (`!c.resolved`)
— **no `localize_and_update`**, so no file content is read and the index is
cheap. It returns `{ "files": [ … ] }` sorted by path. Missing tree → empty list.
Both branches run under the handler's existing `spawn_blocking`. This is a read
of cospan's own owned sidecar state (`telos/kan-is-truth`'s sole exception),
never a mutation.

The overload is the **resource-collection** shape: `GET /comments` (collection)
lists, `GET /comments?file=` (member) drills in, `GET /thread?file=&id=` is the
sub-resource. `GET /fold` + `WS /stream` remain the process/ledger projection.
This is the grammar the Phase-2 PWA and the eventual write verbs (`POST` on the
same resources, Slice C) extend — chosen now, while the only client is our own
embedded page, so regularizing costs nothing.

**Page (`src/web/index.html`).** A fourth tab `Comments` joins Now/Teloi/Browse
in the bottom nav and the view switcher (`setView`). Its view has three states
held in module vars (file list → selected file's comments → selected comment's
thread), each fetched with `withTok` (so Slice B's `?token=` flows through) and
rendered with the existing `el()`/`createTextNode` helpers. Browse's `claimEl`
gains an expand-on-tap detail block resolving `c.cites` through `fold.by_cid`.
`connect()` replaces the fixed `setTimeout(…,1500)` with a backoff variable
(reset in `onopen`) and a `document.hidden` guard plus a `visibilitychange`
listener. `load()`/`onmessage` share a "rendered once" flag so the fetch and the
connect snapshot do not double-render. Nothing is persisted; the page remains a
pure projection.

## Resolved Questions
- RQ-1: **Comment discovery = the index endpoint**, not deriving from the fold's
  `comment/*` subjects (those are only *promoted* comments — one today) nor a
  manual path box. `GET /comments` (no `file`) returns the files-with-sidecars
  index — complete (ephemeral + promoted) and the first instance of the
  resource-collection grammar future views reuse.
- RQ-2: **Comments depth = files → comments → full thread.** The tab drills all
  the way to a single comment's reply thread via `/thread`, mirroring the two
  existing endpoints, rather than stopping at the per-file list.
- RQ-3: **Keep the embedded page**, do not start the separate Phase-2 PWA now.
  A single `include_str!` page holds `telos/disposable` (one binary, no build
  step); the PWA is revisited when the UX genuinely outgrows one file (offline,
  push, routing, installability), not for this pass.

## Out of Scope
- **Comment/thread writes** over the API (add/reply/resolve from the phone) —
  Slice C; this pass is read-only, `command_bus::WriteChannel` untouched. The
  resource-collection shape anticipates it but adds no write verb.
- **The separate PWA codebase** and any build step (RQ-3).
- **Chat/transcripts** over the API and a Chat tab — a later slice (needs
  `Serialize` on the `transcripts` types); the `GET /chat` collection is named
  here only to show the grammar extends, not built.
- **Re-localizing every file in the index** — the index carries counts only; the
  anchored/drifted/unresolvable state is computed on demand when a file is opened
  (`GET /comments?file=`), not for the whole tree at once.
- **Claim-cite graph navigation** beyond one hop — drill-in resolves a claim's
  direct `cites` to their summaries; it does not walk the citation graph.
