# Feature: Chat message handling — server-rendered markdown + highlighted code blocks

## Summary
The Chat transcript renders message bodies as flat plain text
(`turnEl` → `.tbody`/`textContent` at `src/web/index.html:1270`). Render message
turns as markdown — headings, paragraphs, bold/italic, inline code, bullet and
numbered lists, fenced code blocks, and rules — by **pre-rendering on the
server** into a structured block stream that `GET /chat` ships in each event,
mirroring how `highlight::styled_web` already ships colored code runs for the
file viewer. Fenced code blocks are syntax-highlighted through that same
`styled_web`. No client-side markdown library and no CDN (`telos/disposable`).

## Requirements
- REQ-1: **Server emits a block stream for message turns.** A new web markdown
  renderer beside `src/markdown.rs` (which today emits ratatui `Line`s, not
  web-reachable output) parses a message body with `pulldown-cmark` (already a
  dependency, `Cargo.toml:11`) and returns an ordered list of typed blocks:
  `heading` (with `level` 1–6 and inline spans), `para` (inline spans), `list`
  (`ordered` bool, items each carrying `depth` and inline spans), `code` (a
  `lang` tag and highlighted line runs), and `rule`. `chat.rs::event_json`
  (`src/chat.rs:44`) gains a `blocks` field **only** for `EventKind::Message`
  turns; other kinds omit it.
- REQ-2: **Inline spans carry emphasis flags as data.** Each inline span is
  `{ text, strong?, em?, code?, href? }` — the text is content, the emphasis
  flags are booleans, and `href` is present only on a link span. Angle-bracket
  tokens common in transcripts (`Option<i64>`, `<slug>`) are preserved as literal
  text, not dropped as HTML, matching the behavior `src/markdown.rs` already tests
  (`angle_bracket_tokens_in_prose_are_not_dropped`).
- REQ-7: **Links are server-sanitized anchors.** A `pulldown-cmark`
  `Tag::Link` becomes a span with an `href` **only** when the URL's scheme is
  `http` or `https` (an allowlist checked on the server); any other scheme
  (`javascript:`, `data:`, `file:`, …) drops the `href` and the span renders as
  plain text. The client renders an `href` span as
  `<a href target="_blank" rel="noopener noreferrer">`.
- REQ-3: **Fenced code blocks are highlighted via `styled_web`.** A `code`
  block's fence info string (`rust`, `python`, …) is normalized to a file
  extension and passed to `highlight::styled_web` (`src/highlight.rs:156`); its
  `Vec<Vec<(hex, text)>>` output is serialized per run as `{ t, c }` — the exact
  shape the file viewer already consumes (`src/mcp.rs:256-263`, read at
  `src/web/index.html:1111`). An unknown or absent language degrades to plain
  runs (empty hex), which `styled_web` already does for an unknown extension.
- REQ-4: **Client renders blocks into DOM nodes — no parser, no library.** A new
  `mdBlocks(blocks)` in `src/web/index.html` builds a `DocumentFragment` of real
  elements (headings, `.mdp` paragraphs, `ul`/`ol` lists, a `.mdcode` `<pre>` of
  colored spans reusing the file-viewer run pattern, `hr`). Every text value is
  set via `textContent`/`createTextNode` — never `innerHTML` — and code-run
  colors via `span.style.color = run.c`, so untrusted message text is rendered as
  data. No `<script src>` / `<link href>` / CDN is added (`telos/disposable`).
- REQ-5: **Only message turns change; everything else is untouched.** `turnEl`'s
  message branch renders `e.blocks` when present and falls back to the existing
  plain-text `.tbody` when absent (older folds, or a body that produced no
  blocks). The collapsed `thinking`/`toolcall`/`toolresult` turns
  (`COLLAPSES`, `src/web/index.html:1199`) keep their `<pre>`/`textContent`
  one-liner-plus-expand rendering unchanged, and `text` is still shipped for
  every event so that path keeps working.
