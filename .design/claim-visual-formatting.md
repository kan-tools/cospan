# Feature: Claim visual formatting — kind colors + retraction display

## Summary
Give the claim browser richer visual structure: color each claim by kind so the
eye parses a subject's log fast, and render Retraction claims distinctly — showing
what they retract and when. The retracted claim's *content* is not available from
kan's live fold, so this is the visible-retraction annotation, with full
retracted-content trees left blocked on a kan capability.

## Requirements
- REQ-1: `substrate::Claim` gains `supersedes: Option<String>`, parsed from the
  kan JSON (the CID a `Retraction` — or a superseding claim — names); it defaults
  to `None` for claims that supersede nothing.
- REQ-2: A `Retraction` claim's one-line rendering (`Claim::display_line`, shared
  by `cospan subject` and the TUI) shows `retracts @shortcid` using
  `substrate::short_cid` of its `supersedes` target, instead of the bare
  `(retraction)` — so what a retraction acted on, and when, is legible.
- REQ-3: The TUI claim list colors each row by kind via a `kind_style` mapping —
  Decision, Observation, Plan, Result, Subject, Relation, Publication, Retraction
  each get a distinct `ratatui` style using the 16-color ANSI palette (so it
  reads on both light and dark terminals); the selected-row highlight still wins.
- REQ-4: The claim detail pane tints its border title by the claim's kind with
  the same `kind_style`, so a claim's kind is legible in the detail as in the list.
- REQ-5: Formatting is a view concern: the fold is unchanged, and colors never
  reach `cospan subject`/`--once` (plain text). The shared `retracts @shortcid`
  text does appear in both surfaces — it is content, not color.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test parses a `Retraction` claim JSON carrying
  `supersedes` into a `Claim` whose `supersedes` is `Some(that cid)`, and an
  ordinary claim into one whose `supersedes` is `None`.
- [ ] AC-2: (covers REQ-2) A unit test asserts a `Retraction` claim's
  `display_line` contains `retracts @` followed by the short-cid of its
  `supersedes` target, while a non-retraction line is unchanged.
- [ ] AC-3: (covers REQ-3, REQ-4) A unit test asserts `kind_style` returns
  distinct styles for distinct kinds (e.g. `Retraction` differs from `Decision`)
  and a stable style for an unknown kind.
- [ ] AC-4: (covers REQ-5) `cospan subject . telos/disposable` and
  `cospan watch-repo . --once` still exit 0 with plain text, and a `Retraction`
  claim in `cospan subject` output shows its `retracts @…` annotation.

## Architecture
`src/substrate.rs`: `Claim` gains `supersedes`, read by `claim_from_value` via the
existing `opt_str_at`. `Claim::summary` (which `display_line` calls) special-cases
`kind == "Retraction"`: when `supersedes` is present it returns
`format!("retracts {}", short_cid(target))`, else falls back to the current
kind-label. This is the one behavior shared with the CLI; everything else here is
`src/tui.rs`.

`src/tui.rs` gains `kind_style(kind: &str) -> ratatui::style::Style`, a pure map
from kan claim kind to an ANSI-16 foreground (Decision→green, Observation→blue,
Plan→yellow, Result→cyan, Subject→bold/gray, Relation→magenta,
Publication→dark-gray, Retraction→red; unknown→default). `draw_claims` builds each
populated row from the claim itself — `ListItem::new(Line::from(display_line).style(
kind_style(&claim.kind)))` — rather than from `detail_lines`' pre-joined strings,
so it has the kind in hand for color; the loading/error/empty states still come
from `detail_lines`. `claim_selected` still maps 1:1 to a row because the order is
the same `subject_claims` order. `draw_claim_detail` styles its block title with
`kind_style(&claim.kind)`.

Nothing new is read from kan and nothing is written; this is pure rendering over
the same `Claim` fold (`telos/kan-is-truth`). Retracted claims themselves remain
outside kan's live fold, so the retracted *content* cannot be shown here — that
full retraction tree is recorded as blocked on a kan capability, distinct from
this visible-retraction annotation (`claim-visual-formatting`).

## Resolved Questions
- RQ-1: Colors use the ANSI 16-color palette keyed by claim kind, so they render
  on both light and dark terminals without a theme system; the selection
  highlight still overrides the row.
- RQ-2: Retraction is shown as an annotation — `retracts @shortcid` plus the
  retraction's own timestamp — since kan's live fold does not expose the retracted
  claim's content; the tree'd retracted-content view is blocked on a kan
  capability and recorded as such, not attempted here.

## Open Questions

_None outstanding — the two choices above were resolved during design._

## Out of Scope
- Showing the retracted claim's content, or a true parent/child retraction tree —
  blocked on kan surfacing retracted claims (recorded on `claim-visual-formatting`).
- Per-day-block tinting beyond kind (e.g. distinct colors inside a `day-telos`
  vs `day-atom` body); this step colors by claim kind only.
- A configurable palette or theme file; the ANSI-16 map is fixed for now.
- Coloring `cospan subject`/`--once` output; those stay plain text.
