# Feature: Responsive desktop layout for the serve web page

## Summary
Give the `cospan serve` embedded page a desktop layout that uses horizontal
space: at ≥900px a persistent left nav rail replaces the bottom tab bar, and the
list-heavy tabs (Browse, Comments, Chat) become master-detail — a list pane
beside a detail pane, so drilling in opens the detail beside the list instead of
replacing the whole view. Below 900px the current mobile layout is unchanged.
This is the foundational slice of the web-view UX round: it leaves the pane and
grid seams the later slices (teloi grid, browse timeline, file tree, chat
rendering) render into, and it stays one embedded dependency-free document.

## Requirements
- REQ-1: **Desktop shell at a 900px breakpoint, mobile untouched.** A single
  `@media (min-width: 900px)` block in `src/web/index.html`'s `<style>` turns the
  layout into a left nav rail + content area; below 900px the page renders exactly
  as today (the bottom `nav` bar, the single `main` column, `--nav-h` spacing).
  The switch is pure CSS — no JS viewport listener, no `resize` handler — so
  crossing the breakpoint (including a mid-drill-in resize) never re-runs render
  logic.
- REQ-2: **The left nav rail is the same `<nav>` element, restyled.** At desktop
  the existing bottom `<nav>` (the five `data-view` buttons in
  `src/web/index.html`, driven by `setView`) is repositioned to a fixed
  ~180px-wide left rail with icon + label per item, the live-status `.dot`, and
  the existing `.badge`s; it is not a second nav element and `setView`'s
  active-class logic is unchanged.
- REQ-3: **Master-detail for Comments and Chat.** `#view-comments` and
  `#view-chat` are each restructured into two persistent sibling containers — a
  **list pane** and a **detail pane** — both always in the DOM. The list
  renderers (`renderCommentsHome`, `loadChatIndex`) fill the list pane; the
  drill-ins (`openFileViewer` / `openThread`, `openChatSession`) fill the
  **detail pane** rather than replacing the whole view. At desktop the two panes
  sit side by side; below 900px they stack and only one shows at a time. Browse
  is deferred (see REQ-5 and Out of Scope): it uses inline claim-expansion today,
  not a view-replacing drill-in, so its master-detail lands with the
  browse-content slice.
- REQ-4: **Mobile keeps single-pane drill-in via a class, not innerHTML replace.**
  A `.detail-open` class on the master-detail view (toggled in the drill-in / back
  handlers) drives which pane is visible below 900px: no detail open → the list
  pane shows; detail open → the detail pane shows with its back control, the list
  hidden. This reproduces today's replace-in-place feel without destroying the
  list, so returning is a class toggle, not a reload. At desktop the class has no
  visual effect (both panes always show).
- REQ-5: **Now, Teloi, and Browse stay single-pane inside the shell.** `#view-now`
  (the dashboard: hero, `#flow`, `#atoms`, `#daystatus`), `#view-telos` (today's
  flat teloi list), and `#view-browse` (the subject list with its inline
  claim-expansion) render in the content area at a capped readable width; they get
  no detail pane in this slice. Their existing renderers (`renderNow`,
  `renderTeloi`, `renderSubjects`, and the inline `subjectEl`/`claimEl` expansion)
  are unchanged. Browse's master-detail is folded into the later browse-content
  slice, which reworks its list anyway.
- REQ-6: **Ultra-wide readability cap.** On very wide screens the panes may grow
  but the text-bearing columns are capped so content stays readable — the claim
  detail, the file viewer's `.codeview`, and the chat transcript do not stretch to
  full width. `main`'s current `max-width: 860px` is replaced by a
  desktop-content cap that applies per-pane, not to the whole shell.
- REQ-7: **No regressions and no new dependency.** The `/stream` live updates
  (`render` → `renderNow`/`renderTeloi`/`renderSubjects`), the token-in-URL
  (`withTok`), theming (the `prefers-color-scheme` block is extended for both
  widths), and every existing drill-in flow keep working; the page stays one
  `include_str!` document with no external JS/CSS/CDN (`telos/disposable`), and
  the mobile render is visually unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A `server::tests` unit test asserts `INDEX_HTML`
  contains a `@media (min-width: 900px)` block and a desktop nav-rail selector,
  and that the page still has exactly one `<nav ` element (the rail is the
  restyled bottom bar, not a second nav).
- [ ] AC-2: (covers REQ-3, REQ-4) A unit test asserts `INDEX_HTML` contains the
  list-pane and detail-pane container markup/classes for the two master-detail
  views (Comments, Chat) and a `detail-open` toggle, and that the drill-in
  functions (`openFileViewer`, `openThread`, `openChatSession`) target the detail
  pane — i.e. the page no longer routes those through a whole-view `innerHTML`
  replace.
- [ ] AC-3: (covers REQ-6) A unit test asserts `INDEX_HTML` caps a text/detail
  column width at desktop (a `max-width` inside the ≥900px block applied to the
  detail/content column, not only to `main`).
- [ ] AC-4: (covers REQ-7) A unit test asserts the existing wiring is intact —
  `INDEX_HTML` still contains `setView`, `withTok`, `/stream`, `openFileViewer`,
  `startAddAt`, and the mobile bottom-bar markers (`--nav-h`, `data-view=`) — so
  the responsive refactor did not drop the mobile layout or the live/stream/token
  behavior.
