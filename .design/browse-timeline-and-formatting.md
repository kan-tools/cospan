# Feature: Browse rework — master-detail, timeline, per-op formatting, state summaries

## Summary
Rework the serve page's Browse tab into a master-detail view with two list modes
— a subject list (each row carrying a current-state summary) and a chronological
timeline of all claims — plus striking per-operation formatting so each claim
kind reads at a glance. Tapping a subject or a timeline claim opens its detail in
the detail pane. Also completes the Browse master-detail conversion deferred from
the responsive-layout slice. All data is already in the fold; no endpoint change.

## Requirements
- REQ-1: **Browse becomes master-detail.** `#view-browse` in `src/web/index.html`
  becomes a `class="view md"` with `.pane-list` (`#browse-list`) and `.pane-detail`
  (`#browse-detail`), reusing the slice-1 pane machinery
  (`paneList`/`paneDetail`/`setDetailOpen`/`paneReset`, the `.detail-open` mobile
  toggle). The list pane holds a **persistent** shell — a `[Subjects | Timeline]`
  segmented toggle, the existing `#filter` input, and a `#browse-content` div — so
  the toggle and filter are not rebuilt on every fold (typing keeps focus).
- REQ-2: **Two list modes, live-safe.** A `renderBrowse` (called from `render()`
  on every fold tick, replacing the current `renderSubjects` call) fills only
  `#browse-content` per a `browseMode` of `"subjects"` or `"timeline"`, and never
  clears `#browse-detail` — an open detail survives a `/stream` re-render, matching
  `renderTeloi`. It seeds the detail placeholder once (`.pane-hint`) when nothing
  is selected, and keeps the nav `#subjBadge` count. The toggle buttons flip
  `browseMode` (updating their active state) and the `#filter` listener updates the
  filter — both re-render only `#browse-content`.
- REQ-3: **Timeline mode.** `"timeline"` flattens every claim across
  `fold.claims` into one list, sorts by `recorded_at` descending, applies the
  `#filter` substring (over subject + kind + summary), caps at
  `BROWSE_TIMELINE_CAP` (200) with a "showing N of M" note, and renders each as a
  striking per-op row (kind glyph + kind badge + subject leaf + time + one-line
  `summarize`), tapping which opens that claim in the detail pane
  (`openBrowseClaim`).
- REQ-4: **Current-state summary per subject.** For each subject, computed from
  `fold.claims[subject]`: the most-recent claim's `kind` (by max `recorded_at`),
  and badges — **published** (any claim `kind === "Publication"`), **retracted**
  (any `kind === "Retraction"`), and a neutral **status** marker (any
  `kind === "Status"`). Shown on each subject row in `"subjects"` mode and in the
  detail-pane header. The Status *value* (resolved/blocked/open) is **not** in the
  fold and is out of scope for this slice.
- REQ-5: **Striking per-operation formatting.** A `KIND_GLYPH` map giving each of
  the nine kinds (`Subject`, `Decision`, `Publication`, `Plan`, `Observation`,
  `Result`, `Status`, `Relation`, `Retraction`) a distinct glyph, plus per-kind
  color/badge CSS extending the existing `.kind.<Kind>` rules, used both in the
  timeline rows and in `claimEl`'s kind badge, so an operation is identifiable at
  a glance without reading the text.
- REQ-6: **Detail pane.** `openBrowseSubject(subject)` fills `#browse-detail` with
  a state header (subject name + its summary badges) and the subject's claims
  rendered by the existing `claimEl` (inline-expandable — full text and cites via
  `fold.by_cid`), and sets `.detail-open`. `openBrowseClaim(claim)` fills it with a
  header naming the claim's subject (tapping it calls `openBrowseSubject`) and the
  claim via `claimEl`. A `backBar` returns to the list (clearing `.detail-open`).
- REQ-7: **No endpoint/data change; stays disposable.** Everything renders from
  the already-served fold (`subjects`, `claims`, `by_cid`); no server or route
  change. Reuses `claimEl`, `summarize`, `fmtWhen`, `shortAuthor`, and the pane
  helpers (no duplicated renderer). The page stays one `include_str!` document with
  no new external JS/CSS/CDN (`telos/disposable`), theme-aware, and the mobile
  layout is preserved (one pane at a time under 900px).

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A `server::tests` unit test asserts `INDEX_HTML`
  makes Browse master-detail and wires the shell: it contains `id="browse-list"`,
  `id="browse-detail"`, `id="browse-content"`, a `[Subjects | Timeline]` toggle
  (a `segmented` control with `browseMode`), and the retained `id="filter"`; and
  the `class="view md"` / `pane-list` / `pane-detail` counts in the existing
  `index_html_wires_the_responsive_layout` test are updated to **4** (Comments,
  Chat, Teloi, Browse).
- [ ] AC-2: (covers REQ-3) A unit test asserts the timeline is wired: `INDEX_HTML`
  contains `renderBrowse`, a `BROWSE_TIMELINE_CAP`, a sort by `recorded_at`, a
  "showing"/"of" cap note, and `openBrowseClaim(`.
- [ ] AC-3: (covers REQ-4) A unit test asserts the state summary is computed:
  `INDEX_HTML` references `"Publication"`, `"Retraction"`, and `"Status"` in a
  per-subject summary path and renders `published`/`retracted` badge markers.
