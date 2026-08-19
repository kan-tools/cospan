//! L3 — the interactive view layer.
//!
//! Step 2 of the P0 arc: the `watch-repo` dashboard, drawn with ratatui instead
//! of ANSI escapes, driven by one poll-and-fold loop. Everything rendered is a
//! projection of the folded `substrate::Dashboard` — the only state this layer
//! owns is the selection cursor (`telos/kan-is-truth`). The loop never
//! subscribes: a single `event::poll(tick)` is both the key wait and the re-fold
//! tick, and the substrate is re-collected only when `.kan/log/HEAD` changes
//! (`telos/poll-dont-subscribe`).

use crate::substrate::{self, Claim, Dashboard};
use std::collections::HashMap;
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
    /// Per-subject fold outcome, keyed by subject name; cleared on re-fold.
    pub claims: HashMap<String, Result<Vec<Claim>, String>>,
    /// Which pane is shown in the narrow layout.
    pub pane: Pane,
}

/// Which pane a narrow terminal is showing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pane {
    List,
    Detail,
}

/// The responsive layout mode for a given terminal width.
#[derive(PartialEq, Eq, Debug)]
pub enum Fit {
    Wide,
    Narrow,
}

/// Below this width only one pane fits; at or above it, both render side by side.
pub const WIDE_COLS: u16 = 100;

/// Pure breakpoint decision, so the responsive rule is testable.
pub fn layout_mode(width: u16) -> Fit {
    if width >= WIDE_COLS {
        Fit::Wide
    } else {
        Fit::Narrow
    }
}

impl AppState {
    pub fn new(repo: PathBuf, dash: Dashboard, last_mtime: Option<SystemTime>) -> Self {
        let mut s = AppState {
            repo,
            dash,
            rows: Vec::new(),
            selected: 0,
            last_mtime,
            claims: HashMap::new(),
            pane: Pane::List,
        };
        s.rebuild_rows();
        s.selected = s.first_subject_index().unwrap_or(0);
        s
    }

    pub fn open_detail(&mut self) {
        self.pane = Pane::Detail;
    }

    pub fn back_to_list(&mut self) {
        self.pane = Pane::List;
    }

    /// Load the selected subject's claims into the cache if not already present.
    /// Parameterized over the fetch so the cache behavior is testable without
    /// shelling out; the loop passes `substrate::subject_claims`.
    pub fn ensure_selected_loaded<F>(&mut self, fetch: F)
    where
        F: Fn(&Path, &str) -> Result<Vec<Claim>, String>,
    {
        if let Some(name) = self.selected_subject().map(str::to_string) {
            if !self.claims.contains_key(&name) {
                let outcome = fetch(&self.repo, &name);
                self.claims.insert(name, outcome);
            }
        }
    }

    /// The cached fold outcome for the selected subject, if it has been loaded.
    pub fn selected_claims(&self) -> Option<&Result<Vec<Claim>, String>> {
        self.selected_subject().and_then(|n| self.claims.get(n))
    }

