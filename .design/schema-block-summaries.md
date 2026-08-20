# Feature: day-bridge and day-schema block summaries

## Summary
Teach the claim-detail block renderer to summarize the two day fenced-block
types it currently dumps as raw JSON: `day-bridge` (10 subjects — the most
common unsummarized block, whose plan is a nested atom tree) and `day-schema`
(the structural rules on `schema/design-doc`). Both become human-readable
`block_summary` views, closing the gap left when alpha.7's design named
`day-bridge` but never implemented it.

## Requirements
- REQ-1: `block_summary` (`src/substrate.rs:361`) gains a `day-bridge` arm that
  renders `telos:`, `have:` (comma-joined), and `plan:` — where the plan's
  nested atom tree is flattened with day's own composition operators (day's
  `Node` enum, serde `rename_all = "lowercase"`: `seq`/`all`/`any`): a `seq`
  joins with ` > `, an `all` (concurrent) with ` & `, an `any` (alternatives)
  with ` | `, and a leaf `{atom}` is its atom name. Any extra top-level keys are
  still appended (`telos/honest-ambiguity`).
- REQ-2: `block_summary` gains a `day-schema` arm that renders every field of the
  block as `key: value` (arrays comma-joined) via the existing
  `append_extra_keys` (`src/substrate.rs:341`), so `schema/design-doc` and any
  future `schema/*` block reads as a human list instead of raw JSON.
- REQ-3: An unrecognized fence still returns `None` so the caller shows it as raw
  code (`src/tui.rs`'s `claim_detail` block arm) — nothing is hidden, and the
  raw-code fallback for unknown blocks is unchanged.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts `block_summary("day-bridge", j)`
  for the real shape `{"telos":"readable-claim-browser","have":["design-doc"],
  "plan":{"seq":[{"atom":"generative-build"},{"atom":"adversarial-review"}]}}`
  yields lines containing `telos: readable-claim-browser`, `have: design-doc`,
  and `plan: generative-build > adversarial-review`.
- [ ] AC-2: (covers REQ-1) A unit test asserts a plan mixing composition in day's
  real grammar — `{"seq":[{"atom":"a"},{"all":[{"atom":"b"},{"atom":"c"}]},
  {"any":[{"atom":"d"},{"atom":"e"}]}]}` — flattens to `a > b & c > d | e`, and a
  second test asserts a real corpus-shaped `seq` with an `any` branch renders
  without leaking raw JSON.
- [ ] AC-3: (covers REQ-2) A unit test asserts `block_summary("day-schema", j)`
  for a block carrying `requirement_prefix`, `min_requirements`, and `sections`
  yields the corresponding `key: value` lines (e.g. `requirement_prefix: REQ-`,
  `sections: Summary, Requirements`).
- [ ] AC-4: (covers REQ-3) A unit test asserts `block_summary("day-unknown", j)`
  returns `None`.

## Architecture
`block_summary(fence, j) -> Option<Vec<String>>` (`src/substrate.rs:361`) is the
single dispatch for structured-block views; it already handles `day-atom`,
`day-telos`, `day-tension`, `day-witness`, and `cospan-comment`, and returns
`None` for anything else, which `claim_detail` (`src/tui.rs`) renders as a raw
labeled code block. This change adds two arms and one helper, touching only
`src/substrate.rs`.

The `day-schema` arm is a one-liner: start with an empty `Vec`, call
`append_extra_keys(&mut out, j, &[])` (`src/substrate.rs:341`) so every field is
rendered sorted as `key: value` (strings verbatim, arrays comma-joined, others
via their JSON form), and return `Some(out)`. No field is special-cased, so a
schema block that grows new rules stays fully shown without a code change.

The `day-bridge` arm renders `telos:` (from the `telos` string), `have:` (from
`str_array_at(j, "have")` joined with `, `), and `plan:` from a new
`flatten_plan(node: &Value) -> String` helper beside the other day-block
parsers. `flatten_plan` matches on the object's shape against day's real `Node` grammar
(`Seq`/`All`/`Any`, serde `rename_all = "lowercase"`): a `seq`/`all`/`any` key
holding an array recurses over its elements and joins them with ` > ` / ` & ` /
` | ` respectively; an `atom` key returns its string; anything else falls back to
the compact JSON. It composes recursively so a nested plan (an `all` inside a
`seq`) renders inline, mirroring day's `>`/`&`/`|` operators. After
the three lines, `append_extra_keys(&mut out, j, &["telos", "have", "plan"])`
appends any other top-level key. The real bridge blocks live on `bridge/*`
subjects (e.g. `bridge/p0-spine`, `bridge/claim-detail`); the schema block on
`schema/design-doc`.

Nothing is read from or written to kan; `block_summary` stays a pure function of
one parsed block, and the change is additive — the five existing arms and the
`None` fallback are untouched, so `telos/kan-is-truth` (pure projection) and
`telos/honest-ambiguity` (unknown blocks shown, not hidden) both hold.

## Resolved Questions
- RQ-1: `day-schema` is rendered generically (every key via `append_extra_keys`)
  rather than with a bespoke per-field layout, so the view never falls behind
  the schema's vocabulary — a new rule appears automatically.
- RQ-2: `day-bridge`'s plan is flattened with day's `>` / `&` / `|` operators
  (mapped from day's real `seq` / `all` / `any` node keys) rather than shown as a
  bullet tree, matching how day itself writes composition and keeping the summary
  to one line per plan.
- RQ-3: This is the "schemas next" cycle sequenced after the path-aggregated
  tree; it is additive to `block_summary` and does not touch the tree work.

## Open Questions

_None outstanding — the two render choices were resolved during design._

## Out of Scope
- Reconciling day's own format inconsistencies (abbreviated vs full key names,
  the two `schema/` fence names, the untagged witness-probe union); that is the
  separate cross-repo kan-log pass sequenced after this.
- Any change to the `day-witness` arm or the other existing summaries.
- Rendering the plan as an interactive/navigable tree; it stays a one-line
  flattened string.
