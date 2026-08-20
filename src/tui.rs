//! L3 — the interactive view layer.
//!
//! The `watch-repo` browser, drawn with ratatui and driven by one poll-and-fold
//! loop. Everything rendered is a projection of the `substrate::Fold` — the only
//! state this layer owns is the selection cursor (`telos/kan-is-truth`). The loop
//! never subscribes: a single `event::poll(tick)` is both the key wait and the
//! re-fold tick, and the fold is rebuilt only when `.kan/log/HEAD` changes
//! (`telos/poll-dont-subscribe`).

use crate::comments::{self, Comment};
use crate::substrate::{
    self, is_day_subject, namespace, short_cid, Atom, Claim, Fold, ProcessSnapshot,
};
use crate::{Localization, State};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// One row of the left-pane tree. The tree is a path trie over subject names:
/// a collapsible top-level `Section` (`[my work]` / `[day]`), a collapsible
/// intermediate `Branch` (a non-terminal path prefix like `agents/handoff`), or
/// a selectable `Leaf` subject. `depth` is the row's path depth from its section
/// (sections are depth 0), and drives the render indent. A subject that is also
/// a branch prefix appears as both a `Branch` and a `Leaf` one indent deeper.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Row {
    Section(String),
    Branch { path: String, depth: usize },
    Leaf { subject: String, depth: usize },
}

/// A node in the subject path trie: whether the path to here is itself a recorded
/// subject, and its children keyed by the next path segment (`BTreeMap` so
/// siblings flatten in sorted order).
#[derive(Default)]
struct TrieNode {
    is_subject: bool,
    children: std::collections::BTreeMap<String, TrieNode>,
}

/// Flatten a trie node's children into rows at `depth`. A childless node is a
/// plain `Leaf`; a node with children is a `Branch`, and — unless collapsed — its
/// own subject-ness (a dual node) is a `Leaf` one indent deeper, followed by its
/// recursively flattened subtree.
fn flatten_trie(
    node: &TrieNode,
    prefix: &str,
    depth: usize,
    collapsed: &HashSet<String>,
    rows: &mut Vec<Row>,
) {
    for (seg, child) in &node.children {
        let full = if prefix.is_empty() {
            seg.clone()
        } else {
            format!("{prefix}/{seg}")
        };
        if child.children.is_empty() {
            rows.push(Row::Leaf {
                subject: full,
                depth,
            });
            continue;
        }
        rows.push(Row::Branch {
            path: full.clone(),
            depth,
        });
        if collapsed.contains(&format!("path:{full}")) {
            continue;
        }
        if child.is_subject {
            rows.push(Row::Leaf {
                subject: full.clone(),
                depth: depth + 1,
            });
        }
        flatten_trie(child, &full, depth + 1, collapsed, rows);
    }
}

/// The interactive dashboard's state: the current fold plus a selection over the
/// repo's subjects. Row-building, navigation, and the re-fold gate are all pure
/// so they can be unit-tested without a terminal.
pub struct AppState {
    pub repo: PathBuf,
    /// The whole in-memory model, folded from one `kan show --all` per tick.
    pub fold: Fold,
    pub rows: Vec<Row>,
    /// Index into `rows`; always points at a `Row::Leaf` when any subject exists.
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
    /// Which sub-pane the Process tab shows.
    pub process_pane: ProcessPane,
    /// The highlighted atom box in the flowchart.
    pub atom_selected: usize,
    /// Whether the Process atoms pane is drilled into an atom's detail.
    pub process_detail: bool,
    pub atom_scroll: usize,
    pub telos_scroll: usize,
    /// Keys of collapsed tree nodes (`sec:<label>` / `path:<prefix>`).
    pub collapsed: HashSet<String>,

    // --- Comments view (P1): a picker over commented files + a live gutter. ---
    /// Source files that have a sidecar, with their comment count; rebuilt on
    /// entering the view or on a re-fold.
    pub comment_files: Vec<(PathBuf, usize)>,
    /// Index into `comment_files`.
    pub comment_file_selected: usize,
    /// Cached content of the selected file (for the gutter render).
    pub comment_content: String,
    /// The selected file's comments with their current localization, newest gate.
    pub comment_localized: Vec<(Comment, Localization)>,
    /// The repo-relative path whose content is currently loaded into
    /// `comment_content`/`comment_localized` — distinct from the *selected* file,
    /// so a re-read is forced whenever they differ (and a missing source is not
    /// confused with "not yet loaded").
    pub comment_loaded: Option<PathBuf>,
    /// The loaded file's last-seen mtime — the same-file content-change gate.
    pub comment_mtime: Option<SystemTime>,
    /// Cursor over the selected file's comments.
    pub comment_selected: usize,
    /// Scroll offset into the file-content pane.
    pub comment_scroll: usize,

    // --- Footer: day's status line, width-matched from its cache. ---
    pub footer: Vec<String>,
    footer_mtime: Option<SystemTime>,
    footer_width: u16,
}

/// The top-level tabs, switched with `1`/`2`/`3` or `Tab`. `Ledger` is the kan
/// claim browser; `Process` houses the atoms/telos sub-panes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
    Comments,
    Ledger,
    Process,
}

impl View {
    pub fn next(self) -> View {
        match self {
            View::Comments => View::Ledger,
            View::Ledger => View::Process,
            View::Process => View::Comments,
        }
    }

    pub fn from_digit(c: char) -> Option<View> {
        match c {
            '1' => Some(View::Comments),
            '2' => Some(View::Ledger),
            '3' => Some(View::Process),
            _ => None,
        }
    }
}

/// The two sub-panes of the Process tab (today's Atoms and Telos views).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessPane {
    Atoms,
    Telos,
}

