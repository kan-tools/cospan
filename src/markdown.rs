//! A small markdown-to-ratatui renderer for claim bodies.
//!
//! Parses with `pulldown-cmark` and emits styled, owned (`'static`) lines:
//! headings bold, `**strong**` bold, `*emphasis*` italic, inline and fenced code
//! in a distinct style, and list items with a bullet/number prefix. It is
//! deliberately small — enough to make a claim body readable, not a full CommonMark
//! renderer.

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use serde_json::{json, Value};

/// The style for inline and fenced code.
pub fn code_style() -> Style {
    Style::new().fg(Color::Yellow)
}

/// The current inline text style from the active emphasis flags.
fn text_style(heading: bool, strong: bool, emph: bool) -> Style {
    let mut s = Style::new();
    if heading || strong {
        s = s.add_modifier(Modifier::BOLD);
    }
    if emph {
        s = s.add_modifier(Modifier::ITALIC);
    }
    s
}

/// Render markdown to styled lines.
pub fn render(md: &str) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut cur: Vec<Span<'static>> = Vec::new();
    let mut in_code = false;
    // Emphasis flags, tracked so nested/overlapping styles compose instead of one
    // ending clobbering another (e.g. **x** inside a heading keeps the heading bold).
    let mut heading = false;
    let mut strong = false;
    let mut emph = false;
    let mut lists: Vec<Option<u64>> = Vec::new();

    fn flush(lines: &mut Vec<Line<'static>>, cur: &mut Vec<Span<'static>>) {
        lines.push(Line::from(std::mem::take(cur)));
    }

    for ev in Parser::new(md) {
        let style = text_style(heading, strong, emph);
        match ev {
            Event::Start(Tag::Heading { .. }) => heading = true,
            Event::End(TagEnd::Heading(_)) => {
                heading = false;
                flush(&mut lines, &mut cur);
                lines.push(Line::from(""));
            }
            Event::Start(Tag::Strong) => strong = true,
            Event::End(TagEnd::Strong) => strong = false,
            Event::Start(Tag::Emphasis) => emph = true,
            Event::End(TagEnd::Emphasis) => emph = false,
            Event::End(TagEnd::Paragraph) => {
                flush(&mut lines, &mut cur);
                lines.push(Line::from(""));
            }
            Event::Start(Tag::CodeBlock(_)) => in_code = true,
            Event::End(TagEnd::CodeBlock) => {
                if !cur.is_empty() {
                    flush(&mut lines, &mut cur);
                }
                in_code = false;
                lines.push(Line::from(""));
            }
            Event::Start(Tag::List(start)) => lists.push(start),
            Event::End(TagEnd::List(_)) => {
                lists.pop();
            }
            Event::Start(Tag::Item) => {
                let indent = "  ".repeat(lists.len().saturating_sub(1));
                let bullet = match lists.last_mut() {
                    Some(Some(n)) => {
                        let s = format!("{indent}{n}. ");
                        *n += 1;
                        s
                    }
                    _ => format!("{indent}• "),
                };
                cur.push(Span::raw(bullet));
            }
            Event::End(TagEnd::Item) => flush(&mut lines, &mut cur),
            Event::Text(t) => {
                if in_code {
                    // A code block arrives as text with embedded newlines; each
                    // becomes its own styled line.
                    for (i, l) in t.split('\n').enumerate() {
                        if i > 0 {
                            flush(&mut lines, &mut cur);
                        }
                        cur.push(Span::styled(l.to_string(), code_style()));
                    }
                } else {
                    cur.push(Span::styled(t.to_string(), style));
                }
            }
            Event::Code(c) => cur.push(Span::styled(c.to_string(), code_style())),
            // pulldown-cmark classifies any `<tag>`-shaped token as HTML. Claim
            // bodies are full of `Vec<Line>` / `Option<i64>` / `<slug>`; render the
            // raw text rather than dropping it (telos/honest-ambiguity — nothing
            // is silently truncated).
            Event::InlineHtml(h) => cur.push(Span::styled(h.to_string(), style)),
            Event::Html(h) => {
                // pulldown-cmark emits an HTML block as one `Event::Html` per line,
                // each ending in `\n`. Treat every `\n` as a line break — including
                // the trailing one — so consecutive events do not collapse onto one
                // line (each `\n` flushes; an empty segment adds no span).
                let mut first = true;
                for seg in h.split('\n') {
                    if !first {
                        flush(&mut lines, &mut cur);
                    }
                    first = false;
                    if !seg.is_empty() {
                        cur.push(Span::styled(seg.to_string(), style));
                    }
                }
            }
            Event::SoftBreak => cur.push(Span::raw(" ")),
            Event::HardBreak => flush(&mut lines, &mut cur),
            Event::Rule => {
                flush(&mut lines, &mut cur);
                lines.push(Line::from("—".repeat(20)));
            }
            _ => {}
        }
    }
    if !cur.is_empty() {
        flush(&mut lines, &mut cur);
    }
    // Trim trailing blank lines.
    while lines
        .last()
        .is_some_and(|l| l.spans.iter().all(|s| s.content.trim().is_empty()))
    {
        lines.pop();
    }
    lines
}

