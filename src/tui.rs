//! L3 — the interactive view layer.
//!
//! Step 2 of the P0 arc: the `watch-repo` dashboard, drawn with ratatui instead
//! of ANSI escapes, driven by one poll-and-fold loop. Everything rendered is a
//! projection of the folded `substrate::Dashboard` — the only state this layer
//! owns is the selection cursor (`telos/kan-is-truth`). The loop never
//! subscribes: a single `event::poll(tick)` is both the key wait and the re-fold
//! tick, and the substrate is re-collected only when `.kan/log/HEAD` changes
//! (`telos/poll-dont-subscribe`).

use crate::substrate::{self, Dashboard};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// One line in the CLAIMS section: a namespace header, or a selectable subject.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Row {
    Header(String),
    Subject(String),
}

/// The interactive dashboard's state: the current fold plus a selection over the
/// repo's subjects. Row-building, navigation, and the re-fold gate are all pure
/// so they can be unit-tested without a terminal.
pub struct AppState {
    pub repo: PathBuf,
    pub dash: Dashboard,
    pub rows: Vec<Row>,
    /// Index into `rows`; always points at a `Row::Subject` when any subject exists.
    pub selected: usize,
    pub last_mtime: Option<SystemTime>,
}

impl AppState {
    pub fn new(repo: PathBuf, dash: Dashboard, last_mtime: Option<SystemTime>) -> Self {
        let mut s = AppState {
            repo,
            dash,
            rows: Vec::new(),
            selected: 0,
            last_mtime,
        };
        s.rebuild_rows();
        s.selected = s.first_subject_index().unwrap_or(0);
        s
    }

    /// Rebuild the grouped row list from the current dashboard: namespace headers
    /// in `namespace_counts` order, each followed by its subjects.
    fn rebuild_rows(&mut self) {
        let mut rows = Vec::new();
        for (ns, _count) in self.dash.namespace_counts() {
            rows.push(Row::Header(ns.clone()));
            for s in &self.dash.subjects {
                if s.namespace() == ns {
                    rows.push(Row::Subject(s.name.clone()));
                }
            }
        }
        self.rows = rows;
    }

    /// Indices of the selectable (subject) rows, in order.
    fn subject_indices(&self) -> Vec<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter(|(_, r)| matches!(r, Row::Subject(_)))
            .map(|(i, _)| i)
            .collect()
    }

    fn first_subject_index(&self) -> Option<usize> {
        self.rows.iter().position(|r| matches!(r, Row::Subject(_)))
    }

    /// The selection's ordinal among subject rows (0 = first subject), if any.
    fn selected_ordinal(&self) -> Option<usize> {
        self.subject_indices().iter().position(|&i| i == self.selected)
    }

    /// The name of the currently selected subject, if any.
    pub fn selected_subject(&self) -> Option<&str> {
        match self.rows.get(self.selected) {
            Some(Row::Subject(name)) => Some(name.as_str()),
            _ => None,
        }
    }

    /// Move the selection to the next subject row, clamped at the last.
    pub fn select_next(&mut self) {
        let idx = self.subject_indices();
        if let Some(pos) = idx.iter().position(|&i| i == self.selected) {
            if pos + 1 < idx.len() {
                self.selected = idx[pos + 1];
            }
        } else if let Some(&first) = idx.first() {
            self.selected = first;
        }
    }

    /// Move the selection to the previous subject row, clamped at the first.
    pub fn select_prev(&mut self) {
        let idx = self.subject_indices();
        if let Some(pos) = idx.iter().position(|&i| i == self.selected) {
            if pos > 0 {
                self.selected = idx[pos - 1];
            }
        } else if let Some(&first) = idx.first() {
            self.selected = first;
        }
    }

    /// Replace the fold with a fresh one, preserving the selection **by subject
    /// name** so a background edit to the log never jumps the cursor. If the
    /// selected subject is gone, the index clamps into the new range.
    pub fn refold(&mut self, dash: Dashboard, mtime: Option<SystemTime>) {
        let prev = self.selected_subject().map(str::to_string);
        let prev_ordinal = self.selected_ordinal();
        self.dash = dash;
        self.last_mtime = mtime;
        self.rebuild_rows();

        let subjects = self.subject_indices();
        self.selected = prev
            // Same subject still present: keep the cursor on it.
            .and_then(|name| {
                self.rows
                    .iter()
                    .position(|r| matches!(r, Row::Subject(s) if *s == name))
            })
            // Gone: clamp the old ordinal into the new subject list (positional
            // locality), rather than snapping to the top.
            .or_else(|| {
                prev_ordinal.and_then(|o| {
                    subjects
                        .get(o.min(subjects.len().saturating_sub(1)))
                        .copied()
                })
            })
            .or_else(|| subjects.first().copied())
            .unwrap_or(0);
    }
}

