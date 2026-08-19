//! A small markdown-to-ratatui renderer for claim bodies.
//!
//! Parses with `pulldown-cmark` and emits styled, owned (`'static`) lines:
//! headings bold, `**strong**` bold, `*emphasis*` italic, inline and fenced code
//! in a distinct style, and list items with a bullet/number prefix. It is
//! deliberately small — enough to make a claim body readable, not a full CommonMark
//! renderer.

use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

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
                for (i, l) in h.trim_end_matches('\n').split('\n').enumerate() {
                    if i > 0 {
                        flush(&mut lines, &mut cur);
                    }
                    cur.push(Span::styled(l.to_string(), style));
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
}
