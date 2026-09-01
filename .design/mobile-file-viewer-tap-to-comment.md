# Feature: Mobile file browser + viewer with tap-to-comment

## Summary
Close the on-device comment-creation dead-end by giving the phone a way to reach
*any* file, not only files that already have a comment. Add a read-only file
browser and a syntax-highlighted file viewer over `cospan serve`, and let a
tap on a line open the add-comment box pre-anchored to that line. This makes the
doc-comment round trip start from the phone: browse a file with highlighting →
tap a line → write the first comment (`telos/comment-roundtrip`), while reads
stay observation and writes stay opt-in sidecar-only (`telos/observe-now-control-later`).

## Requirements
- REQ-1: **`GET /files` — the browsable file list.** A new read-only route
  (always mounted, behind Slice-B auth) returns `{ "files": [ { "path": <rel>,
  "status": <char> } ] }`, sourced from `filetree::list(repo)` (`src/filetree.rs`)
  — the same tracked ∪ untracked-not-ignored set, sorted by path, that the TUI
  browses — with `status` the one-char `filetree::marker` value. It runs under
  `spawn_blocking` like the other read cores and needs no `--allow-writes`.
- REQ-2: **`GET /file?path=<rel>` — one file's highlighted content.** A new
  read-only route (auth, path-guarded by the same `mcp::guard` the comment routes
  use, so `path=../x` returns the guard error) returns
  `{ "path": <rel>, "lines": [ [ { "t": <text>, "c": <#rrggbb> } ] ],
  "truncated": <bool>, "total": <n> }` — one array per source line, each a list
  of styled runs. It reads the file from disk, caps the highlighted prefix at
  `FILE_VIEW_MAX_LINES` (with `truncated:true` and `total` the real line count
  when the file is longer), and returns a `404`-shaped `{error}` for a path that
  is not a readable file.
- REQ-3: **Server-side highlighting, page stays dependency-free.** `GET /file`
  colors runs by reusing the existing `syntect` pipeline in `src/highlight.rs`
  (the loaded `Hl` syntax set + theme) through a new hex projection
  (`highlight::styled_web` returning `Vec<Vec<(String /*#rrggbb*/, String)>>`),
  built by feeding the same per-line highlight and converting each `syntect`
  style's foreground to `#rrggbb` — so no syntax-highlighting library is added to
  the embedded page and `telos/disposable` (the page is one `include_str!`
  document) is preserved.
