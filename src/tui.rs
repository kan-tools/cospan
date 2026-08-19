//! L3 — the interactive view layer.
//!
//! The `watch-repo` browser, drawn with ratatui and driven by one poll-and-fold
//! loop. Everything rendered is a projection of the `substrate::Fold` — the only
//! state this layer owns is the selection cursor (`telos/kan-is-truth`). The loop
//! never subscribes: a single `event::poll(tick)` is both the key wait and the
//! re-fold tick, and the fold is rebuilt only when `.kan/log/HEAD` changes
//! (`telos/poll-dont-subscribe`).

use crate::substrate::{self, is_day_subject, namespace, short_cid, Claim, Fold, ProcessSnapshot};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// One row of the left-pane tree: a collapsible top-level section, a collapsible
/// namespace group, or a selectable subject leaf.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Row {
    Section(String),
    Group(String),
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
    /// Keys of collapsed tree nodes (`sec:<label>` / `grp:<ns>`).
    pub collapsed: HashSet<String>,
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
            collapsed: HashSet::new(),
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
            Some(c) => claim_detail(c, Some(&self.fold.by_cid)).len(),
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

    /// Rebuild the tree rows from the fold: a `[my work]` section of bare
    /// subjects and a `[day]` section of day vocabulary (namespace groups then
    /// bare day subjects), skipping the children of any collapsed node.
    fn rebuild_rows(&mut self) {
        let mut rows = Vec::new();

        rows.push(Row::Section("my work".into()));
        if !self.collapsed.contains("sec:my work") {
            let mut mine: Vec<&str> = self
                .fold
                .subjects
                .iter()
                .filter(|n| !is_day_subject(n))
                .map(String::as_str)
                .collect();
            mine.sort();
            for n in mine {
                rows.push(Row::Subject(n.to_string()));
            }
        }

        rows.push(Row::Section("day".into()));
        if !self.collapsed.contains("sec:day") {
            for (ns, _count) in self.fold.namespace_counts() {
                if !matches!(
                    ns.as_str(),
                    "telos" | "atom" | "bridge" | "tension" | "schema"
                ) {
                    continue;
                }
                rows.push(Row::Group(ns.clone()));
                if !self.collapsed.contains(&format!("grp:{ns}")) {
                    let mut members: Vec<&str> = self
                        .fold
                        .subjects
                        .iter()
                        .filter(|n| namespace(n) == ns)
                        .map(String::as_str)
                        .collect();
                    members.sort();
                    for n in members {
                        rows.push(Row::Subject(n.to_string()));
                    }
                }
            }
            let mut bare_day: Vec<&str> = self
                .fold
                .subjects
                .iter()
                // Day subjects with no group of their own (practice, general).
                // A bare subject literally named a group word (e.g. "telos") is
                // rendered under that group, not here, so it appears once.
                .filter(|n| {
                    is_day_subject(n)
                        && !matches!(
                            namespace(n),
                            "telos" | "atom" | "bridge" | "tension" | "schema"
                        )
                })
                .map(String::as_str)
                .collect();
            bare_day.sort();
            for n in bare_day {
                rows.push(Row::Subject(n.to_string()));
            }
        }

        self.rows = rows;
    }

    fn first_subject_index(&self) -> Option<usize> {
        self.rows.iter().position(|r| matches!(r, Row::Subject(_)))
    }

    /// A stable identity for the row at `i`, so the cursor can stay put across a
    /// re-fold or a collapse toggle.
    fn row_key(&self, i: usize) -> Option<String> {
        Some(match self.rows.get(i)? {
            Row::Section(l) => format!("sec:{l}"),
            Row::Group(g) => format!("grp:{g}"),
            Row::Subject(n) => format!("sub:{n}"),
        })
    }

    fn index_of_key(&self, key: &str) -> Option<usize> {
        (0..self.rows.len()).find(|&i| self.row_key(i).as_deref() == Some(key))
    }

    /// The name of the currently selected subject, if the selected row is one.
    pub fn selected_subject(&self) -> Option<&str> {
        match self.rows.get(self.selected) {
            Some(Row::Subject(name)) => Some(name.as_str()),
            _ => None,
        }
    }

    /// Move the selection down over all visible rows, clamped at the last.
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.rows.len() {
            self.selected += 1;
        }
    }

    /// Move the selection up over all visible rows, clamped at the first.
    pub fn select_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Enter in the Subjects focus: toggle a Section/Group node, or descend a
    /// Subject into its claims.
    pub fn activate(&mut self) {
        match self.rows.get(self.selected) {
            Some(Row::Section(l)) => {
                let key = format!("sec:{l}");
                self.toggle(&key);
            }
            Some(Row::Group(g)) => {
                let key = format!("grp:{g}");
                self.toggle(&key);
            }
            Some(Row::Subject(_)) => self.descend(),
            None => {}
        }
    }

    /// Flip a node's collapsed state and rebuild, keeping the cursor on it.
    fn toggle(&mut self, key: &str) {
        if !self.collapsed.remove(key) {
            self.collapsed.insert(key.to_string());
        }
        let sel = self.row_key(self.selected);
        self.rebuild_rows();
        self.selected = sel
            .and_then(|k| self.index_of_key(&k))
            .unwrap_or_else(|| self.selected.min(self.rows.len().saturating_sub(1)));
    }

    /// Replace the fold, preserving the selection by the selected row's identity
    /// so a background log change never jumps the cursor.
    pub fn refold(&mut self, fold: Fold, mtime: Option<SystemTime>) {
        let sel = self.row_key(self.selected);
        self.fold = fold;
        self.last_mtime = mtime;
        self.claim_selected = 0;
        self.detail_scroll = 0;
        self.atom_scroll = 0;
        self.telos_scroll = 0;
        self.rebuild_rows();
        self.selected = sel
            .and_then(|k| self.index_of_key(&k))
            .or_else(|| self.first_subject_index())
            .unwrap_or(0)
            .min(self.rows.len().saturating_sub(1));
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

/// One segment of a claim body: prose (rendered as markdown) or a named fenced
/// block (a human summary if supported, else code).
enum Segment {
    Prose(String),
    Block { name: String, content: String },
}

/// Split a body into prose and named fenced blocks (```` ```name ````).
fn split_blocks(text: &str) -> Vec<Segment> {
    let lines: Vec<&str> = text.lines().collect();
    let mut segs = Vec::new();
    let mut prose: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let name = lines[i]
            .strip_prefix("```")
            .map(str::trim)
            .filter(|n| !n.is_empty());
        if let Some(name) = name {
            let mut j = i + 1;
            let mut content: Vec<&str> = Vec::new();
            while j < lines.len() && lines[j].trim_end() != "```" {
                content.push(lines[j]);
                j += 1;
            }
            if j < lines.len() {
                if !prose.is_empty() {
                    segs.push(Segment::Prose(prose.join("\n")));
                    prose.clear();
                }
                segs.push(Segment::Block {
                    name: name.to_string(),
                    content: content.join("\n"),
                });
                i = j + 1;
                continue;
            }
        }
        prose.push(lines[i]);
        i += 1;
    }
    if !prose.is_empty() {
        segs.push(Segment::Prose(prose.join("\n")));
    }
    segs
}

