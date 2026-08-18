# 04 — The re-localizer (BUILT)

The one load-bearing algorithm, and the only thing built so far. Source:
[`../src/lib.rs`](../src/lib.rs), driver [`../src/main.rs`](../src/main.rs).
Pure `std`, pure function, 6/6 tests.

## The problem

A human/agent pins a comment to some text. Agents then rewrite the file. Keep each
comment on the *right* text as the file moves — or, when we honestly can't, say so
rather than guess silently. Line numbers are the first casualty of any edit, so we
anchor to a **text fingerprint** (target + surrounding context) and re-resolve it.

## Three honest states

| state | meaning | UI |
|-------|---------|----|
| `Anchored` | found uniquely | show in gutter, confident |
| `Drifted` | text changed; best-guess match + confidence | show with a "moved" marker |
| `Unresolvable` | lost, or ambiguous between candidates | send to resolve-by-hand list |

Three, not two: "80% sure it moved here" is different information from "lost it,"
and collapsing them is how comment tools feel untrustworthy. The `Unresolvable`
list is the same honest-ambiguity idiom `day` uses for process position.

## The algorithm (`relocalize(anchor, new_content) -> Localization`)

1. **Exact, contiguous match of the target block.**
   - Unique → `Anchored`, confidence 1.0. (Covers pure line-shift: an agent
     inserted/deleted lines elsewhere and the block moved wholesale.)
   - Multiple → disambiguate by **context score** (similarity of remembered
     before/after to the file around each candidate, + a tiny proximity tiebreak to
     `line_hint`). Clear winner → `Anchored`; within `AMBIGUITY_MARGIN` →
     `Unresolvable`.
2. **No exact match → the target text itself changed.** Slide a window the size of
   the target; score each by `0.8·similarity(target) + 0.2·context`. Best above
   `FUZZY_FLOOR` and clear of the runner-up → `Drifted` (confidence = score);
   else `Unresolvable`.

`similarity` = normalized character Levenshtein (`1 - dist/maxlen`), hand-rolled
two-row DP. Zero dependencies (builds offline anywhere).

### Tunable constants (the knobs to feel out)
- `FUZZY_FLOOR = 0.60` — below this, not worth showing even as a guess.
- `AMBIGUITY_MARGIN = 0.05` — candidates within this are "can't choose" →
  `Unresolvable`. (Small, because a perfect-context hit only edges out a near-miss
  by a little.)

## What the demo proves

```
v0  unchanged                         ANCHORED   line 2   conf 1.00
v1  agent prepends docs (line-shift)  ANCHORED   line 4   conf 1.00
v2  agent renames the call (edited)   DRIFTED    line 2   conf 0.86
v3  agent rewrites the function       UNRESOLVED —        conf 0.31
```

## Deliberate simplifications (to revisit)

- **Whole-file scan** on the fuzzy pass. The live tool should scope it to the
  changed hunks via a **diff against last-seen** contents (`similar`/`imara-diff`),
  which is both faster and more accurate — the fingerprint search here is the
  source of truth, the diff is the optimization.
- **Incremental tracking**: cache last-seen file content per file and re-anchor
  against *that* each tick (not against the original), so many small edits don't
  accumulate drift. (Designed; the current pure function doesn't need old content.)
- **Structural anchoring** (later): for code, a tree-sitter node path ("3rd stmt of
  `fn foo`") survives reformatting where text similarity fails. Use as a confidence
  tie-breaker in a later phase — not P1.
- **Multiline & sub-line** anchors: current model is line-block granularity.
  Character-span anchors within a line are a later refinement.

## Try it
```
cargo run -- demo
cargo run -- watch <file> --line <N> [--ctx <N>]
cargo test
```
