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

/// Cache key: content hash, extension, and the bucketed line count highlighted
/// (windowing — see `styled_upto`).
type Key = (u64, String, usize);

/// A small LRU of recently highlighted (windows of) files, newest last. Bounded
/// so browsing many files does not grow without limit, but large enough that
/// flipping back to a just-viewed file is instant instead of re-highlighting.
#[derive(Default)]
struct Memo {
    entries: Vec<(Key, Vec<StyledLine>)>,
}

const CACHE_N: usize = 8;

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

/// Per-line styled runs for the whole of `content` (every line highlighted).
/// Choosing a grammar by file extension `ext` (e.g. `"rs"`, `"md"`, `"tex"`); an
/// unknown extension yields one unstyled run per line.
pub fn styled(content: &str, ext: &str) -> Vec<StyledLine> {
    styled_upto(content, ext, usize::MAX)
}

/// Like [`styled`], but only the first `upto` lines are actually run through the
/// grammar; lines past the window are returned plain (unstyled). Highlighting a
/// large file is O(lines) and slow, but only the visible window is ever on
/// screen — so a caller passes `scroll + viewport + margin` and a 6000-line file
/// previews in the cost of a screenful, not the whole file. Every result is
/// memoized in a small LRU keyed by `(content hash, ext, bucketed upto)`, so
/// scrolling a little or flipping back to a recent file is instant.
pub fn styled_upto(content: &str, ext: &str, upto: usize) -> Vec<StyledLine> {
    let h = hl();
    let key = (
        crate::comments::content_hash(content),
        ext.to_string(),
        bucket(upto),
    );
    if let Ok(m) = h.memo.lock() {
        if let Some((_, lines)) = m.entries.iter().find(|(k, _)| *k == key) {
            return lines.clone();
        }
    }
    // Compute outside the lock — highlighting can be slow.
    let lines = compute(h, ext, key.2, content);
    if let Ok(mut m) = h.memo.lock() {
        m.entries.retain(|(k, _)| *k != key); // de-dup, then push as newest
        m.entries.push((key, lines.clone()));
        let n = m.entries.len();
        if n > CACHE_N {
            m.entries.drain(0..n - CACHE_N); // evict oldest
        }
    }
    lines
}

/// Round `upto` up to a 128-line boundary (plus a lookahead bucket) so small
/// scrolls reuse a cached window rather than re-highlighting each tick. A
/// full-file request (`usize::MAX`) maps to its own stable key.
fn bucket(upto: usize) -> usize {
    if upto >= 1_000_000 {
        usize::MAX
    } else {
        (upto / 128 + 2) * 128
    }
}

/// Highlight the first `upto` lines of `content`, plain beyond. Pure; the caller
/// handles caching.
fn compute(h: &Hl, ext: &str, upto: usize, content: &str) -> Vec<StyledLine> {
    let Some(syntax) = h.ss.find_syntax_by_extension(ext) else {
        return plain(content);
    };
    let truecolor = truecolor();
    let mut liner = HighlightLines::new(syntax, &h.theme);
    LinesWithEndings::from(content)
        .enumerate()
        .map(|(i, line)| {
            let text = line.trim_end_matches(['\r', '\n']).to_string();
            // Past the window: skip the (expensive) grammar work entirely.
            if i >= upto {
                return vec![(Style::default(), text)];
            }
            match liner.highlight_line(line, &h.ss) {
                Ok(runs) => runs
                    .into_iter()
                    .map(|(st, t)| {
                        (
                            conv(st, truecolor),
                            t.trim_end_matches(['\r', '\n']).to_string(),
                        )
                    })
                    .collect(),
                Err(_) => vec![(Style::default(), text)],
            }
        })
        .collect()
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

    #[test]
    fn windowing_highlights_only_the_leading_lines() {
        // A long file: with a small window only the leading lines run the grammar;
        // lines past the window are plain, but every line is still present (so
        // gutter markers and comment anchors stay aligned).
        let mut src = String::from("fn a() {}\n");
        for _ in 0..500 {
            src.push_str("let z = 1;\n");
        }
        let out = styled_upto(&src, "rs", 10); // bucket -> first 256 lines
        assert_eq!(out.len(), 501, "line count is preserved");
        assert_eq!(out[500].len(), 1, "a beyond-window line is one plain run");
        assert_eq!(
            out[500][0].0,
            Style::default(),
            "a beyond-window line is unstyled"
        );
        // The full highlight colors that same deep line — proving the window, not
        // the grammar, is what left it plain.
        let full = styled(&src, "rs");
        assert!(
            full[500].iter().any(|(s, _)| *s != Style::default()),
            "the deep line highlights when the whole file is requested"
        );
    }

    #[test]
    fn cache_returns_recent_files_consistently() {
        // Two files both stay cached; revisiting one is stable (the LRU hit path).
        let a = "let x = 1;\n";
        let b = "fn main() {}\n";
        let ra1 = styled_upto(a, "rs", 50);
        let _rb = styled_upto(b, "rs", 50);
        let ra2 = styled_upto(a, "rs", 50);
        assert_eq!(ra1, ra2, "revisiting a recently-seen file is consistent");
    }
}
