//! cospan — the comment re-localizer.
//!
//! The load-bearing algorithm of the tool. A human (or agent) pins a comment to
//! some text in a file. Then agents rewrite that file out from under it. We must
//! keep each comment pointing at the *right place* as the underlying file moves —
//! or, when we honestly can't, say so instead of guessing silently.
//!
//! The anchor is a text *fingerprint* (the target text + surrounding context),
//! never a bare line number: line numbers are the first thing an edit invalidates.
//! On each change we re-resolve the fingerprint against the new file contents and
//! classify the result into one of three states:
//!
//!   * `Anchored` — found it, uniquely. High confidence.
//!   * `Drifted` — the text changed, but a close-enough match sits somewhere;
//!     best guess with a confidence score. Render a "moved" marker.
//!   * `Unresolvable` — lost it, or the match is ambiguous. Goes to the list the
//!     human resolves by hand (same idiom `day` uses for an ambiguous process
//!     position).
//!
//! This module is pure `std` and pure function: `relocalize(anchor, new_content)`.
//! The live tool wraps it in the poll-and-refold watch loop and, as an
//! optimisation, diffs against the *last-seen* contents rather than re-searching
//! the whole file — but the fingerprint search below is the source of truth.

pub mod comments;
pub mod substrate;
pub mod tui;

/// A comment's fingerprint into a file: the text it sits on, plus the context
/// that disambiguates it from lookalikes elsewhere in the file.
#[derive(Clone, Debug)]
pub struct Anchor {
    /// The exact line(s) the comment is attached to.
    pub target: String,
    /// A few lines immediately preceding `target` (may be empty at top of file).
    pub before: String,
    /// A few lines immediately following `target` (may be empty at end of file).
    pub after: String,
    /// 0-based line index where `target` last resolved. A hint, not a truth:
    /// used only to break ties between otherwise-equal candidates.
    pub line_hint: usize,
}

impl Anchor {
    /// Build an anchor from a file's contents and the 0-based line to pin to,
    /// capturing `ctx` lines of surrounding context on each side.
    pub fn from_file(content: &str, line: usize, ctx: usize) -> Anchor {
        let lines: Vec<&str> = content.lines().collect();
        let line = line.min(lines.len().saturating_sub(1));
        let before_start = line.saturating_sub(ctx);
        let after_end = (line + 1 + ctx).min(lines.len());
        Anchor {
            target: lines.get(line).copied().unwrap_or("").to_string(),
            before: lines.get(before_start..line).unwrap_or(&[]).join("\n"),
            // `line + 1` can exceed `after_end` on a 0- or 1-line file; a checked
            // slice yields an empty context rather than panicking.
            after: lines.get(line + 1..after_end).unwrap_or(&[]).join("\n"),
            line_hint: line,
        }
    }
}

/// The three honest outcomes of a re-localization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Anchored,
    Drifted,
    Unresolvable,
}

/// Where a comment landed after a file changed.
#[derive(Clone, Debug)]
pub struct Localization {
    pub state: State,
    /// 0-based inclusive `(start, end)` line range in the new file, if resolved.
    pub span: Option<(usize, usize)>,
    /// 0.0..=1.0. 1.0 for a unique exact hit; the similarity score for a drift.
    pub confidence: f64,
}

// Below this, a fuzzy match is not worth showing even as a guess.
const FUZZY_FLOOR: f64 = 0.60;
// Two candidates whose scores are within this margin are "ambiguous" — we refuse
// to guess between them and send the comment to the resolve-by-hand list. Small,
// because a perfect-context hit only edges out a near-miss by a little.
const AMBIGUITY_MARGIN: f64 = 0.05;

/// Re-resolve `anchor` against the current contents of a file.
///
/// This is the whole game. It never mutates anything and never panics on
/// pathological input; the worst case is `Unresolvable`.
pub fn relocalize(anchor: &Anchor, new_content: &str) -> Localization {
    let lines: Vec<&str> = new_content.lines().collect();
    let target_lines: Vec<&str> = anchor.target.lines().collect();
    if target_lines.is_empty() || lines.is_empty() {
        return Localization {
            state: State::Unresolvable,
            span: None,
            confidence: 0.0,
        };
    }
    let win = target_lines.len();

    // --- Pass 1: exact, contiguous matches of the target block. --------------
    let mut exact: Vec<usize> = Vec::new();
    if lines.len() >= win {
        for start in 0..=lines.len() - win {
            if lines[start..start + win] == target_lines[..] {
                exact.push(start);
            }
        }
    }

    match exact.len() {
        // Unique exact hit — the happy path (covers pure line-shift: an agent
        // inserted or deleted lines elsewhere and our block moved wholesale).
        1 => {
            return Localization {
                state: State::Anchored,
                span: Some((exact[0], exact[0] + win - 1)),
                confidence: 1.0,
            };
        }
        // Several identical blocks — disambiguate by surrounding context.
        n if n > 1 => {
            let mut scored: Vec<(usize, f64)> = exact
                .iter()
                .map(|&s| (s, context_score(anchor, &lines, s, win)))
                .collect();
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            let (best_start, best) = scored[0];
            let runner_up = scored[1].1;
            // If context can't tell the copies apart, refuse to guess.
            if best - runner_up < AMBIGUITY_MARGIN {
                return Localization {
                    state: State::Unresolvable,
                    span: None,
                    confidence: best,
                };
            }
            return Localization {
                state: State::Anchored,
                span: Some((best_start, best_start + win - 1)),
                confidence: 0.5 + 0.5 * best, // exact text, context-chosen
            };
        }
        _ => {}
    }

    // --- Pass 2: the target text itself changed. Slide a window and score. ---
    // (In the live tool this is scoped to the changed hunks via a diff against
    // last-seen contents; here we scan the file, which is fine for a prototype.)
    let target_joined = target_lines.join("\n");
    let mut best_start = 0usize;
    let mut best = -1.0f64;
    let mut second = -1.0f64;
    if lines.len() >= win {
        for start in 0..=lines.len() - win {
            let window = lines[start..start + win].join("\n");
            let sim = similarity(&target_joined, &window);
            // Nudge by context so a drift near the old spot beats a coincidence.
            let ctx = context_score(anchor, &lines, start, win);
            let score = 0.8 * sim + 0.2 * ctx;
            if score > best {
                second = best;
                best = score;
                best_start = start;
            } else if score > second {
                second = score;
            }
        }
    }

    if best >= FUZZY_FLOOR && best - second >= AMBIGUITY_MARGIN {
        Localization {
            state: State::Drifted,
            span: Some((best_start, best_start + win - 1)),
            confidence: best,
        }
    } else {
        // Lost it, or too many equally-weak candidates to responsibly pick one.
        Localization {
            state: State::Unresolvable,
            span: None,
            confidence: best.max(0.0),
        }
    }
}