/// Clip `text` to `max` display lines, appending an explicit overflow cue when
/// lines are hidden. A fixed-height pane must never silently drop day's
/// candidate list or warnings (`telos/honest-ambiguity`) — the reader is told
/// there is more and where to see it.
pub fn clip_lines(text: &str, max: usize) -> Vec<String> {
    if max == 0 {
        return Vec::new();
    }
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() <= max {
        return lines.into_iter().map(str::to_string).collect();
    }
    let shown = max - 1;
    let hidden = lines.len() - shown;
    let mut out: Vec<String> = lines[..shown].iter().map(|s| s.to_string()).collect();
    out.push(format!("… +{hidden} more · run `day status`"));
    out
}

/// The re-fold gate: true only when the stored modified-time differs from the
/// current one. Pure, so the poll loop's core decision is testable.
pub fn should_refold(last: Option<SystemTime>, current: Option<SystemTime>) -> bool {
    last != current
}

fn head_mtime(repo: &Path) -> Option<SystemTime> {
    std::fs::metadata(repo.join(".kan/log/HEAD"))
        .and_then(|m| m.modified())
        .ok()
}

// --- Plain (non-interactive) render: --once, and the testable content source --

/// Render the dashboard as plain text — the `--once` frame, and the shared
/// content the interactive view mirrors. Scriptable and unit-testable.
pub fn plain_frame(state: &AppState) -> String {
    let rule = "-".repeat(64);
    let mut out = String::new();
    out.push_str(&format!("cospan · {}\n{rule}\n", state.repo.display()));

    out.push_str("PROCESS  (day status)\n");
    match &state.dash.day_status {
        Some(text) if !text.is_empty() => {
            for line in text.lines() {
                out.push_str(&format!("  {line}\n"));
            }
        }
        _ => out.push_str("  (unavailable)\n"),
    }
    out.push_str(&format!("{rule}\n"));

    let sessions = state.dash.sessions();
    out.push_str(&format!(
        "SESSIONS  (agents/handoff · {} live)\n",
        sessions.len()
    ));
    if sessions.is_empty() {
        out.push_str("  (none)\n");
    }
    for s in &sessions {
        let short = s.name.trim_start_matches("agents/handoff/");
        out.push_str(&format!("  · {short}  [{}]\n", s.state));
    }
    out.push_str(&format!("{rule}\n"));

    out.push_str(&format!("CLAIMS  ({} subjects)\n", state.dash.subjects.len()));
    if state.rows.is_empty() {
        out.push_str("  (none)\n");
    }
    for (i, row) in state.rows.iter().enumerate() {
        match row {
            Row::Header(ns) => out.push_str(&format!("  {ns}\n")),
            Row::Subject(name) => {
                let marker = if i == state.selected { ">" } else { " " };
                out.push_str(&format!("  {marker} {name}\n"));
            }
        }
    }
    out
}

// --- Interactive ratatui app -------------------------------------------------