/// A fenced-block info string (`rust`, `python,ignore`) normalized to the file
/// extension `highlight::styled_web` understands. Only the leading token is used;
/// an unknown or empty language falls through to itself, which `styled_web`
/// degrades to plain (un-highlighted) runs — honest ambiguity, not a guess.
fn lang_to_ext(info: &str) -> String {
    let lang = info
        .split([' ', '\t', ','])
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    match lang.as_str() {
        "rust" => "rs",
        "python" => "py",
        "javascript" | "node" => "js",
        "typescript" => "ts",
        "shell" | "sh" | "bash" | "zsh" | "console" => "sh",
        "markdown" => "md",
        "yaml" => "yml",
        "" => "",
        other => other,
    }
    .to_string()
}

/// A link URL, sanitized to an `http`/`https` scheme. Any other scheme
/// (`javascript:`, `data:`, `file:`, a bare fragment) yields `None`, so the span
/// renders as plain text and the client never sees an unsafe `href` (REQ-7).
fn sanitize_href(url: &str) -> Option<String> {
    let u = url.trim();
    let lower = u.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        Some(u.to_string())
    } else {
        None
    }
}

/// One inline run: text plus the emphasis flags and optional link `href` that
/// apply to it. Emitted as `{ text, strong?, em?, code?, href? }` — flags only
/// present when true, so a plain run is just `{ text }`.
fn span_json(text: &str, strong: bool, em: bool, code: bool, href: &Option<String>) -> Value {
    let mut o = serde_json::Map::new();
    o.insert("text".into(), json!(text));
    if strong {
        o.insert("strong".into(), json!(true));
    }
    if em {
        o.insert("em".into(), json!(true));
    }
    if code {
        o.insert("code".into(), json!(true));
    }
    if let Some(h) = href {
        o.insert("href".into(), json!(h));
    }
    Value::Object(o)
}

