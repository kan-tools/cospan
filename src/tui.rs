//! L3 — the interactive view layer.
//!
//! The `watch-repo` browser, drawn with ratatui and driven by one poll-and-fold
//! loop. Everything rendered is a projection of the `substrate::Fold` — the only
//! state this layer owns is the selection cursor (`telos/kan-is-truth`). The loop
//! never subscribes: a single `event::poll(tick)` is both the key wait and the
//! re-fold tick, and the fold is rebuilt only when `.kan/log/HEAD` changes
//! (`telos/poll-dont-subscribe`).

use crate::substrate::{self, namespace, short_cid, Claim, Fold, ProcessSnapshot};
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
    /// The whole in-memory model, folded from one `kan show --all` per tick.
    pub fold: Fold,
    pub rows: Vec<Row>,
    /// Index into `rows`; always points at a `Row::Subject` when any subject exists.
    pub selected: usize,
    pub last_mtime: Option<SystemTime>,
    /// Which of the three levels currently has focus.
    pub focus: Focus,
    /// Index into the selected subject's claim list (Claims focus).
    pub claim_selected: usize,
    /// Scroll offset into the claim-detail text (Detail focus).
    pub detail_scroll: usize,
    /// The active top-level view.
    pub view: View,
    pub atom_scroll: usize,
    pub telos_scroll: usize,
}

/// The top-level views, switched with `1`/`2`/`3` or `Tab`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
    Browser,
    Atoms,
    Telos,
}

impl View {
    pub fn next(self) -> View {
        match self {
            View::Browser => View::Atoms,
            View::Atoms => View::Telos,
            View::Telos => View::Browser,
        }
    }

    pub fn from_digit(c: char) -> Option<View> {
        match c {
            '1' => Some(View::Browser),
            '2' => Some(View::Atoms),
            '3' => Some(View::Telos),
            _ => None,
        }
    }
}

