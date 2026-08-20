# Feature: short-author collision + block-HTML newline fixes

## Summary
Two low-severity display cleanups deferred from earlier reviews: `short_author`
front-truncation collides once a log has multiple signers, and block-level HTML
in a markdown claim body collapses its lines onto one row. Both are contained
fixes with regression tests.

## Requirements
- REQ-1: `Claim::short_author` (`src/substrate.rs`) must distinguish two authors
  that share a leading key prefix. For a stripped key longer than 11 chars it
  renders `head6…tail4` (both ends distinguish); a shorter key renders whole.
  `display_line`'s author column widens to fit.
- REQ-2: The markdown renderer (`src/markdown.rs`) must keep a block-HTML block's
  lines on separate rows. pulldown-cmark emits an HTML block as one `Event::Html`
  per line, each ending in `\n`; the handler must treat every `\n` (including the
  trailing one) as a line break so consecutive events do not collapse onto one
  row.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test asserts a short key renders whole
  (prefix stripped), a real-length key renders `head6…tail4`, and two keys that
  share their first 8 chars but differ later produce different `short_author`s.
- [ ] AC-2: (covers REQ-2) A unit test renders a multi-line `<div>` HTML block and
  asserts each inner line (`<p>alpha</p>`, `<p>beta</p>`) lands on its own row and
  nothing is collapsed into a single combined line.

## Architecture
`Claim::short_author` (`src/substrate.rs`) currently returns the first 8 chars of
the `did:key:`-stripped author. That collides for signers sharing a prefix — a
real case once roles or cross-repo authors write to a log. It becomes a
head-and-tail abbreviation (`head6…tail4`) for keys long enough to truncate,
returning short keys whole; `display_line`'s `{:<8}` author field widens to
`{:<12}` so the abbreviation aligns.

`markdown::render` (`src/markdown.rs`) handles `Event::Html` by splitting on `\n`
and flushing between segments, but it first `trim_end_matches('\n')` and only
flushes *within* one event — so consecutive per-line events (each `"<p>…</p>\n"`)
concatenate onto one `cur` line. The fix drops the trim and flushes on every
`\n`: `split('\n')` yields a trailing empty segment for a `\n`-terminated event,
which flushes the line and pushes no span, so the next event starts fresh. Single
multi-line events and events without a trailing newline both still render
correctly; the closing `if !cur.is_empty()` flush is unchanged.

Both changes are display-only projections of a claim; nothing is read from or
written to kan.

## Resolved Questions
- RQ-1: `short_author` uses head+tail rather than more leading chars or a hash,
  matching the familiar address-abbreviation idiom while keeping the readable
  `did:key` prefix; exact widths (6/4) are a display choice, easily tuned.
- RQ-2: The renderer is corrected rather than pre-joining HTML events, so a
  genuine single-line HTML fragment is unaffected.

## Open Questions

_None — both fixes are contained and regression-tested._

## Out of Scope
- The third deferred finding (a design doc's on-disk hash drifting from its
  recorded chain after post-record edits) is inherent to day's record-then-edit
  flow, not a cospan defect; it is a `kan-tools/day` concern, not fixed here.
- Rendering HTML blocks as anything richer than their raw text.