- REQ-4: **Files browser folded into the Comments tab (client).** Rather than a
  sixth nav tab, the existing Comments tab (`src/web/index.html`) becomes the one
  file surface, with a segmented toggle — `all files` (from `GET /files`) and
  `commented` (today's `GET /comments` index) — defaulting to `commented` so the
  existing landing is unchanged. The `all files` mode is shown even when
  read-only (`telos/observe-now-control-later`: reads are observation), renders
  each file as a row with its path (`mono`) and status marker, and has a
  client-side filter input (reusing the existing `.filter` style the Browse tab
  uses) doing substring match over the full list — no server-side paging. A row
  with existing comments is badged as it is today. Tapping a row opens the file
  viewer. The bottom nav stays five tabs.
- REQ-5: **File viewer with existing comments (client).** Opening a file fetches
  `GET /file?path=` and the existing `GET /comments?file=` in parallel, renders
  the highlighted lines with line numbers, and marks lines that already carry a
  comment in a gutter; tapping a marked line's comment opens that comment's
  thread via the existing `openThread(file, id)`. A `truncated` response shows a
  "showing first N lines" notice.
- REQ-6: **Tap-a-line to create the first comment.** When `/capabilities`
  reports `writes` (`--allow-writes`), tapping a source line in the viewer opens
  the existing add-comment box pre-filled with that line number, and submitting
  `POST`s to the existing `POST /comments?file=<rel>` `{ line, body }` route
  (`mcp::add_comment_as`, already guarded and attributed `who:"human"`). Because
  the viewer reaches a file directly from `GET /files` rather than from the
  comment index, a file with **zero** comments can now receive its first one —
  the recorded dead-end is closed. No new write endpoint is added.
- REQ-7: **No new server state; writes unchanged.** The two new routes are pure
  reads over the working tree; `serve` gains no on-disk state and no daemon
  (`telos/disposable`). No kan claim is written and `command_bus::WriteChannel`
  is untouched; comment creation still goes only to the `.cospan/comments/`
  sidecar via the Slice-C write path, still opt-in behind `--allow-writes`
  (`telos/kan-is-truth`, `telos/observe-now-control-later`).

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A CI integration test asserts `GET /files` behind the
  token returns a `files` array containing a known repo file with a `status`
  field, and that `GET /files` with no token returns `401`.
- [ ] AC-2: (covers REQ-2) A CI integration test asserts `GET /file?path=<f>`
  for a known file returns `lines` (non-empty, each an array of `{t,c}` runs);
  `GET /file?path=../../etc/passwd` returns the guard error and no file outside
  the repo; and a file longer than `FILE_VIEW_MAX_LINES` returns
  `truncated:true` with `total` greater than the returned line count.
- [ ] AC-3: (covers REQ-3) A unit test on `highlight::styled_web` asserts a Rust
  snippet yields runs whose `c` values are `#rrggbb` strings and that a keyword
  run differs in color from an identifier/plain run (highlighting is real, not a
  single flat color), reusing the same theme as `highlight::styled`.
- [ ] AC-4: (covers REQ-6, REQ-7) A CI integration test with `--allow-writes`
  **on**: for a file that has **no** existing comment (absent from
  `GET /comments`), `POST /comments?file=<f>` `{line:N,body}` succeeds; a
  subsequent `GET /comments?file=<f>` shows the comment authored
  `who:"human"`; and `GET /comments` (the index) now lists that file — the
  first-comment path works end to end.
- [ ] AC-5: (covers REQ-4, REQ-5, REQ-6) A unit test asserts `INDEX_HTML` wires
  `/files` and `/file`, renders a file-browser surface with a filter input, and
  contains a tap-a-line add path that is gated on the `writes` capability (the
  add affordance is absent from the read-only render path).

## Architecture
**Cores.** `GET /files` is `filetree::list(repo)` (`src/filetree.rs`) mapped to
JSON `{path, status}` using `filetree::marker`; no new walk logic. `GET /file`
adds a small core (in `src/mcp.rs`, beside the comment cores so it shares
`guard`): guard the path, read the file to a `String`, and call the new
`highlight::styled_web(content, ext)` capped at `FILE_VIEW_MAX_LINES`, returning
the `{path, lines, truncated, total}` value. The extension is the path's
extension, matching how the TUI derives `ext` for `highlight::styled_upto`
(`src/tui.rs:1899`).

**Highlighting (`src/highlight.rs`).** Add `styled_web(content, ext) ->
Vec<Vec<(String, String)>>` reusing the cached `Hl` (syntax set + theme, `fn hl`)
and the same per-line `HighlightLines` loop as `compute`, but converting each
`syntect` style's `foreground` straight to `#rrggbb` instead of through the
ratatui `Style`/`Color` path in `conv`. The existing `styled` / `styled_upto` /
`plain` (ratatui-facing, used by the TUI) are unchanged; the web projection is
additive and shares the expensive syntax/theme load.

**Server (`src/server.rs`).** `app` mounts `.route("/files", get(get_files))`
and `.route("/file", get(get_file))` alongside the existing read routes; both sit
behind the same auth middleware and loopback bind. `get_files` calls the
`filetree` projection under `spawn_blocking`; `get_file` extracts `Query{path}`,
runs the `mcp` file core under `spawn_blocking`, and returns its JSON (guard
error and not-a-file both surface as the core's `{error}` value). Neither route
touches `Shared.allow_writes` — they are reads. The write path for the first
comment is the **existing** `POST /comments` (Slice C), unchanged.

**Page (`src/web/index.html`).** The Comments tab gains an `all files | commented`
toggle (default `commented`, today's `GET /comments` index). The `all files` mode
loads `GET /files`, keeps the returned list, and renders rows filtered by a
client-side substring box (reusing `.filter`), badging rows that already have
comments. Selecting a file opens a viewer that fetches `GET /file` and
`GET /comments?file=` together, renders highlighted lines (a run per `{t,c}`,
colored inline; text set via `textContent`, never `innerHTML`) with line numbers
and a comment gutter; tapping an existing comment calls `openThread`. When
`caps.writes` is set, tapping a source line opens the existing add box with that
line pre-filled and `POST`s to `/comments`; when it is not, the viewer is
read-only. This reuses `openThread`, `writeBox`, `postJson`, and the `caps`
gating already in the page.

The `observe-now-control-later ↔ disposable` posture is unchanged from Slice C:
the new surface is all reads (no new state, no daemon), and the one write it
enables is the already-opt-in, sidecar-only, `who:"human"` comment add — reaching
a file it previously could not, not a new kind of write.

## Resolved Questions
- RQ-1: **Show file content and let the user tap a line**, rather than a
  file-level comment defaulting to line 1 — the first comment anchors to the line
  the user chooses, which is what `telos/comment-roundtrip` ("browses any file
  with syntax highlighting and adds … comments") describes. This is the larger of
  the two scopes considered, taken deliberately.
- RQ-2: **The file browser is always visible (a read affordance)**, not gated on
  `--allow-writes`; only the add-a-comment action is gated. Browsing files is
  observation, so it belongs to the read tier.
- RQ-3: **Highlight server-side and ship a dependency-free page.** Reuse the
  existing `syntect` highlighter via a hex projection rather than adding a
  client-side JS highlighter, keeping the page one embedded `include_str!`
  document (`telos/disposable`).
- RQ-4: **Client-side filter over the whole `GET /files` list**, not a
  server-side `?q=` search — the browsable list is cheap to send whole, and a
  substring box avoids per-keystroke round-trips.
- RQ-5: **Fold the browser into the Comments tab** (an `all files | commented`
  toggle) rather than adding a sixth nav tab or burying it behind a write-mode
  entry. The phone keeps five tabs and gains one file surface where the two lists
  (all files, files-with-comments) live together; the `commented` default keeps
  today's landing unchanged.

## Out of Scope
- **Editing or deleting files** over the API — the viewer is strictly read-only;
  the only write is adding a comment via the existing Slice-C path.
- **Diff / blame views** (`src/diff.rs`) over the API — the viewer shows current
  content with git *status* only, not a diff. A later slice.
- **kan claim writes / promote-to-kan** — unchanged: promoting a comment into the
  kan log stays a human TUI action on the machine holding the seed.
- **Comment editing / deleting** over the API — this slice adds comment *creation*
  via tap-a-line; reply and resolve stay as Slice C shipped them.
- **Pagination / streaming of very large files** beyond the
  `FILE_VIEW_MAX_LINES` truncation cap — a huge file shows a truncated head with
  a notice, not an infinite-scroll viewer.
- **Live re-localization animation** in the viewer — comment gutter markers
  reflect the current fold on load; watching a comment drift as the file changes
  is the TUI comment-gutter's job, not this slice.