    /// Rebuild the grouped row list from the current dashboard: namespace headers
    /// in `namespace_counts` order, each followed by its subjects.
    fn rebuild_rows(&mut self) {
        let mut rows = Vec::new();
        for (ns, _count) in self.dash.namespace_counts() {
            let members: Vec<&str> = self
                .dash
                .subjects
                .iter()
                .filter(|s| s.namespace() == ns)
                .map(|s| s.name.as_str())
                .collect();
            // A bare subject (no `/`) is its own namespace; don't render a header
            // line identical to the single subject beneath it.
            let bare_solo = members.len() == 1 && members[0] == ns;
            if !bare_solo {
                rows.push(Row::Header(ns.clone()));
            }
            for name in members {
                rows.push(Row::Subject(name.to_string()));
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
        self.claims.clear(); // the log changed — stale claim detail must not persist
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

/// The detail pane's lines for a subject, from its cached fold outcome. Each
/// state is explicit — loading, error, empty, or the newest-first claim lines —
/// so the pane is never blank or fabricated (`telos/honest-ambiguity`).
pub fn detail_lines(subject: &str, claims: Option<&Result<Vec<Claim>, String>>) -> Vec<String> {
    match claims {
        None => vec!["(loading …)".to_string()],
        Some(Err(e)) => vec![format!("error: {e}")],
        Some(Ok(cs)) if cs.is_empty() => {
            vec![format!("{subject}: (no live claims — unused, or all claims retracted)")]
        }
        Some(Ok(cs)) => cs.iter().map(Claim::display_line).collect(),
    }
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
    state.ensure_selected_loaded(substrate::subject_claims);
    let tick = Duration::from_millis(250);

    let result = loop {
        // Poll-and-fold: re-collect only when HEAD's mtime changed.
        let now = head_mtime(&state.repo);
        if should_refold(state.last_mtime, now) {
            let fresh = substrate::collect(&state.repo);
            state.refold(fresh, now);
            state.ensure_selected_loaded(substrate::subject_claims);
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
                    KeyCode::Char('j') | KeyCode::Down => {
                        state.select_next();
                        state.ensure_selected_loaded(substrate::subject_claims);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        state.select_prev();
                        state.ensure_selected_loaded(substrate::subject_claims);
                    }
                    KeyCode::Enter => state.open_detail(),
                    KeyCode::Esc => state.back_to_list(),
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
    use ratatui::style::Stylize;
    use ratatui::text::Line;
    use ratatui::widgets::{Block, Paragraph};

    let area = frame.area();
    let [header, process, body] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Max(8),
        Constraint::Min(3),
    ])
    .areas(area);

    frame.render_widget(
        Line::from(format!(
            "cospan · {}  ·  j/k move · Enter detail · Esc back · q quit",
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

    // Responsive body: both panes when wide, one at a time when narrow.
    match layout_mode(area.width) {
        Fit::Wide => {
            let [left, right] =
                Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
                    .areas(body);
            draw_list(frame, state, left);
            draw_detail(frame, state, right);
        }
        Fit::Narrow => match state.pane {
            Pane::List => draw_list(frame, state, body),
            Pane::Detail => draw_detail(frame, state, body),
        },
    }
}

fn draw_list(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    use ratatui::style::{Modifier, Style, Stylize};
    use ratatui::text::Line;
    use ratatui::widgets::{Block, List, ListItem, ListState};

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
            .block(Block::bordered().title(format!(" subjects · {} ", state.dash.subjects.len())))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> "),
        area,
        &mut list_state,
    );
}

fn draw_detail(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    use ratatui::widgets::{Block, Paragraph};

    let (title, lines) = match state.selected_subject() {
        Some(subject) => (
            subject.to_string(),
            detail_lines(subject, state.selected_claims()),
        ),
        // No subject at all (empty repo): don't imply a load is in flight.
        None => (
            "(no subject)".to_string(),
            vec!["(no subject selected)".to_string()],
        ),
    };
    frame.render_widget(
        Paragraph::new(lines.join("\n")).block(Block::bordered().title(format!(" {title} "))),
        area,
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
    fn bare_subject_renders_without_a_redundant_header() {
        // `release` has no namespace prefix, so its namespace is the whole name;
        // it renders as one Subject row with no identical header above it.
        let a = app(&["release", "telos/a"]);
        assert_eq!(
            a.rows,
            vec![
                Row::Subject("release".into()),
                Row::Header("telos".into()),
                Row::Subject("telos/a".into()),
            ]
        );
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

    fn mk_claim(kind: &str, text: &str) -> Claim {
        Claim {
            cid: "bafy".into(),
            kind: kind.into(),
            subject: "telos/a".into(),
            author: "did:key:zABCDEFGH".into(),
            recorded_at: Some(0),
            text: Some(text.into()),
            title: None,
        }
    }

    #[test]
    fn claims_fetched_once_per_fold_then_cache_hit_until_refold() {
        use std::cell::Cell;
        let calls = Cell::new(0);
        let fetch = |_: &Path, _: &str| {
            calls.set(calls.get() + 1);
            Ok(Vec::<Claim>::new())
        };

        let mut a = app(&["telos/a", "telos/b"]);
        a.ensure_selected_loaded(fetch); // telos/a fetched
        assert_eq!(calls.get(), 1);
        a.ensure_selected_loaded(fetch); // cache hit
        assert_eq!(calls.get(), 1);
        a.select_next(); // telos/b
        a.ensure_selected_loaded(fetch);
        assert_eq!(calls.get(), 2);
        a.select_prev(); // back to telos/a — cache hit
        a.ensure_selected_loaded(fetch);
        assert_eq!(calls.get(), 2);
        // Re-fold clears the cache: telos/a is fetched again.
        a.refold(dash(&["telos/a", "telos/b"]), None);
        a.ensure_selected_loaded(fetch);
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn layout_mode_breaks_at_100_and_pane_toggles() {
        assert_eq!(layout_mode(100), Fit::Wide);
        assert_eq!(layout_mode(200), Fit::Wide);
        assert_eq!(layout_mode(99), Fit::Narrow);

        let mut a = app(&["telos/a"]);
        assert_eq!(a.pane, Pane::List);
        a.open_detail();
        assert_eq!(a.pane, Pane::Detail);
        a.back_to_list();
        assert_eq!(a.pane, Pane::List);
    }

    #[test]
    fn detail_lines_render_loading_error_empty_and_claims() {
        assert_eq!(detail_lines("telos/a", None), vec!["(loading …)"]);

        let err: Result<Vec<Claim>, String> = Err("boom".into());
        assert_eq!(detail_lines("telos/a", Some(&err)), vec!["error: boom"]);

        let empty: Result<Vec<Claim>, String> = Ok(vec![]);
        let out = detail_lines("telos/a", Some(&empty));
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("no live claims"), "{out:?}");

        let populated: Result<Vec<Claim>, String> = Ok(vec![mk_claim("Decision", "hello")]);
        let out = detail_lines("telos/a", Some(&populated));
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("Decision") && out[0].contains("hello"), "{out:?}");
    }
}