- [ ] AC-4: (covers REQ-5) A unit test asserts `INDEX_HTML` contains a
  `KIND_GLYPH` map and a `.kind.` rule for each of the nine kinds
  (`Subject`, `Decision`, `Publication`, `Plan`, `Observation`, `Result`,
  `Status`, `Relation`, `Retraction`).
- [ ] AC-5: (covers REQ-6, REQ-7) A unit test asserts the detail pane is wired —
  `openBrowseSubject` and `openBrowseClaim` target `paneDetail("browse")` and call
  `claimEl(`, while `renderBrowse` targets `#browse-content` (`paneList` shell),
  not the detail — and that `INDEX_HTML` contains no new `<script src`/`<link href`
  (no dependency added).
- [ ] AC-6: (covers REQ-1..REQ-7) `cargo test`, `cargo clippy --all-targets --
  -D warnings`, and `cargo fmt --check` are green; the render itself is page-only
  and confirmed by an operator eyeball (stated, not machine-checked).

## Architecture
**Page-only** (`src/web/index.html`) plus test updates in `src/server.rs`; no Rust
behavior or endpoint change. The fold already carries `subjects: Vec<String>`,
`claims: {subject: [Claim]}` (each `{subject,kind,author,recorded_at,text,title,
artifacts,cites,cid,supersedes}`), and `by_cid` — everything the timeline, the
state summary, and the detail need.

**Markup.** `#view-browse` changes from `<input id="filter"> + <div id="subjects">`
to `class="view md"` with `#browse-list` (holding the `segmented` toggle, the
`#filter` input, and `#browse-content`) and `#browse-detail`, mirroring the other
`.view.md` tabs.

**List pane.** `render()`'s `renderSubjects()` call becomes `renderBrowse()`.
`renderBrowse` dispatches on `browseMode`: `"subjects"` groups `fold.subjects` by
namespace (`ns`) and renders one row per subject — leaf name + a
`subjectState(subject)` summary (most-recent kind + published/retracted/status
badges) — tapping which calls `openBrowseSubject`; `"timeline"` builds the flat
sorted-capped-filtered claim list of `timelineRow`s tapping into `openBrowseClaim`.
It writes only into `#browse-content`, so a fold re-render never disturbs an open
detail (the `renderTeloi` live-safety pattern). The toggle and `#filter` listeners
(wired once at init) flip `browseMode`/`filter` and re-render `#browse-content`.

**Detail pane.** `openBrowseSubject(subject)` → `setDetailOpen("browse", true)`,
a `subjectState` header, then `fold.claims[subject]` through the existing
`claimEl`. `openBrowseClaim(claim)` → the claim's subject as a tappable header
(`openBrowseSubject`) then `claimEl(claim)`. `backBar` runs `paneReset("browse", …)`.

**Per-op formatting.** A `KIND_GLYPH` object (kind → glyph) drives the timeline
row and `claimEl`'s badge; CSS adds a `.kind.<Kind>` color rule for each of the
nine kinds (extending the current `.kind.Plan`/`.kind.Subject`/`.kind.Retraction`/
`.kind.Status` set). `claimEl`, `summarize`, `fmtWhen`, `shortAuthor`, and
`fold.by_cid` cite resolution are reused unchanged.

**Reuse, not duplication.** No new claim renderer — `claimEl` is used in both
detail modes; the only new functions are `renderBrowse`, `subjectState`,
`timelineRow`, `openBrowseSubject`, `openBrowseClaim`. The
`index_html_wires_the_responsive_layout` test's master-detail counts move 3→4 as
Browse joins the `.view.md` tabs.

## Resolved Questions
- RQ-1: **Master-detail with a `[Subjects | Timeline]` toggle**, not a single-pane
  layered view — Browse joins the `.view.md` tabs (finishing the slice-1
  deferral), the toggle switches the list-pane mode, and the detail pane shows a
  tapped subject's claims or a tapped claim's detail. This uses the desktop width
  and gives the timeline room.
- RQ-2: **The timeline is flat, newest-first, capped at 200, and filterable** —
  not day-grouped — with a "showing N of M" note; the existing filter box narrows
  it. The cap keeps the DOM light over 578+ claims.
- RQ-3: **The state summary shows most-recent kind + published + retracted + a
  neutral status marker**, computed client-side; the Status *value*
  (resolved/blocked/open) is **not serialized in the fold**, so it is deferred to a
  later slice that would add it to the fold — this slice stays page-only.

## Out of Scope
- **The Status value** (resolved/blocked/open) — not in the fold JSON; showing it
  needs a `substrate` change, deferred to keep this slice page-only.
- **Retracted-content trees** — the live fold hides retracted claims' content
  (only the `Retraction` claim shows, naming its target via `supersedes`); showing
  what a retraction removed is blocked on a kan capability, unchanged here.
- **Editing / writing claims from the page** — Browse is read-only, like the rest
  of the browser.
- **A relation graph view** — `Relation` claims get a glyph/badge like any kind,
  but a graph of relation edges is not in this slice.
- **URL deep-linking** a subject or claim (`#subject/…`) — selection is in-page
  state, not a routable URL, in this slice.
- **Server-side timeline paging** — the cap + client filter suffice; no `?since=`
  or cursor endpoint.