/// The three navigation levels: the subject list, a subject's claim list, and one
/// claim's full detail. Enter descends, Esc ascends.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Focus {
    Subjects,
    Claims,
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
    pub fn new(repo: PathBuf, fold: Fold, last_mtime: Option<SystemTime>) -> Self {
        let mut s = AppState {
            repo,
            fold,
            rows: Vec::new(),
            selected: 0,
            last_mtime,
            focus: Focus::Subjects,
            claim_selected: 0,
            detail_scroll: 0,
            view: View::Browser,
            atom_scroll: 0,
            telos_scroll: 0,
        };
        s.rebuild_rows();
        s.selected = s.first_subject_index().unwrap_or(0);
        s
    }

    /// Descend one focus level (Subjects → Claims → Detail), clamped at Detail.
    /// Resets the claim selection or the detail scroll for the level entered.
    pub fn descend(&mut self) {
        match self.focus {
            Focus::Subjects => {
                self.focus = Focus::Claims;
                self.claim_selected = 0;
            }
            Focus::Claims => {
                self.focus = Focus::Detail;
                self.detail_scroll = 0;
            }
            Focus::Detail => {}
        }
    }

    /// Ascend one focus level (Detail → Claims → Subjects), clamped at Subjects.
    pub fn ascend(&mut self) {
        self.focus = match self.focus {
            Focus::Detail => Focus::Claims,
            Focus::Claims => Focus::Subjects,
            Focus::Subjects => Focus::Subjects,
        };
    }

    /// Move the selection/scroll down at the focused level.
    pub fn move_down(&mut self) {
        match self.focus {
            Focus::Subjects => self.select_next(),
            Focus::Claims => {
                let n = self.claim_count();
                if n > 0 && self.claim_selected + 1 < n {
                    self.claim_selected += 1;
                }
            }
            Focus::Detail => {
                let max = self.detail_line_count().saturating_sub(1);
                if self.detail_scroll < max {
                    self.detail_scroll += 1;
                }
            }
        }
    }

    /// Move the selection/scroll up at the focused level.
    pub fn move_up(&mut self) {
        match self.focus {
            Focus::Subjects => self.select_prev(),
            Focus::Claims => self.claim_selected = self.claim_selected.saturating_sub(1),
            Focus::Detail => self.detail_scroll = self.detail_scroll.saturating_sub(1),
        }
    }

    fn claim_count(&self) -> usize {
        self.selected_claims().len()
    }

    /// The claim currently selected in the claim list.
    pub fn selected_claim(&self) -> Option<&Claim> {
        self.selected_claims().get(self.claim_selected)
    }

    fn detail_line_count(&self) -> usize {
        match self.selected_claim() {
            Some(c) => detail_view(c, Some(&self.fold.by_cid)).len(),
            None => 0,
        }
    }

    /// The selected subject's claims (newest first) — a direct read from the
    /// fold, no fetch.
    pub fn selected_claims(&self) -> &[Claim] {
        match self.selected_subject() {
            Some(name) => self.fold.claims_for(name),
            None => &[],
        }
    }

    /// Rebuild the grouped row list from the fold: namespace headers in
    /// `namespace_counts` order, each followed by its subjects.
    fn rebuild_rows(&mut self) {
        let mut rows = Vec::new();
        for (ns, _count) in self.fold.namespace_counts() {
            let members: Vec<&str> = self
                .fold
                .subjects
                .iter()
                .filter(|n| namespace(n) == ns)
                .map(String::as_str)
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
        self.subject_indices()
            .iter()
            .position(|&i| i == self.selected)
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
    pub fn refold(&mut self, fold: Fold, mtime: Option<SystemTime>) {
        let prev = self.selected_subject().map(str::to_string);
        let prev_ordinal = self.selected_ordinal();
        self.fold = fold;
        self.last_mtime = mtime;
        self.claim_selected = 0;
        self.detail_scroll = 0;
        self.atom_scroll = 0;
        self.telos_scroll = 0;
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
pub fn detail_lines(subject: &str, claims: &[Claim]) -> Vec<String> {
    if claims.is_empty() {
        vec![format!(
            "{subject}: (no live claims — unused, or all claims retracted)"
        )]
    } else {
        claims.iter().map(Claim::display_line).collect()
    }
}

/// The full-detail lines for a single claim: header fields, artifact anchors, the
/// untruncated text, and each cite resolved through the index to a one-line
/// preview (falling back to the bare short-cid). A pure projection — nothing
/// synthesized (`telos/kan-is-truth`), nothing truncated (`telos/honest-ambiguity`).
pub fn detail_view(claim: &Claim, cite_index: Option<&HashMap<String, Claim>>) -> Vec<String> {
    let mut out = vec![
        format!("{}  {}", claim.kind, claim.cid),
        format!("author {}   {}", claim.short_author(), claim.recorded_utc()),
        format!("subject {}", claim.subject),
    ];
    if !claim.artifacts.is_empty() {
        out.push(format!("anchor {}", claim.artifacts.join(", ")));
    }
    out.push(String::new());

    let body = claim
        .text
        .as_deref()
        .or(claim.title.as_deref())
        .unwrap_or("");
    if body.is_empty() {
        out.push(format!("({})", claim.kind.to_lowercase()));
    } else {
        out.extend(body.lines().map(str::to_string));
    }

    if !claim.cites.is_empty() {
        out.push(String::new());
        out.push(format!("cites ({}):", claim.cites.len()));
        for cid in &claim.cites {
            let short = short_cid(cid);
            match cite_index.and_then(|idx| idx.get(cid)) {
                Some(c) => {
                    let first = c
                        .text
                        .as_deref()
                        .or(c.title.as_deref())
                        .and_then(|t| t.lines().find(|l| !l.trim().is_empty()))
                        .unwrap_or("")
                        .trim();
                    let first = if first.chars().count() > 60 {
                        format!("{}…", first.chars().take(60).collect::<String>())
                    } else {
                        first.to_string()
                    };
                    out.push(format!("  {short}  {}  {first}", c.kind));
                }
                None => out.push(format!("  {short}")),
            }
        }
    }
    out
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
    match &state.fold.day_status {
        Some(text) if !text.is_empty() => {
            for line in text.lines() {
                out.push_str(&format!("  {line}\n"));
            }
        }
        _ => out.push_str("  (unavailable)\n"),
    }
    out.push_str(&format!("{rule}\n"));

    let sessions = state.fold.sessions();
    out.push_str(&format!(
        "SESSIONS  (agents/handoff · {} live)\n",
        sessions.len()
    ));
    if sessions.is_empty() {
        out.push_str("  (none)\n");
    }
    for s in &sessions {
        let short = s.trim_start_matches("agents/handoff/");
        out.push_str(&format!("  · {short}\n"));
    }
    out.push_str(&format!("{rule}\n"));

    out.push_str(&format!(
        "CLAIMS  ({} subjects)\n",
        state.fold.subjects.len()
    ));
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
    if !state.fold.errors.is_empty() {
        out.push_str(&format!("{rule}\nNOTES\n"));
        for e in &state.fold.errors {
            out.push_str(&format!("  ! {e}\n"));
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
        eprintln!(
            "warning: {} has no .kan/ — is this a kan repo?",
            repo.display()
        );
    }

    let mut terminal = ratatui::init();
    let mtime = head_mtime(&repo);
    let fold = substrate::fold(&repo);
    let mut state = AppState::new(repo, fold, mtime);
    let tick = Duration::from_millis(250);

    let result = loop {
        // Poll-and-fold: re-fold only when HEAD's mtime changed.
        let now = head_mtime(&state.repo);
        if should_refold(state.last_mtime, now) {
            let fresh = substrate::fold(&state.repo);
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
                    KeyCode::Char(d @ '1'..='3') => {
                        if let Some(v) = View::from_digit(d) {
                            state.view = v;
                        }
                    }
                    KeyCode::Tab => state.view = state.view.next(),
                    KeyCode::Char('j') | KeyCode::Down => match state.view {
                        View::Browser => state.move_down(),
                        View::Atoms => {
                            let max = process_view_lines(&state.fold.process, View::Atoms)
                                .len()
                                .saturating_sub(1);
                            state.atom_scroll = (state.atom_scroll + 1).min(max);
                        }
                        View::Telos => {
                            let max = process_view_lines(&state.fold.process, View::Telos)
                                .len()
                                .saturating_sub(1);
                            state.telos_scroll = (state.telos_scroll + 1).min(max);
                        }
                    },
                    KeyCode::Char('k') | KeyCode::Up => match state.view {
                        View::Browser => state.move_up(),
                        View::Atoms => state.atom_scroll = state.atom_scroll.saturating_sub(1),
                        View::Telos => state.telos_scroll = state.telos_scroll.saturating_sub(1),
                    },
                    KeyCode::Enter if state.view == View::Browser => state.descend(),
                    KeyCode::Esc if state.view == View::Browser => state.ascend(),
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

    frame.render_widget(Line::from(view_header(state.view)).bold(), header);

    let mut process_text = match &state.fold.day_status {
        Some(t) if !t.is_empty() => t.clone(),
        _ => "(day status unavailable)".to_string(),
    };
    // Surface fold errors so a failed `kan show --all` is not mistaken for an
    // empty repo (telos/honest-ambiguity).
    if !state.fold.errors.is_empty() {
        let errs = state
            .fold
            .errors
            .iter()
            .map(|e| format!("! {e}"))
            .collect::<Vec<_>>()
            .join("\n");
        process_text = format!("{errs}\n{process_text}");
    }
    // Clip to the pane height with an explicit overflow cue; no wrap, so the
    // line count is exact and day's candidate list is never silently truncated.
    let process_lines = clip_lines(&process_text, (process.height as usize).saturating_sub(2));
    frame.render_widget(
        Paragraph::new(process_lines.join("\n"))
            .block(Block::bordered().title(" process (day status) ")),
        process,
    );

    match state.view {
        View::Browser => draw_browser(frame, state, area.width, body),
        View::Atoms => draw_atoms(frame, state, body),
        View::Telos => draw_telos(frame, state, body),
    }
}

fn view_header(view: View) -> String {
    let tab = |v: View, label: &str| {
        if v == view {
            format!("[{label}]")
        } else {
            format!(" {label} ")
        }
    };
    format!(
        "cospan  {}{}{}  · Tab switch · q quit",
        tab(View::Browser, "1 browser"),
        tab(View::Atoms, "2 atoms"),
        tab(View::Telos, "3 telos"),
    )
}

fn draw_browser(
    frame: &mut ratatui::Frame,
    state: &AppState,
    width: u16,
    body: ratatui::layout::Rect,
) {
    use ratatui::layout::{Constraint, Layout};
    // Both panes when wide, the focused level when narrow. The right pane (wide)
    // or single pane (narrow) shows the claim list, or the claim detail when
    // focus has descended to it.
    match layout_mode(width) {
        Fit::Wide => {
            let [left, right] =
                Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
                    .areas(body);
            draw_list(frame, state, left);
            if state.focus == Focus::Detail {
                draw_claim_detail(frame, state, right);
            } else {
                draw_claims(frame, state, right);
            }
        }
        Fit::Narrow => match state.focus {
            Focus::Subjects => draw_list(frame, state, body),
            Focus::Claims => draw_claims(frame, state, body),
            Focus::Detail => draw_claim_detail(frame, state, body),
        },
    }
}

/// The full rendered lines (deferral note + declared structure) for a process
/// view. Shared by the renderer and the scroll clamp, so a held `j` cannot
/// inflate the offset past the content.
pub fn process_view_lines(p: &ProcessSnapshot, view: View) -> Vec<String> {
    let mut lines = vec![
        "(live position & witness state need machine-readable day — declared structure only)"
            .to_string(),
        String::new(),
    ];
    match view {
        View::Atoms => {
            if p.atoms.is_empty() {
                lines.push("(no atoms declared)".to_string());
            }
            for a in &p.atoms {
                lines.push(format!(
                    "{}   in[{}] -> out[{}]   next[{}]",
                    a.slug,
                    a.inputs.join(", "),
                    a.outputs.join(", "),
                    a.next.join(", ")
                ));
            }
        }
        View::Telos => {
            for t in &p.teloi {
                lines.push(format!("{}  ({})", t.title, t.slug));
                lines.push(format!("  {}", t.statement));
                lines.push(format!("  witnesses: [{}]", t.witnesses.join(", ")));
                lines.push(String::new());
            }
            if !p.tensions.is_empty() {
                lines.push("tensions:".to_string());
                for tension in &p.tensions {
                    lines.push(format!("  {tension}"));
                }
            }
            if p.teloi.is_empty() && p.tensions.is_empty() {
                lines.push("(no teloi declared)".to_string());
            }
        }
        View::Browser => {} // never routes here
    }
    lines
}

fn render_scrolled(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    title: &str,
    lines: &[String],
    scroll: usize,
) {
    use ratatui::widgets::{Block, Paragraph};
    let scroll = scroll.min(lines.len().saturating_sub(1));
    frame.render_widget(
        Paragraph::new(lines[scroll..].join("\n"))
            .block(Block::bordered().title(title.to_string())),
        area,
    );
}

fn draw_atoms(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    let lines = process_view_lines(&state.fold.process, View::Atoms);
    render_scrolled(frame, area, " atoms ", &lines, state.atom_scroll);
}

fn draw_telos(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    let lines = process_view_lines(&state.fold.process, View::Telos);
    render_scrolled(frame, area, " teloi ", &lines, state.telos_scroll);
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
            .block(Block::bordered().title(format!(" subjects · {} ", state.fold.subjects.len())))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> "),
        area,
        &mut list_state,
    );
}

/// A foreground style per kan claim kind, from the ANSI-16 palette so it reads on
/// both light and dark terminals.
pub fn kind_style(kind: &str) -> ratatui::style::Style {
    use ratatui::style::{Color, Modifier, Style};
    let base = Style::new();
    match kind {
        "Decision" => base.fg(Color::Green),
        "Observation" => base.fg(Color::Blue),
        "Plan" => base.fg(Color::Yellow),
        "Result" => base.fg(Color::Cyan),
        "Subject" => base.fg(Color::Gray).add_modifier(Modifier::BOLD),
        "Relation" => base.fg(Color::Magenta),
        "Publication" => base.fg(Color::DarkGray),
        "Retraction" => base.fg(Color::Red),
        _ => base,
    }
}

fn draw_claims(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    use ratatui::style::{Modifier, Style};
    use ratatui::widgets::{Block, List, ListItem, ListState};

    let subject = state.selected_subject().unwrap_or("(no subject)");
    let claims = state.selected_claims();
    // One list row per claim, colored by kind; the row order matches the fold, so
    // claim_selected maps 1:1. An empty subject falls back to detail_lines' single
    // info line, uncolored.
    let items: Vec<ListItem> = if claims.is_empty() {
        detail_lines(subject, claims)
            .into_iter()
            .map(ListItem::new)
            .collect()
    } else {
        claims
            .iter()
            .map(|c| ListItem::new(c.display_line()).style(kind_style(&c.kind)))
            .collect()
    };

    let mut ls = ListState::default();
    let populated = !claims.is_empty();
    let active = matches!(state.focus, Focus::Claims | Focus::Detail);
    if populated && active {
        ls.select(Some(
            state.claim_selected.min(items.len().saturating_sub(1)),
        ));
    }
    frame.render_stateful_widget(
        List::new(items)
            .block(Block::bordered().title(format!(" {subject} · claims ")))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> "),
        area,
        &mut ls,
    );
}

fn draw_claim_detail(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    use ratatui::style::Style;
    use ratatui::text::Span;
    use ratatui::widgets::{Block, Paragraph};

    let (title, style, lines) = match state.selected_claim() {
        Some(c) => (
            short_cid(&c.cid),
            kind_style(&c.kind),
            detail_view(c, Some(&state.fold.by_cid)),
        ),
        None => (
            "(no claim)".to_string(),
            Style::new(),
            vec!["(no claim selected)".to_string()],
        ),
    };
    // Scroll offset, clamped so it can never slice past the end.
    let scroll = state.detail_scroll.min(lines.len().saturating_sub(1));
    frame.render_widget(
        Paragraph::new(lines[scroll..].join("\n"))
            .block(Block::bordered().title(Span::styled(format!(" {title} "), style))),
        area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal fold with the given subject names (each with no claims) and a
    /// stub day_status.
    fn fold_of(names: &[&str]) -> Fold {
        let mut f = Fold {
            day_status: Some("Current atom: design".to_string()),
            ..Default::default()
        };
        for n in names {
            f.subjects.push(n.to_string());
            f.claims.insert(n.to_string(), Vec::new());
        }
        f.subjects.sort();
        f
    }

    fn app(names: &[&str]) -> AppState {
        AppState::new(PathBuf::from("."), fold_of(names), None)
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
        a.refold(fold_of(&["telos/b", "atom/x"]), None);
        assert_eq!(a.selected_subject(), Some("telos/b"));

        // Remove the *selected* subject. Using one namespace keeps ordering
        // stable, so the cursor must land on the subject now at the old ordinal
        // (telos/c at ordinal 1) — NOT the first (telos/a). This distinguishes a
        // real clamp from a jump-to-top.
        let mut b = app(&["telos/a", "telos/b", "telos/c"]);
        b.select_next(); // telos/b at ordinal 1
        b.refold(fold_of(&["telos/a", "telos/c"]), None);
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
        assert!(
            out.contains("Current atom: design"),
            "day status not verbatim:\n{out}"
        );
    }

    fn mk_claim(kind: &str, text: &str) -> Claim {
        Claim {
            cid: "bafyreiXXXXXXX".into(),
            kind: kind.into(),
            subject: "telos/a".into(),
            author: "did:key:zABCDEFGH".into(),
            recorded_at: Some(0),
            text: Some(text.into()),
            title: None,
            artifacts: vec![],
            cites: vec![],
            supersedes: None,
        }
    }

    #[test]
    fn selected_claims_read_directly_from_the_fold() {
        let mut f = fold_of(&["telos/a", "telos/b"]);
        f.claims
            .insert("telos/a".into(), vec![mk_claim("Decision", "hello")]);
        let mut a = AppState::new(PathBuf::from("."), f, None);
        // telos/a is first; its claims resolve with no fetch closure.
        assert_eq!(a.selected_subject(), Some("telos/a"));
        assert_eq!(a.selected_claims().len(), 1);
        assert_eq!(
            a.selected_claim().map(|c| c.kind.as_str()),
            Some("Decision")
        );
        a.select_next(); // telos/b has no claims
        assert!(a.selected_claims().is_empty());
    }

    #[test]
    fn layout_mode_breaks_at_100() {
        assert_eq!(layout_mode(100), Fit::Wide);
        assert_eq!(layout_mode(200), Fit::Wide);
        assert_eq!(layout_mode(99), Fit::Narrow);
    }

    #[test]
    fn focus_descends_and_ascends_and_clamps() {
        let mut a = app(&["telos/a"]);
        assert_eq!(a.focus, Focus::Subjects);
        a.descend();
        assert_eq!(a.focus, Focus::Claims);
        a.descend();
        assert_eq!(a.focus, Focus::Detail);
        a.descend(); // clamps at Detail
        assert_eq!(a.focus, Focus::Detail);
        a.ascend();
        assert_eq!(a.focus, Focus::Claims);
        a.ascend();
        assert_eq!(a.focus, Focus::Subjects);
        a.ascend(); // clamps at Subjects
        assert_eq!(a.focus, Focus::Subjects);
    }

    #[test]
    fn detail_scroll_clamps_at_top() {
        let mut a = app(&["telos/a"]);
        a.focus = Focus::Detail;
        assert_eq!(a.detail_scroll, 0);
        a.move_up(); // already at top
        assert_eq!(a.detail_scroll, 0);
    }

    #[test]
    fn detail_view_shows_fields_and_resolves_cites() {
        let mut claim = mk_claim("Decision", "the full body text\nsecond line");
        claim.artifacts = vec!["Commit(\"abc\")".into()];
        claim.cites = vec!["bafyreiPRESENT".into(), "bafyreiMISSING".into()];

        let mut idx = HashMap::new();
        idx.insert(
            "bafyreiPRESENT".to_string(),
            mk_claim("Result", "cut a release"),
        );

        let lines = detail_view(&claim, Some(&idx));
        let joined = lines.join("\n");
        assert!(joined.contains("Decision"), "kind: {joined}");
        assert!(joined.contains("the full body text"), "full text: {joined}");
        assert!(joined.contains("Commit(\"abc\")"), "artifact: {joined}");
        // Present cite -> short-cid + kind + first line.
        assert!(
            joined.contains("@PRESENT…  Result  cut a release"),
            "resolved cite: {joined}"
        );
        // Missing cite -> bare short-cid.
        assert!(joined.contains("@MISSING…"), "unresolved cite: {joined}");
    }

    #[test]
    fn kind_style_distinguishes_kinds() {
        assert_ne!(kind_style("Decision"), kind_style("Retraction"));
        assert_ne!(kind_style("Result"), kind_style("Observation"));
        // An unknown kind is the plain default style.
        assert_eq!(kind_style("Whatever"), ratatui::style::Style::new());
    }

    #[test]
    fn view_selector_cycles_and_maps_digits() {
        assert_eq!(View::Browser.next(), View::Atoms);
        assert_eq!(View::Atoms.next(), View::Telos);
        assert_eq!(View::Telos.next(), View::Browser);
        assert_eq!(View::from_digit('1'), Some(View::Browser));
        assert_eq!(View::from_digit('2'), Some(View::Atoms));
        assert_eq!(View::from_digit('3'), Some(View::Telos));
        assert_eq!(View::from_digit('9'), None);
    }

    #[test]
    fn process_view_lines_are_note_led_and_bound_the_scroll() {
        // Empty snapshot: note + the "no atoms" line.
        let empty = process_view_lines(&ProcessSnapshot::default(), View::Atoms);
        assert!(empty[0].contains("machine-readable day"), "{:?}", empty[0]);
        assert!(empty.iter().any(|l| l.contains("no atoms")));

        let snap = ProcessSnapshot {
            atoms: vec![substrate::Atom {
                slug: "build".into(),
                inputs: vec!["d".into()],
                outputs: vec!["c".into()],
                next: vec!["r".into()],
            }],
            teloi: vec![],
            tensions: vec![],
        };
        let lines = process_view_lines(&snap, View::Atoms);
        assert!(lines.iter().any(|l| l.contains("build")));
        // The clamp the key handler applies can never slice past the end.
        let max = lines.len().saturating_sub(1);
        assert!((999usize).min(max) < lines.len());
    }

    #[test]
    fn detail_lines_empty_and_populated() {
        // Empty subject -> the honest empty-state line.
        let out = detail_lines("telos/a", &[]);
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("no live claims"), "{out:?}");

        // Populated -> one display_line per claim.
        let cs = vec![mk_claim("Decision", "hello")];
        let out = detail_lines("telos/a", &cs);
        assert_eq!(out.len(), 1);
        assert!(
            out[0].contains("Decision") && out[0].contains("hello"),
            "{out:?}"
        );
    }
}