/// Run the interactive TUI over `repo`. Enters the alternate screen + raw mode
/// (with ratatui's panic hook restoring the terminal on a crash) and restores it
/// on exit — `telos/disposable`: a crash never leaves the terminal broken.
pub fn run(repo: PathBuf) -> std::io::Result<()> {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

    if !repo.join(".kan").is_dir() {
        eprintln!("warning: {} has no .kan/ — is this a kan repo?", repo.display());
    }

    let mut terminal = ratatui::init();
    let dash = substrate::collect(&repo);
    let mtime = head_mtime(&repo);
    let mut state = AppState::new(repo, dash, mtime);
    let tick = Duration::from_millis(250);

    let result = loop {
        // Poll-and-fold: re-collect only when HEAD's mtime changed.
        let now = head_mtime(&state.repo);
        if should_refold(state.last_mtime, now) {
            let fresh = substrate::collect(&state.repo);
            state.refold(fresh, now);
        }

        if let Err(e) = terminal.draw(|frame| draw(frame, &state)) {
            break Err(e);
        }

        match event::poll(tick) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break Ok(())
                    }
                    KeyCode::Char('j') | KeyCode::Down => state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => state.select_prev(),
                    _ => {}
                },
                Ok(_) => {}
                Err(e) => break Err(e),
            },
            Ok(false) => {} // tick elapsed with no input — loop re-checks mtime
            Err(e) => break Err(e),
        }
    };

    ratatui::restore();
    result
}

