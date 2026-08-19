# Feature: kan claim detail view

## Summary
Extend the two-pane browser so you can drill from a subject's claim list into a
single claim's full detail: its kind, id, author, time, subject, artifact
anchor, untruncated (scrollable) text, and each cite resolved to a one-line
preview. Navigation is a three-level focus — Subjects → Claims → Detail — that
Enter descends and Esc ascends. It shows the whole claim rather than a truncated
line, serving honest-ambiguity and kan-is-truth.

## Requirements
- REQ-1: `substrate::Claim` gains `artifacts: Vec<String>` and `cites:
  Vec<String>`, parsed from `kan show --json` (both are arrays of strings and
  default to empty when absent); `subject_claims` populates them for every claim.
- REQ-2: A `substrate::short_cid` renders a kan CID compactly as `@` + the seven
  characters after the shared `bafyrei` prefix + `…` (e.g.
  `bafyreictf6g6fq4…` → `@ctf6g6f…`); a CID without that prefix falls back to `@`
  + its first seven characters + `…` (recorded on `cid-shortcut-notation`).
- REQ-3: A `substrate::claim_index(repo)` folds `kan show --all --json` — whose
  claims are nested under `subjects[].claims[]`, not the top-level `claims` — into
  a `cid → Claim` map, so a cited CID can be resolved to its kind and text across
  subjects in one process spawn (kan#123/#181).
- REQ-4: `AppState` gains a three-level `Focus` (Subjects, Claims, Detail): Enter
  descends one level and Esc ascends one, each clamped at the ends. `j`/`k` (and
  Down/Up) act on the focused level — moving the subject, then the selected
  claim, then scrolling the detail. Descending into Claims loads the subject's
  claims; descending into Detail resets the scroll to the top.
- REQ-5: The detail pane renders the selected claim in full — kind, its full cid,
  author, `recorded_at` as UTC, subject, artifact anchor(s), the untruncated
  text (scrollable via the Detail focus), and each entry of `cites` resolved
  through `claim_index` to `@shortcid  Kind  "first line"`, falling back to the
  bare `@shortcid` when the cited claim is absent from the index. It is a pure
  projection of what kan returned — nothing synthesized, nothing hidden
  (`telos/kan-is-truth`, `telos/honest-ambiguity`).
- REQ-6: The responsive layout holds: at width ≥ 100 the subject list sits beside
  a right pane that shows the claim list (Subjects/Claims focus) or the claim
  detail (Detail focus); below 100 a single pane shows the focused level.
  `watch-repo --once` and the `demo`/`watch`/`subject`/`comments` subcommands are
  unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test parses a `kan show --json` claim carrying
  `artifacts` and `cites` into a `Claim` with both populated, and a claim without
  `cites` into one whose `cites` is empty.
- [ ] AC-2: (covers REQ-2) A unit test asserts `short_cid` maps a real
  `bafyrei…` CID to `@` + seven chars + `…` and falls back sensibly for a CID
  without the prefix.
- [ ] AC-3: (covers REQ-3) A unit test folds an `--all`-shaped JSON payload
  (claims nested under `subjects[].claims[]`) with `claim_index` and asserts the
  `cid → Claim` map has the expected entries.
- [ ] AC-4: (covers REQ-4) A unit test drives focus: Enter steps Subjects → Claims
  → Detail and stops; Esc steps Detail → Claims → Subjects and stops; `j`/`k`
  moves the claim index within bounds at Claims focus and clamps the detail scroll
  at 0 at Detail focus.
- [ ] AC-5: (covers REQ-5) A unit test of a pure `detail_view(claim, &index)`
  returns lines containing the claim's kind and full text, a cite resolved to
  `@shortcid Kind "first line"`, and — for a cite absent from the index — the bare
  `@shortcid`.
- [ ] AC-6: (covers REQ-6) `cospan watch-repo . --once` exits 0 and still prints
  the plain dashboard, confirming the detail-view work left the scriptable path
  and the other subcommands intact.

## Architecture
Step 3 left `src/tui.rs` with `AppState` (subject `rows` + `selected`, a per-fold
`claims` cache, a `Pane` narrow flag) and the `draw`/`draw_list`/`draw_detail`
split, and `src/substrate.rs` with `Claim`, `subject_claims`, `claim_from_value`,
and `Claim::display_line`. This step deepens that, touching the interactive path
only.

`src/substrate.rs` grows three things. `Claim` gains `artifacts` and `cites`
(`claim_from_value` reads the JSON arrays via a small string-array helper beside
`str_at`, defaulting to empty). `short_cid` is a pure string function.
`claim_index` shells `kan show --all --json` — mirroring `kan_status`/`subject_claims`
— and walks the **nested** `subjects[].claims[]` (the `--all` shape differs from
the single-subject one, whose `claims` is top-level and whose `subjects` is a list
of name strings), mapping each `cid` to its parsed `Claim`. It is the cross-subject
fold that lets a cite become a preview, and it is one spawn for the whole graph.

`src/tui.rs` replaces the two-state `Pane` with a three-level `Focus { Subjects,
Claims, Detail }`, and `AppState` gains `claim_selected` (index into the current
subject's claim vec), `detail_scroll`, and a lazily-loaded `cite_index`
(`HashMap<String, Claim>`, cleared on `refold`). The key handler routes by focus:
`descend`/`ascend` move between levels (Enter/Esc), and `j`/`k` move the subject,
the claim, or the scroll. Descending into Claims runs the existing
`ensure_selected_loaded`; descending into Detail loads `cite_index` if the claim
has cites and zeroes `detail_scroll`. All of this — the focus transitions, the
clamps, and a pure `detail_view(claim, &cite_index) -> Vec<String>` — is testable
without a terminal (AC-4, AC-5), the same seam Steps 2–3 used.

`draw` chooses the body from `layout_mode(width)` as before; the right pane (wide)
or the single pane (narrow) renders the claim list when focus is Subjects/Claims
and `detail_view` when focus is Detail. `detail_view` composes the header fields,
the artifact anchors, the wrapped text sliced by `detail_scroll`, and the cites —
each `@short_cid` plus, from `cite_index`, the cited claim's kind and
`first non-empty line`. day's process pane and the `clip_lines` cue are retained.

No new write path and nothing reaches kan beyond reads; the whole view remains a
projection of the fold (`telos/kan-is-truth`). The forward-facing UX the user
noted — the kan-claims view eventually being a **separate tab** from an agent/
harness interface view, and a claim detail being **context-summoned** when a cite
or verdict elsewhere references it — is out of scope here and recorded on
`claim-detail-view`; this step is the in-browser detail only.

## Resolved Questions
- RQ-1: Navigation is a three-level focus, Subjects → Claims → Detail, that Enter
  descends and Esc ascends; `j`/`k` acts on the focused level (move subject, move
  claim, scroll detail).
- RQ-2: The detail pane scrolls when focused, so a long claim (a multi-paragraph
  build result, a telos with a fenced block) is fully readable rather than clipped.
- RQ-3: Cites resolve to a one-line preview (kind + first line) via a
  `claim_index` fold of `kan show --all --json`, rendered with the `@shortcid`
  notation; a cite absent from the index shows the bare `@shortcid`.
- RQ-4: CIDs display in the shortcut notation `@` + seven chars after `bafyrei` +
  `…`, recorded on `cid-shortcut-notation`; the claim's own full cid is shown in
  its detail header.

## Open Questions

_None outstanding — the four choices above were resolved during design._

## Out of Scope
- A tab system separating the kan-claims view from a future agent/harness
  interface view (`claim-detail-view` records the intent).
- Context-summoning a claim's detail from a cite/verdict/handoff reference
  elsewhere; this step lists cites but does not jump to them.
- Retraction grouping and schema-driven colors (`claim-visual-formatting`).
- The atom-DAG and telos/witness views (`process-atom-telos-views`).
- Editing, resolving, or persisting claims; the browser stays read-only.