/// The full styled detail for a single claim: header fields, the body (markdown
/// prose with supported blocks summarized and unsupported blocks shown as code),
/// and each cite resolved through the index. A pure projection of the claim
/// (`telos/kan-is-truth`); an unsupported block is shown, never hidden
/// (`telos/honest-ambiguity`).
pub fn claim_detail(
    claim: &Claim,
    cite_index: Option<&HashMap<String, Claim>>,
) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::{Modifier, Style};
    use ratatui::text::{Line, Span};

    let mut out: Vec<Line<'static>> = vec![
        Line::from(format!("{}  {}", claim.kind, claim.cid)),
        Line::from(format!(
            "author {}   {}",
            claim.short_author(),
            claim.recorded_utc()
        )),
        Line::from(format!("subject {}", claim.subject)),
    ];
    if !claim.artifacts.is_empty() {
        out.push(Line::from(format!("anchor {}", claim.artifacts.join(", "))));
    }
    out.push(Line::from(""));

    let body = claim
        .text
        .as_deref()
        .or(claim.title.as_deref())
        .unwrap_or("");
    if body.is_empty() {
        out.push(Line::from(format!("({})", claim.kind.to_lowercase())));
    } else {
        for seg in split_blocks(body) {
            match seg {
                Segment::Prose(p) => out.extend(crate::markdown::render(&p)),
                Segment::Block { name, content } => {
                    let summary = serde_json::from_str::<serde_json::Value>(&content)
                        .ok()
                        .and_then(|j| substrate::block_summary(&name, &j));
                    match summary {
                        Some(lines) => {
                            out.push(Line::from(Span::styled(
                                format!("{name}:"),
                                Style::new().add_modifier(Modifier::BOLD),
                            )));
                            out.extend(lines.into_iter().map(|l| Line::from(format!("  {l}"))));
                        }
                        None => {
                            out.push(Line::from(Span::styled(
                                format!("```{name}"),
                                crate::markdown::code_style(),
                            )));
                            for l in content.lines() {
                                out.push(Line::from(Span::styled(
                                    l.to_string(),
                                    crate::markdown::code_style(),
                                )));
                            }
                        }
                    }
                    out.push(Line::from(""));
                }
            }
        }
    }

    if !claim.cites.is_empty() {
        out.push(Line::from(""));
        out.push(Line::from(format!("cites ({}):", claim.cites.len())));
        for cid in &claim.cites {
            let short = short_cid(cid);
            let line = match cite_index.and_then(|idx| idx.get(cid)) {
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
                    format!("  {short}  {}  {first}", c.kind)
                }
                None => format!("  {short}"),
            };
            out.push(Line::from(line));
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
    if state.fold.subjects.is_empty() {
        out.push_str("  (none)\n");
    }
    // A flat namespace grouping, independent of the interactive tree, so the
    // --once frame stays scriptable.
    for (ns, _count) in state.fold.namespace_counts() {
        let members: Vec<&String> = state
            .fold
            .subjects
            .iter()
            .filter(|n| namespace(n) == ns)
            .collect();
        // Skip a header identical to its single bare subject.
        if !(members.len() == 1 && *members[0] == ns) {
            out.push_str(&format!("  {ns}\n"));
        }
        for name in &members {
            out.push_str(&format!("    {name}\n"));
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
                    KeyCode::Enter if state.view == View::Browser => {
                        if state.focus == Focus::Subjects {
                            state.activate(); // toggle a node, or descend a subject
                        } else {
                            state.descend();
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
    use ratatui::style::{Modifier, Style};
    use ratatui::widgets::{Block, List, ListItem, ListState};

    let marker = |key: &str| {
        if state.collapsed.contains(key) {
            "▸"
        } else {
            "▾"
        }
    };
    let items: Vec<ListItem> = state
        .rows
        .iter()
        .map(|row| match row {
            Row::Section(l) => ListItem::new(format!("{} [{l}]", marker(&format!("sec:{l}"))))
                .style(Style::new().add_modifier(Modifier::BOLD)),
            Row::Group(g) => ListItem::new(format!("  {} {g}", marker(&format!("grp:{g}"))))
                .style(Style::new().add_modifier(Modifier::DIM)),
            Row::Subject(name) => ListItem::new(format!("    {name}")),
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
    use ratatui::text::{Line, Span};
    use ratatui::widgets::{Block, Paragraph, Wrap};

    let (title, style, lines) = match state.selected_claim() {
        Some(c) => (
            short_cid(&c.cid),
            kind_style(&c.kind),
            claim_detail(c, Some(&state.fold.by_cid)),
        ),
        None => (
            "(no claim)".to_string(),
            Style::new(),
            vec![Line::from("(no claim selected)")],
        ),
    };
    // Scroll offset, clamped so it can never slice past the end.
    let scroll = state.detail_scroll.min(lines.len().saturating_sub(1));
    frame.render_widget(
        Paragraph::new(lines[scroll..].to_vec())
            .wrap(Wrap { trim: false })
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
    fn tree_splits_my_work_and_day_with_groups() {
        // a bare design subject (my work), two telos + one atom (day groups, in
        // count order), and practice (a bare day subject after the groups).
        let a = app(&[
            "claim-detail-view",
            "telos/a",
            "telos/b",
            "atom/x",
            "practice",
        ]);
        assert_eq!(
            a.rows,
            vec![
                Row::Section("my work".into()),
                Row::Subject("claim-detail-view".into()),
                Row::Section("day".into()),
                Row::Group("telos".into()),
                Row::Subject("telos/a".into()),
                Row::Subject("telos/b".into()),
                Row::Group("atom".into()),
                Row::Subject("atom/x".into()),
                Row::Subject("practice".into()),
            ]
        );
        // Initial selection lands on the first subject leaf.
        assert_eq!(a.selected_subject(), Some("claim-detail-view"));
    }

    #[test]
    fn collapsing_day_hides_its_children_and_toggles_back() {
        let mut a = app(&["claim-detail-view", "telos/a"]);
        a.selected = a.index_of_key("sec:day").unwrap();
        a.activate(); // collapse [day]
        assert!(a.collapsed.contains("sec:day"));
        assert!(!a.rows.iter().any(|r| matches!(r, Row::Group(_))));
        assert!(!a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Subject(s) if s == "telos/a")));
        // [my work] and its subject remain.
        assert!(a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Subject(s) if s == "claim-detail-view")));
        a.activate(); // expand again (cursor stayed on the [day] section)
        assert!(a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Subject(s) if s == "telos/a")));
    }

    #[test]
    fn jk_moves_over_all_rows_and_clamps() {
        // rows: Section(my work), Section(day), Group(telos), Subject(telos/a)
        let mut a = app(&["telos/a"]);
        a.selected = 0;
        a.select_prev(); // clamps at the first row
        assert_eq!(a.selected, 0);
        a.select_next();
        a.select_next();
        a.select_next();
        assert_eq!(a.selected, 3);
        a.select_next(); // clamps at the last row
        assert_eq!(a.selected, 3);
        assert_eq!(a.selected_subject(), Some("telos/a"));
    }

    #[test]
    fn activate_toggles_a_group_but_descends_a_subject() {
        let mut a = app(&["telos/a"]);
        a.selected = a.index_of_key("grp:telos").unwrap();
        a.activate(); // on a Group: toggle, focus unchanged
        assert_eq!(a.focus, Focus::Subjects);
        assert!(a.collapsed.contains("grp:telos"));
        a.activate(); // re-expand (cursor stays on the group)
        a.selected = a.index_of_key("sub:telos/a").unwrap();
        a.activate(); // on a Subject: descend to Claims
        assert_eq!(a.focus, Focus::Claims);
    }

    #[test]
    fn refold_keeps_the_cursor_on_the_same_row_by_identity() {
        let mut a = app(&["telos/a", "telos/b"]);
        a.selected = a.index_of_key("grp:telos").unwrap();
        a.refold(fold_of(&["telos/a", "telos/b", "telos/c"]), None);
        assert_eq!(a.row_key(a.selected).as_deref(), Some("grp:telos"));
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

    fn detail_text(claim: &Claim, idx: Option<&HashMap<String, Claim>>) -> String {
        claim_detail(claim, idx)
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn claim_detail_shows_fields_and_resolves_cites() {
        let mut claim = mk_claim("Decision", "the full body text\nsecond line");
        claim.artifacts = vec!["Commit(\"abc\")".into()];
        claim.cites = vec!["bafyreiPRESENT".into(), "bafyreiMISSING".into()];

        let mut idx = HashMap::new();
        idx.insert(
            "bafyreiPRESENT".to_string(),
            mk_claim("Result", "cut a release"),
        );

        let joined = detail_text(&claim, Some(&idx));
        assert!(joined.contains("Decision"), "kind: {joined}");
        assert!(joined.contains("the full body text"), "full text: {joined}");
        assert!(joined.contains("Commit(\"abc\")"), "artifact: {joined}");
        assert!(
            joined.contains("@PRESENT…  Result  cut a release"),
            "resolved cite: {joined}"
        );
        assert!(joined.contains("@MISSING…"), "unresolved cite: {joined}");
    }

    #[test]
    fn claim_detail_summarizes_a_supported_block() {
        let body =
            "some prose here\n\n```day-telos\n{\"witnesses\":[\"code-change\",\"verdict\"]}\n```";
        let claim = mk_claim("Decision", body);
        let joined = detail_text(&claim, None);
        assert!(joined.contains("some prose here"), "prose: {joined}");
        // Human summary, not the raw JSON.
        assert!(
            joined.contains("witnesses: code-change, verdict"),
            "summary: {joined}"
        );
        assert!(
            !joined.contains("\"witnesses\""),
            "raw json leaked: {joined}"
        );
    }

    #[test]
    fn claim_detail_shows_an_unsupported_block_as_code() {
        let body = "```json\n{\"k\": 1}\n```";
        let claim = mk_claim("Observation", body);
        let joined = detail_text(&claim, None);
        // The block content is shown (as code), not dropped.
        assert!(
            joined.contains("{\"k\": 1}"),
            "unsupported block content: {joined}"
        );
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