- REQ-6: **Disposable / reuse.** The renderer reuses `pulldown-cmark` and
  `highlight::styled_web`; parsing correctness lives in Rust with unit tests
  rather than in hand-rolled JS. The only serialization change is the additive
  `blocks` field on message events in `GET /chat`; no route, no fold-shape, and
  no other endpoint changes.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-2) A unit test on the web markdown renderer
  asserts that a body with a heading, a bold+italic+inline-code paragraph, a
  bullet list, a numbered list, and a fenced code block produces blocks of the
  right `t` in order, with `level` on the heading, `strong`/`em`/`code` flags on
  the right spans, `ordered` set correctly per list, and that an angle-bracket
  token (`Option<i64>`) survives as literal span text.
- [ ] AC-2: (covers REQ-3) A unit test asserts a ```rust fenced block yields
  `code` runs with non-empty hex colors (really highlighted) and a fenced block
  with an unknown/absent language yields plain runs (empty hex) — reusing the
  assertions already proven for `styled_web` in `src/highlight.rs`.
- [ ] AC-3: (covers REQ-1, REQ-5) A `chat.rs` unit test asserts `event_json`
  carries `blocks` for an `EventKind::Message` event whose text is markdown, and
  carries **no** `blocks` for a `Thinking`/`ToolResult` event; `text` is present
  on all of them.
- [ ] AC-4: (covers REQ-4, REQ-5, REQ-6) A `server::tests` unit test asserts
  `INDEX_HTML` contains the block renderer and its wiring — an `mdBlocks`
  builder, a `.mdcode` code path reusing `run.c`/`run.t`, the message-turn branch
  consuming `e.blocks` with a plain-text fallback — and that `INDEX_HTML`
  contains no new `<script src`/`<link href` (no dependency added).
- [ ] AC-6: (covers REQ-7) A renderer unit test asserts an `http(s)` link
  produces a span carrying that `href`, and a `javascript:` (and one other
  non-http scheme) link produces a plain span with **no** `href`; a
  `server::tests` assertion confirms `INDEX_HTML` renders an `href` span as an
  `<a>` with `target="_blank"` and `rel="noopener noreferrer"`.
- [ ] AC-5: (covers REQ-1..REQ-7) `cargo test`, `cargo clippy --all-targets --
  -D warnings`, and `cargo fmt --check` are green, and the rendered transcript
  (markdown formatting + highlighted code blocks) is confirmed by an operator
  eyeball (stated, not machine-checked — Chrome automation is declined this
  round).

## Architecture
**Server-side pre-render, client renders dumbly** — the same split the file
viewer already uses for syntax highlighting.

**Web markdown renderer (`src/markdown.rs`, new `render_web`).** `markdown.rs`
already parses with `pulldown-cmark` for the TUI but emits ratatui `Line<'static>`
— not usable from the page. Add a sibling `render_web(md: &str) -> Vec<Block>`
walking the same `Parser` event stream but accumulating typed blocks instead of
styled lines. It reuses the emphasis-flag tracking (`heading`/`strong`/`emph`)
and the inline-HTML-as-text handling already there (so `Vec<Line>` / `<slug>`
tokens survive). Blocks are a small enum:
`Heading{level, spans}`, `Para{spans}`, `List{ordered, items:Vec<Item>}`
(`Item{depth, spans}`), `Code{lang, lines}`, `Rule`; a span is
`{text, strong, em, code}`. `lines` for a `Code` block is
`highlight::styled_web(code, ext)` where `ext` comes from a small
`lang_to_ext` normalizer (`rust`→`rs`, `python`→`py`, `js`/`javascript`→`js`,
`ts`→`ts`, …) falling back to the raw fence token; an unknown token yields plain
runs from `styled_web` itself. The renderer is serialized by the caller (as
`styled_web` is), keeping `markdown.rs`'s ratatui path independent.

