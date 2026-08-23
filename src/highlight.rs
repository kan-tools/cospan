//! Syntax highlighting for the Comments file view (S3).
//!
//! Turns a file's content into per-line styled runs via `syntect` (pure-Rust
//! fancy-regex, no C `onig`). The result is memoized on `(content hash, ext)` so
//! the 4×/second redraw re-highlights only when the file actually changes, and an
//! unknown extension degrades to one unstyled run per line rather than erroring
//! (`telos/honest-ambiguity`: no guessing a grammar we do not have).

use ratatui::style::{Color, Style};
use std::sync::{Mutex, OnceLock};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

/// One line of highlighted source: styled text runs, left to right.
pub type StyledLine = Vec<(Style, String)>;

struct Hl {
    ss: SyntaxSet,
    theme: Theme,
    memo: Mutex<Memo>,
}

#[derive(Default)]
struct Memo {
    key: Option<(u64, String)>,
    lines: Vec<StyledLine>,
}

fn hl() -> &'static Hl {
    static H: OnceLock<Hl> = OnceLock::new();
    H.get_or_init(|| {
        // two-face bundles bat's large syntax set (~150 languages incl. Lean),
        // a superset of syntect's defaults, so md/tex/rs still resolve.
        let ss = two_face::syntax::extra_newlines();
        let ts = ThemeSet::load_defaults();
        // A mid-contrast dark theme that reads on the default terminal background.
        let theme = ts.themes["base16-ocean.dark"].clone();
        Hl {
            ss,
            theme,
            memo: Mutex::new(Memo::default()),
        }
    })
}

/// Per-line styled runs for `content`, choosing a grammar by file extension
/// `ext` (e.g. `"rs"`, `"md"`, `"tex"`). Memoized on `(content hash, ext)`.
/// An unknown extension yields one unstyled run per line.
pub fn styled(content: &str, ext: &str) -> Vec<StyledLine> {
    let h = hl();
    let key = (crate::comments::content_hash(content), ext.to_string());
    if let Ok(m) = h.memo.lock() {
        if m.key.as_ref() == Some(&key) {
            return m.lines.clone();
        }
    }
    let lines = match h.ss.find_syntax_by_extension(ext) {
        Some(syntax) => {
            let truecolor = truecolor();
            let mut liner = HighlightLines::new(syntax, &h.theme);
            LinesWithEndings::from(content)
                .map(|line| match liner.highlight_line(line, &h.ss) {
                    Ok(runs) => runs
                        .into_iter()
                        .map(|(st, text)| {
                            (
                                conv(st, truecolor),
                                text.trim_end_matches(['\r', '\n']).to_string(),
                            )
                        })
                        .collect(),
                    Err(_) => vec![(
                        Style::default(),
                        line.trim_end_matches(['\r', '\n']).to_string(),
                    )],
                })
                .collect()
        }
        None => plain(content),
    };
    if let Ok(mut m) = h.memo.lock() {
        m.key = Some(key);
        m.lines = lines.clone();
    }
    lines
}

/// One unstyled run per line — the fallback, and what a bare content pane wants.
pub fn plain(content: &str) -> Vec<StyledLine> {
    content
        .lines()
        .map(|l| vec![(Style::default(), l.to_string())])
        .collect()
}

fn conv(st: syntect::highlighting::Style, truecolor: bool) -> Style {
    let c = st.foreground;
    let color = if truecolor {
        Color::Rgb(c.r, c.g, c.b)
    } else {
        Color::Indexed(rgb_to_256(c.r, c.g, c.b))
    };
    Style::default().fg(color)
}

/// True when the terminal advertises 24-bit color; otherwise the palette degrades
/// to the 256-color cube so highlighting is still legible.
fn truecolor() -> bool {
    std::env::var("COLORTERM")
        .map(|v| v.contains("truecolor") || v.contains("24bit"))
        .unwrap_or(false)
}

/// Nearest xterm-256 index for an RGB color (6×6×6 cube plus the gray ramp).
fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return (232 + (r as u16 - 8) * 24 / 247) as u8;
    }
    let cube = |v: u8| -> u16 {
        if v < 48 {
            0
        } else if v < 115 {
            1
        } else {
            (v as u16 - 35) / 40
        }
    };
    (16 + 36 * cube(r) + 6 * cube(g) + cube(b)) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_and_tex_produce_more_than_one_style() {
        // (AC-9) real highlighting for the two flagship formats: a heading/markup
        // sample yields more than one distinct foreground style, not all-plain.
        for (ext, sample) in [
            ("md", "# Title\n\nSome **bold** text and `code`.\n"),
            ("tex", "\\section{Intro}\nText with \\emph{emphasis}.\n"),
        ] {
            let styles: std::collections::HashSet<_> = styled(sample, ext)
                .into_iter()
                .flatten()
                .map(|(st, _)| format!("{:?}", st.fg))
                .collect();
            assert!(
                styles.len() > 1,
                "{ext} highlighted with only {} style(s)",
                styles.len()
            );
        }
    }

    #[test]
    fn unknown_extension_falls_back_to_plain_without_error() {
        // (AC-10) an unknown grammar degrades to one unstyled run per line.
        let out = styled("just text\nno grammar\n", "nonesuch");
        assert_eq!(out.len(), 2);
        for line in &out {
            assert_eq!(line.len(), 1, "a plain line is a single run");
            assert_eq!(line[0].0, Style::default());
        }
        // Text is preserved verbatim (sans line ending).
        assert_eq!(out[0][0].1, "just text");
    }

    #[test]
    fn truecolor_degrades_to_256_when_absent() {
        // (AC-10) with no COLORTERM the palette maps into the 256-color cube.
        let s = conv(
            syntect::highlighting::Style {
                foreground: syntect::highlighting::Color {
                    r: 200,
                    g: 40,
                    b: 40,
                    a: 255,
                },
                background: syntect::highlighting::Color::BLACK,
                font_style: syntect::highlighting::FontStyle::empty(),
            },
            false,
        );
        assert!(
            matches!(s.fg, Some(Color::Indexed(_))),
            "non-truecolor is indexed"
        );
        let t = conv(
            syntect::highlighting::Style {
                foreground: syntect::highlighting::Color {
                    r: 200,
                    g: 40,
                    b: 40,
                    a: 255,
                },
                background: syntect::highlighting::Color::BLACK,
                font_style: syntect::highlighting::FontStyle::empty(),
            },
            true,
        );
        assert!(
            matches!(t.fg, Some(Color::Rgb(200, 40, 40))),
            "truecolor is rgb"
        );
    }

    #[test]
    fn memo_returns_same_result_across_calls() {
        let c = "let x = 1;\n";
        assert_eq!(styled(c, "rs"), styled(c, "rs"));
    }

    #[test]
    fn broadened_language_set_covers_lean_and_friends() {
        // two-face lifts the grammar set from syntect's ~40 defaults to ~150. Lean
        // (the operator's language) and a few other non-default languages now
        // highlight instead of falling back to plain.
        for (ext, sample) in [
            (
                "lean",
                "theorem foo : 1 + 1 = 2 := by rfl\n-- a comment\n#check Nat\n",
            ),
            ("go", "package main\nfunc main() { println(\"hi\") }\n"),
            ("swift", "let x = 1 // a comment\nprint(x)\n"),
        ] {
            let styles: std::collections::HashSet<_> = styled(sample, ext)
                .into_iter()
                .flatten()
                .map(|(st, _)| format!("{:?}", st.fg))
                .collect();
            assert!(
                styles.len() > 1,
                "{ext} did not highlight (only {} style(s)) — grammar missing?",
                styles.len()
            );
        }
    }
}
