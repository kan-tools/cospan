//! L3 — the interactive view layer.
//!
//! Step 2 of the P0 arc: the `watch-repo` dashboard, drawn with ratatui instead
//! of ANSI escapes, driven by one poll-and-fold loop. Everything rendered is a
//! projection of the folded `substrate::Dashboard` — the only state this layer
//! owns is the selection cursor (`telos/kan-is-truth`). The loop never
//! subscribes: a single `event::poll(tick)` is both the key wait and the re-fold
//! tick, and the substrate is re-collected only when `.kan/log/HEAD` changes
//! (`telos/poll-dont-subscribe`).

use crate::substrate::{self, short_cid, Claim, Dashboard, ProcessSnapshot};
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
    /// Which of the three levels currently has focus.
    pub focus: Focus,
    /// Index into the selected subject's claim list (Claims focus).
    pub claim_selected: usize,
    /// Scroll offset into the claim-detail text (Detail focus).
    pub detail_scroll: usize,
    /// Lazily-folded `cid -> Claim` index for resolving cites; cleared on re-fold.
    pub cite_index: Option<HashMap<String, Claim>>,
    /// The active top-level view.
    pub view: View,
    /// Lazily-folded declared process structure; cleared on re-fold.
    pub process: Option<ProcessSnapshot>,
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
    pub fn new(repo: PathBuf, dash: Dashboard, last_mtime: Option<SystemTime>) -> Self {
        let mut s = AppState {
            repo,
            dash,
            rows: Vec::new(),
            selected: 0,
            last_mtime,
            claims: HashMap::new(),
            focus: Focus::Subjects,
            claim_selected: 0,
            detail_scroll: 0,
            cite_index: None,
            view: View::Browser,
            process: None,
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
        match self.selected_claims() {
            Some(Ok(cs)) => cs.len(),
            _ => 0,
        }
    }

    /// The claim currently selected in the claim list, if the fold is loaded.
    pub fn selected_claim(&self) -> Option<&Claim> {
        match self.selected_claims() {
            Some(Ok(cs)) => cs.get(self.claim_selected),
            _ => None,
        }
    }

    fn detail_line_count(&self) -> usize {
        match self.selected_claim() {
            Some(c) => detail_view(c, self.cite_index.as_ref()).len(),
            None => 0,
        }
    }

    /// Fold the `cid -> Claim` index once (for cite previews) if not yet loaded.
    /// Parameterized over the fold so it is testable without shelling out.
    pub fn ensure_cite_index<F>(&mut self, fold: F)
    where
        F: Fn(&Path) -> Result<HashMap<String, Claim>, String>,
    {
        if self.cite_index.is_none() {
            // On error, cache an empty index so we don't re-fold every tick.
            self.cite_index = Some(fold(&self.repo).unwrap_or_default());
        }
    }

    /// Fold the declared process structure once (for the Atoms/Telos views) if
    /// not yet loaded. Parameterized over the fold so it is testable.
    pub fn ensure_process<F>(&mut self, fold: F)
    where
        F: Fn(&Path) -> Result<ProcessSnapshot, String>,
    {
        if self.process.is_none() {
            self.process = Some(fold(&self.repo).unwrap_or_default());
        }
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
    pub fn refold(&mut self, dash: Dashboard, mtime: Option<SystemTime>) {
        let prev = self.selected_subject().map(str::to_string);
        let prev_ordinal = self.selected_ordinal();
        self.dash = dash;
        self.last_mtime = mtime;
        self.claims.clear(); // the log changed — stale claim detail must not persist
        self.cite_index = None;
        self.claim_selected = 0;
        self.detail_scroll = 0;
        self.process = None;
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
pub fn detail_lines(subject: &str, claims: Option<&Result<Vec<Claim>, String>>) -> Vec<String> {
    match claims {
        None => vec!["(loading …)".to_string()],
        Some(Err(e)) => vec![format!("error: {e}")],
        Some(Ok(cs)) if cs.is_empty() => {
            vec![format!(
                "{subject}: (no live claims — unused, or all claims retracted)"
            )]
        }
        Some(Ok(cs)) => cs.iter().map(Claim::display_line).collect(),
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

    out.push_str(&format!(
        "CLAIMS  ({} subjects)\n",
        state.dash.subjects.len()
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
                    KeyCode::Char(d @ '1'..='3') => {
                        if let Some(v) = View::from_digit(d) {
                            state.view = v;
                            if v != View::Browser {
                                state.ensure_process(substrate::process_snapshot);
                            }
                        }
                    }
                    KeyCode::Tab => {
                        state.view = state.view.next();
                        if state.view != View::Browser {
                            state.ensure_process(substrate::process_snapshot);
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => match state.view {
                        View::Browser => {
                            state.move_down();
                            if state.focus == Focus::Subjects {
                                state.ensure_selected_loaded(substrate::subject_claims);
                            }
                        }
                        View::Atoms => {
                            let max = process_view_lines(state.process.as_ref(), View::Atoms)
                                .len()
                                .saturating_sub(1);
                            state.atom_scroll = (state.atom_scroll + 1).min(max);
                        }
                        View::Telos => {
                            let max = process_view_lines(state.process.as_ref(), View::Telos)
                                .len()
                                .saturating_sub(1);
                            state.telos_scroll = (state.telos_scroll + 1).min(max);
                        }
                    },
                    KeyCode::Char('k') | KeyCode::Up => match state.view {
                        View::Browser => {
                            state.move_up();
                            if state.focus == Focus::Subjects {
                                state.ensure_selected_loaded(substrate::subject_claims);
                            }
                        }
                        View::Atoms => state.atom_scroll = state.atom_scroll.saturating_sub(1),
                        View::Telos => state.telos_scroll = state.telos_scroll.saturating_sub(1),
                    },
                    KeyCode::Enter if state.view == View::Browser => {
                        state.descend();
                        match state.focus {
                            Focus::Claims => {
                                state.ensure_selected_loaded(substrate::subject_claims)
                            }
                            Focus::Detail => state.ensure_cite_index(substrate::claim_index),
                            Focus::Subjects => {}
                        }
                    }
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
pub fn process_view_lines(process: Option<&ProcessSnapshot>, view: View) -> Vec<String> {
    let mut lines = vec![
        "(live position & witness state need machine-readable day — declared structure only)"
            .to_string(),
        String::new(),
    ];
    match (process, view) {
        (Some(p), View::Atoms) => {
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
        (Some(p), View::Telos) => {
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
        // Not yet folded, or the Browser view (which never routes here).
        _ => lines.push("(loading …)".to_string()),
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
    let lines = process_view_lines(state.process.as_ref(), View::Atoms);
    render_scrolled(frame, area, " atoms ", &lines, state.atom_scroll);
}

fn draw_telos(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    let lines = process_view_lines(state.process.as_ref(), View::Telos);
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
            .block(Block::bordered().title(format!(" subjects · {} ", state.dash.subjects.len())))
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
    // One list row per claim, colored by kind; the row order matches
    // subject_claims, so claim_selected maps 1:1. Non-populated states fall back
    // to detail_lines' single info line, uncolored.
    let items: Vec<ListItem> = match state.selected_claims() {
        Some(Ok(cs)) if !cs.is_empty() => cs
            .iter()
            .map(|c| ListItem::new(c.display_line()).style(kind_style(&c.kind)))
            .collect(),
        other => detail_lines(subject, other)
            .into_iter()
            .map(ListItem::new)
            .collect(),
    };

    let mut ls = ListState::default();
    let populated = matches!(state.selected_claims(), Some(Ok(cs)) if !cs.is_empty());
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
            detail_view(c, state.cite_index.as_ref()),
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
        // Not yet folded: note + a loading line.
        let none = process_view_lines(None, View::Atoms);
        assert!(none[0].contains("machine-readable day"), "{:?}", none[0]);
        assert!(none.iter().any(|l| l.contains("loading")));

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
        let lines = process_view_lines(Some(&snap), View::Atoms);
        assert!(lines.iter().any(|l| l.contains("build")));
        // The clamp the key handler applies can never slice past the end.
        let max = lines.len().saturating_sub(1);
        assert!((999usize).min(max) < lines.len());
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
        assert!(
            out[0].contains("Decision") && out[0].contains("hello"),
            "{out:?}"
        );
    }
}