/// How well the file's context around `start` matches the anchor's remembered
/// before/after context. Also gently rewards proximity to the last-known line.
fn context_score(anchor: &Anchor, lines: &[&str], start: usize, win: usize) -> f64 {
    let before_n = anchor.before.lines().count();
    let after_n = anchor.after.lines().count();

    let file_before = {
        let lo = start.saturating_sub(before_n);
        lines[lo..start].join("\n")
    };
    let file_after = {
        let hi = (start + win + after_n).min(lines.len());
        lines[start + win..hi].join("\n")
    };

    let mut parts = 0.0f64;
    let mut score = 0.0f64;
    if before_n > 0 {
        score += similarity(&anchor.before, &file_before);
        parts += 1.0;
    }
    if after_n > 0 {
        score += similarity(&anchor.after, &file_after);
        parts += 1.0;
    }
    let ctx = if parts > 0.0 { score / parts } else { 0.5 };

    // Tiny proximity tiebreak: closer to the last-known line is marginally better.
    let dist = (start as isize - anchor.line_hint as isize).unsigned_abs();
    let proximity = 1.0 / (1.0 + dist as f64);
    0.95 * ctx + 0.05 * proximity
}

/// Normalized similarity in `0.0..=1.0`, `1.0` == identical.
/// `1 - levenshtein / max(len)`, character-based.
fn similarity(a: &str, b: &str) -> f64 {
    if a == b {
        return 1.0;
    }
    let max = a.chars().count().max(b.chars().count());
    if max == 0 {
        return 1.0;
    }
    1.0 - levenshtein(a, b) as f64 / max as f64
}

/// Classic two-row DP Levenshtein over `char`s.
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0usize; b.len() + 1];
    for (i, &ca) in a.iter().enumerate() {
        curr[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIG: &str = "\
fn login(user: &str) -> bool {
    let token = fetch_token(user);
    validate(token)
}";

    #[test]
    fn unique_exact_is_anchored() {
        let a = Anchor::from_file(ORIG, 1, 1); // the `let token` line
        let loc = relocalize(&a, ORIG);
        assert_eq!(loc.state, State::Anchored);
        assert_eq!(loc.span, Some((1, 1)));
        assert_eq!(loc.confidence, 1.0);
    }

    #[test]
    fn pure_line_shift_stays_anchored() {
        // Agent prepends a doc comment: target moves down 2 lines, text unchanged.
        let a = Anchor::from_file(ORIG, 1, 1);
        let shifted = format!("// docs\n// more docs\n{ORIG}");
        let loc = relocalize(&a, &shifted);
        assert_eq!(loc.state, State::Anchored);
        assert_eq!(loc.span, Some((3, 3)));
        assert_eq!(loc.confidence, 1.0);
    }

    #[test]
    fn edited_target_drifts_with_confidence() {
        // Agent renames the local: exact match gone, but it's clearly the line.
        let a = Anchor::from_file(ORIG, 1, 1);
        let edited = ORIG.replace(
            "let token = fetch_token(user);",
            "let token = fetch_token_cached(user);",
        );
        let loc = relocalize(&a, &edited);
        assert_eq!(loc.state, State::Drifted);
        assert_eq!(loc.span, Some((1, 1)));
        assert!(
            loc.confidence > FUZZY_FLOOR && loc.confidence < 1.0,
            "confidence was {}",
            loc.confidence
        );
    }

    #[test]
    fn deleted_target_is_unresolvable() {
        let a = Anchor::from_file(ORIG, 1, 1);
        let gone = "fn unrelated() {\n    println!(\"hello\");\n}";
        let loc = relocalize(&a, gone);
        assert_eq!(loc.state, State::Unresolvable);
    }

    #[test]
    fn duplicate_blocks_disambiguated_by_context() {
        // Two identical `    return None;` lines; context picks the right one.
        let file = "\
fn a() -> Option<u8> {
    let x = compute_a();
    return None;
}
fn b() -> Option<u8> {
    let y = compute_b();
    return None;
}";
        let a = Anchor::from_file(file, 6, 1); // the second `return None;`
        let loc = relocalize(&a, file);
        assert_eq!(loc.state, State::Anchored);
        assert_eq!(loc.span, Some((6, 6)));
    }

    #[test]
    fn indistinguishable_duplicates_are_unresolvable() {
        // Same target, same context on both sides — genuinely can't choose.
        let file = "    return None;\n    return None;";
        let a = Anchor {
            target: "    return None;".into(),
            before: String::new(),
            after: String::new(),
            line_hint: 0,
        };
        let loc = relocalize(&a, file);
        assert_eq!(loc.state, State::Unresolvable);
    }
}
