# Feature: Teloi grid + drill-down

## Summary
Turn the serve page's Teloi tab from a flat list into a master-detail view: a
list of telos rows in the list pane, and — on tapping one — that telos's full
detail in the detail pane (its statement, each witness with the human probe
description from the `schema/witness` map, the tensions that name it, and its
recorded claims). Reuses the responsive-layout slice's pane machinery and the
Browse tab's claim renderer; all data is already in the fold, so no endpoint
changes.

## Requirements
- REQ-1: **Teloi becomes master-detail.** `#view-telos` in `src/web/index.html`
  is restructured from its flat `#teloi`/`#tensions` divs into a `class="view md"`
  with `.pane-list` (`#telos-list`) and `.pane-detail` (`#telos-detail`), reusing
  the slice-1 pane machinery (`paneList`/`paneDetail`/`setDetailOpen`/`paneReset`,
  the `.detail-open` mobile toggle). At desktop the grid sits beside the detail;
  below 900px one pane shows at a time with a back control.
- REQ-2: **The list pane is tappable telos rows + a tensions overview.**
  `renderTeloi` fills `#telos-list` with the `fold.process.teloi` cards laid out
  as a vertical list of full-width rows (a `.telos-rows` flex column),
  each card (title + statement snippet + witness count) calling `openTelos(slug)`
  on click and marking the selected telos; and it keeps a compact "Tensions held"
  overview (`fold.process.tensions`, each `between[0] ⇄ between[1]` + `why`) below
  the rows — the at-a-glance map is retained, not moved.
- REQ-3: **A telos drills into full detail.** `openTelos(slug)` fills
  `#telos-detail` with: a back control (`backBar`); the telos title + full
  `statement`; each of its `witnesses` shown **with** its probe description looked
  up in `fold.process.witnesses` (the `schema/witness` type→description map); the
  tensions naming it (`fold.process.tensions` filtered to those whose `between`
  includes the slug); and the telos's recorded claims from
  `fold.claims["telos/" + slug]` rendered with the **existing** Browse `claimEl`
  (kind badge, summary, expandable detail) — no second claim renderer. Opening a
  telos sets `.detail-open`; the back control clears it.
- REQ-4: **Live-safe against `/stream`.** `renderTeloi` runs from `render()` on
  every fold tick; it rebuilds only `#telos-list` and never clears `#telos-detail`,
  so a fold update while a telos detail is open does not close it (the detail is a
  snapshot, refreshed when the telos is re-opened). When no telos is selected the
  detail pane shows a placeholder (`.pane-hint`, "Select a telos").
- REQ-5: **No data/endpoint change; stays disposable.** Everything renders from
  the already-served fold (`process.teloi`, `process.tensions`, `process.witnesses`,
  `claims`); no server or route change. The page stays one `include_str!` document
  with no new external JS/CSS/CDN (`telos/disposable`), theme-aware, and the
  mobile layout for other tabs is untouched. Reusing `claimEl` and the pane
  helpers means no duplicated renderer.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A `server::tests` unit test asserts
  `INDEX_HTML` marks the Teloi view master-detail and wires the grid: it contains
  `id="telos-list"`, `id="telos-detail"`, a `telos-rows`
  class, and `openTelos(`; and the count of `class="view md"` is now **3**
  (Comments, Chat, Teloi) — the existing `index_html_wires_the_responsive_layout`
  test's count is updated to match, with a note that Teloi joined the
  master-detail views.
- [ ] AC-2: (covers REQ-3) A unit test asserts the telos detail is wired:
  `INDEX_HTML` contains `openTelos`, a lookup into `process.witnesses` (the
  description map) within the telos detail, a tensions filter for the slug
  (`between`/`.includes`), a read of `fold.claims["telos/"` for the telos's
  claims, and a call to the shared `claimEl(` inside `openTelos` — proving the
  detail reuses the Browse claim renderer rather than a new one.
- [ ] AC-3: (covers REQ-2) A unit test asserts the standalone tensions overview
  is retained on the list page — `INDEX_HTML` still renders a tensions section in
  the Teloi view (a `tensions`/`Tensions held` marker inside `#telos-list`'s
  rendering path), so the global map was kept, not dropped.