fn draw(frame: &mut ratatui::Frame, state: &AppState) {
    use ratatui::layout::{Constraint, Layout};
    use ratatui::style::{Modifier, Style, Stylize};
    use ratatui::text::Line;
    use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};

    let [header, process, sessions, claims] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Max(9),
        Constraint::Max(6),
        Constraint::Min(3),
    ])
    .areas(frame.area());

    frame.render_widget(
        Line::from(format!(
            "cospan · {}  ·  j/k move · q quit",
            state.repo.display()
        ))
        .bold(),
        header,
    );

    let process_text = match &state.dash.day_status {
        Some(t) if !t.is_empty() => t.clone(),
        _ => "(day status unavailable)".to_string(),
    };
    // Clip to the pane height with an explicit overflow cue; no wrap, so the
    // line count is exact and day's candidate list is never silently truncated.
    let process_lines = clip_lines(&process_text, (process.height as usize).saturating_sub(2));
    frame.render_widget(
        Paragraph::new(process_lines.join("\n"))
            .block(Block::bordered().title(" process (day status) ")),
        process,
    );

    let session_list = state.dash.sessions();
    let session_text = if session_list.is_empty() {
        "(none)".to_string()
    } else {
        session_list
            .iter()
            .map(|s| format!("· {}  [{}]", s.name.trim_start_matches("agents/handoff/"), s.state))
            .collect::<Vec<_>>()
            .join("\n")
    };
    frame.render_widget(
        Paragraph::new(session_text).block(Block::bordered().title(format!(
            " sessions · {} live ",
            session_list.len()
        ))),
        sessions,
    );

    let items: Vec<ListItem> = state
        .rows
        .iter()
        .map(|row| match row {
            Row::Header(ns) => ListItem::new(Line::from(ns.clone()).add_modifier(Modifier::DIM)),
            Row::Subject(name) => ListItem::new(Line::from(format!("  {name}"))),
        })
        .collect();
    let mut list_state = ListState::default();
    if !items.is_empty() {
        list_state.select(Some(state.selected));
    }
    frame.render_stateful_widget(
        List::new(items)
            .block(Block::bordered().title(format!(" claims · {} subjects ", state.dash.subjects.len())))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> "),
        claims,
        &mut list_state,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::substrate::Subject;

    fn subj(name: &str) -> Subject {
        Subject {
            name: name.to_string(),
            state: "open".to_string(),
            durability: "local".to_string(),
        }
    }

    fn dash(names: &[&str]) -> Dashboard {
        Dashboard {
            subjects: names.iter().map(|n| subj(n)).collect(),
            day_status: Some("Current atom: design".to_string()),
            errors: vec![],
        }
    }

    fn app(names: &[&str]) -> AppState {
        AppState::new(PathBuf::from("."), dash(names), None)
    }

    #[test]
    fn rows_are_grouped_by_namespace_with_all_subjects_selectable() {
        // two telos, one atom -> namespace_counts orders telos (2) before atom (1).
        let a = app(&["telos/a", "telos/b", "atom/x"]);
        assert_eq!(
            a.rows,
            vec![
                Row::Header("telos".into()),
                Row::Subject("telos/a".into()),
                Row::Subject("telos/b".into()),
                Row::Header("atom".into()),
                Row::Subject("atom/x".into()),
            ]
        );
        let selectable = a.subject_indices().len();
        assert_eq!(selectable, 3);
        // First subject is selected initially.
        assert_eq!(a.selected_subject(), Some("telos/a"));
    }

    #[test]
    fn jk_moves_one_subject_and_clamps_at_both_ends() {
        let mut a = app(&["telos/a", "telos/b", "atom/x"]);
        a.select_prev(); // already at first -> stays
        assert_eq!(a.selected_subject(), Some("telos/a"));
        a.select_next();
        assert_eq!(a.selected_subject(), Some("telos/b"));
        a.select_next(); // skips the atom header row
        assert_eq!(a.selected_subject(), Some("atom/x"));
        a.select_next(); // at last -> stays
        assert_eq!(a.selected_subject(), Some("atom/x"));
    }

    #[test]
    fn refold_preserves_by_name_then_clamps_to_ordinal_when_gone() {
        let mut a = app(&["telos/a", "telos/b", "atom/x"]);
        a.select_next(); // telos/b (subject ordinal 1)
        assert_eq!(a.selected_subject(), Some("telos/b"));

        // telos/a vanishes but telos/b remains: cursor stays on telos/b by name.
        a.refold(dash(&["telos/b", "atom/x"]), None);
        assert_eq!(a.selected_subject(), Some("telos/b"));

        // Remove the *selected* subject. Using one namespace keeps ordering
        // stable, so the cursor must land on the subject now at the old ordinal
        // (telos/c at ordinal 1) — NOT the first (telos/a). This distinguishes a
        // real clamp from a jump-to-top.
        let mut b = app(&["telos/a", "telos/b", "telos/c"]);
        b.select_next(); // telos/b at ordinal 1
        b.refold(dash(&["telos/a", "telos/c"]), None);
        assert_eq!(b.selected_subject(), Some("telos/c"));
    }

    #[test]
    fn clip_lines_appends_overflow_cue_when_truncated() {
        // Fits within the height: returned unchanged.
        assert_eq!(clip_lines("a\nb\nc", 5), vec!["a", "b", "c"]);
        // Overflows: the last visible line names how many lines are hidden.
        let clipped = clip_lines("a\nb\nc\nd\ne", 3);
        assert_eq!(clipped, vec!["a", "b", "… +3 more · run `day status`"]);
        // Degenerate height renders nothing rather than panicking.
        assert!(clip_lines("a\nb", 0).is_empty());
    }

    #[test]
    fn refold_gate_fires_only_on_mtime_change() {
        let t0 = SystemTime::UNIX_EPOCH;
        let t1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1);
        assert!(should_refold(None, Some(t0)));
        assert!(should_refold(Some(t0), Some(t1)));
        assert!(!should_refold(Some(t0), Some(t0)));
        assert!(!should_refold(None, None));
    }

    #[test]
    fn plain_frame_shows_grouped_subjects_and_day_status_verbatim() {
        let a = app(&["telos/p0-spine", "atom/x"]);
        let out = plain_frame(&a);
        assert!(out.contains("telos\n"), "namespace header missing:\n{out}");
        assert!(out.contains("telos/p0-spine"), "subject missing:\n{out}");
        assert!(out.contains("Current atom: design"), "day status not verbatim:\n{out}");
    }
}