/// Render markdown to a web block stream: an ordered list of block objects the
/// Chat pane renders into DOM nodes without any client-side parser (REQ-1). Block
/// shapes: `{t:"heading",level,spans}`, `{t:"para",spans}`,
/// `{t:"list",items:[{depth,ordered,num?,spans}]}`,
/// `{t:"code",lang,lines:[[{t,c}]]}` (fenced code highlighted via
/// `highlight::styled_web`), and `{t:"rule"}`. Inline spans carry emphasis flags
/// and sanitized link hrefs as data. This is the web counterpart to `render`,
/// which emits ratatui lines for the TUI; both walk the same parser stream.
pub fn render_web(md: &str) -> Vec<Value> {
    let mut blocks: Vec<Value> = Vec::new();
    let mut spans: Vec<Value> = Vec::new();
    let mut strong = false;
    let mut emph = false;
    let mut href: Option<String> = None;

    // Nesting counters for lists: `Some(n)` is an ordered list's running number,
    // `None` a bullet list. `cur_items` accumulates every item (including nested,
    // flattened by `depth`) for the one open top-level list block.
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut cur_items: Vec<Value> = Vec::new();
    // The item currently being built (between Start/End(Item)).
    let mut item_open = false;
    let mut item_depth = 0usize;
    let mut item_ordered = false;
    let mut item_num: Option<u64> = None;

    // Fenced/indented code accumulation; highlighted at End(CodeBlock).
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_text = String::new();

    // A flushable inline run becomes a para only when it is not part of a list
    // item or heading; those flush through their own End events.
    let flush_para = |blocks: &mut Vec<Value>, spans: &mut Vec<Value>| {
        if !spans.is_empty() {
            blocks.push(json!({ "t": "para", "spans": std::mem::take(spans) }));
        }
    };
    let flush_item = |cur_items: &mut Vec<Value>,
                      spans: &mut Vec<Value>,
                      depth: usize,
                      ordered: bool,
                      num: Option<u64>| {
        let mut o = serde_json::Map::new();
        o.insert("depth".into(), json!(depth));
        o.insert("ordered".into(), json!(ordered));
        if let Some(n) = num {
            o.insert("num".into(), json!(n));
        }
        o.insert("spans".into(), Value::Array(std::mem::take(spans)));
        cur_items.push(Value::Object(o));
    };

    for ev in Parser::new(md) {
        match ev {
            Event::Start(Tag::Heading { .. }) => {
                flush_para(&mut blocks, &mut spans);
            }
            Event::End(TagEnd::Heading(level)) => {
                let lvl = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                blocks.push(json!({
                    "t": "heading", "level": lvl, "spans": std::mem::take(&mut spans),
                }));
            }
            Event::Start(Tag::Strong) => strong = true,
            Event::End(TagEnd::Strong) => strong = false,
            Event::Start(Tag::Emphasis) => emph = true,
            Event::End(TagEnd::Emphasis) => emph = false,
            Event::Start(Tag::Link { dest_url, .. }) => href = sanitize_href(&dest_url),
            Event::End(TagEnd::Link) => href = None,
            Event::End(TagEnd::Paragraph) => {
                // A paragraph inside a list item is flushed by End(Item) (or by a
                // nested list starting); only a top-level paragraph flushes here.
                if !item_open {
                    flush_para(&mut blocks, &mut spans);
                }
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                flush_para(&mut blocks, &mut spans);
                in_code = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(info) => info.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                code_text.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code = false;
                let ext = lang_to_ext(&code_lang);
                let lines: Vec<Value> = crate::highlight::styled_web(&code_text, &ext)
                    .into_iter()
                    .map(|runs| {
                        Value::Array(
                            runs.into_iter()
                                .map(|(c, t)| json!({ "t": t, "c": c }))
                                .collect(),
                        )
                    })
                    .collect();
                let lang = lang_to_ext(&code_lang);
                blocks.push(json!({ "t": "code", "lang": lang, "lines": lines }));
            }
            Event::Start(Tag::List(start)) => {
                // A nested list interrupts its parent item: flush the parent's own
                // text now so its inline run precedes the nested items.
                if item_open && !spans.is_empty() {
                    flush_item(
                        &mut cur_items,
                        &mut spans,
                        item_depth,
                        item_ordered,
                        item_num,
                    );
                    item_open = false;
                }
                lists.push(start);
            }
            Event::End(TagEnd::List(_)) => {
                lists.pop();
                if lists.is_empty() && !cur_items.is_empty() {
                    blocks.push(json!({ "t": "list", "items": std::mem::take(&mut cur_items) }));
                }
            }
            Event::Start(Tag::Item) => {
                item_open = true;
                item_depth = lists.len().saturating_sub(1);
                match lists.last_mut() {
                    Some(Some(n)) => {
                        item_ordered = true;
                        item_num = Some(*n);
                        *n += 1;
                    }
                    _ => {
                        item_ordered = false;
                        item_num = None;
                    }
                }
                spans.clear();
            }
            Event::End(TagEnd::Item) => {
                if item_open {
                    flush_item(
                        &mut cur_items,
                        &mut spans,
                        item_depth,
                        item_ordered,
                        item_num,
                    );
                    item_open = false;
                }
            }
            Event::Text(t) => {
                if in_code {
                    code_text.push_str(&t);
                } else {
                    spans.push(span_json(&t, strong, emph, false, &href));
                }
            }
            Event::Code(c) => spans.push(span_json(&c, strong, emph, true, &None)),
            // pulldown-cmark classifies `<tag>`-shaped tokens as HTML; transcript
            // bodies are full of `Vec<Line>` / `Option<i64>` / `<slug>`. Render the
            // raw text rather than dropping it (telos/honest-ambiguity), matching
            // `render`'s handling.
            Event::InlineHtml(h) => spans.push(span_json(&h, strong, emph, false, &href)),
            Event::Html(h) => {
                for seg in h.split('\n') {
                    if !seg.is_empty() {
                        spans.push(span_json(seg, strong, emph, false, &href));
                    }
                }
            }
            // A soft or hard break inside a run becomes a space; the client wraps
            // prose, so a hard line break is not preserved as its own row here.
            Event::SoftBreak | Event::HardBreak => {
                spans.push(span_json(" ", strong, emph, false, &href))
            }
            Event::Rule => {
                flush_para(&mut blocks, &mut spans);
                blocks.push(json!({ "t": "rule" }));
            }
            _ => {}
        }
    }
    flush_para(&mut blocks, &mut spans);
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_of(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn renders_heading_bold_list_and_code() {
        let md = "# Title\n\nsome **bold** words\n\n- one\n- two\n\n```\ncode line\n```";
        let lines = render(md);
        let joined: Vec<String> = lines.iter().map(text_of).collect();
        let all = joined.join("\n");
        assert!(all.contains("Title"));
        assert!(all.contains("bold"));
        assert!(all.contains("• one"), "{all}");
        assert!(all.contains("code line"), "{all}");

        // The code line's style differs from a plain paragraph line's.
        let code_line = lines
            .iter()
            .find(|l| text_of(l).contains("code line"))
            .unwrap();
        let prose_line = lines.iter().find(|l| text_of(l).contains("words")).unwrap();
        assert_ne!(code_line.spans[0].style, prose_line.spans[0].style);
    }

    #[test]
    fn angle_bracket_tokens_in_prose_are_not_dropped() {
        // pulldown-cmark parses `<...>` as inline HTML; these must survive, not
        // vanish (the truncation bug the bundle review blocked on).
        for (input, needle) in [
            ("recorded_at is now Option<i64>", "Option<i64>"),
            ("claim_detail -> Vec<Line>, splitting", "Vec<Line>"),
            (".cospan/comments/<path>.jsonl", "<path>.jsonl"),
            ("record with day review record <subject>", "<subject>"),
        ] {
            let out: String = render(input)
                .iter()
                .map(text_of)
                .collect::<Vec<_>>()
                .join("");
            assert!(
                out.contains(needle),
                "dropped {needle:?} from {input:?} -> {out:?}"
            );
        }
    }

    #[test]
    fn strong_inside_heading_keeps_it_bold() {
        // The heading stays bold after a nested **strong** closes.
        let lines = render("# Title **x** rest");
        let line = &lines[0];
        assert!(line
            .spans
            .iter()
            .all(|s| s.style.add_modifier.contains(Modifier::BOLD)));
    }

    #[test]
    fn block_html_keeps_its_lines_separate() {
        // pulldown-cmark emits an HTML block one Event::Html per line; the lines
        // must not collapse onto one row (they used to).
        let lines = render("<div>\n<p>alpha</p>\n<p>beta</p>\n</div>\n");
        let texts: Vec<String> = lines.iter().map(text_of).collect();
        assert!(
            texts.iter().any(|t| t == "<p>alpha</p>"),
            "alpha not on its own line: {texts:?}"
        );
        assert!(
            texts.iter().any(|t| t == "<p>beta</p>"),
            "beta not on its own line: {texts:?}"
        );
        // And nothing is collapsed into a single mega-line.
        assert!(
            !texts.iter().any(|t| t.contains("alpha</p><p>beta")),
            "lines collapsed: {texts:?}"
        );
    }

    // ---- render_web (the Chat markdown block stream) ----

    fn span_text(sp: &Value) -> &str {
        sp.get("text").and_then(|t| t.as_str()).unwrap_or("")
    }
    fn spans_of(block: &Value) -> &Vec<Value> {
        block.get("spans").and_then(|s| s.as_array()).unwrap()
    }

    #[test]
    fn render_web_emits_typed_blocks_with_flags() {
        let md = "# Title\n\nsome **bold** and *italic* and `code` with Option<i64>\n\n\
                  - a\n- b\n\n1. one\n2. two\n\n```rust\nfn main() {}\n```";
        let blocks = render_web(md);
        let kinds: Vec<&str> = blocks
            .iter()
            .map(|b| b.get("t").and_then(|t| t.as_str()).unwrap())
            .collect();
        assert_eq!(
            kinds,
            vec!["heading", "para", "list", "list", "code"],
            "block order: {kinds:?}"
        );

        // Heading level + text.
        assert_eq!(blocks[0].get("level").and_then(|l| l.as_u64()), Some(1));
        assert!(spans_of(&blocks[0]).iter().any(|s| span_text(s) == "Title"));

        // Inline flags on the right spans, and the angle-bracket token survives.
        let para = spans_of(&blocks[1]);
        assert!(para
            .iter()
            .any(|s| span_text(s) == "bold" && s.get("strong").is_some()));
        assert!(para
            .iter()
            .any(|s| span_text(s) == "italic" && s.get("em").is_some()));
        assert!(para
            .iter()
            .any(|s| span_text(s) == "code" && s.get("code").is_some()));
        // The angle-bracket token survives (pulldown splits `<i64>` into its own
        // inline-HTML span, so check the concatenated text, not one span).
        let joined: String = para.iter().map(span_text).collect();
        assert!(
            joined.contains("Option<i64>"),
            "angle-bracket token dropped: {para:?}"
        );

        // The two lists carry the right `ordered` flag on every item.
        let bullet_items = blocks[2].get("items").and_then(|i| i.as_array()).unwrap();
        assert!(bullet_items
            .iter()
            .all(|it| it.get("ordered") == Some(&json!(false))));
        let ordered_items = blocks[3].get("items").and_then(|i| i.as_array()).unwrap();
        assert!(ordered_items
            .iter()
            .all(|it| it.get("ordered") == Some(&json!(true)) && it.get("num").is_some()));

        // The code block is tagged and non-empty.
        assert_eq!(blocks[4].get("lang").and_then(|l| l.as_str()), Some("rs"));
        assert!(!blocks[4]
            .get("lines")
            .and_then(|l| l.as_array())
            .unwrap()
            .is_empty());
    }

    #[test]
    fn render_web_highlights_known_lang_and_leaves_unknown_plain() {
        // A rust fence is really highlighted: some run carries a non-empty hex.
        let hl = render_web("```rust\nfn main() { let x = 1; }\n```");
        let lines = hl[0].get("lines").and_then(|l| l.as_array()).unwrap();
        let any_color = lines.iter().flat_map(|l| l.as_array().unwrap()).any(|run| {
            run.get("c")
                .and_then(|c| c.as_str())
                .is_some_and(|c| !c.is_empty())
        });
        assert!(any_color, "rust code was not highlighted: {lines:?}");

        // An unknown language degrades to plain runs (empty hex), not an error.
        let plain = render_web("```nonesuch\njust text\nno grammar\n```");
        let plines = plain[0].get("lines").and_then(|l| l.as_array()).unwrap();
        let all_empty = plines
            .iter()
            .flat_map(|l| l.as_array().unwrap())
            .all(|run| {
                run.get("c")
                    .and_then(|c| c.as_str())
                    .is_some_and(str::is_empty)
            });
        assert!(
            all_empty,
            "unknown lang should be un-highlighted: {plines:?}"
        );
    }

    #[test]
    fn render_web_ships_ordered_list_start_offset() {
        // An ordered list starting at 3 must carry num:3, num:4 so the client can
        // render it from 3 rather than 1 (review F1 — the wire contract mdList honors).
        let blocks = render_web("3. three\n4. four");
        let items = blocks[0].get("items").and_then(|i| i.as_array()).unwrap();
        let nums: Vec<u64> = items
            .iter()
            .map(|it| it.get("num").and_then(|n| n.as_u64()).unwrap())
            .collect();
        assert_eq!(
            nums,
            vec![3, 4],
            "ordered-list start offset lost on the wire"
        );
    }

    #[test]
    fn render_web_sanitizes_link_hrefs() {
        let blocks = render_web(
            "see [docs](https://example.com) and [bad](javascript:alert(1)) and [d](data:x)",
        );
        let spans = spans_of(&blocks[0]);
        let href_of = |text: &str| -> Option<String> {
            spans
                .iter()
                .find(|s| span_text(s) == text)
                .and_then(|s| s.get("href"))
                .and_then(|h| h.as_str())
                .map(str::to_string)
        };
        assert_eq!(href_of("docs").as_deref(), Some("https://example.com"));
        // Non-http(s) schemes drop the href entirely — the text still renders.
        assert_eq!(href_of("bad"), None);
        assert_eq!(href_of("d"), None);
        assert!(spans.iter().any(|s| span_text(s) == "bad"));
    }
}