impl ProcessPane {
    /// Toggle Atoms <-> Telos (what `←`/`→` do in the Process view).
    pub fn toggled(self) -> ProcessPane {
        match self {
            ProcessPane::Atoms => ProcessPane::Telos,
            ProcessPane::Telos => ProcessPane::Atoms,
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
            view: View::Comments,
            process_pane: ProcessPane::Atoms,
            atom_selected: 0,
            process_detail: false,
            atom_scroll: 0,
            telos_scroll: 0,
            collapsed: HashSet::new(),
            comment_files: Vec::new(),
            comment_file_selected: 0,
            comment_content: String::new(),
            comment_localized: Vec::new(),
            comment_loaded: None,
            comment_mtime: None,
            comment_selected: 0,
            comment_scroll: 0,
            footer: Vec::new(),
            footer_mtime: None,
            footer_width: 0,
        };
        s.rebuild_rows();
        s.selected = s.first_subject_index().unwrap_or(0);
        s.comment_files = commented_files(&s.repo);
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

    /// Rebuild the tree rows from the fold: a `[my work]` section and a `[day]`
    /// section, each a recursive path trie over its subjects (split on `/`),
    /// skipping the subtree of any collapsed node.
    fn rebuild_rows(&mut self) {
        let mut rows = Vec::new();

        for (label, is_day) in [("my work", false), ("day", true)] {
            rows.push(Row::Section(label.into()));
            if self.collapsed.contains(&format!("sec:{label}")) {
                continue;
            }
            let mut members: Vec<&str> = self
                .fold
                .subjects
                .iter()
                .filter(|n| is_day_subject(n) == is_day)
                .map(String::as_str)
                .collect();
            members.sort();
            self.push_trie(&mut rows, &members);
        }

        self.rows = rows;
    }

    /// Emit the path-trie rows for one section's `subjects`. Builds a trie keyed
    /// by path segment (a `BTreeMap` keeps siblings sorted), then flattens it
    /// depth-first, skipping the subtree of any collapsed `Branch`.
    fn push_trie(&self, rows: &mut Vec<Row>, subjects: &[&str]) {
        let mut root = TrieNode::default();
        for s in subjects {
            let mut node = &mut root;
            for seg in s.split('/') {
                node = node.children.entry(seg.to_string()).or_default();
            }
            node.is_subject = true;
        }
        flatten_trie(&root, "", 1, &self.collapsed, rows);
    }

    fn first_subject_index(&self) -> Option<usize> {
        self.rows.iter().position(|r| matches!(r, Row::Leaf { .. }))
    }

    /// A stable identity for the row at `i`, so the cursor can stay put across a
    /// re-fold or a collapse toggle.
    fn row_key(&self, i: usize) -> Option<String> {
        Some(match self.rows.get(i)? {
            Row::Section(l) => format!("sec:{l}"),
            Row::Branch { path, .. } => format!("path:{path}"),
            Row::Leaf { subject, .. } => format!("sub:{subject}"),
        })
    }

    fn index_of_key(&self, key: &str) -> Option<usize> {
        (0..self.rows.len()).find(|&i| self.row_key(i).as_deref() == Some(key))
    }

    /// The name of the currently selected subject, if the selected row is one.
    pub fn selected_subject(&self) -> Option<&str> {
        match self.rows.get(self.selected) {
            Some(Row::Leaf { subject, .. }) => Some(subject.as_str()),
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

    /// Enter in the Subjects focus: toggle a Section/Branch node, or descend a
    /// Leaf subject into its claims.
    pub fn activate(&mut self) {
        match self.rows.get(self.selected) {
            Some(Row::Section(l)) => {
                let key = format!("sec:{l}");
                self.toggle(&key);
            }
            Some(Row::Branch { path, .. }) => {
                let key = format!("path:{path}");
                self.toggle(&key);
            }
            Some(Row::Leaf { .. }) => self.descend(),
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

    /// Re-scan the sidecar tree for commented files. Called on entering the
    /// Comments view so newly-commented files appear.
    pub fn reload_comment_files(&mut self) {
        self.comment_files = commented_files(&self.repo);
        self.comment_file_selected = self
            .comment_file_selected
            .min(self.comment_files.len().saturating_sub(1));
    }

    /// Refresh the selected file's comment localizations. Re-reads when the
    /// selected file differs from the loaded one (first open / file switch) or its
    /// content changed since last tick; otherwise a single `stat` and early
    /// return, so an unchanged file triggers no re-read or `save`
    /// (`telos/poll-dont-subscribe`). A missing source reads as empty, so its
    /// comments localize to `Unresolvable` and reach the resolve-by-hand list
    /// rather than leaving another file's content on screen (`honest-ambiguity`).
    pub fn refresh_comments(&mut self) {
        if self.comment_files.is_empty() {
            self.comment_content.clear();
            self.comment_localized.clear();
            self.comment_loaded = None;
            self.comment_mtime = None;
            return;
        }
        self.comment_file_selected = self.comment_file_selected.min(self.comment_files.len() - 1);
        let rel = self.comment_files[self.comment_file_selected].0.clone();
        let src = self.repo.join(&rel);
        let mtime = std::fs::metadata(&src).ok().and_then(|m| m.modified().ok());
        let already_loaded = self.comment_loaded.as_deref() == Some(rel.as_path());
        // Reload on a different file, or a same-file content change. `comment_mtime`
        // only gates the *same* file, so a missing source's `None` mtime can never
        // masquerade as "already current".
        if already_loaded && !should_refold(self.comment_mtime, mtime) {
            return;
        }
        let content = std::fs::read_to_string(&src).unwrap_or_default();
        let sidecar = self
            .repo
            .join(comments::sidecar_path(&rel.to_string_lossy()));
        let mut cs = comments::load(&sidecar).unwrap_or_default();
        let mut localized: Vec<(Comment, Localization)> = cs
            .iter_mut()
            .map(|c| {
                let loc = comments::localize_and_update(c, &content);
                (c.clone(), loc)
            })
            .collect();
        // Order by anchored line (top to bottom) so `j`/`k` follow the file's
        // vertical layout; Unresolvable comments (no line) sort last, stably.
        localized.sort_by_key(|(_, loc)| loc.span.map(|(s, _)| s).unwrap_or(usize::MAX));
        // Persist the re-anchored last-seen state, like `cospan comments`.
        let _ = comments::save(&sidecar, &cs);
        self.comment_content = content;
        self.comment_localized = localized;
        self.comment_loaded = Some(rel);
        self.comment_mtime = mtime;
        self.comment_selected = self
            .comment_selected
            .min(self.comment_localized.len().saturating_sub(1));
    }

    /// Refresh the footer (day's status line) when its cache changed or the width
    /// changed — one `stat` per tick, re-read only on change; once loaded it never
    /// re-shells (`telos/poll-dont-subscribe`).
    pub fn refresh_footer(&mut self, width: u16, emoji: bool) {
        let mtime = std::fs::metadata(substrate::footer_cache_path(&self.repo))
            .ok()
            .and_then(|m| m.modified().ok());
        if !self.footer.is_empty()
            && self.footer_width == width
            && !should_refold(self.footer_mtime, mtime)
        {
            return;
        }
        self.footer = substrate::status_footer(&self.repo, width, emoji);
        self.footer_mtime = mtime;
        self.footer_width = width;
    }

    /// Move the comment cursor and scroll the content pane to its anchored line.
    pub fn select_comment(&mut self, delta: isize) {
        let n = self.comment_localized.len();
        if n == 0 {
            return;
        }
        self.comment_selected =
            (self.comment_selected as isize + delta).clamp(0, n as isize - 1) as usize;
        if let Some((start, _)) = self.comment_localized[self.comment_selected].1.span {
            self.comment_scroll = start;
        }
    }

    /// Move within the active Process sub-pane: select an atom box (atoms graph),
    /// scroll the atom detail (atoms drill-down), or scroll the telos list.
    pub fn process_move(&mut self, delta: isize) {
        match (self.process_pane, self.process_detail) {
            (ProcessPane::Atoms, false) => self.atom_select(delta),
            (ProcessPane::Atoms, true) => {
                let max = self
                    .fold
                    .process
                    .atoms
                    .get(self.atom_selected)
                    .map(|a| atom_detail(a).len())
                    .unwrap_or(1)
                    .saturating_sub(1);
                self.atom_scroll =
                    (self.atom_scroll as isize + delta).clamp(0, max as isize) as usize;
            }
            (ProcessPane::Telos, _) => {
                let max = process_view_lines(&self.fold.process, ProcessPane::Telos)
                    .len()
                    .saturating_sub(1);
                self.telos_scroll =
                    (self.telos_scroll as isize + delta).clamp(0, max as isize) as usize;
            }
        }
    }

    /// Move the highlighted atom box in the flowchart, clamped.
    pub fn atom_select(&mut self, delta: isize) {
        let n = self.fold.process.atoms.len();
        if n == 0 {
            return;
        }
        self.atom_selected =
            (self.atom_selected as isize + delta).clamp(0, n as isize - 1) as usize;
    }

    /// Enter/leave the atom drill-down detail (Enter/Esc in the Process atoms pane).
    pub fn process_drill(&mut self, into: bool) {
        if self.process_pane == ProcessPane::Atoms {
            self.process_detail = into;
            self.atom_scroll = 0;
        }
    }

    /// Switch the selected commented file, resetting the per-file re-read gate.
    pub fn select_comment_file(&mut self, delta: isize) {
        let n = self.comment_files.len();
        if n == 0 {
            return;
        }
        self.comment_file_selected =
            (self.comment_file_selected as isize + delta).clamp(0, n as isize - 1) as usize;
        self.comment_selected = 0;
        self.comment_scroll = 0;
        // The next `refresh_comments` re-reads because the selected file now
        // differs from `comment_loaded`; no sentinel reset needed.
    }
}

/// Discover source files that have a comment sidecar, with their comment count,
/// by walking `.cospan/comments`. Repo-relative source paths, sorted.
pub fn commented_files(repo: &Path) -> Vec<(PathBuf, usize)> {
    let base = repo.join(".cospan/comments");
    let mut out = Vec::new();
    collect_sidecars(&base, &base, &mut out);
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn collect_sidecars(base: &Path, dir: &Path, out: &mut Vec<(PathBuf, usize)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        // `file_type()` does not follow symlinks, so a symlinked directory is not
        // recursed into — no cycle can blow the stack under this ephemeral tree.
        let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir {
            collect_sidecars(base, &p, out);
        } else if p.extension().is_some_and(|x| x == "jsonl") {
            // The source path is the sidecar's path under `base`, minus `.jsonl`.
            if let Ok(rel) = p.strip_prefix(base) {
                let s = rel.to_string_lossy();
                let src = PathBuf::from(s.strip_suffix(".jsonl").unwrap_or(&s));
                let count = comments::load(&p).map(|v| v.len()).unwrap_or(0);
                out.push((src, count));
            }
        }
    }
}

/// A foreground style per re-localizer state, from the ANSI-16 palette.
fn state_style(state: State) -> ratatui::style::Style {
    use ratatui::style::{Color, Style};
    match state {
        State::Anchored => Style::new().fg(Color::Green),
        State::Drifted => Style::new().fg(Color::Yellow),
        State::Unresolvable => Style::new().fg(Color::Red),
    }
}

/// Build the Comments content pane: each source line prefixed by a gutter marker
/// for any comment anchored on it (styled by state; the selected comment
/// highlighted), and the `Unresolvable` comments (span `None`) returned
/// separately since they cannot be placed on a line (`telos/honest-ambiguity`).
pub fn gutter_lines<'a>(
    content: &str,
    localized: &'a [(Comment, Localization)],
    selected: usize,
) -> (Vec<ratatui::text::Line<'static>>, Vec<&'a Comment>) {
    use ratatui::style::{Modifier, Style};
    use ratatui::text::{Line, Span};
    let unresolved: Vec<&Comment> = localized
        .iter()
        .filter(|(_, loc)| loc.span.is_none())
        .map(|(c, _)| c)
        .collect();
    let num_w = content.lines().count().max(1).to_string().len();
    let lines = content
        .lines()
        .enumerate()
        .map(|(i, text)| {
            let covers = |loc: &Localization| loc.span.is_some_and(|(s, e)| i >= s && i <= e);
            // Prefer the selected comment when it covers this line, so its marker
            // (not an overlapping neighbour's) carries the highlight.
            let hit = localized
                .get(selected)
                .filter(|(_, loc)| covers(loc))
                .map(|c| (selected, c))
                .or_else(|| {
                    localized
                        .iter()
                        .enumerate()
                        .find(|(_, (_, loc))| covers(loc))
                });
            let (marker, style) = match hit {
                Some((idx, (_, loc))) => {
                    let mut st = state_style(loc.state);
                    if idx == selected {
                        st = st.add_modifier(Modifier::REVERSED);
                    }
                    ("●", st)
                }
                None => (" ", Style::new()),
            };
            Line::from(vec![
                Span::styled(marker.to_string(), style),
                Span::styled(
                    format!(" {:>num_w$} ", i + 1),
                    Style::new().add_modifier(Modifier::DIM),
                ),
                Span::raw(text.to_string()),
            ])
        })
        .collect();
    (lines, unresolved)
}

/// The detail-strip lines for one comment: a header (state · where · confidence ·
/// author), the body, each reply indented and attributed, and a `[resolved]`
/// marker when resolved. Pure, so the thread render is unit-testable.
pub fn thread_lines(c: &Comment, loc: &Localization) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::{Modifier, Style};
    use ratatui::text::{Line, Span};
    let at = loc
        .span
        .map(|(s, _)| format!("line {}", s + 1))
        .unwrap_or_else(|| "unplaced".into());
    let mut header = vec![
        Span::styled(format!("{:?}", loc.state), state_style(loc.state)),
        Span::raw(format!(
            "  {at}  conf {:.2}  @{}",
            loc.confidence, c.author.id
        )),
    ];
    if c.resolved {
        header.push(Span::styled(
            "  [resolved]",
            Style::new().add_modifier(Modifier::DIM),
        ));
    }
    let mut lines = vec![Line::from(header), Line::from(c.body.clone())];
    for r in &c.thread {
        lines.push(Line::from(vec![
            Span::styled("  └ ", Style::new().add_modifier(Modifier::DIM)),
            Span::styled(
                format!("@{}: ", r.author.id),
                Style::new().add_modifier(Modifier::DIM),
            ),
            Span::raw(r.body.clone()),
        ]));
    }
    lines
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
        // Second gate, same tick: re-localize the Comments view's file on change.
        if state.view == View::Comments {
            state.refresh_comments();
        }
        // Footer gate: refresh day's status line on a cache/width change.
        let footer_w = terminal.size().map(|s| s.width).unwrap_or(0);
        state.refresh_footer(footer_w, true);

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
                            if v == View::Comments {
                                state.reload_comment_files();
                            }
                        }
                    }
                    KeyCode::Tab => {
                        state.view = state.view.next();
                        if state.view == View::Comments {
                            state.reload_comment_files();
                        }
                    }
                    // ←/→ (or [ / ]): switch commented file (Comments) or toggle the
                    // atoms/telos sub-pane (Process).
                    KeyCode::Char('[') | KeyCode::Left if state.view == View::Comments => {
                        state.select_comment_file(-1)
                    }
                    KeyCode::Char(']') | KeyCode::Right if state.view == View::Comments => {
                        state.select_comment_file(1)
                    }
                    KeyCode::Char('[' | ']') | KeyCode::Left | KeyCode::Right
                        if state.view == View::Process =>
                    {
                        state.process_pane = state.process_pane.toggled();
                        state.process_detail = false; // leave any drill-down on a pane switch
                    }
                    KeyCode::Char('j') | KeyCode::Down => match state.view {
                        View::Ledger => state.move_down(),
                        View::Process => state.process_move(1),
                        View::Comments => state.select_comment(1),
                    },
                    KeyCode::Char('k') | KeyCode::Up => match state.view {
                        View::Ledger => state.move_up(),
                        View::Process => state.process_move(-1),
                        View::Comments => state.select_comment(-1),
                    },
                    KeyCode::Enter if state.view == View::Ledger => {
                        if state.focus == Focus::Subjects {
                            state.activate(); // toggle a node, or descend a subject
                        } else {
                            state.descend();
                        }
                    }
                    // Drill the selected atom's detail in/out of view in Process.
                    KeyCode::Enter if state.view == View::Process => state.process_drill(true),
                    KeyCode::Esc if state.view == View::Process && state.process_detail => {
                        state.process_drill(false)
                    }
                    KeyCode::Esc if state.view == View::Ledger => state.ascend(),
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
    use ratatui::widgets::Paragraph;

    let area = frame.area();

    // The footer: day's status line, plus any fold errors surfaced above it so a
    // failed `kan show --all` is not mistaken for an empty repo
    // (telos/honest-ambiguity).
    let mut footer_lines: Vec<String> =
        state.fold.errors.iter().map(|e| format!("! {e}")).collect();
    if state.footer.is_empty() {
        footer_lines.push("(day status-line unavailable)".to_string());
    } else {
        footer_lines.extend(state.footer.iter().cloned());
    }
    let footer_h = (footer_lines.len().clamp(1, 6)) as u16;

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(3),
        Constraint::Length(footer_h),
    ])
    .areas(area);

    frame.render_widget(Line::from(view_header(state.view)).bold(), header);

    match state.view {
        View::Comments => draw_comments(frame, state, area.width, body),
        View::Ledger => draw_browser(frame, state, area.width, body),
        View::Process => draw_process(frame, state, body),
    }

    // The thin status-bar footer (day's status line), sourced from day's cache.
    frame.render_widget(Paragraph::new(footer_lines.join("\n")).dim(), footer);
}