- [ ] AC-5: (covers REQ-5) A unit test asserts `#view-now`, `#view-telos`, and
  `#view-browse` remain present and single-pane (no detail-pane container inside
  them), so the dashboard, teloi list, and subject browser are untouched by the
  master-detail refactor.
- [ ] AC-6: (covers REQ-1, REQ-7) `cargo test`, `cargo clippy --all-targets --
  -D warnings`, and `cargo fmt --check` are green; the server serves `/` at both
  a narrow and wide layout with no runtime change (the layout is CSS-only).

## Architecture
**All changes are in `src/web/index.html`** (the `include_str!` page in
`src/server.rs`); no Rust behavior changes, so `server.rs`/`mcp.rs` are untouched
except the `INDEX_HTML` wiring tests in `server::tests` (mirroring
`index_html_wires_the_file_viewer` and friends).

**Shell (CSS).** Today `main` is a centered 860px column and `nav` is a fixed
bottom bar (`--nav-h: 62px`), with only a `@media (prefers-color-scheme: light)`
block. Add one `@media (min-width: 900px)` block that: lays the page out as a
left rail + content region (CSS grid or flex on `body`/a wrapper); repositions the
existing `<nav>` from bottom-fixed to a fixed ~180px left rail and restyles its
buttons from stacked icon/label to a horizontal icon + label row (the same
buttons, `setView` unchanged); and drops `main`'s bottom-bar padding. The mobile
rules stay as the default (unmediated) cascade, so nothing below 900px changes.

**Panes.** Restructure `#view-comments` and `#view-chat` so each holds a
`.pane-list` and a `.pane-detail` sibling. The list renderers
(`renderCommentsHome`; `loadChatIndex`) write into `.pane-list`; the drill-ins
write into `.pane-detail`:
- Comments: `openFileViewer` and `openThread` currently overwrite `#comments`;
  they instead render into the comments detail pane, and `renderCommentsHome` owns
  the list pane. `startAddAt`/`backBar` keep working against the detail pane.
- Chat: `loadChatIndex` fills the list pane, `openChatSession` the detail pane.
A small helper toggles the view's `.detail-open` class when a detail opens or a
back control fires; below 900px CSS uses that class to show exactly one pane, at
≥900px both panes are always shown and the class is inert. Because both panes
persist in the DOM, a viewport resize needs no re-render.

**Browse stays single-pane.** `#view-browse` keeps today's model — the filter +
`#subjects` tree with inline `subjectEl`/`claimEl` expansion — rendered in the
content area at a capped width. Making it master-detail would change its mobile
inline-expand behavior and duplicates work the browse-content slice must do to its
list, so it is deferred there.

**Readability.** Replace the single `main { max-width: 860px }` with a
desktop-content cap applied to the detail/text columns (claim detail, `.codeview`,
transcript) so ultra-wide screens grow the panes but not the prose. Mobile keeps
the full-width single column.

**Live updates & token.** `render()` and its `renderNow`/`renderTeloi`/
`renderSubjects` fan-out are unchanged — they write into the always-present
Now/Teloi/Browse views, so a `/stream` push keeps updating them regardless of
which tab is active or whether a Comments/Chat detail is open. The Comments and
Chat lists are fetched on tab-open (not from the fold); their list panes persist
in the DOM, so opening a detail never discards the list. `withTok` and the
`/stream` reconnect logic are untouched.

## Resolved Questions
- RQ-1: **Master-detail two-pane**, not a lighter widen-and-grid — the list-heavy
  tabs get a persistent list + detail side by side at desktop, which is the real
  use of the space and the clean seam the later UX slices render into. The cost is
  reworking the drill-ins to fill a detail pane instead of replacing the view;
  that rework is the point of doing the layout slice first. Applied to Comments and
  Chat here (which already replace the view on drill-in); Browse is deferred to the
  browse-content slice because it uses inline expansion, not view replacement.
- RQ-2: **A ~180px icon + label left rail**, reusing the existing `<nav>` element
  restyled at the breakpoint (not a second nav, not an icon-only rail) — labeled
  is more discoverable and the single-nav reuse keeps `setView` and the active
  state exactly as they are.
- RQ-3: **Breakpoint at 900px, with the detail/text columns capped on ultra-wide**
  so a claim or a line of code never stretches to an unreadable width while the
  panes still use the extra room.
- RQ-4: **CSS-first with a `.detail-open` class**, not a JS viewport listener —
  the panes always exist in the DOM and media queries arrange them; the only JS is
  the class toggle that gives mobile its one-pane-at-a-time behavior. This keeps
  the mobile bytes/behavior identical and makes a resize free (no re-render).

## Out of Scope
- **The teloi grid + drill-down** (a later slice) — Teloi stays today's flat list
  in the content area here; this slice only gives it a place to live.
- **Browse master-detail and the browse timeline / per-operation formatting /
  current-state summaries** (the later browse-content slice) — Browse keeps today's
  single-pane inline claim-expansion in the wider content area here; its list +
  detail panes land with that slice, which reworks its list content anyway.
- **The folded file tree and `.claims/` de-noise** (a later slice) — Comments gets
  the list/detail panes; the file list stays the current flat filtered list.
- **Chat markdown/code rendering** (a later slice) — Chat gets the panes, not the
  transcript rendering changes.
- **A collapsible / resizable splitter between panes** — fixed proportions in this
  slice; a draggable divider is polish for later.
- **Any Rust/endpoint change** — this is a page-only slice; the API is untouched.