**Chat projection (`src/chat.rs`).** `event_json` (`src/chat.rs:44`) adds
`blocks` only when `e.kind == EventKind::Message`, computed by walking the new
`Vec<Block>` into `json!` values; each `Code` block's runs serialize as
`{ t, c }` — the identical shape the file viewer reads (`src/mcp.rs:256`,
`src/web/index.html:1111`). `text` stays on every event (collapsed turns and the
fallback need it). `session_json` and the `/chat` route are unchanged.

**Client (`src/web/index.html`).** Add `mdBlocks(blocks)` returning a
`DocumentFragment`: `heading`→`h3`/`h4` (level capped for the chat pane),
`para`→`.mdp`, `list`→`ul`/`ol` with an `li` per item indented by `depth`,
`code`→a `.mdcode` `<pre>` whose lines are spans with `style.color = run.c`
(the exact `runs.forEach` idiom from `openFileViewer`), `rule`→`hr`. Inline
spans render through a helper that wraps text in `<strong>`/`<em>`/`<code>` per
flag, always via `textContent` — no `innerHTML`; an `href` span becomes an
`<a href target="_blank" rel="noopener noreferrer">` (the server already
guaranteed an `http(s)` scheme, so the client sets the attribute directly). `turnEl`'s message branch
(`src/web/index.html:1265-1271`) becomes: build the `.tbody`, then if `e.blocks`
append `mdBlocks(e.blocks)`, else keep the plain-text `el("div","tbody",e.text)`
fallback. New CSS for `.mdp`, headings, lists, `.mdcode`, inline `code`, and
`hr` goes inline in the existing `<style>` block.

**Invariants.** No route or fold-shape change; append-only log and comment
sidecars are untouched (this is a read-only projection concern). The block
stream is additive on message events, so an older client (plain `text`) and a
newer server interoperate.

## Resolved Questions
- RQ-1: **Parse on the server**, not in inline JS. `highlight::styled_web` +
  the file viewer already establish server-pre-render/client-dumb-render as the
  house pattern, and reusing `pulldown-cmark` (already a dep) puts parsing
  correctness in Rust with tests rather than a hand-rolled JS parser. Cost
  accepted: a Rust change and a wider `/chat` JSON, departing from the round's
  otherwise page-only slices.
- RQ-2: **Fenced code blocks are highlighted via `styled_web`** by their
  language tag, so chat code reads like the file viewer, rather than a plain
  monochrome `<pre>`. Essentially free given the server-side parse.
- RQ-3: **Message turns only.** Thinking/toolcall/toolresult keep their collapsed
  `<pre>` rendering — tool results are frequently raw logs/JSON that markdown
  parsing would mangle, and the handoff scoped the slice to message bodies.
- RQ-4: **Links are rendered as sanitized anchors**, not dropped to plain text.
  A `Tag::Link` becomes an `<a target="_blank" rel="noopener noreferrer">` — but
  only when its scheme is `http`/`https`, checked on the server (an allowlist, so
  the client never sees a `javascript:`/`data:` href); any other scheme falls
  back to plain span text. This keeps chat links useful without opening an href
  surface on untrusted transcript content.

## Out of Scope
- **An inline JS markdown parser or any client-side markdown/highlighting
  library or CDN** — rejected by RQ-1 and `telos/disposable`.
- **Markdown in thinking/tool turns** — RQ-3; those stay collapsed `<pre>`.
- **Blockquotes, tables, images, and task-list checkboxes** — not rendered as
  distinct blocks in this slice; a blockquote/table degrades to its inline text.
  A later slice can add them as new block types without changing the wire shape.
- **A full CommonMark renderer** — the block set is deliberately the subset
  `src/markdown.rs` already covers, plus web serialization; edge cases beyond it
  degrade to text rather than being guaranteed.
- **Block constructs nested inside a list item** (a code block or heading inside
  a `- item`) — these are lifted to top level rather than nested in the `<li>`
  (the flat item model carries inline spans, not sub-blocks); the client skips
  the resulting empty item so no stray `<li>` shows (review F2).
- **Any kan write, route change, or fold-shape change** — the only change to the
  wire is the additive `blocks` field on message events.