fn view_header(view: View) -> String {
    let tab = |v: View, label: &str| {
        if v == view {
            format!("[{label}]")
        } else {
            format!(" {label} ")
        }
    };
    // A per-view key legend so navigation is discoverable without reading source.
    let keys = match view {
        View::Comments => "· ←→ file · j/k comment ",
        View::Process => "· ←→ atoms/telos · j/k scroll ",
        View::Ledger => "",
    };
    format!(
        "cospan  {}{}{}  {keys}· Tab switch · q quit",
        tab(View::Comments, "1 comments"),
        tab(View::Ledger, "2 ledger"),
        tab(View::Process, "3 process"),
    )
}

/// The Process tab: today's atoms or telos content, per the active sub-pane.
fn draw_process(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    match state.process_pane {
        ProcessPane::Atoms => draw_atoms(frame, state, area),
        ProcessPane::Telos => draw_telos(frame, state, area),
    }
}

/// Greedy word-wrap of `s` to width `w`, splitting on the text's own newlines
/// first. A single over-long word is left intact (the terminal clips it); always
/// returns at least one line.
pub fn wrap_text(s: &str, w: usize) -> Vec<String> {
    if w == 0 {
        return vec![s.to_string()];
    }
    let mut out = Vec::new();
    for para in s.split('\n') {
        let mut line = String::new();
        for word in para.split_whitespace() {
            if line.is_empty() {
                line = word.to_string();
            } else if line.chars().count() + 1 + word.chars().count() <= w {
                line.push(' ');
                line.push_str(word);
            } else {
                out.push(std::mem::take(&mut line));
                line = word.to_string();
            }
        }
        out.push(line);
    }
    if out.is_empty() {
        out.push(String::new());
    }
    out
}

/// A compact note for the comment column: a state-colored header (`● @author ·
/// STATE`, `[resolved]` when resolved), the body word-wrapped to `w` and capped
/// (trailing `…` when longer), and a `+N replies` line. The full body and thread
/// stay in the strip, so this reads at a glance.
pub fn note_block(
    c: &Comment,
    loc: &Localization,
    w: usize,
    selected: bool,
) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::{Modifier, Style};
    use ratatui::text::{Line, Span};
    const BODY_CAP: usize = 3;
    let mut label = format!("@{} · {:?}", c.author.id, loc.state);
    if c.resolved {
        label.push_str(" [resolved]");
    }
    let header_style = if selected {
        Style::new().add_modifier(Modifier::REVERSED)
    } else {
        Style::new()
    };
    let mut lines = vec![Line::from(vec![
        Span::styled("● ", state_style(loc.state)),
        Span::styled(label, header_style),
    ])];

    let mut body = wrap_text(&c.body, w.saturating_sub(2));
    let truncated = body.len() > BODY_CAP;
    body.truncate(BODY_CAP);
    if truncated {
        if let Some(last) = body.last_mut() {
            last.push('…');
        }
    }
    for b in body {
        lines.push(Line::from(format!("  {b}")));
    }
    if !c.thread.is_empty() {
        let n = c.thread.len();
        lines.push(Line::from(Span::styled(
            format!("  +{n} repl{}", if n == 1 { "y" } else { "ies" }),
            Style::new().add_modifier(Modifier::DIM),
        )));
    }
    lines
}