- [ ] AC-4: (covers REQ-4, REQ-5) A unit test asserts `renderTeloi` targets the
  list pane (`paneList("telos")` / `#telos-list`) and that `openTelos` targets the
  detail pane (`paneDetail("telos")`), so live fold re-renders touch only the list;
  and `INDEX_HTML` contains no new `<script src`/`<link href`/`cdn` (grep-style
  assertion) — the page gained no dependency.
- [ ] AC-5: (covers REQ-1..REQ-5) `cargo test`, `cargo clippy --all-targets --
  -D warnings`, and `cargo fmt --check` are green; the layout/render itself is
  page-only and is confirmed by an operator eyeball (stated, not machine-checked).

## Architecture
**Page-only** (`src/web/index.html`) plus one test update in `src/server.rs`; no
Rust behavior or endpoint change. The fold already carries everything
(`substrate::ProcessSnapshot`: `teloi: Vec<TelosView{slug,title,statement,
witnesses}>`, `tensions: Vec<Tension{between,why}>`, `witnesses: BTreeMap<String,
String>`; and `claims["telos/<slug>"]`).

**Markup.** `#view-telos` changes from `<h2>Teloi</h2><div id="teloi">…<div
id="tensions">` to `class="view md"` holding `<div class="pane-list"
id="telos-list">` + `<div class="pane-detail" id="telos-detail">`, mirroring
`#view-comments`/`#view-chat`.

**List pane.** `renderTeloi` (already called from `render()` on every fold) now
writes into `#telos-list`: a `.telos-rows` container (a `display:flex;
flex-direction:column` stack) of full-width `.card.telos` rows, each with a click
handler `openTelos(v.slug)` and an `.active` marker for the open one; followed by
the retained "Tensions held" overview. It does **not** touch `#telos-detail`, so a
`/stream` re-render never closes an open detail.

**Detail pane.** New `openTelos(slug)`: `setDetailOpen("telos", true)`, then into
`#telos-detail` — `backBar(title, () => paneReset("telos", …))`, the statement, a
witnesses block mapping each `v.witnesses[i]` to `fold.process.witnesses[type]`
(falling back to the bare type name when absent — `telos/honest-ambiguity`, don't
invent a description), a tensions block filtering `fold.process.tensions` by
`between.includes(slug)`, and a claims block iterating
`fold.claims["telos/"+slug]` through the existing `claimEl`. `setView("telos")`
leaves the list rendered by `render()` and the detail showing its placeholder
until a card is tapped.

**Reuse, not duplication.** `claimEl` (the Browse claim component) and the
`paneList`/`paneDetail`/`setDetailOpen`/`paneReset` helpers are used as-is; the
only new function is `openTelos`. The `index_html_wires_the_responsive_layout`
test's `class="view md"` count moves 2→3 as Teloi joins the master-detail views.

## Resolved Questions
- RQ-1: **Master-detail**, not a full-width grid-that-replaces — Teloi reuses the
  `.view.md` list/detail panes like Comments and Chat, for one consistent
  drill-in model across the page. The telos cards live in the (narrower) list pane
  as a single-column list of full-width rows (the operator chose rows over a
  card grid on eyeball); the operator chose master-detail over a wide grid.
- RQ-2: **The telos detail includes its recorded claims**, rendered with the
  shared Browse `claimEl`, rather than only linking out to Browse — a telos opens
  to everything about it (`telos/readable-claim-browser`), and reuse avoids a
  second claim renderer.
- RQ-3: **Keep the standalone "Tensions held" overview** on the list page in
  addition to per-telos tensions in the detail — the global map is worth an
  at-a-glance view, and a tension naturally appears under both teloi it names.

## Out of Scope
- **Any endpoint or fold-shape change** — this slice is pure page rendering over
  the fold the server already sends.
- **Editing teloi / witnesses / tensions from the page** — read-only, like the
  rest of the browser; declaring a telos stays a `kan`/`day` action.
- **Atom and bridge drill-down** — this slice is teloi only; the process/atom
  views (the Now tab) are untouched.
- **A dedicated tensions drill-in** (tapping a tension to see its full claim) —
  the overview stays display-only; tensions are read in a telos's detail.
- **Deep-linking a telos via URL** (e.g. `#telos/<slug>`) — selection is
  in-page state, not a routable URL, in this slice.
