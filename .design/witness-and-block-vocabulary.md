# Feature: robust day-witness rendering + day-docs/day-injection

## Summary
Make cospan render day's real, full block vocabulary as observed across the
`day`, `kan`, `cospan`, and `mingus` logs. Fix a `day-witness` rendering bug (a
single-probe block like `{"command":"cargo test"}` renders `command: ?`), handle
the untagged probe union, and add summaries for the two day block types cospan
currently dumps as raw JSON: `day-docs` and `day-injection`.

## Requirements
- REQ-1: The `day-witness` arm (`src/substrate.rs`) renders **both** shapes the
  fence carries. A whole-schema **map** (`{ "<name>": <probe>, ... }`) renders one
  `name: <probe-kind>` line per entry, as today. A **single probe** — a block
  whose top-level keys are all in the probe-kind set (`path`, `command`, `claim`,
  `tag`, `material`, `record`, `every`) — renders one line naming the probe and, for a
  scalar-valued probe, its value: `{"command":"cargo test"}` renders
  `command: cargo test`, not `command: ?`.
- REQ-2: A shared `probe_kind`/`describe_probe` helper handles the untagged probe
  union: `path`/`command`/`tag` (string-valued) show their kind (and, in a lone
  probe, their value); `claim` (object) and the nested `{material, record}` show
  their kind(s) joined with `+` for a multi-key probe (so `code-change`'s
  `{material, record}` renders `material+record`, never `?`).
- REQ-3: `block_summary` gains `day-docs` and `day-injection` arms that render
  every field generically via `append_extra_keys` (`src/substrate.rs`), sharing
  the generic path with the existing `day-schema` arm, so those blocks read as
  `key: value` lists instead of raw JSON.
- REQ-4: An unrecognized fence still returns `None` (raw-code fallback), and the
  other arms (`day-atom`, `day-telos`, `day-tension`, `day-bridge`,
  `cospan-comment`) are untouched.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts `block_summary("day-witness",
  {"command":"cargo test"})` yields a line `command: cargo test` (the bug fix),
  and that `{"path":"src/*.rs"}` yields `path: src/*.rs`.
- [ ] AC-2: (covers REQ-1, REQ-2) A unit test asserts the whole-schema map
  `{"design-doc":{"path":"..."},"code-change":{"material":{},"record":{}},
  "passing-tests":{"command":"cargo test"}}` yields `design-doc: path`,
  `code-change: material+record`, and `passing-tests: command` (probe kind only,
  one line per witness, sorted).
- [ ] AC-3: (covers REQ-3) A unit test asserts `block_summary("day-docs",
  {"doc_files":["README.md"],"version_key":"version"})` yields
  `doc_files: README.md` and `version_key: version`, and `block_summary(
  "day-injection", {"cadence":"turn","max_practice_items":3})` yields
  `cadence: turn` and `max_practice_items: 3`.
- [ ] AC-4: (covers REQ-4) A unit test asserts `block_summary("day-unknown", j)`
  returns `None`.

## Architecture
`block_summary(fence, j)` (`src/substrate.rs`) is the single dispatch for
structured-block views. This change reworks the `day-witness` arm, adds two arms
that reuse the generic renderer, and adds one small helper — all within
`src/substrate.rs`.

Today the `day-witness` arm assumes the map shape: for each top-level key it
reads the value's first sub-key as the probe kind, defaulting to `?`. That breaks
on a single-probe block, whose top-level key (`command`) *is* the probe kind and
whose value is a string with no sub-keys — so it prints `command: ?`. The fix
adds a `PROBE_KINDS` set (`path`, `command`, `claim`, `tag`, `material`,
`record`, `every`) and an `is_probe(obj)` test (non-empty and every key in the set). When
the whole block is a probe, the arm renders one line via `describe_probe` — the
probe's kind, plus `: value` when the probe is a single string-valued key.
Otherwise it treats the block as a `name -> probe` map and renders `name:
<probe_kind>` per entry, where `probe_kind` joins an object probe's keys with `+`
(so `{material, record}` is `material+record`). This mirrors the untagged union
day emits (see the `day` repo issues on `day-witness` overloading and the probe
union); cospan shape-sniffs so a human still gets a readable line either way
(`telos/honest-ambiguity`).

`day-docs` and `day-injection` join `day-schema` in a single match arm that calls
`append_extra_keys(&mut out, j, &[])`, rendering every field sorted as
`key: value` (arrays comma-joined). No field is special-cased, so a block that
grows keys stays fully shown without a code change. The `_ => None` fallback is
unchanged, so a genuinely unknown fence is still shown as raw code by
`claim_detail` in `src/tui.rs`.

Nothing is read from or written to kan; `block_summary` stays a pure function of
one parsed block. The change is additive plus one bug fix, honoring
`telos/kan-is-truth` (pure projection) and `telos/readable-claim-browser` (every
block cospan can name now renders readably rather than as raw JSON or `?`).

## Resolved Questions
- RQ-1: `day-witness` is disambiguated by shape-sniffing (top-level keys all in
  `PROBE_KINDS` → a lone probe; otherwise a name→probe map), because day's fence
  carries no discriminator. The ambiguity itself is reported upstream as a `day`
  repo issue rather than worked around silently.
- RQ-2: `day-docs`/`day-injection` are rendered generically (every field via
  `append_extra_keys`) rather than with bespoke layouts, so the view never falls
  behind day's evolving keys — matching the `day-schema` decision.
- RQ-3: This is the reconciliation cycle sequenced after the schema-block
  summaries; the day-side format inconsistencies are handed off as `day` repo
  issues, not fixed here.

## Open Questions

_None outstanding — the shape-sniff and generic-render choices were resolved
during design._

## Out of Scope
- Changing day's own emission format (the overloaded `day-witness`, two schema
  fence names, untagged probe union, sporadic `_version`); those are filed as
  `kan-tools/day` issues.
- A bespoke per-field layout for `day-docs`/`day-injection`; they render
  generically.
- Rendering a `claim` probe's inner spec (kind/subject/starts_with) beyond naming
  it `claim`; the nested detail stays collapsed to the kind.