/// Pair each code line with a right-column note cell, reflowing the code down so a
/// multi-line note never overlaps code or another note. `notes` is
/// `(localized_index, start_line, note_lines)`, sorted by `start_line`. Returns
/// the paired `(left, right)` rows and, per note, `(localized_index, first_row)`
/// so the caller can scroll to a selected comment.
#[allow(clippy::type_complexity)]
pub fn reflow_rows(
    code_lines: Vec<ratatui::text::Line<'static>>,
    notes: &[(usize, usize, Vec<ratatui::text::Line<'static>>)],
) -> (
    Vec<(ratatui::text::Line<'static>, ratatui::text::Line<'static>)>,
    Vec<(usize, usize)>,
) {
    use ratatui::text::Line;
    let blank = || Line::from("");
    let mut rows: Vec<(Line, Line)> = Vec::new();
    let mut note_rows: Vec<(usize, usize)> = Vec::new();
    let mut ni = 0;
    for (i, code) in code_lines.into_iter().enumerate() {
        rows.push((code, blank()));
        let code_row = rows.len() - 1;
        let mut right_free = true; // the code row's right cell is still empty
        while ni < notes.len() && notes[ni].1 == i {
            let (loc_idx, _start, note_lines) = &notes[ni];
            for (j, nl) in note_lines.iter().enumerate() {
                if j == 0 && right_free {
                    rows[code_row].1 = nl.clone();
                    right_free = false;
                    note_rows.push((*loc_idx, code_row));
                } else {
                    rows.push((blank(), nl.clone()));
                    if j == 0 {
                        note_rows.push((*loc_idx, rows.len() - 1));
                    }
                }
            }
            ni += 1;
        }
    }
    // Any note anchored past the last code line (a shrunk file) still shows.
    while ni < notes.len() {
        let (loc_idx, _s, note_lines) = &notes[ni];
        for (j, nl) in note_lines.iter().enumerate() {
            rows.push((blank(), nl.clone()));
            if j == 0 {
                note_rows.push((*loc_idx, rows.len() - 1));
            }
        }
        ni += 1;
    }
    (rows, note_rows)
}

fn draw_comments(
    frame: &mut ratatui::Frame,
    state: &AppState,
    width: u16,
    body: ratatui::layout::Rect,
) {
    use ratatui::layout::{Constraint, Layout};
    use ratatui::style::{Modifier, Style};
    use ratatui::text::{Line, Span};
    use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};

    if state.comment_files.is_empty() {
        frame.render_widget(
            Paragraph::new(
                "no comments yet — drop one with `cospan comment add <file> --line <N> <body>`",
            )
            .block(Block::bordered().title(" comments ")),
            body,
        );
        return;
    }

    // Wide: a commented-files rail beside the content pane; narrow: content only.
    let (files_area, main_area) = match layout_mode(width) {
        Fit::Wide => {
            let [l, r] =
                Layout::horizontal([Constraint::Percentage(32), Constraint::Percentage(68)])
                    .areas(body);
            (Some(l), r)
        }
        Fit::Narrow => (None, body),
    };

    if let Some(area) = files_area {
        let items: Vec<ListItem> = state
            .comment_files
            .iter()
            .map(|(p, n)| ListItem::new(format!("{} ({n})", p.to_string_lossy())))
            .collect();
        let mut ls = ListState::default();
        ls.select(Some(state.comment_file_selected));
        frame.render_stateful_widget(
            List::new(items)
                .block(
                    Block::bordered()
                        .title(format!(" files · {} · ←/→ ", state.comment_files.len())),
                )
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol("> "),
            area,
            &mut ls,
        );
    }

    let [content_area, strip_area] =
        Layout::vertical([Constraint::Min(3), Constraint::Length(6)]).areas(main_area);

    let (code_lines, unresolved) = gutter_lines(
        &state.comment_content,
        &state.comment_localized,
        state.comment_selected,
    );
    let file_title = state
        .comment_files
        .get(state.comment_file_selected)
        .map(|(p, _)| p.to_string_lossy().to_string())
        .unwrap_or_default();
    match layout_mode(width) {
        Fit::Wide => {
            // Code column beside a right comment column; the code reflows down so
            // a multi-line note never overlaps code or a neighbour.
            let [code_area, note_area] =
                Layout::horizontal([Constraint::Percentage(58), Constraint::Percentage(42)])
                    .areas(content_area);
            let note_w = note_area.width.saturating_sub(2) as usize;
            let notes: Vec<(usize, usize, Vec<Line>)> = state
                .comment_localized
                .iter()
                .enumerate()
                .filter_map(|(idx, (c, loc))| {
                    loc.span.map(|(s, _)| {
                        (
                            idx,
                            s,
                            note_block(c, loc, note_w, idx == state.comment_selected),
                        )
                    })
                })
                .collect();
            let (rows, note_rows) = reflow_rows(code_lines, &notes);
            // Scroll so the selected comment's note is in view.
            let sel_row = note_rows
                .iter()
                .find(|(idx, _)| *idx == state.comment_selected)
                .map(|(_, r)| *r)
                .unwrap_or(0);
            let scroll = sel_row.min(rows.len().saturating_sub(1));
            let left: Vec<Line> = rows.iter().skip(scroll).map(|(l, _)| l.clone()).collect();
            let right: Vec<Line> = rows.iter().skip(scroll).map(|(_, r)| r.clone()).collect();
            frame.render_widget(
                Paragraph::new(left)
                    .block(Block::bordered().title(format!(" {file_title} · ←/→ file "))),
                code_area,
            );
            frame.render_widget(
                Paragraph::new(right).block(Block::bordered().title(" comments ")),
                note_area,
            );
        }
        Fit::Narrow => {
            let scroll = state.comment_scroll.min(code_lines.len().saturating_sub(1));
            frame.render_widget(
                Paragraph::new(code_lines[scroll..].to_vec())
                    .block(Block::bordered().title(format!(" {file_title} · ←/→ file "))),
                content_area,
            );
        }
    }

    // Strip: the selected comment's full thread, then the unresolvable list.
    let mut strip: Vec<Line> = Vec::new();
    if let Some((c, loc)) = state.comment_localized.get(state.comment_selected) {
        strip.extend(thread_lines(c, loc));
    }
    if !unresolved.is_empty() {
        strip.push(Line::from(Span::styled(
            format!("unresolvable ({}) — replace by hand:", unresolved.len()),
            state_style(State::Unresolvable),
        )));
        for c in unresolved.iter().take(3) {
            strip.push(Line::from(format!(
                "  · {}",
                c.body.lines().next().unwrap_or("")
            )));
        }
    }
    let strip_title = match state.comment_localized.get(state.comment_selected) {
        Some((c, _)) => {
            let s = comments::thread_summary(c);
            if s.is_empty() {
                " comment ".to_string()
            } else {
                format!(" comment · {s} ")
            }
        }
        None => " comment ".to_string(),
    };
    frame.render_widget(
        Paragraph::new(strip)
            .block(Block::bordered().title(strip_title))
            .wrap(Wrap { trim: false }),
        strip_area,
    );
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
pub fn process_view_lines(p: &ProcessSnapshot, pane: ProcessPane) -> Vec<String> {
    let mut lines = vec![
        "(live position & witness state need machine-readable day — declared structure only)"
            .to_string(),
        String::new(),
    ];
    match pane {
        ProcessPane::Atoms => {
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
        ProcessPane::Telos => {
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
    }
    lines
}

/// An atom's grid position in the flowchart: column (DAG depth) and row (stack
/// position within the column).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    pub col: usize,
    pub row: usize,
}

/// Lay atoms out by longest-path column (bounded relaxation, so a back-edge or
/// cycle cannot loop forever) and by row within a column (input order).
pub fn layout_atoms(atoms: &[Atom]) -> Vec<Placed> {
    let n = atoms.len();
    let idx_of: HashMap<&str, usize> = atoms
        .iter()
        .enumerate()
        .map(|(i, a)| (a.slug.as_str(), i))
        .collect();
    let edges: Vec<(usize, usize)> = atoms
        .iter()
        .enumerate()
        .flat_map(|(u, a)| {
            a.next
                .iter()
                .filter_map(|s| idx_of.get(s.as_str()).map(|&v| (u, v)))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut col = vec![0usize; n];
    for _ in 0..n {
        let mut changed = false;
        for &(u, v) in &edges {
            if col[v] < col[u] + 1 {
                col[v] = col[u] + 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let mut used: HashMap<usize, usize> = HashMap::new();
    (0..n)
        .map(|i| {
            let c = col[i];
            let r = used.entry(c).or_insert(0);
            let placed = Placed { col: c, row: *r };
            *r += 1;
            placed
        })
        .collect()
}

/// Render the atom DAG as an ASCII box-and-arrow flowchart. The `selected` atom's
/// box uses a double border; a `──▶` arrow is drawn for every `next` edge landing
/// one column right on the same row; other `next` edges and every `revisits`
/// back-edge are listed below the grid rather than drawn (`telos/honest-ambiguity`).
pub fn atom_flowchart(atoms: &[Atom], selected: usize) -> Vec<String> {
    if atoms.is_empty() {
        return vec!["(no atoms declared)".to_string()];
    }
    let placed = layout_atoms(atoms);
    let n = atoms.len();
    let box_w = |i: usize| atoms[i].slug.chars().count() + 4;
    let n_cols = placed.iter().map(|p| p.col).max().unwrap_or(0) + 1;
    let mut col_w = vec![0usize; n_cols];
    for i in 0..n {
        col_w[placed[i].col] = col_w[placed[i].col].max(box_w(i));
    }
    const GAP: usize = 5;
    let mut col_x = vec![0usize; n_cols];
    for c in 1..n_cols {
        col_x[c] = col_x[c - 1] + col_w[c - 1] + GAP;
    }
    const ROW_H: usize = 4;
    let n_rows = placed.iter().map(|p| p.row).max().unwrap_or(0) + 1;
    let single_row = n_rows == 1;
    let grid_w = col_x[n_cols - 1] + col_w[n_cols - 1];
    let box_center = |i: usize| col_x[placed[i].col] + box_w(i) / 2;

    // Classify every edge: a same-row/next-column `next` is a forward arrow; every
    // other edge is a back-flow — routed as a dashed arrow below the boxes when the
    // layout is single-row, else listed as text (routing would cross boxes).
    let idx_of: HashMap<&str, usize> = atoms
        .iter()
        .enumerate()
        .map(|(i, a)| (a.slug.as_str(), i))
        .collect();
    let mut forward: Vec<(usize, usize)> = Vec::new();
    let mut routed: Vec<(usize, usize)> = Vec::new();
    let mut annotations: Vec<String> = Vec::new();
    let mut classify = |u: usize, target: Option<usize>, slug: &str, revisit: bool| match target {
        Some(v)
            if !revisit && placed[v].row == placed[u].row && placed[v].col == placed[u].col + 1 =>
        {
            forward.push((u, v))
        }
        Some(v) if single_row => routed.push((u, v)),
        Some(_) | None => {
            let mark = if revisit { '↻' } else { '⇢' };
            let tail = if target.is_none() { " (unknown)" } else { "" };
            annotations.push(format!("{} {mark} {slug}{tail}", atoms[u].slug))
        }
    };
    for (u, atom) in atoms.iter().enumerate() {
        for s in &atom.next {
            classify(u, idx_of.get(s.as_str()).copied(), s, false);
        }
        for rv in &atom.revisits {
            classify(u, idx_of.get(rv.as_str()).copied(), rv, true);
        }
    }

    // Grid height: the boxes, plus one arrow-head row and a lane per routed edge.
    let boxes_h = n_rows * ROW_H - 1;
    let grid_h = boxes_h
        + if routed.is_empty() {
            0
        } else {
            1 + routed.len()
        };
    let mut grid = vec![vec![' '; grid_w]; grid_h];

    for i in 0..n {
        let (c, r) = (placed[i].col, placed[i].row);
        let x = col_x[c];
        let y = r * ROW_H;
        let w = box_w(i);
        let (tl, tr, bl, br, h, v) = if i == selected {
            ('╔', '╗', '╚', '╝', '═', '║')
        } else {
            ('┌', '┐', '└', '┘', '─', '│')
        };
        grid[y][x] = tl;
        grid[y][x + w - 1] = tr;
        grid[y + 2][x] = bl;
        grid[y + 2][x + w - 1] = br;
        for k in 1..w - 1 {
            grid[y][x + k] = h;
            grid[y + 2][x + k] = h;
        }
        grid[y + 1][x] = v;
        grid[y + 1][x + w - 1] = v;
        for (k, ch) in atoms[i].slug.chars().enumerate() {
            grid[y + 1][x + 2 + k] = ch;
        }
    }

    // Forward arrows: solid `──▶` on the boxes' middle row.
    for (u, v) in forward {
        let y = placed[u].row * ROW_H + 1;
        let x0 = col_x[placed[u].col] + box_w(u);
        let x1 = col_x[placed[v].col];
        if x1 > x0 {
            for x in grid[y].iter_mut().take(x1 - 1).skip(x0) {
                *x = '─';
            }
            grid[y][x1 - 1] = '▶';
        }
    }

    // Back-flows (single-row only): a dashed arrow routed in a lane below the
    // boxes, `▲` rising into the target — visually distinct from the solid forward
    // arrows.
    let arrow_y = boxes_h; // one row below the box bottoms
    for (li, (u, v)) in routed.iter().enumerate() {
        let (sx, tx) = (box_center(*u), box_center(*v));
        let lane_y = arrow_y + 1 + li;
        for row in grid.iter_mut().take(lane_y).skip(arrow_y) {
            row[sx] = '┆'; // source drop
        }
        grid[arrow_y][tx] = '▲'; // arrow-head rising into the target
        for row in grid.iter_mut().take(lane_y).skip(arrow_y + 1) {
            row[tx] = '┆';
        }
        let (lo, hi) = (sx.min(tx), sx.max(tx));
        // Guard the degenerate self-edge (sx == tx) so the slice never inverts.
        if lo + 1 < hi {
            for cell in &mut grid[lane_y][lo + 1..hi] {
                *cell = '┄';
            }
        }
        let (src_corner, tgt_corner) = if sx > tx {
            ('╯', '╰')
        } else {
            ('╰', '╯')
        };
        grid[lane_y][sx] = src_corner;
        grid[lane_y][tx] = tgt_corner;
    }

    let mut out: Vec<String> = grid
        .into_iter()
        .map(|row| row.into_iter().collect::<String>().trim_end().to_string())
        .collect();
    if !annotations.is_empty() {
        out.push(String::new());
        out.push("edges not drawn:".to_string());
        for a in annotations {
            out.push(format!("  {a}"));
        }
    }
    out
}

/// The drill-down detail lines for one atom.
pub fn atom_detail(a: &Atom) -> Vec<String> {
    let field = |k: &str, v: &[String]| format!("{k:<10}{}", v.join(", "));
    let mut out = vec![format!("atom/{}", a.slug), String::new()];
    out.push(field("in:", &a.inputs));
    out.push(field("out:", &a.outputs));
    out.push(field("next:", &a.next));
    if !a.done.is_empty() {
        out.push(field("done:", &a.done));
    }
    if !a.revisits.is_empty() {
        out.push(field("revisits:", &a.revisits));
    }
    out
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
    let atoms = &state.fold.process.atoms;
    if state.process_detail {
        let lines = atoms
            .get(state.atom_selected)
            .map(atom_detail)
            .unwrap_or_else(|| vec!["(no atom selected)".to_string()]);
        render_scrolled(
            frame,
            area,
            " process · atom · Esc back ",
            &lines,
            state.atom_scroll,
        );
    } else {
        let lines = atom_flowchart(atoms, state.atom_selected);
        render_scrolled(
            frame,
            area,
            " process · atoms · ←→ telos · ↵ detail ",
            &lines,
            state.atom_scroll,
        );
    }
}

fn draw_telos(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    let lines = process_view_lines(&state.fold.process, ProcessPane::Telos);
    render_scrolled(
        frame,
        area,
        " process · telos · ←→ atoms ",
        &lines,
        state.telos_scroll,
    );
}

/// A foreground color per top-level section, from the ANSI-16 palette so it reads
/// on both light and dark terminals (like `kind_style`).
fn section_color(label: &str) -> ratatui::style::Color {
    use ratatui::style::Color;
    match label {
        "my work" => Color::Cyan,
        "day" => Color::Magenta,
        _ => Color::Reset,
    }
}

/// One rendered left-pane line for a tree row: indent from its depth, a collapse
/// marker on the collapsible `Section`/`Branch` rows, a colored bold section
/// header, a full-weight branch segment, and a `Leaf` whose path prefix is dimmed
/// so the final segment reads as the subject's own name.
fn row_line(row: &Row, collapsed: &HashSet<String>) -> ratatui::text::Line<'static> {
    use ratatui::style::{Modifier, Style};
    use ratatui::text::{Line, Span};
    let marker = |key: &str| {
        if collapsed.contains(key) {
            "▸ "
        } else {
            "▾ "
        }
    };
    match row {
        Row::Section(l) => Line::from(vec![
            Span::raw(marker(&format!("sec:{l}")).to_string()),
            Span::styled(
                format!("[{l}]"),
                Style::new()
                    .fg(section_color(l))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Row::Branch { path, depth } => {
            let seg = path.rsplit('/').next().unwrap_or(path);
            Line::from(vec![
                Span::raw(format!(
                    "{}{}",
                    "  ".repeat(*depth),
                    marker(&format!("path:{path}"))
                )),
                Span::raw(seg.to_string()),
            ])
        }
        Row::Leaf { subject, depth } => {
            // Reserve the collapse-marker width so a leaf aligns under its branch.
            let indent = format!("{}  ", "  ".repeat(*depth));
            match subject.rfind('/') {
                Some(i) => Line::from(vec![
                    Span::raw(indent),
                    Span::styled(
                        subject[..=i].to_string(),
                        Style::new().add_modifier(Modifier::DIM),
                    ),
                    Span::raw(subject[i + 1..].to_string()),
                ]),
                None => Line::from(vec![Span::raw(indent), Span::raw(subject.clone())]),
            }
        }
    }
}

fn draw_list(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    use ratatui::style::{Modifier, Style};
    use ratatui::widgets::{Block, List, ListItem, ListState};

    let items: Vec<ListItem> = state
        .rows
        .iter()
        .map(|row| ListItem::new(row_line(row, &state.collapsed)))
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

    fn branch(path: &str, depth: usize) -> Row {
        Row::Branch {
            path: path.into(),
            depth,
        }
    }

    fn leaf(subject: &str, depth: usize) -> Row {
        Row::Leaf {
            subject: subject.into(),
            depth,
        }
    }

    #[test]
    fn tree_aggregates_subjects_into_a_recursive_path_trie() {
        // (AC-1) A bare my-work subject; a three-deep day path; and two telos
        // leaves — the whitelist is gone, so `agents/handoff/main` nests fully.
        let a = app(&["cospan", "telos/a", "telos/b", "agents/handoff/main"]);
        assert_eq!(
            a.rows,
            vec![
                Row::Section("my work".into()),
                leaf("cospan", 1),
                Row::Section("day".into()),
                branch("agents", 1),
                branch("agents/handoff", 2),
                leaf("agents/handoff/main", 3),
                branch("telos", 1),
                leaf("telos/a", 2),
                leaf("telos/b", 2),
            ]
        );
        // Initial selection lands on the first subject leaf.
        assert_eq!(a.selected_subject(), Some("cospan"));
    }

    #[test]
    fn a_subject_that_is_also_a_branch_appears_as_both() {
        // (AC-3) `foo` is a subject and a prefix of `foo/bar`: it is a Branch
        // header plus a Leaf child one indent deeper.
        let mut a = app(&["foo", "foo/bar"]);
        assert_eq!(
            a.rows,
            vec![
                Row::Section("my work".into()),
                branch("foo", 1),
                leaf("foo", 2),
                leaf("foo/bar", 2),
                Row::Section("day".into()),
            ]
        );
        // Enter on the Branch toggles it; Enter on the self-Leaf descends.
        a.selected = a.index_of_key("path:foo").unwrap();
        a.activate();
        assert!(a.collapsed.contains("path:foo"));
        assert_eq!(a.focus, Focus::Subjects);
        a.activate(); // re-expand
        a.selected = a.index_of_key("sub:foo").unwrap();
        a.activate();
        assert_eq!(a.focus, Focus::Claims);
    }

    #[test]
    fn collapsing_a_branch_hides_its_whole_subtree() {
        // (AC-4) Collapsing `agents` hides the nested handoff branch and leaf,
        // the cursor stays on `agents`, and re-expanding restores them.
        let mut a = app(&["cospan", "agents/handoff/main"]);
        a.selected = a.index_of_key("path:agents").unwrap();
        a.activate(); // collapse agents
        assert!(a.collapsed.contains("path:agents"));
        assert_eq!(a.row_key(a.selected).as_deref(), Some("path:agents"));
        assert!(!a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Branch { path, .. } if path == "agents/handoff")));
        assert!(!a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Leaf { subject, .. } if subject == "agents/handoff/main")));
        // The my-work leaf is untouched.
        assert!(a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Leaf { subject, .. } if subject == "cospan")));
        a.activate(); // re-expand (cursor stayed on the branch)
        assert!(a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Leaf { subject, .. } if subject == "agents/handoff/main")));
    }

    #[test]
    fn collapsing_day_hides_its_children_and_toggles_back() {
        let mut a = app(&["cospan", "telos/a"]);
        a.selected = a.index_of_key("sec:day").unwrap();
        a.activate(); // collapse [day]
        assert!(a.collapsed.contains("sec:day"));
        assert!(!a.rows.iter().any(|r| matches!(r, Row::Branch { .. })));
        assert!(!a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Leaf { subject, .. } if subject == "telos/a")));
        // [my work] and its subject remain.
        assert!(a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Leaf { subject, .. } if subject == "cospan")));
        a.activate(); // expand again (cursor stayed on the [day] section)
        assert!(a
            .rows
            .iter()
            .any(|r| matches!(r, Row::Leaf { subject, .. } if subject == "telos/a")));
    }

    #[test]
    fn jk_moves_over_all_rows_and_clamps() {
        // rows: Section(my work), Section(day), Branch(telos), Leaf(telos/a)
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
    fn activate_toggles_a_branch_but_descends_a_leaf() {
        let mut a = app(&["telos/a"]);
        a.selected = a.index_of_key("path:telos").unwrap();
        a.activate(); // on a Branch: toggle, focus unchanged
        assert_eq!(a.focus, Focus::Subjects);
        assert!(a.collapsed.contains("path:telos"));
        a.activate(); // re-expand (cursor stays on the branch)
        a.selected = a.index_of_key("sub:telos/a").unwrap();
        a.activate(); // on a Leaf: descend to Claims
        assert_eq!(a.focus, Focus::Claims);
    }

    #[test]
    fn refold_keeps_the_cursor_on_the_same_row_by_identity() {
        let mut a = app(&["telos/a", "telos/b"]);
        a.selected = a.index_of_key("path:telos").unwrap();
        a.refold(fold_of(&["telos/a", "telos/b", "telos/c"]), None);
        assert_eq!(a.row_key(a.selected).as_deref(), Some("path:telos"));
    }

    #[test]
    fn row_line_fades_a_leaf_prefix_but_not_a_branch_or_section() {
        // (AC-2) A leaf's `telos/` prefix is dimmed and its final segment is not;
        // a branch segment carries no dim; a section carries its color.
        use ratatui::style::{Color, Modifier};
        let collapsed = HashSet::new();
        let leaf_line = row_line(&leaf("telos/readable-claim-browser", 2), &collapsed);
        let dimmed: Vec<&str> = leaf_line
            .spans
            .iter()
            .filter(|s| s.style.add_modifier.contains(Modifier::DIM))
            .map(|s| s.content.as_ref())
            .collect();
        assert_eq!(dimmed, vec!["telos/"]);
        assert!(leaf_line
            .spans
            .iter()
            .any(|s| s.content == "readable-claim-browser"
                && !s.style.add_modifier.contains(Modifier::DIM)));

        let branch_line = row_line(&branch("telos", 1), &collapsed);
        assert!(branch_line
            .spans
            .iter()
            .all(|s| !s.style.add_modifier.contains(Modifier::DIM)));

        let section_line = row_line(&Row::Section("day".into()), &collapsed);
        assert!(section_line
            .spans
            .iter()
            .any(|s| s.style.fg == Some(Color::Magenta)));
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
        // (AC-1) Comments · Ledger · Process.
        assert_eq!(View::Comments.next(), View::Ledger);
        assert_eq!(View::Ledger.next(), View::Process);
        assert_eq!(View::Process.next(), View::Comments);
        assert_eq!(View::from_digit('1'), Some(View::Comments));
        assert_eq!(View::from_digit('2'), Some(View::Ledger));
        assert_eq!(View::from_digit('3'), Some(View::Process));
        assert_eq!(View::from_digit('4'), None);
        assert_eq!(View::from_digit('9'), None);
    }

    #[test]
    fn tab_bar_names_the_new_tabs_with_legends() {
        // (AC-2) Ledger + Process tabs; Process names its sub-pane keys; no browser.
        assert!(view_header(View::Ledger).contains("2 ledger"));
        let p = view_header(View::Process);
        assert!(p.contains("3 process"), "{p}");
        assert!(p.contains("←→ atoms/telos"), "no sub-pane keys: {p}");
        assert!(p.contains("j/k scroll"), "{p}");
        assert!(!view_header(View::Comments).contains("browser"));
    }

    #[test]
    fn process_pane_toggles_atoms_and_telos() {
        // (AC-4) the action ←/→ invoke in the Process view.
        assert_eq!(ProcessPane::Atoms.toggled(), ProcessPane::Telos);
        assert_eq!(ProcessPane::Telos.toggled(), ProcessPane::Atoms);
        let mut a = app(&["telos/a"]);
        a.view = View::Process;
        assert_eq!(a.process_pane, ProcessPane::Atoms);
        a.process_pane = a.process_pane.toggled();
        assert_eq!(a.process_pane, ProcessPane::Telos);
    }

    #[test]
    fn comments_header_shows_the_navigation_legend() {
        // (AC-1) the file-switch and comment-move keys are visible in the header.
        let h = view_header(View::Comments);
        assert!(h.contains("←→ file"), "no file-switch hint: {h}");
        assert!(h.contains("j/k comment"), "no comment-move hint: {h}");
        // Other views do not carry the comment-move hint.
        assert!(!view_header(View::Ledger).contains("j/k comment"));
    }

    #[test]
    fn select_comment_file_moves_and_clamps() {
        // (AC-2) the action ←/→ (and [ ]) invoke: ±1, clamped at both ends.
        let mut a = app(&["telos/a"]);
        a.comment_files = vec![
            (PathBuf::from("a.rs"), 1),
            (PathBuf::from("b.rs"), 1),
            (PathBuf::from("c.rs"), 1),
        ];
        a.comment_file_selected = 0;
        a.select_comment_file(-1); // clamps at the first
        assert_eq!(a.comment_file_selected, 0);
        a.select_comment_file(1);
        a.select_comment_file(1);
        assert_eq!(a.comment_file_selected, 2);
        a.select_comment_file(1); // clamps at the last
        assert_eq!(a.comment_file_selected, 2);
    }

    // --- Comments view (P1) ---------------------------------------------------

    fn comments_tmp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("cospan-tui-{}-{}", std::process::id(), tag));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn write_sidecar(repo: &Path, src_rel: &str, comments: &[crate::comments::Comment]) {
        let path = repo.join(crate::comments::sidecar_path(src_rel));
        crate::comments::save(&path, comments).unwrap();
    }

    fn mk_comment(content: &str, line0: usize, body: &str) -> crate::comments::Comment {
        crate::comments::Comment {
            id: format!("c_{body}"),
            anchor: crate::comments::StoredAnchor::capture(content, line0, 2),
            body: body.into(),
            author: crate::comments::Author {
                who: "human".into(),
                id: "tester".into(),
            },
            created_at: 0,
            resolved: false,
            thread: Vec::new(),
        }
    }

    #[test]
    fn commented_files_discovers_only_files_with_a_sidecar() {
        // (AC-2) two commented files, one un-commented; discovery returns the two.
        let repo = comments_tmp("discover");
        let content = "fn a() {}\nfn b() {}\n";
        write_sidecar(&repo, "src/a.rs", &[mk_comment(content, 0, "one")]);
        write_sidecar(
            &repo,
            "src/b.rs",
            &[
                mk_comment(content, 0, "two"),
                mk_comment(content, 1, "three"),
            ],
        );
        // A plain source file with no sidecar must not appear.
        std::fs::write(repo.join("README.md"), "hi").unwrap();

        let found = commented_files(&repo);
        assert_eq!(
            found,
            vec![
                (PathBuf::from("src/a.rs"), 1),
                (PathBuf::from("src/b.rs"), 2),
            ]
        );
    }

    #[test]
    fn gutter_lines_marks_anchored_lines_and_lists_unresolvable() {
        // (AC-3, AC-5) an Anchored comment marks its line; an Unresolvable one
        // (span None) is returned in the list, never placed on a line.
        let content = "one\ntwo\nthree\n";
        let localized = vec![
            (
                mk_comment(content, 1, "on two"),
                Localization {
                    state: State::Anchored,
                    span: Some((1, 1)),
                    confidence: 1.0,
                },
            ),
            (
                mk_comment(content, 0, "lost"),
                Localization {
                    state: State::Unresolvable,
                    span: None,
                    confidence: 0.0,
                },
            ),
        ];
        let (lines, unresolved) = gutter_lines(content, &localized, 0);
        assert_eq!(lines.len(), 3);
        // Line index 1 (the anchored one) carries the ● marker; the others a space.
        let marker = |i: usize| lines[i].spans[0].content.to_string();
        assert_eq!(marker(1), "●");
        assert_eq!(marker(0), " ");
        assert_eq!(marker(2), " ");
        // The unresolvable comment is surfaced separately, not on any line.
        assert_eq!(unresolved.len(), 1);
        assert_eq!(unresolved[0].body, "lost");
    }

    #[test]
    fn wrap_text_wraps_and_keeps_long_words_intact() {
        // (AC-1)
        let w = wrap_text("the quick brown fox jumps", 9);
        assert!(w.iter().all(|l| l.chars().count() <= 9), "{w:?}");
        assert!(w.len() >= 2, "{w:?}");
        assert_eq!(wrap_text("", 5), vec![""]);
        let long = wrap_text("superlongword ok", 4);
        assert!(long.iter().any(|l| l == "superlongword"), "{long:?}");
    }

    fn line_text(l: &ratatui::text::Line) -> String {
        l.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn note_block_shows_header_body_resolved_and_reply_count() {
        // (AC-2)
        let content = "a\nb\n";
        let mut c = mk_comment(content, 0, "this body wraps across several lines here yes");
        c.resolved = true;
        c.thread.push(crate::comments::Reply {
            author: crate::comments::Author {
                who: "agent".into(),
                id: "claude".into(),
            },
            body: "r".into(),
            created_at: 1,
        });
        let loc = Localization {
            state: State::Drifted,
            span: Some((0, 0)),
            confidence: 0.8,
        };
        let lines = note_block(&c, &loc, 12, false);
        let joined = lines.iter().map(line_text).collect::<Vec<_>>().join("\n");
        assert!(joined.contains("@tester"), "{joined}");
        assert!(joined.contains("Drifted"), "{joined}");
        assert!(joined.contains("[resolved]"), "{joined}");
        assert!(joined.contains("+1 reply"), "{joined}");
        assert!(
            joined.contains("wraps"),
            "wrapped body text missing: {joined}"
        );
        assert!(lines.len() >= 3, "header + body + reply expected: {joined}");
    }

    #[test]
    fn reflow_pushes_code_down_for_a_multiline_note() {
        // (AC-3) a height-3 note at code line 1 shifts code line 2 down by two rows.
        use ratatui::text::Line;
        let code: Vec<Line> = ["l0", "l1", "l2", "l3"]
            .iter()
            .map(|s| Line::from(s.to_string()))
            .collect();
        let note = vec![Line::from("n0"), Line::from("n1"), Line::from("n2")];
        let (rows, note_rows) = reflow_rows(code, &[(0usize, 1usize, note)]);
        assert_eq!(rows.len(), 6);
        assert_eq!(line_text(&rows[1].0), "l1");
        assert_eq!(line_text(&rows[1].1), "n0"); // note's first line beside l1
        assert_eq!(line_text(&rows[2].0), ""); // reflow: code paused
        assert_eq!(line_text(&rows[2].1), "n1");
        assert_eq!(line_text(&rows[4].0), "l2"); // l2 pushed from row 2 to row 4
        assert_eq!(note_rows, vec![(0, 1)]); // note (loc idx 0) starts at row 1
    }

    #[test]
    fn unresolvable_comment_makes_no_note_but_stays_listed() {
        // (AC-4) span None -> no note in the column; still in the unresolvable list.
        let content = "one\ntwo\n";
        let localized = vec![
            (
                mk_comment(content, 0, "anchored"),
                Localization {
                    state: State::Anchored,
                    span: Some((0, 0)),
                    confidence: 1.0,
                },
            ),
            (
                mk_comment(content, 0, "lost"),
                Localization {
                    state: State::Unresolvable,
                    span: None,
                    confidence: 0.0,
                },
            ),
        ];
        let notes: Vec<(usize, usize)> = localized
            .iter()
            .enumerate()
            .filter_map(|(i, (_, loc))| loc.span.map(|(s, _)| (i, s)))
            .collect();
        assert_eq!(notes, vec![(0, 0)]); // only the anchored one
        let (_lines, unresolved) = gutter_lines(content, &localized, 0);
        assert_eq!(unresolved.len(), 1);
        assert_eq!(unresolved[0].body, "lost");
    }

    #[test]
    fn refresh_footer_loads_the_cache_and_tracks_width() {
        // (AC-3) the footer loads day's status-line variant and gates on width.
        let repo = comments_tmp("footer");
        std::fs::create_dir_all(repo.join(".day")).unwrap();
        std::fs::write(
            repo.join(".day/statusline.variants"),
            "#day-footer emoji 43\nhello\nworld\n",
        )
        .unwrap();
        let mut a = AppState::new(repo, fold_of(&[]), None);
        a.refresh_footer(50, true);
        assert_eq!(a.footer, vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(a.footer_width, 50);
        // Idempotent when width and cache are unchanged.
        a.refresh_footer(50, true);
        assert_eq!(a.footer, vec!["hello".to_string(), "world".to_string()]);
    }

    #[test]
    fn thread_lines_render_root_reply_and_resolved() {
        // (AC-3) the strip shows the root body, an attributed indented reply, and
        // a [resolved] marker.
        let content = "one\ntwo\n";
        let mut c = mk_comment(content, 0, "root body");
        c.resolved = true;
        c.thread.push(crate::comments::Reply {
            author: crate::comments::Author {
                who: "agent".into(),
                id: "claude".into(),
            },
            body: "a reply".into(),
            created_at: 1,
        });
        let loc = Localization {
            state: State::Anchored,
            span: Some((0, 0)),
            confidence: 1.0,
        };
        let texts: Vec<String> = thread_lines(&c, &loc)
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect();
        assert!(texts.iter().any(|t| t.contains("root body")), "{texts:?}");
        assert!(
            texts
                .iter()
                .any(|t| t.contains("@claude:") && t.contains("a reply")),
            "reply not attributed/indented: {texts:?}"
        );
        assert!(texts.iter().any(|t| t.contains("[resolved]")), "{texts:?}");
    }

    #[test]
    fn a_comment_relocalizes_to_the_moved_line() {
        // (AC-4) anchor on a line, then feed content where it moved; the recomputed
        // localization reports the new span rather than the stale stored line.
        let before = "alpha\ntarget line\nbeta\n";
        let mut c = mk_comment(before, 1, "on target");
        let after = "new header\nanother\nalpha\ntarget line\nbeta\n";
        let loc = crate::comments::localize_and_update(&mut c, after);
        assert_ne!(loc.state, State::Unresolvable);
        assert_eq!(loc.span, Some((3, 3))); // moved from line 2 to line 4 (0-based 3)
    }

    #[test]
    fn refresh_comments_is_safe_with_no_commented_files() {
        // (AC-5) the per-file refresh no-ops cleanly when nothing is commented.
        let mut a = app(&["telos/a"]);
        a.comment_files.clear();
        a.refresh_comments();
        assert!(a.comment_localized.is_empty());
        assert!(a.comment_content.is_empty());
    }

    #[test]
    fn comments_load_ordered_by_anchored_line_not_creation() {
        // Added bottom-first, but the cursor order must follow the file top-down,
        // so `j`/Down moves the anchored line downward.
        let repo = comments_tmp("order");
        let content = "l0\nl1\nl2\nl3\n";
        std::fs::create_dir_all(repo.join("src")).unwrap();
        std::fs::write(repo.join("src/a.rs"), content).unwrap();
        write_sidecar(
            &repo,
            "src/a.rs",
            &[
                mk_comment(content, 3, "on l3"),
                mk_comment(content, 1, "on l1"),
            ],
        );
        let mut a = AppState::new(repo, fold_of(&[]), None);
        a.refresh_comments();
        let starts: Vec<usize> = a
            .comment_localized
            .iter()
            .map(|(_, loc)| loc.span.unwrap().0)
            .collect();
        assert_eq!(starts, vec![1, 3], "comments not ordered by line");
    }

    #[test]
    fn refresh_comments_loads_the_selected_file() {
        // (AC-4/AC-5 happy path) the selected file's content + localizations load.
        let repo = comments_tmp("load-view");
        let content = "fn a() {}\nfn b() {}\n";
        std::fs::create_dir_all(repo.join("src")).unwrap();
        std::fs::write(repo.join("src/a.rs"), content).unwrap();
        write_sidecar(&repo, "src/a.rs", &[mk_comment(content, 1, "on b")]);

        let mut a = AppState::new(repo, fold_of(&[]), None);
        a.refresh_comments();
        assert_eq!(a.comment_content, content);
        assert_eq!(a.comment_localized.len(), 1);
        assert_eq!(a.comment_localized[0].1.state, State::Anchored);
        assert_eq!(a.comment_loaded.as_deref(), Some(Path::new("src/a.rs")));
    }

    #[test]
    fn selecting_a_deleted_source_clears_content_and_lists_its_comments() {
        // The BLOCK regression: a commented file whose source was deleted must not
        // leave the previous file's content on screen; its comments become
        // Unresolvable and reach the resolve-by-hand list (honest-ambiguity).
        let repo = comments_tmp("deleted-src");
        let content = "one\ntwo\nthree\n";
        std::fs::create_dir_all(repo.join("src")).unwrap();
        std::fs::write(repo.join("src/real.rs"), content).unwrap();
        write_sidecar(&repo, "src/real.rs", &[mk_comment(content, 0, "real")]);
        // A sidecar whose source file does NOT exist.
        write_sidecar(
            &repo,
            "src/gone.rs",
            &[
                mk_comment(content, 0, "gone-a"),
                mk_comment(content, 1, "gone-b"),
            ],
        );

        // Files sort as [src/gone.rs, src/real.rs]; load real first.
        let mut a = AppState::new(repo, fold_of(&[]), None);
        a.comment_file_selected = 1;
        a.refresh_comments();
        assert_eq!(a.comment_content, content);

        // Switch to the deleted-source file and refresh.
        a.select_comment_file(-1);
        a.refresh_comments();
        assert!(
            a.comment_content.is_empty(),
            "stale content from the previous file leaked: {:?}",
            a.comment_content
        );
        assert_eq!(a.comment_loaded.as_deref(), Some(Path::new("src/gone.rs")));
        assert_eq!(a.comment_localized.len(), 2);
        assert!(
            a.comment_localized
                .iter()
                .all(|(_, loc)| loc.state == State::Unresolvable),
            "a deleted-source comment should be Unresolvable"
        );
        // gutter_lines surfaces them in the resolve-by-hand list, on no line.
        let (lines, unresolved) =
            gutter_lines(&a.comment_content, &a.comment_localized, a.comment_selected);
        assert!(lines.is_empty());
        assert_eq!(unresolved.len(), 2);
    }

    #[test]
    fn process_view_lines_are_note_led_and_bound_the_scroll() {
        // Empty snapshot: note + the "no atoms" line.
        let empty = process_view_lines(&ProcessSnapshot::default(), ProcessPane::Atoms);
        assert!(empty[0].contains("machine-readable day"), "{:?}", empty[0]);
        assert!(empty.iter().any(|l| l.contains("no atoms")));

        let snap = ProcessSnapshot {
            atoms: vec![substrate::Atom {
                slug: "build".into(),
                inputs: vec!["d".into()],
                outputs: vec!["c".into()],
                next: vec!["r".into()],
                ..Default::default()
            }],
            teloi: vec![],
            tensions: vec![],
        };
        let lines = process_view_lines(&snap, ProcessPane::Atoms);
        assert!(lines.iter().any(|l| l.contains("build")));
        // (AC-3) the Telos sub-pane yields telos content, not atoms.
        let tsnap = ProcessSnapshot {
            atoms: vec![],
            teloi: vec![substrate::TelosView {
                slug: "p0-spine".into(),
                title: "P0".into(),
                statement: "the spine runs".into(),
                witnesses: vec!["code-change".into()],
            }],
            tensions: vec![],
        };
        let tlines = process_view_lines(&tsnap, ProcessPane::Telos);
        assert!(tlines.iter().any(|l| l.contains("p0-spine")), "{tlines:?}");
        assert!(tlines.iter().any(|l| l.contains("witnesses")));
        // The clamp the key handler applies can never slice past the end.
        let max = lines.len().saturating_sub(1);
        assert!((999usize).min(max) < lines.len());
    }

    fn atom_with(slug: &str, next: &[&str]) -> Atom {
        Atom {
            slug: slug.into(),
            next: next.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn layout_atoms_columns_by_depth_and_rows_by_fan() {
        // (AC-1) a linear chain lays out in one row across columns.
        let chain = vec![
            atom_with("a", &["b"]),
            atom_with("b", &["c"]),
            atom_with("c", &[]),
        ];
        assert_eq!(
            layout_atoms(&chain),
            vec![
                Placed { col: 0, row: 0 },
                Placed { col: 1, row: 0 },
                Placed { col: 2, row: 0 },
            ]
        );
        // A fan puts the two targets in the same column, stacked in rows.
        let fan = vec![
            atom_with("a", &["b", "c"]),
            atom_with("b", &[]),
            atom_with("c", &[]),
        ];
        let p = layout_atoms(&fan);
        assert_eq!(p[0], Placed { col: 0, row: 0 });
        assert_eq!(p[1], Placed { col: 1, row: 0 });
        assert_eq!(p[2], Placed { col: 1, row: 1 });
    }

    #[test]
    fn atom_flowchart_boxes_arrows_selection_and_backedges() {
        // (AC-2) a→b, b revisits a; b is selected.
        let atoms = vec![
            atom_with("a", &["b"]),
            Atom {
                slug: "b".into(),
                revisits: vec!["a".into()],
                ..Default::default()
            },
        ];
        let out = atom_flowchart(&atoms, 1).join("\n");
        assert!(out.contains("▶"), "no forward arrow: {out}");
        assert!(out.contains('╔'), "selected box not double-bordered: {out}");
        assert!(
            out.contains('┌'),
            "unselected box not single-bordered: {out}"
        );
        // Empty atoms: the declared-empty note, not a panic.
        assert_eq!(
            atom_flowchart(&[], 0),
            vec!["(no atoms declared)".to_string()]
        );
    }

    #[test]
    fn back_edges_route_in_single_row_and_list_in_multi_row() {
        // (AC-2b) single-row: a→b, b revisits a → the back-edge is ROUTED as a
        // dashed arrow (no ↻ text list).
        let single = vec![
            atom_with("a", &["b"]),
            Atom {
                slug: "b".into(),
                revisits: vec!["a".into()],
                ..Default::default()
            },
        ];
        let out = atom_flowchart(&single, 1).join("\n");
        assert!(
            out.contains('┄') && out.contains('▲'),
            "back-edge not routed: {out}"
        );
        assert!(
            out.contains('╰') || out.contains('╯'),
            "no routed corner: {out}"
        );
        assert!(
            !out.contains('↻'),
            "single-row back-edge should not be text-listed: {out}"
        );

        // Multi-row: a fan a→b, a→c puts c at row 1; the off-row edge stays a ⇢
        // text list (routing across rows would cross boxes).
        let fan = vec![
            atom_with("a", &["b", "c"]),
            atom_with("b", &[]),
            atom_with("c", &[]),
        ];
        let fout = atom_flowchart(&fan, 0).join("\n");
        assert!(
            fout.contains("a ⇢ c"),
            "off-row edge not listed in multi-row: {fout}"
        );

        // A self-revisiting atom (sx == tx) must not panic on the routed lane.
        let selfy = vec![Atom {
            slug: "loop".into(),
            revisits: vec!["loop".into()],
            ..Default::default()
        }];
        let _ = atom_flowchart(&selfy, 0); // must not panic
    }

    #[test]
    fn atom_select_clamps_within_the_atom_set() {
        // (AC-3)
        let mut a = app(&[]);
        a.fold.process.atoms = vec![atom_with("x", &["y"]), atom_with("y", &[])];
        a.view = View::Process;
        a.atom_select(-1);
        assert_eq!(a.atom_selected, 0);
        a.atom_select(1);
        a.atom_select(1); // clamps at the last
        assert_eq!(a.atom_selected, 1);
        // The re-render carries the selection (a double-bordered box).
        assert!(atom_flowchart(&a.fold.process.atoms, a.atom_selected)
            .join("\n")
            .contains('╔'));
    }

    #[test]
    fn atom_detail_shows_every_field() {
        // (AC-4)
        let a = Atom {
            slug: "build".into(),
            inputs: vec!["design-doc".into()],
            outputs: vec!["code-change".into()],
            next: vec!["review".into()],
            done: vec!["passing-tests".into()],
            revisits: vec!["design".into()],
        };
        let out = atom_detail(&a).join("\n");
        for needle in [
            "build",
            "design-doc",
            "code-change",
            "review",
            "passing-tests",
            "design",
        ] {
            assert!(out.contains(needle), "missing {needle}: {out}");
        }
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
