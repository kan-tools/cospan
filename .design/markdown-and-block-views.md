# Feature: Markdown claim bodies + structured-block views

## Summary
Render a claim's body as markdown in the detail pane — wrapped, with headers,
emphasis, lists, inline code, and code fences styled — instead of raw text. And
give the fenced structured blocks claims carry a readable view: a human summary
for the supported `day-*` / `cospan-comment` types, and a code-formatted markdown
view for unsupported ones, so nothing renders as an unreadable wall.

## Requirements
- REQ-1: A `markdown::render(md) -> Vec<Line>` parses claim-body markdown with
  `pulldown-cmark` into styled `ratatui` lines: ATX headings bold, `**strong**`
  bold, `*emphasis*` italic, `` `inline code` `` and fenced code blocks in a
  distinct style, list items with a bullet/number prefix, and paragraphs
  separated by blank lines.
- REQ-2: The detail pane wraps long lines to the pane width (`ratatui`
  `Wrap`), so a claim body is never cut off horizontally.
- REQ-3: A `block_summary(fence, json) -> Option<Vec<String>>` renders a
  human-readable view of a supported fenced block — `day-atom` (in/out/next/
  revisits), `day-telos` (witnesses + scope), `day-bridge` (plan), `day-witness`
  (type → probe), `day-tension` (the pair), `cospan-comment` (body/author/
  resolved) — from its parsed JSON.
- REQ-4: The claim detail composes: the header fields (kind, cid, author, time,
  subject, anchor), then the body split into segments — each fenced block
  replaced by its `block_summary` when supported, or shown as a labeled code
  block when not, and the prose between blocks rendered as markdown — then the
  resolved cites. It is a pure projection of the claim (`telos/kan-is-truth`), and
  an unsupported block is shown as code rather than hidden (`telos/honest-ambiguity`).
- REQ-5: The detail is a `Vec<Line>`; `detail_line_count` (for scroll clamping)
  and the Detail-focus scroll continue to work over it, and the header title
  keeps its kind color.
- REQ-6: `watch-repo --once` and the `subject` CLI keep their plain one-line
  rendering (no markdown); this is an interactive detail-pane change only.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1) A unit test renders markdown with a heading, a
  `**bold**` span, an `- item` list, and a fenced code block, and asserts the
  produced lines carry the expected text and that the code line's style differs
  from a plain paragraph line's.
- [ ] AC-2: (covers REQ-3) A unit test asserts `block_summary("day-atom", json)`
  yields lines naming the in/out/next values, and `block_summary("unknown", …)`
  returns `None`.
- [ ] AC-3: (covers REQ-4) A unit test builds a claim whose body is prose plus a
  fenced `day-telos` block, renders the detail, and asserts it contains both the
  prose and the human `witnesses:` summary (not the raw JSON), plus the header
  kind and a resolved cite.
- [ ] AC-4: (covers REQ-4) A unit test builds a claim whose body carries an
  *unsupported* fenced block (e.g. ```` ```json ````) and asserts the detail shows
  the block's content in a code style, not dropped.
- [ ] AC-5: (covers REQ-6) `cospan watch-repo . --once` exits 0 and
  `cospan subject . telos/p0-spine` still lists claims as plain one-liners.

## Architecture
Today `src/tui.rs`'s `detail_view(claim, cite_index) -> Vec<String>` renders the
claim detail as plain strings, joined into a `Paragraph`. This step turns it into
styled, markdown-aware lines.

A new module (src/markdown.rs) exposes `render(md: &str) -> Vec<Line<'static>>`
built on `pulldown-cmark`'s event stream: a small state machine tracks the
current styles (heading/strong/emph/code) and list depth, pushing `Span`s into
the current `Line` and flushing on hard breaks / block ends. Code blocks and
inline code use a distinct dim/colored style; headings bold. It owns its strings
(`'static` lines) so the detail can be built without borrowing the claim.

In `src/tui.rs`, `detail_view` becomes `claim_detail(claim, cite_index) ->
Vec<Line<'static>>`: it emits the header field lines, then walks the body,
alternating prose (via `markdown::render`) and fenced blocks. For each block it
extracts the fence name and body with `substrate::extract_fenced`-style scanning,
tries `block_summary(name, parsed_json)` for a human view, and otherwise renders
a `` name `` label plus the block content styled as code. `block_summary` lives
in `src/substrate.rs` beside the day-block parsers it reuses (`parse_atom` shapes,
`str_array_at`, `flatten_witnesses`). `draw_claim_detail` renders the
`Vec<Line>` as a `Paragraph` with `Wrap { trim: false }` and the existing scroll
offset; `detail_line_count` returns the line count. The header block title keeps
its `kind_style` tint.

Nothing new is read from kan and nothing written; the detail stays a projection of
the one claim plus the cite index. `plain_frame` and the `subject` CLI are
untouched (REQ-6) — markdown is an interactive affordance, and the plain paths
stay grep-friendly.

## Resolved Questions
- RQ-1: Markdown is parsed with `pulldown-cmark` (already vendored) rather than a
  hand-rolled parser, and rendered to styled `ratatui` lines — headers/emphasis/
  code/lists — with pane-width wrapping.
- RQ-2: Supported `day-*`/`cospan-comment` blocks get a human summary; every other
  fenced block is shown as labeled code, never hidden or dumped as raw JSON in
  prose flow.

## Open Questions

_None outstanding — the two choices above were resolved during design._

## Out of Scope
- Syntax highlighting inside code blocks (tree-sitter); code is one flat style.
- Rendering markdown tables/images/links as anything richer than their text.
- Markdown in the claim *list* rows or in `--once`; those stay one-line plain.
- Interpreting a declared `schema/blocks` project block type; only the built-in
  `day-*` and `cospan-comment` fences get a human view here.
