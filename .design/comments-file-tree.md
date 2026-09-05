# Feature: Comments file browser — folded directory tree + review follow-ups

## Summary
Turn the Comments tab's "All files" list from a flat ~157-row list into a
collapsible directory tree that starts folded, excludes kan's `.claims/`
published tree, and badges (rather than dead-lists) symlinks. Also folds in the
file-viewer review follow-ups: F2 (symlinks were dead rows) and F3 (a line that
already has a comment could not receive a second). All page-side except a small
additive `symlink` field on `GET /files`.

## Requirements
- REQ-1: **Folded directory tree.** `renderFilesBrowser` (`src/web/index.html`)
  builds a nested directory tree from the flat `GET /files` paths instead of the
  flat `.ffile` list. Directories render as collapsible rows starting **folded**
  (only top-level entries visible); a session `Set` of expanded directory paths
  tracks open ones; tapping a directory toggles it; tapping a file opens
  `openFileViewer`. A file that has comments keeps its `✎` mark (from
  `commentsIndex`) and its git-status marker.
- REQ-2: **Filter still finds files.** When the `filter` input is non-empty, the
  browser shows a flat list of files whose path matches (substring), so a search
  works without manually expanding directories; an empty filter shows the folded
  tree.
- REQ-3: **Exclude `.claims/`.** The browser omits the `.claims/` tree (and
  `.cospan/` if present) — kan's own published state, not source to comment on —
  from both the tree and the filtered list. `GET /files` is unchanged (still the
  full browsable set); the exclusion is a client-side filter in the browser, so
  the endpoint stays honest.
- REQ-4: **F2 — symlinks badged, not dead rows.** `get_files` (`src/server.rs`)
  adds an additive `symlink` boolean per entry, from
  `symlink_metadata().is_symlink()` on `repo.join(path)`; the shared
  `filetree::list` (used by the TUI) is untouched. The tree badges a symlink entry
  `[link]` and, on tap, shows a "symlinks aren't viewable" note instead of calling
  `openFileViewer` (which would return the guard error).
- REQ-5: **F3 — any line can take a new comment.** In `openFileViewer`, the line
  number (`.clnum`) becomes a distinct add target under `caps.writes`: tapping it
  calls `startAddAt(file, lineNo)` (stopping propagation), so a line that already
  has a comment can still receive a **new** one; tapping the gutter/code of a
  commented line still opens its thread (`openThread`), and an uncommented line
  still adds.
- REQ-6: **Disposable / reuse.** The tree is built client-side in the one
  `include_str!` document with no new external JS/CSS/CDN (`telos/disposable`);
  it reuses `openFileViewer` / `startAddAt` / `openThread` / `commentsIndex`, and
  the file viewer's syntax highlighting is unchanged. The only server change is the
  additive `symlink` field; no route or fold-shape change otherwise.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A `server::tests` unit test asserts
  `INDEX_HTML` builds a directory tree in the files browser: it contains a tree
  builder / directory-row class (a `ftree`/`fdir` marker), an expanded-directory
  `Set`, a directory toggle, and file rows that still call `openFileViewer(`; and
  a filtered-flat-list path keyed on the `filter`.
- [ ] AC-2: (covers REQ-3) A unit test asserts the browser excludes `.claims/`
  — `INDEX_HTML` contains a `.claims` (and `.cospan`) skip in the file-browser
  path.
- [ ] AC-3: (covers REQ-4) A CI integration test (`tests/serve_auth.rs`) asserts
  `GET /files` entries carry a `symlink` boolean — `false` for a normal committed
  file and `true` for a committed symlink (unix) — and a unit test asserts
  `INDEX_HTML` badges a symlink (`[link]`) and guards the tap (no `openFileViewer`
  on a symlink).
- [ ] AC-4: (covers REQ-5) A unit test asserts `INDEX_HTML` wires the line-number
  add target — the `.clnum` element has its own click that calls `startAddAt` and
  `stopPropagation`, distinct from the row's thread-open tap.
- [ ] AC-5: (covers REQ-1..REQ-6) `cargo test`, `cargo clippy --all-targets --
  -D warnings`, and `cargo fmt --check` are green; `INDEX_HTML` contains no new
  `<script src`/`<link href` (no dependency added); the render is confirmed by an
  operator eyeball (stated, not machine-checked).

## Architecture
**Mostly page-side** (`src/web/index.html`) plus a small additive server field
(`src/server.rs`) and its tests.

**Server (`get_files`).** The projection currently maps `filetree::list(repo)` to
`{path, status}`. Add `symlink`: for each entry, `repo.join(&e.path)`'s
`symlink_metadata().map(|m| m.is_symlink()).unwrap_or(false)`. `filetree::list`
(shared with the TUI) is unchanged; this is one extra field per row, computed in
the handler under `spawn_blocking`.

**Client tree (`renderFilesBrowser`).** Replace the flat `.ffile` loop with:
(1) filter the `filesList` to drop paths under `.claims/` / `.cospan/`; (2) if
`filesFilter` is set, render the matching files as a flat list (today's row shape,
reused); (3) else build a tree — split each path on `/` into a nested
`{dirs: Map, files: []}`, then render recursively: a `.fdir` row per directory
(a chevron + name + child count) that toggles membership in a session
`expandedDirs` Set and re-renders, and a `.ffile` row per file. Directory rows
sort before files, alphabetically. A symlink file (`f.symlink`) shows a `[link]`
badge and its tap shows a note instead of `openFileViewer`. The existing
commented-`✎` mark (from `commentsIndex`) and git-status marker are preserved on
file rows.

**Viewer line-number add (`openFileViewer`).** The `.clnum` span gains a click
handler (under `caps.writes`) that calls `e.stopPropagation()` then
`startAddAt(file, lineNo)`. The row's existing handler is unchanged: a commented
line opens its thread, an uncommented line adds — so the number is the
always-available "comment on exactly this line" target that closes F3.

**Reuse.** `openFileViewer`, `openThread`, `startAddAt`, `commentsIndex`, and the
pane machinery are reused; no new claim/file renderer. No fold-shape or route
change; `GET /files` gains one field.

## Resolved Questions
- RQ-1: **Exclude `.claims/` (and `.cospan/`) from the browser**, rather than
  folding them in as ordinary directories — they are kan's own published state and
  cospan's owned sidecar tree, not source files to browse or comment on. `GET
  /files` still returns them (honest endpoint); the browser filters them out.
- RQ-2: **Keep symlinks and badge them `[link]`**, rather than skipping them
  server-side — the entry stays visible but is clearly marked and, on tap, says
  "symlinks aren't viewable" instead of surfacing the raw guard error. This needs
  an additive `symlink` field on `GET /files`.
- RQ-3: **The line number is the second-comment affordance** — tapping `.clnum`
  always starts an add for that line (F3), while tapping the gutter/code of a
  commented line still opens its thread. One always-available target, no chooser
  or per-line button.

## Out of Scope
- **A server-side file tree / `?dir=` endpoint** — the tree is built client-side
  from the flat `GET /files` list; no new endpoint.
- **Following or previewing symlink targets** — symlinks are badged and declared
  unviewable, not resolved.
- **Single-child directory chain collapsing** (`a/b/c` shown as one row) and
  drag-to-resize — the tree is plain folders for this slice.
- **Editing / deleting comments** — F3 adds a *new* comment on a line; reply and
  resolve are unchanged from earlier slices.
- **Excluding other dot-directories or a configurable ignore list** — only
  `.claims/` and `.cospan/` are filtered; a general ignore config is not in scope.
