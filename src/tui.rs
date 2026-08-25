//! L3 — the interactive view layer.
//!
//! The `watch-repo` browser, drawn with ratatui and driven by one poll-and-fold
//! loop. Everything rendered is a projection of the `substrate::Fold` — the only
//! state this layer owns is the selection cursor (`telos/kan-is-truth`). The loop
//! never subscribes: a single `event::poll(tick)` is both the key wait and the
//! re-fold tick, and the fold is rebuilt only when `.kan/log/HEAD` changes
//! (`telos/poll-dont-subscribe`).

use crate::comments::{self, Comment};
use crate::filetree;
use crate::substrate::{
    self, is_day_subject, namespace, short_cid, Atom, Claim, Fold, ProcessSnapshot,
};
use crate::transcripts;
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

/// One row of the Comments file tree (S2): a collapsible `Dir` or a selectable
/// `File` carrying its git status. `guide` is the pre-rendered ancestry prefix
/// (`│  ` / `   ` per ancestor, then a `├─ ` / `└─ ` connector) that draws the
/// nesting lines and keeps every row at the same depth aligned.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileRow {
    Dir {
        path: String,
        guide: String,
        collapsed: bool,
    },
    File {
        path: String,
        guide: String,
        status: filetree::GitStatus,
    },
}

/// Which pane of the Comments view has focus: the file tree (rail) or the
/// commented gutter. `Enter` on a file descends Tree -> Comments; `Esc` ascends.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommentFocus {
    Tree,
    Comments,
}

/// A file-tree node: a leaf carries its git status; a node with children is a
/// directory.
#[derive(Default)]
struct FileNode {
    status: Option<filetree::GitStatus>,
    children: std::collections::BTreeMap<String, FileNode>,
}

/// Flatten file entries into a collapsible tree of rows, sorted by path, skipping
/// the subtree of any collapsed directory (`dir:<path>` in `collapsed`).
pub fn build_file_rows(
    entries: &[filetree::FileEntry],
    expanded: &HashSet<String>,
) -> Vec<FileRow> {
    let mut root = FileNode::default();
    for e in entries {
        let comps: Vec<String> = e
            .path
            .iter()
            .map(|c| c.to_string_lossy().to_string())
            .collect();
        let n = comps.len();
        let mut node = &mut root;
        for (i, seg) in comps.into_iter().enumerate() {
            node = node.children.entry(seg).or_default();
            if i == n - 1 {
                node.status = Some(e.status);
            }
        }
    }
    // Directories are collapsed by DEFAULT; a dir is walked into only when its
    // `dir:<path>` key is in `expanded`. `ancestors` is the guide prefix drawn to
    // the left of this level (`│  ` where an ancestor has more siblings below,
    // `   ` where it was the last child).
    fn walk(
        node: &FileNode,
        prefix: &str,
        ancestors: &str,
        expanded: &HashSet<String>,
        rows: &mut Vec<FileRow>,
    ) {
        let n = node.children.len();
        for (i, (seg, child)) in node.children.iter().enumerate() {
            let last = i == n - 1;
            let full = if prefix.is_empty() {
                seg.clone()
            } else {
                format!("{prefix}/{seg}")
            };
            let guide = format!("{ancestors}{}", if last { "└─ " } else { "├─ " });
            if child.children.is_empty() {
                rows.push(FileRow::File {
                    path: full,
                    guide,
                    status: child.status.unwrap_or(filetree::GitStatus::Clean),
                });
            } else {
                let is_open = expanded.contains(&format!("dir:{full}"));
                rows.push(FileRow::Dir {
                    path: full.clone(),
                    guide,
                    collapsed: !is_open,
                });
                if is_open {
                    let child_anc = format!("{ancestors}{}", if last { "   " } else { "│  " });
                    walk(child, &full, &child_anc, expanded, rows);
                }
            }
        }
    }
    let mut rows = Vec::new();
    walk(&root, "", "", expanded, &mut rows);
    rows
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
    /// The highlighted telos in the telos list.
    pub telos_selected: usize,
    /// Whether the Process pane is drilled into the selected item's detail
    /// (shared by the Atoms and Telos sub-panes).
    pub process_detail: bool,
    pub atom_scroll: usize,
    /// Scroll offset into the drilled telos detail (reused; the list itself uses
    /// `telos_selected`).
    pub telos_scroll: usize,
    /// Keys of collapsed tree nodes (`sec:<label>` / `path:<prefix>`).
    pub collapsed: HashSet<String>,

    // --- Comments view: a repo file tree (S2) beside a live commented gutter. ---
    /// The repo's browsable files with git status, rebuilt on entering the view,
    /// on a re-fold, and when `.git/index` changes.
    pub file_entries: Vec<filetree::FileEntry>,
    /// The file tree flattened to visible rows (dirs are collapsed by default and
    /// expanded via `file_expanded`).
    pub file_rows: Vec<FileRow>,
    /// Cursor into `file_rows` (the rail).
    pub file_selected: usize,
    /// Expanded directory keys (`dir:<path>`); everything else is collapsed.
    pub file_expanded: HashSet<String>,
    /// The repo-relative file the operator has opened for commenting, or `None`
    /// before one is chosen. Drives `comment_content`/`comment_localized`.
    pub open_file: Option<PathBuf>,
    /// `.git/index` mtime, the cheap gate for rebuilding the file list.
    file_index_mtime: Option<SystemTime>,
    /// Whether the rail (tree) or the gutter (comments) has focus.
    pub comment_focus: CommentFocus,
    /// Cached content of the open file (for the gutter render).
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
    /// The active authoring interaction over the Comments view (pick-line /
    /// compose / reply / edit), or `None` in read-navigate mode. See [`Editing`].
    pub editing: Option<Editing>,
    /// A transient one-line status for the Comments view (e.g. "not your
    /// comment"), shown until the next authoring action clears it. Surfaced in
    /// the footer now that the bottom strip is gone.
    pub comment_msg: Option<String>,
    /// Whether the file tray is held open while reading a file (Comments focus).
    /// In Tree focus the rail always shows (you are browsing); toggled with `t`.
    pub tray_open: bool,
    /// Whether the thread popup overlay is open over the selected comment. It
    /// reads the full untruncated thread and still accepts r/e/d/x on it.
    pub popup_open: bool,
    /// The open file's working-tree diff (vs HEAD), recomputed with the content on
    /// the same mtime gate — drives the editor pane's change highlighting.
    pub file_diff: crate::diff::FileDiff,
    /// Whether the diff highlighting is shown (toggled with `D`, default on).
    pub diff_on: bool,
    /// The wide note/code viewport top (in reflowed-row space), followed
    /// stickily: it moves only when the selected comment scrolls out of view, so
    /// stepping between visible comments does not snap the selection to the top.
    /// A render cache updated in `draw`, hence `Cell` behind the `&AppState` draw
    /// takes; reset to 0 when the open file changes.
    note_scroll: std::cell::Cell<usize>,

    // --- Chat view: cross-harness session buffers from transcripts (read-only). ---
    /// The repo's discovered sessions across all harnesses, newest-active first.
    pub chat_sessions: Vec<transcripts::SessionHandle>,
    /// Index into `chat_sessions`.
    pub chat_selected: usize,
    /// The read body of the selected session (its ordered turns).
    pub chat_session: Option<transcripts::Session>,
    /// Per-session mtime the reader is "caught up" to — set when a session is
    /// read (opened, or tailed at the bottom). A session whose `last_active` is
    /// newer than this carries a "new activity" dot in the rail, and is re-read
    /// only when the reader is at its bottom (no yank while scrolled up).
    chat_seen: std::collections::HashMap<String, SystemTime>,
    /// The session id whose body is loaded into `chat_session`, so a switch
    /// forces a re-read (distinct from the *selected* index).
    pub chat_loaded: Option<String>,
    /// The loaded session's `last_active` when it was read — so a re-read that
    /// preserves the cursor (a new turn appended) is told apart from one that
    /// resets it (a switch to a different session).
    chat_loaded_active: Option<SystemTime>,
    /// Last-seen aggregate change signal — the transcript re-read gate.
    chat_signal: Option<SystemTime>,
    /// Top visible *line* of the conversation (`j`/`k`/PgUp/PgDn scroll it);
    /// `Shift`+`↑`/`↓` jump between messages.
    pub chat_scroll: usize,
    /// When true, the view stays pinned to the bottom (newest turn) — the
    /// default on opening a session, and it re-pins as turns arrive. Scrolling up
    /// releases it; scrolling back to the bottom re-arms it (tail-follow).
    pub chat_follow: bool,
    /// Event indices whose collapsed turn (thinking / tool / sidechain) the
    /// operator has expanded with `Enter`.
    pub chat_expanded: HashSet<usize>,
    /// Session group keys (Codex `session_id`) whose subagents are shown in the
    /// rail. Default (absent) = collapsed, so a director's many subagents stay
    /// folded under it until expanded.
    pub chat_expanded_groups: HashSet<String>,
    /// The cached styled conversation rows, rebuilt by `chat_relayout` only when
    /// the session, expansions, or width change — not per frame (markdown is
    /// parsed once per change, not 4×/second).
    chat_rows: Vec<ChatRow>,
    /// True when `chat_rows` needs rebuilding; set by the read/expand/resize
    /// mutators and cleared by `chat_relayout`.
    chat_dirty: bool,
    /// The width `chat_rows` was laid out for, to detect a resize.
    chat_layout_w: usize,
    /// Total rendered conversation lines, from the last layout — clamps scroll.
    pub chat_total_lines: usize,
    /// `(start line, event index)` for each message, from the last layout —
    /// drives message-skip (`Shift`+arrows) and current-message detection.
    pub chat_msg_starts: Vec<(usize, usize)>,

    // --- Viewport: the body area's size, refreshed each tick for paging. ---
    body_w: u16,
    body_h: u16,

    // --- Footer: day's status line, width-matched from its cache. ---
    pub footer: Vec<String>,
    footer_mtime: Option<SystemTime>,
    footer_width: u16,
}

/// The top-level tabs, switched with `1`/`2`/`3`/`4` or `Tab`. `Chat` is the
/// cross-harness session buffers; `Ledger` is the kan claim browser; `Process`
/// houses the atoms/telos sub-panes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
    Chat,
    Comments,
    Ledger,
    Process,
}

impl View {
    pub fn next(self) -> View {
        match self {
            View::Chat => View::Comments,
            View::Comments => View::Ledger,
            View::Ledger => View::Process,
            View::Process => View::Chat,
        }
    }

    pub fn from_digit(c: char) -> Option<View> {
        match c {
            '1' => Some(View::Chat),
            '2' => Some(View::Comments),
            '3' => Some(View::Ledger),
            '4' => Some(View::Process),
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

/// An in-progress authoring action over the Comments view. `None` on `AppState`
/// is read/navigate mode; `Some(_)` captures every keystroke until it commits
/// (`Ctrl-S`) or cancels (`Esc`), so a letter like `q` types into the buffer
/// rather than quitting the app.
pub enum Editing {
    /// Choosing the source line a new comment will anchor to (the gutter shows
    /// the cursor). `Enter` opens the compose; `Esc` cancels.
    PickLine { cursor: usize },
    /// Composing body text; `kind` says what the committed text becomes.
    Compose { kind: ComposeKind, buf: TextBuf },
}

/// What a composed body becomes on commit.
pub enum ComposeKind {
    /// A new comment anchored at this 0-based source line.
    NewComment { line: usize },
    /// A reply appended to the comment with this id.
    Reply { id: String },
    /// A rewrite of the body of the comment with this id (author-gated).
    Edit { id: String },
}

/// A minimal multi-line text buffer with one insertion cursor — enough to author
/// a paragraph of comment prose in the TUI (ratatui ships no text input). The
/// cursor is a *character* index into `text`, so a multi-byte character moves and
/// deletes as one unit.
#[derive(Default, Clone)]
pub struct TextBuf {
    pub text: String,
    pub cursor: usize,
}

impl TextBuf {
    /// A buffer pre-filled with `s`, cursor at the end — for editing an existing body.
    pub fn prefilled(s: &str) -> TextBuf {
        TextBuf {
            text: s.to_string(),
            cursor: s.chars().count(),
        }
    }

    /// Byte offset of character index `i` (or the string end for `i` past the last).
    fn byte_of(&self, i: usize) -> usize {
        self.text
            .char_indices()
            .nth(i)
            .map(|(b, _)| b)
            .unwrap_or(self.text.len())
    }

    fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    /// Insert a character (including `'\n'`) at the cursor and advance past it.
    pub fn insert(&mut self, ch: char) {
        let b = self.byte_of(self.cursor);
        self.text.insert(b, ch);
        self.cursor += 1;
    }

    /// Delete the character before the cursor, if any.
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            let b = self.byte_of(self.cursor - 1);
            self.text.remove(b);
            self.cursor -= 1;
        }
    }

    pub fn left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.char_count());
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.char_count();
    }

    /// The `(row, col)` of the cursor in logical lines (0-based), splitting on `'\n'`.
    pub fn row_col(&self) -> (usize, usize) {
        let pre = &self.text[..self.byte_of(self.cursor)];
        let row = pre.matches('\n').count();
        let col = pre
            .rsplit('\n')
            .next()
            .map(|s| s.chars().count())
            .unwrap_or(0);
        (row, col)
    }

    /// Move the cursor up one line, keeping the column where the line is long enough.
    pub fn up(&mut self) {
        self.move_vert(-1);
    }

    /// Move the cursor down one line, keeping the column where the line is long enough.
    pub fn down(&mut self) {
        self.move_vert(1);
    }

    fn move_vert(&mut self, d: isize) {
        let (row, col) = self.row_col();
        let lines: Vec<&str> = self.text.split('\n').collect();
        let target_row =
            (row as isize + d).clamp(0, lines.len().saturating_sub(1) as isize) as usize;
        if target_row == row {
            return;
        }
        let target_col = col.min(lines[target_row].chars().count());
        // Char index = every char of the preceding lines (+1 per `'\n'`) then the column.
        let mut idx = 0;
        for l in &lines[..target_row] {
            idx += l.chars().count() + 1;
        }
        self.cursor = (idx + target_col).min(self.char_count());
    }
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

/// Whether the file-tree rail is shown in the Comments view: always while
/// browsing (Tree focus), and while reading a file (Comments focus) only if the
/// tray was toggled open with `t` — and never in a narrow terminal, which has no
/// room for it beside the code. Pure, so the tray rule is testable.
pub fn rail_visible(focus: CommentFocus, tray_open: bool, wide: bool) -> bool {
    wide && (focus == CommentFocus::Tree || tray_open)
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
            telos_selected: 0,
            process_detail: false,
            atom_scroll: 0,
            telos_scroll: 0,
            collapsed: HashSet::new(),
            file_entries: Vec::new(),
            file_rows: Vec::new(),
            file_selected: 0,
            file_expanded: HashSet::new(),
            open_file: None,
            file_index_mtime: None,
            comment_focus: CommentFocus::Tree,
            comment_content: String::new(),
            comment_localized: Vec::new(),
            comment_loaded: None,
            comment_mtime: None,
            comment_selected: 0,
            comment_scroll: 0,
            editing: None,
            comment_msg: None,
            tray_open: false,
            popup_open: false,
            file_diff: crate::diff::FileDiff::empty(),
            diff_on: true,
            note_scroll: std::cell::Cell::new(0),
            chat_sessions: Vec::new(),
            chat_selected: 0,
            chat_session: None,
            chat_loaded: None,
            chat_loaded_active: None,
            chat_signal: None,
            chat_seen: std::collections::HashMap::new(),
            chat_scroll: 0,
            chat_follow: true,
            chat_expanded: HashSet::new(),
            chat_expanded_groups: HashSet::new(),
            chat_rows: Vec::new(),
            chat_dirty: true,
            chat_layout_w: 0,
            chat_total_lines: 0,
            chat_msg_starts: Vec::new(),
            body_w: 0,
            body_h: 0,
            footer: Vec::new(),
            footer_mtime: None,
            footer_width: 0,
        };
        s.rebuild_rows();
        s.selected = s.first_subject_index().unwrap_or(0);
        s.reload_files();
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

    /// Rebuild the browsable file list (git status) and its tree rows. Called on
    /// entering the Comments view, on a re-fold, and when `.git/index` changes.
    pub fn reload_files(&mut self) {
        self.file_entries = filetree::list(&self.repo);
        self.file_index_mtime = std::fs::metadata(self.repo.join(".git/index"))
            .ok()
            .and_then(|m| m.modified().ok());
        self.rebuild_file_rows();
    }

    /// Reflatten `file_rows` from the cached entries and current expansion set,
    /// keeping the cursor in range.
    pub fn rebuild_file_rows(&mut self) {
        self.file_rows = build_file_rows(&self.file_entries, &self.file_expanded);
        self.file_selected = self
            .file_selected
            .min(self.file_rows.len().saturating_sub(1));
    }

    /// Cheap per-tick gate: rebuild the file list when `.git/index` changed (a
    /// stage/commit), without a subprocess unless it did (`telos/poll-dont-subscribe`).
    pub fn refresh_files(&mut self) {
        // Gate purely on the index mtime: the initial load already ran (via
        // `enter_comments` / `AppState::new`), so a non-git or empty repo — where
        // the entry list stays empty forever — must NOT re-shell git every tick.
        let mtime = std::fs::metadata(self.repo.join(".git/index"))
            .ok()
            .and_then(|m| m.modified().ok());
        if should_refold(self.file_index_mtime, mtime) {
            self.reload_files();
        }
    }

    /// Enter the Comments view: rebuild the file tree, focus the rail, and preview
    /// the file under the cursor.
    pub fn enter_comments(&mut self) {
        self.reload_files();
        self.comment_focus = CommentFocus::Tree;
        self.preview_selected();
    }

    /// Move the file-tree cursor, clamped, then live-preview the file it lands on.
    pub fn file_move(&mut self, delta: isize) {
        let n = self.file_rows.len();
        if n == 0 {
            return;
        }
        self.file_selected =
            (self.file_selected as isize + delta).clamp(0, n as isize - 1) as usize;
        self.preview_selected();
    }

    /// Load the file under the cursor into the gutter *without* changing focus —
    /// the live preview as you browse. A directory row leaves the last preview up.
    pub fn preview_selected(&mut self) {
        if let Some(FileRow::File { path, .. }) = self.file_rows.get(self.file_selected).cloned() {
            self.set_preview(PathBuf::from(path));
        }
    }

    /// Point the gutter at `rel`, loading it only when it differs from what is
    /// already shown (so scrolling the rail doesn't thrash a re-read).
    fn set_preview(&mut self, rel: PathBuf) {
        if self.open_file.as_deref() == Some(rel.as_path()) {
            return;
        }
        self.open_file = Some(rel);
        self.comment_selected = 0;
        self.comment_scroll = 0;
        self.note_scroll.set(0);
        self.comment_msg = None;
        self.reload_after_write(); // comment_loaded now differs -> forces the read
    }

    /// `Enter` in the tree: toggle a directory's expansion, or focus the gutter on
    /// the file under the cursor (already previewed) so it can be commented on.
    pub fn file_activate(&mut self) {
        match self.file_rows.get(self.file_selected).cloned() {
            Some(FileRow::Dir { path, .. }) => {
                let key = format!("dir:{path}");
                if !self.file_expanded.remove(&key) {
                    self.file_expanded.insert(key);
                }
                self.rebuild_file_rows();
            }
            Some(FileRow::File { path, .. }) => self.open_path(PathBuf::from(path)),
            None => {}
        }
    }

    /// Open a repo-relative file into the gutter and give it focus for commenting.
    /// Opening collapses the tray so the code + notes take the full width; press
    /// `t` to bring the tree back (REQ-1: closed by default while reading).
    pub fn open_path(&mut self, rel: PathBuf) {
        self.set_preview(rel);
        self.comment_focus = CommentFocus::Comments;
        self.tray_open = false;
    }

    /// Toggle the file tray held open while reading a file. In Tree focus the rail
    /// always shows; this only bites in Comments focus (see [`rail_visible`]).
    pub fn toggle_tray(&mut self) {
        self.tray_open = !self.tray_open;
    }

    /// Toggle the working-tree diff highlighting in the code pane.
    pub fn toggle_diff(&mut self) {
        self.diff_on = !self.diff_on;
    }

    /// Open the thread popup over the selected comment, if there is one to read.
    pub fn open_thread_popup(&mut self) {
        if self.comment_localized.get(self.comment_selected).is_some() {
            self.popup_open = true;
        }
    }

    /// Route a key while the thread popup is open. `r`/`e` hand off to the
    /// full-screen composer (so the popup closes first), `d` deletes and closes,
    /// `x` resolves in place and keeps it open, Esc/Enter close it.
    pub fn handle_popup_key(&mut self, code: crossterm::event::KeyCode) {
        use crossterm::event::KeyCode;
        match code {
            KeyCode::Esc | KeyCode::Enter => self.popup_open = false,
            KeyCode::Char('r') => {
                self.popup_open = false;
                self.begin_reply();
            }
            KeyCode::Char('e') => {
                self.popup_open = false;
                self.begin_edit();
            }
            KeyCode::Char('d') => {
                self.popup_open = false;
                self.delete_selected();
            }
            KeyCode::Char('x') => self.toggle_resolve_selected(),
            _ => {}
        }
    }

    /// Refresh the open file's comment localizations. Re-reads when the open file
    /// differs from the loaded one (first open / switch) or its content changed
    /// since last tick; otherwise one `stat` and early return
    /// (`telos/poll-dont-subscribe`). A missing source reads as empty, so its
    /// comments localize to `Unresolvable` rather than leaving stale content on
    /// screen (`honest-ambiguity`).
    pub fn refresh_comments(&mut self) {
        let Some(rel) = self.open_file.clone() else {
            self.comment_content.clear();
            self.comment_localized.clear();
            self.comment_loaded = None;
            self.comment_mtime = None;
            self.file_diff = crate::diff::FileDiff::empty();
            return;
        };
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
        let anchors_before: Vec<comments::StoredAnchor> =
            cs.iter().map(|c| c.anchor.clone()).collect();
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
        // Persist the re-anchored last-seen state (like `cospan comments`) ONLY when
        // a comment exists and an anchor actually moved. Browsing a sidecar-less
        // file must not create an empty sidecar, and an unchanged file must not be
        // rewritten on every switch — both were per-switch disk I/O.
        let reanchored = cs.iter().zip(&anchors_before).any(|(c, a)| c.anchor != *a);
        if !cs.is_empty() && reanchored {
            let _ = comments::save(&sidecar, &cs);
        }
        self.comment_content = content;
        // Recompute the working-tree diff on the same gate that reloaded the
        // content, so the two never drift and git runs only on a real change.
        let status = self
            .file_entries
            .iter()
            .find(|e| e.path == rel)
            .map(|e| e.status)
            .unwrap_or(filetree::GitStatus::Modified);
        self.file_diff = crate::diff::FileDiff::compute(
            &self.repo,
            &rel,
            status,
            self.comment_content.lines().count(),
        );
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

    /// Re-discover the repo's sessions across harnesses. Called on entering the
    /// Chat view so newly-started sessions appear.
    pub fn reload_chat_sessions(&mut self) {
        self.chat_sessions = transcripts::discover_all(&self.repo);
        self.chat_selected = self
            .chat_selected
            .min(self.chat_sessions.len().saturating_sub(1));
        // Seed "caught up" at the current mtimes so nothing shows a stale dot on
        // first open; genuinely new activity after this raises the dot.
        for h in &self.chat_sessions {
            if let Some(t) = h.last_active {
                self.chat_seen.entry(h.id.clone()).or_insert(t);
            }
        }
    }

    /// Whether a session has activity newer than the reader has caught up to —
    /// the rail's "new activity" dot.
    pub fn chat_session_stale(&self, h: &transcripts::SessionHandle) -> bool {
        match (h.last_active, self.chat_seen.get(&h.id)) {
            (Some(cur), Some(&seen)) => cur > seen,
            (Some(_), None) => true, // appeared after we seeded — unseen
            _ => false,
        }
    }

    /// Refresh the Chat view. Re-enumerates sessions only when the aggregate
    /// transcript change signal advances (a new turn or session) — keeping the
    /// cursor on the same session by id across a re-sort. The selected session's
    /// body is then re-read per `chat_reread_plan`: a switch to a *different*
    /// session resets the cursor and expansions; a new turn appended to the
    /// *same* session re-reads but preserves them (appends land at the end, so
    /// existing event indices are stable); an unrelated session changing
    /// disturbs neither. One stat-driven gate, no push channel
    /// (`telos/poll-dont-subscribe`); the read is pure projection, nothing is
    /// written back (`telos/kan-is-truth`).
    pub fn refresh_chat(&mut self) {
        let signal = transcripts::change_signal(&self.repo);
        if should_refold(self.chat_signal, signal) {
            let prev_id = self
                .chat_sessions
                .get(self.chat_selected)
                .map(|h| h.id.clone());
            self.chat_sessions = transcripts::discover_all(&self.repo);
            self.chat_signal = signal;
            if let Some(id) = prev_id {
                if let Some(i) = self.chat_sessions.iter().position(|h| h.id == id) {
                    self.chat_selected = i;
                }
            }
        }
        if self.chat_sessions.is_empty() {
            self.chat_session = None;
            self.chat_loaded = None;
            self.chat_loaded_active = None;
            self.chat_dirty = true;
            return;
        }
        self.chat_selected = self.chat_selected.min(self.chat_sessions.len() - 1);
        let handle = &self.chat_sessions[self.chat_selected];
        match chat_reread_plan(
            self.chat_loaded.as_deref(),
            self.chat_loaded_active,
            &handle.id,
            handle.last_active,
        ) {
            ChatReread::None => {}
            ChatReread::Append if self.chat_follow => {
                // At the bottom: catch up to the new turns and keep tailing.
                self.chat_session = Some(transcripts::read(handle));
                self.chat_loaded_active = handle.last_active;
                self.mark_chat_seen();
                self.chat_dirty = true;
            }
            ChatReread::Append => {
                // Scrolled up: do NOT re-read or move the pane — the rail's dot
                // shows the session has new activity; scrolling back to the bottom
                // (which re-arms follow) catches up on the next tick.
            }
            ChatReread::Switch => {
                self.chat_session = Some(transcripts::read(handle));
                self.chat_loaded = Some(handle.id.clone());
                self.chat_loaded_active = handle.last_active;
                self.chat_expanded.clear();
                self.mark_chat_seen();
                // Open a session at its newest turn, and tail it.
                self.chat_follow = true;
                self.chat_dirty = true;
            }
        }
    }

    /// Mark the loaded session caught up to its current mtime (clears its dot).
    fn mark_chat_seen(&mut self) {
        if let (Some(id), Some(t)) = (self.chat_loaded.clone(), self.chat_loaded_active) {
            self.chat_seen.insert(id, t);
        }
    }

    /// Switch the selected session (`←`/`→`). The body re-read happens on the
    /// next tick via the `chat_loaded` mismatch in `refresh_chat`.
    /// The visible rail rows (directors + standalone, plus the subagents of any
    /// expanded group), in display order.
    pub fn chat_rail(&self) -> Vec<RailRow> {
        chat_rail_rows(&self.chat_sessions, &self.chat_expanded_groups)
    }

    /// Move the selection over the *visible* rail rows (`←`/`→`), so a collapsed
    /// group's hidden subagents are skipped.
    pub fn select_chat_session(&mut self, delta: isize) {
        let rows = self.chat_rail();
        if rows.is_empty() {
            return;
        }
        let cur = rows
            .iter()
            .position(|r| r.idx == self.chat_selected)
            .unwrap_or(0);
        let next = (cur as isize + delta).clamp(0, rows.len() as isize - 1) as usize;
        self.chat_selected = rows[next].idx;
    }

    /// Fold/unfold the group of the selected session (`z`) — expanding a director
    /// reveals its subagents; collapsing one with a subagent selected moves the
    /// selection back to the director so it never lands on a hidden row.
    pub fn chat_toggle_fold(&mut self) {
        let Some(sel) = self.chat_sessions.get(self.chat_selected) else {
            return;
        };
        let Some(group) = sel.group.clone() else {
            return;
        };
        let was_subagent = sel.is_subagent;
        if !self.chat_expanded_groups.remove(&group) {
            self.chat_expanded_groups.insert(group.clone());
        } else if was_subagent {
            // The group just collapsed while a child was selected — snap to the
            // director of that group.
            if let Some(i) = self
                .chat_sessions
                .iter()
                .position(|s| !s.is_subagent && s.group.as_deref() == Some(group.as_str()))
            {
                self.chat_selected = i;
            }
        }
    }

    /// Rebuild and cache the styled conversation rows plus their line metadata
    /// (total lines + per-message start lines), and clamp the scroll — but only
    /// when something changed (`chat_dirty` or a resize), so markdown is parsed
    /// once per change rather than every frame. `draw_chat` renders the cache.
    pub fn chat_relayout(&mut self) {
        let w = self.chat_convo_width();
        if !self.chat_dirty && self.chat_layout_w == w {
            return;
        }
        self.chat_rows = match &self.chat_session {
            Some(s) => chat_layout(s, &self.chat_expanded, w),
            None => Vec::new(),
        };
        self.chat_msg_starts = self
            .chat_rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.is_start)
            .map(|(line, r)| (line, r.msg))
            .collect();
        self.chat_total_lines = self.chat_rows.len();
        self.chat_layout_w = w;
        self.chat_dirty = false;
        // Tail-follow: stay pinned to the newest turn unless the reader scrolled
        // up; otherwise just clamp the existing offset.
        self.chat_scroll = if self.chat_follow {
            self.chat_max_scroll()
        } else {
            self.chat_scroll.min(self.chat_max_scroll())
        };
    }

    /// The cached conversation rows for rendering (see `chat_relayout`).
    pub fn chat_rows(&self) -> &[ChatRow] {
        &self.chat_rows
    }

    /// Visible conversation rows (the pane height minus its border).
    fn chat_visible_rows(&self) -> usize {
        (self.body_h.saturating_sub(2)).max(1) as usize
    }

    /// The largest scroll offset that still fills the pane — the "bottom".
    fn chat_max_scroll(&self) -> usize {
        self.chat_total_lines
            .saturating_sub(self.chat_visible_rows())
    }

    /// Width available to the conversation text, from the last viewport (minus
    /// the pane border), so markdown wraps and separators match the render.
    fn chat_convo_width(&self) -> usize {
        let w = match layout_mode(self.body_w) {
            Fit::Wide => (self.body_w as usize) * 68 / 100,
            Fit::Narrow => self.body_w as usize,
        };
        w.saturating_sub(2)
    }

    /// Scroll the conversation by `delta` lines (`j`/`k`, PgUp/PgDn), clamped to
    /// the bottom. Re-arms tail-follow when scrolled to the bottom, releases it
    /// otherwise.
    pub fn chat_scroll_by(&mut self, delta: isize) {
        let max = self.chat_max_scroll() as isize;
        self.chat_scroll = (self.chat_scroll as isize + delta).clamp(0, max.max(0)) as usize;
        self.chat_follow = self.chat_scroll >= self.chat_max_scroll();
    }

    /// Jump the scroll to the previous/next message's start line (`Shift`+arrows
    /// or `{`/`}`) — the "skip to message" motion over line-by-line scrolling.
    pub fn chat_msg_jump(&mut self, delta: isize) {
        if self.chat_msg_starts.is_empty() {
            return;
        }
        // The message we're in: the last start at or above the current top line.
        let cur = self
            .chat_msg_starts
            .partition_point(|(line, _)| *line <= self.chat_scroll)
            .saturating_sub(1);
        let target =
            (cur as isize + delta).clamp(0, self.chat_msg_starts.len() as isize - 1) as usize;
        self.chat_scroll = self.chat_msg_starts[target].0.min(self.chat_max_scroll());
        self.chat_follow = self.chat_scroll >= self.chat_max_scroll();
    }

    /// The event index of the message at the top of the viewport — what `Enter`
    /// expand/collapse acts on.
    fn chat_current_event(&self) -> Option<usize> {
        self.chat_msg_starts
            .iter()
            .rev()
            .find(|(line, _)| *line <= self.chat_scroll)
            .map(|(_, ev)| *ev)
    }

    /// Expand/collapse the message at the top of the viewport, if it is a
    /// collapsible turn (thinking, tool traffic, or a sidechain). `Enter`.
    pub fn chat_toggle_expand(&mut self) {
        let Some(ev) = self.chat_current_event() else {
            return;
        };
        let collapses = self
            .chat_session
            .as_ref()
            .and_then(|s| s.events.get(ev))
            .map(|e| e.kind.collapses() || e.is_sidechain)
            .unwrap_or(false);
        if collapses {
            if !self.chat_expanded.remove(&ev) {
                self.chat_expanded.insert(ev);
            }
            self.chat_dirty = true;
            self.chat_relayout(); // the line count changed
        }
    }

    /// Set the body viewport size (refreshed each tick), for page-sized motion.
    pub fn set_viewport(&mut self, w: u16, h: u16) {
        self.body_w = w;
        self.body_h = h;
    }

    /// One page of rows for PgUp/PgDn — a screen minus a line of overlap.
    pub fn page_rows(&self) -> isize {
        (self.body_h.saturating_sub(1)).max(1) as isize
    }

    /// Move down (`delta > 0`) or up by `delta` steps in the active view — the
    /// shared body of `j`/`k` (delta ±1) and PgUp/PgDn (delta ±a page).
    pub fn nav_step(&mut self, delta: isize) {
        match self.view {
            View::Chat => self.chat_scroll_by(delta),
            View::Process => self.process_move(delta),
            View::Comments => match self.comment_focus {
                CommentFocus::Tree => self.file_move(delta),
                CommentFocus::Comments => self.select_comment(delta),
            },
            View::Ledger => {
                for _ in 0..delta.unsigned_abs() {
                    if delta > 0 {
                        self.move_down();
                    } else {
                        self.move_up();
                    }
                }
            }
        }
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

    /// How many leading lines the syntax highlighter needs to cover for the
    /// current view: the furthest of the scroll and the selected comment's line,
    /// plus a viewport and margin. Windowing the grammar to this keeps previewing
    /// a large file cheap (a screenful, not the whole file).
    fn highlight_upto(&self, view_h: u16) -> usize {
        let sel_line = self
            .comment_localized
            .get(self.comment_selected)
            .and_then(|(_, loc)| loc.span.map(|(s, _)| s))
            .unwrap_or(0);
        self.comment_scroll.max(sel_line) + view_h as usize + 8
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
            (ProcessPane::Telos, false) => self.telos_select(delta),
            (ProcessPane::Telos, true) => {
                let p = &self.fold.process;
                let w = self.telos_detail_width();
                let max = p
                    .teloi
                    .get(self.telos_selected)
                    .map(|t| telos_detail(t, &p.tensions, &p.witnesses, w).len())
                    .unwrap_or(1)
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

    /// Width the telos detail wraps to — the detail pane's inner width, matching
    /// `draw_telos`'s split (62% when wide, full when narrow), for the scroll clamp.
    fn telos_detail_width(&self) -> usize {
        let w = match layout_mode(self.body_w) {
            Fit::Wide => (self.body_w as usize) * 62 / 100,
            Fit::Narrow => self.body_w as usize,
        };
        w.saturating_sub(2).max(1)
    }

    /// Move the highlighted telos in the list, clamped.
    pub fn telos_select(&mut self, delta: isize) {
        let n = self.fold.process.teloi.len();
        if n == 0 {
            return;
        }
        self.telos_selected =
            (self.telos_selected as isize + delta).clamp(0, n as isize - 1) as usize;
    }

    /// Enter/leave the drill-down detail of the selected atom or telos
    /// (Enter/Esc in the Process view).
    pub fn process_drill(&mut self, into: bool) {
        self.process_detail = into;
        self.atom_scroll = 0;
        self.telos_scroll = 0;
    }

    // --- Comment authoring (S1): interactive add/reply/edit/delete/resolve. ---

    /// The `Author` of a comment written here — the human at `$USER`.
    fn me(&self) -> comments::Author {
        comments::Author {
            who: "human".into(),
            id: std::env::var("USER").unwrap_or_else(|_| "local".into()),
        }
    }

    /// The absolute sidecar path for the open file.
    fn current_sidecar(&self) -> Option<PathBuf> {
        let rel = self.open_file.clone()?;
        Some(
            self.repo
                .join(comments::sidecar_path(&rel.to_string_lossy())),
        )
    }

    /// The extension of the open file, for choosing a syntax grammar (empty when
    /// there is none — the plain fallback).
    fn selected_ext(&self) -> String {
        self.open_file
            .as_ref()
            .and_then(|p| p.extension())
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Force the Comments view to re-read the sidecar after a write: the *source*
    /// file's mtime is unchanged, so the normal same-file gate would skip it.
    fn reload_after_write(&mut self) {
        self.comment_loaded = None;
        self.refresh_comments();
    }

    /// Move the comment cursor onto the comment with `id`, if it is present.
    fn select_comment_by_id(&mut self, id: &str) {
        if let Some(i) = self.comment_localized.iter().position(|(c, _)| c.id == id) {
            self.comment_selected = i;
            if let Some((s, _)) = self.comment_localized[i].1.span {
                self.comment_scroll = s;
            }
        }
    }

    /// The source line a new comment starts on: the selected comment's anchored
    /// line, else the current scroll, clamped to the file.
    fn authoring_line(&self) -> usize {
        let start = self
            .comment_localized
            .get(self.comment_selected)
            .and_then(|(_, loc)| loc.span.map(|(s, _)| s))
            .unwrap_or(self.comment_scroll);
        start.min(self.comment_content.lines().count().saturating_sub(1))
    }

    /// `a`/`i`: begin choosing the line for a new comment.
    pub fn begin_new_comment(&mut self) {
        self.comment_msg = None;
        if self.open_file.is_none() {
            return;
        }
        let cursor = self.authoring_line();
        self.comment_scroll = cursor;
        self.editing = Some(Editing::PickLine { cursor });
    }

    /// `r`: begin a reply to the selected comment.
    pub fn begin_reply(&mut self) {
        self.comment_msg = None;
        if let Some((c, _)) = self.comment_localized.get(self.comment_selected) {
            let id = c.id.clone();
            self.editing = Some(Editing::Compose {
                kind: ComposeKind::Reply { id },
                buf: TextBuf::default(),
            });
        }
    }

    /// `e`: begin editing the selected comment's body — only your own.
    pub fn begin_edit(&mut self) {
        self.comment_msg = None;
        let me = self.me().id;
        if let Some((c, _)) = self.comment_localized.get(self.comment_selected) {
            if c.author.id == me {
                let (id, buf) = (c.id.clone(), TextBuf::prefilled(&c.body));
                self.editing = Some(Editing::Compose {
                    kind: ComposeKind::Edit { id },
                    buf,
                });
            } else {
                self.comment_msg = Some("not your comment".into());
            }
        }
    }

    /// `d`: delete the selected comment — only your own.
    pub fn delete_selected(&mut self) {
        self.comment_msg = None;
        let me = self.me().id;
        let Some((c, _)) = self.comment_localized.get(self.comment_selected) else {
            return;
        };
        let id = c.id.clone();
        let Some(path) = self.current_sidecar() else {
            return;
        };
        let mut cs = comments::load(&path).unwrap_or_default();
        match comments::delete(&mut cs, &id, &me) {
            comments::Mutation::Applied => {
                let _ = comments::save(&path, &cs);
                self.comment_selected = self.comment_selected.saturating_sub(1);
                self.comment_msg = Some("deleted".into());
                self.reload_after_write();
            }
            comments::Mutation::Forbidden => self.comment_msg = Some("not your comment".into()),
            comments::Mutation::NotFound => {}
        }
    }

    /// `x`: toggle the resolved flag of the selected comment (anyone may resolve).
    pub fn toggle_resolve_selected(&mut self) {
        self.comment_msg = None;
        let Some((c, _)) = self.comment_localized.get(self.comment_selected) else {
            return;
        };
        let (id, target) = (c.id.clone(), !c.resolved);
        let Some(path) = self.current_sidecar() else {
            return;
        };
        let mut cs = comments::load(&path).unwrap_or_default();
        if comments::set_resolved(&mut cs, &id, target) {
            let _ = comments::save(&path, &cs);
            self.reload_after_write();
            self.select_comment_by_id(&id);
        }
    }

    // --- Promote-to-kan (S4): the explicit human action ---

    /// `p`: promote the selected comment into a durable kan claim, never touching
    /// the sidecar (the snapshot is immutable; the sidecar keeps re-localizing).
    pub fn promote_selected(&mut self) {
        self.comment_msg = None;
        let Some(rel) = self.open_file.clone() else {
            return;
        };
        let Some((c, loc)) = self.comment_localized.get(self.comment_selected) else {
            return;
        };
        let (c, span) = (c.clone(), loc.span);
        let rel = rel.to_string_lossy().to_string();
        self.comment_msg = Some(match self.promote_one(&rel, &c, span) {
            Ok(cid) => format!("promoted → {}", short_cid(&cid)),
            Err(e) => format!("promote failed: {e}"),
        });
    }

    /// `P`: promote the open file's whole comment set — one claim per comment.
    pub fn promote_file(&mut self) {
        self.comment_msg = None;
        let Some(rel) = self.open_file.clone() else {
            return;
        };
        let rel = rel.to_string_lossy().to_string();
        let items: Vec<(Comment, Option<(usize, usize)>)> = self
            .comment_localized
            .iter()
            .map(|(c, l)| (c.clone(), l.span))
            .collect();
        if items.is_empty() {
            self.comment_msg = Some("no comments to promote".into());
            return;
        }
        let mut ok = 0usize;
        let mut failed = None;
        for (c, span) in &items {
            match self.promote_one(&rel, c, *span) {
                Ok(_) => ok += 1,
                Err(e) => {
                    failed = Some(e);
                    break;
                }
            }
        }
        self.comment_msg = Some(match failed {
            Some(e) => format!("promoted {ok}, then failed: {e}"),
            None => format!("promoted {ok} comment(s) → kan"),
        });
    }

    /// Shell `kan observe` to snapshot one comment onto `comment/<file>`, citing
    /// the prior promoted claim on a re-promote. Returns the new claim's CID.
    fn promote_one(
        &self,
        rel: &str,
        c: &Comment,
        span: Option<(usize, usize)>,
    ) -> Result<String, String> {
        let subject = comments::comment_subject(rel);
        let prior = self.prior_promoted_cid(&subject, &c.id);
        let argv = comments::promote_argv(rel, c, span, prior.as_deref());
        let out = std::process::Command::new("kan")
            .current_dir(&self.repo)
            .args(&argv)
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr)
                .lines()
                .next()
                .unwrap_or("kan failed")
                .to_string());
        }
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    /// The CID of the newest claim on `subject` that already carries this comment
    /// id in its `cospan-comment` block, so a re-promote cites its own prior
    /// snapshot rather than writing an unlinked duplicate. `None` on first promote.
    fn prior_promoted_cid(&self, subject: &str, comment_id: &str) -> Option<String> {
        let out = std::process::Command::new("kan")
            .current_dir(&self.repo)
            .args(["show", subject, "--json"])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
        let claims = v.get("claims")?.as_array()?;
        let needle = format!("\"id\":\"{comment_id}\"");
        claims.iter().rev().find_map(|claim| {
            let text = claim.get("text").and_then(|t| t.as_str())?;
            if text.contains(&needle) {
                claim.get("cid").and_then(|c| c.as_str()).map(String::from)
            } else {
                None
            }
        })
    }

    /// The ids of the open file's comments that have been promoted to kan — read
    /// from the in-memory fold (a projection of the log, `telos/kan-is-truth`), so
    /// the indicator appears once a promote's claim lands in the next re-fold and
    /// reflects promotions from any session, not just this one. No subprocess.
    pub fn promoted_ids(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        let Some(rel) = self.open_file.as_ref() else {
            return set;
        };
        let subject = comments::comment_subject(&rel.to_string_lossy());
        let Some(claims) = self.fold.claims.get(&subject) else {
            return set;
        };
        for (c, _) in &self.comment_localized {
            let needle = format!("\"id\":\"{}\"", c.id);
            if claims
                .iter()
                .any(|cl| cl.text.as_deref().is_some_and(|t| t.contains(&needle)))
            {
                set.insert(c.id.clone());
            }
        }
        set
    }

    /// Commit the active compose: create the comment / append the reply / rewrite
    /// the body, persist, and re-read. A no-op unless a `Compose` is active — so
    /// `Ctrl-S` in pick-line mode does nothing.
    pub fn commit_editing(&mut self) {
        // Peek before taking: `Ctrl-S` in pick-line mode (or with nothing active)
        // must leave `editing` intact rather than silently cancelling the pick.
        if !matches!(self.editing, Some(Editing::Compose { .. })) {
            return;
        }
        let Some(Editing::Compose { kind, buf }) = self.editing.take() else {
            return;
        };
        let body = buf.text.trim_end().to_string();
        if body.is_empty() {
            self.comment_msg = Some("empty — discarded".into());
            return;
        }
        let Some(path) = self.current_sidecar() else {
            return;
        };
        let mut cs = comments::load(&path).unwrap_or_default();
        let me = self.me();
        let focus: Option<String>;
        match kind {
            ComposeKind::NewComment { line } => {
                let created_at = comments::now_micros();
                let id = format!("c_{created_at}_{}", cs.len());
                let anchor = comments::StoredAnchor::capture(&self.comment_content, line, 2);
                cs.push(comments::Comment {
                    id: id.clone(),
                    anchor,
                    body,
                    author: me,
                    created_at,
                    resolved: false,
                    thread: Vec::new(),
                });
                focus = Some(id);
            }
            ComposeKind::Reply { id } => {
                let r = comments::Reply {
                    author: me,
                    body,
                    created_at: comments::now_micros(),
                };
                if !comments::add_reply(&mut cs, &id, r) {
                    self.comment_msg = Some("comment is gone".into());
                    return;
                }
                focus = Some(id);
            }
            ComposeKind::Edit { id } => {
                match comments::edit_body(&mut cs, &id, &me.id, &body, &self.comment_content) {
                    comments::Mutation::Applied => focus = Some(id),
                    comments::Mutation::Forbidden => {
                        self.comment_msg = Some("not your comment".into());
                        return;
                    }
                    comments::Mutation::NotFound => {
                        self.comment_msg = Some("comment is gone".into());
                        return;
                    }
                }
            }
        }
        let _ = comments::save(&path, &cs);
        self.reload_after_write();
        if let Some(id) = focus {
            self.select_comment_by_id(&id);
        }
    }

    /// True while the Comments view is capturing keys for an authoring action.
    pub fn editing_active(&self) -> bool {
        self.view == View::Comments && self.editing.is_some()
    }

    /// Mouse-wheel scroll while composing: move the caret vertically a few lines,
    /// so the wheel scrolls the editor rather than the comment list underneath.
    pub fn compose_scroll(&mut self, delta: isize) {
        if let Some(Editing::Compose { buf, .. }) = &mut self.editing {
            for _ in 0..3 {
                if delta > 0 {
                    buf.down();
                } else {
                    buf.up();
                }
            }
        }
    }

    /// Route a keystroke while an `Editing` interaction is active. Terminal keys
    /// (`Esc` cancel, `Ctrl-S` commit, `Enter` in pick-line) replace `editing`
    /// wholesale; the rest move the pick cursor or edit the compose buffer.
    pub fn handle_editing_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::{KeyCode, KeyModifiers};
        // Whole-state transitions first, so the in-place borrow below never has to
        // reassign `self.editing`.
        match key.code {
            KeyCode::Esc => {
                self.editing = None;
                return;
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.commit_editing();
                return;
            }
            KeyCode::Enter => {
                if let Some(Editing::PickLine { cursor }) = &self.editing {
                    let line = *cursor;
                    self.editing = Some(Editing::Compose {
                        kind: ComposeKind::NewComment { line },
                        buf: TextBuf::default(),
                    });
                    return;
                }
            }
            _ => {}
        }
        match &mut self.editing {
            Some(Editing::PickLine { cursor }) => {
                let last = self.comment_content.lines().count().saturating_sub(1);
                // A page is a screenful (same size as the read-view PgUp/PgDn),
                // read from body_h by direct field access to avoid borrowing self
                // while `cursor` is held.
                let page = (self.body_h.saturating_sub(1)).max(1) as usize;
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => *cursor = cursor.saturating_sub(1),
                    KeyCode::Down | KeyCode::Char('j') => *cursor = (*cursor + 1).min(last),
                    KeyCode::PageUp => *cursor = cursor.saturating_sub(page),
                    KeyCode::PageDown => *cursor = (*cursor + page).min(last),
                    _ => {}
                }
                self.comment_scroll = *cursor;
            }
            Some(Editing::Compose { buf, .. }) => match key.code {
                KeyCode::Enter => buf.insert('\n'),
                KeyCode::Backspace => buf.backspace(),
                KeyCode::Left => buf.left(),
                KeyCode::Right => buf.right(),
                KeyCode::Up => buf.up(),
                KeyCode::Down => buf.down(),
                KeyCode::Home => buf.home(),
                KeyCode::End => buf.end(),
                KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    buf.insert(ch)
                }
                _ => {}
            },
            None => {}
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
#[allow(clippy::too_many_arguments)]
pub fn gutter_lines<'a>(
    content: &str,
    ext: &str,
    upto: usize,
    localized: &'a [(Comment, Localization)],
    selected: usize,
    promoted: &HashSet<String>,
    diff: &crate::diff::FileDiff,
    diff_on: bool,
) -> (Vec<ratatui::text::Line<'static>>, Vec<&'a Comment>) {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    let unresolved: Vec<&Comment> = localized
        .iter()
        .filter(|(_, loc)| loc.span.is_none())
        .map(|(c, _)| c)
        .collect();
    // Syntax-highlighted source (only the first `upto` lines run the grammar; the
    // rest are plain), one entry of styled runs per line — memoized/LRU-cached.
    let styled = crate::highlight::styled_upto(content, ext, upto);
    let num_w = styled.len().max(1).to_string().len();
    let lines = styled
        .iter()
        .enumerate()
        .map(|(i, runs)| {
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
            // A comment-covered line gets a full-line background band (stronger
            // for the selected comment) so the anchored line reads at a glance; a
            // promoted comment's marker is a filled diamond, an ephemeral one a dot.
            let (marker, marker_style, line_bg) = match hit {
                Some((idx, (c, loc))) => {
                    let glyph = if promoted.contains(&c.id) {
                        "◆"
                    } else {
                        "●"
                    };
                    let bg = if idx == selected {
                        Color::Indexed(240)
                    } else {
                        Color::Indexed(237)
                    };
                    (glyph, state_style(loc.state), Some(bg))
                }
                None => (" ", Style::new(), None),
            };
            // A deletion is framed on BOTH sides: the line just above the removed
            // block (`del_above`, the gap is below it) and the line just below it
            // (`del_below`, the gap is above it). Each gets a red row highlight and a
            // bar in the sign column pointing at the gap.
            let del_above = diff.deletions.contains_key(&(i + 1));
            let del_below = diff.deletions.contains_key(&i);
            // Working-tree diff sign: `+` added, `~` changed, `▁`/`▔` framing a
            // deletion, blank otherwise. The column is present only when the diff
            // toggle is on, at a fixed one cell.
            let (sign, sign_style) = if diff.added.contains(&i) {
                ("+", Style::new().fg(Color::Green))
            } else if diff.changed.contains(&i) {
                ("~", Style::new().fg(Color::Yellow))
            } else if del_below {
                ("▔", Style::new().fg(Color::Red).bg(Color::Indexed(52)))
            } else if del_above {
                ("▁", Style::new().fg(Color::Red).bg(Color::Indexed(52)))
            } else {
                (" ", Style::new())
            };
            // A subtle change tint on added/changed lines, only when the line has no
            // comment band (the band always wins the row background). Deletions are
            // NOT row-tinted — their red lives only in the gutter (the `▁`/`▔` bars).
            let diff_tint = if diff_on && line_bg.is_none() {
                if diff.added.contains(&i) {
                    Some(Color::Indexed(22))
                } else if diff.changed.contains(&i) {
                    Some(Color::Indexed(58))
                } else {
                    None
                }
            } else {
                None
            };
            let bg = line_bg.or(diff_tint);
            // The deletion red extends across the gutter (sign + line-number cells)
            // but not the marker, so it reads as a gutter highlight without
            // fill_line_bg spreading it across the whole row (that keys on the
            // marker span). A comment band, if present, overrides it below.
            let del_gutter = diff_on
                && line_bg.is_none()
                && !diff.added.contains(&i)
                && !diff.changed.contains(&i)
                && (del_above || del_below);
            let mut num_style = Style::new().add_modifier(Modifier::DIM);
            if del_gutter {
                num_style = num_style.bg(Color::Indexed(52));
            }
            let mut spans = vec![Span::styled(marker.to_string(), marker_style)];
            if diff_on {
                spans.push(Span::styled(sign.to_string(), sign_style));
            }
            spans.push(Span::styled(format!(" {:>num_w$} ", i + 1), num_style));
            // The syntax-highlighted text of the line, run by run.
            for (st, text) in runs {
                spans.push(Span::styled(text.clone(), *st));
            }
            // Paint the background across the whole line (marker, sign, number, code).
            if let Some(bg) = bg {
                for sp in &mut spans {
                    sp.style = sp.style.bg(bg);
                }
            }
            Line::from(spans)
        })
        .collect();
    (lines, unresolved)
}

/// The detail-strip lines for one comment: a header (state · where · confidence ·
/// author), the body, each reply indented and attributed, and a `[resolved]`
/// marker when resolved. Pure, so the thread render is unit-testable.
pub fn thread_lines(
    c: &Comment,
    loc: &Localization,
    promoted: bool,
) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::{Color, Modifier, Style};
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
    if promoted {
        header.push(Span::styled(
            "  ◆ kan",
            Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
    }
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

/// What a Chat refresh should do with the selected session's body.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChatReread {
    /// Nothing changed — keep the loaded body, cursor, and expansions.
    None,
    /// Same session, a turn was appended — re-read, but preserve cursor/expand.
    Append,
    /// A different session — re-read and reset cursor/expand.
    Switch,
}

/// Decide how to (re)load the selected session, so scroll and expansion survive
/// an append to the session being read but reset on an actual switch — and an
/// unrelated session changing does neither. Pure, so the rule is testable.
pub fn chat_reread_plan(
    loaded_id: Option<&str>,
    loaded_active: Option<SystemTime>,
    selected_id: &str,
    selected_active: Option<SystemTime>,
) -> ChatReread {
    if loaded_id != Some(selected_id) {
        ChatReread::Switch
    } else if loaded_active != selected_active {
        ChatReread::Append
    } else {
        ChatReread::None
    }
}

/// One visible row of the Chat session rail: which session it is, its tree
/// depth (0 top-level, 1 a nested subagent), and — for a director with
/// subagents — whether it heads a group and whether that group is expanded.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RailRow {
    pub idx: usize,
    pub depth: usize,
    pub is_parent: bool,
    pub expanded: bool,
    pub child_count: usize,
}

/// Flatten the session list into the visible rail rows: standalone sessions and
/// directors at depth 0, each director's subagents nested at depth 1 and shown
/// only when its group is expanded. A subagent whose director is absent is shown
/// at top level rather than dropped (`telos/honest-ambiguity`). Pure/testable.
pub fn chat_rail_rows(
    sessions: &[transcripts::SessionHandle],
    expanded_groups: &HashSet<String>,
) -> Vec<RailRow> {
    use std::collections::HashMap;
    let mut director: HashMap<&str, usize> = HashMap::new();
    let mut children: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, s) in sessions.iter().enumerate() {
        if let Some(g) = s.group.as_deref() {
            if s.is_subagent {
                children.entry(g).or_default().push(i);
            } else {
                director.entry(g).or_insert(i);
            }
        }
    }
    let mut rows = Vec::new();
    for (i, s) in sessions.iter().enumerate() {
        // Skip subagents that will be emitted under a present director.
        if s.is_subagent {
            if let Some(g) = s.group.as_deref() {
                if director.contains_key(g) {
                    continue;
                }
            }
        }
        let kids = if s.is_subagent {
            &[][..]
        } else {
            s.group
                .as_deref()
                .and_then(|g| children.get(g))
                .map(Vec::as_slice)
                .unwrap_or(&[])
        };
        let expanded = s
            .group
            .as_deref()
            .map(|g| expanded_groups.contains(g))
            .unwrap_or(false);
        rows.push(RailRow {
            idx: i,
            depth: 0,
            is_parent: !kids.is_empty(),
            expanded,
            child_count: kids.len(),
        });
        if !kids.is_empty() && expanded {
            for &c in kids {
                rows.push(RailRow {
                    idx: c,
                    depth: 1,
                    is_parent: false,
                    expanded: false,
                    child_count: 0,
                });
            }
        }
    }
    rows
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

/// One rendered conversation row: which message (event index) it belongs to,
/// whether it is that message's first row (for message-skip and current-message
/// detection), and the styled line to draw.
#[derive(Clone)]
pub struct ChatRow {
    pub msg: usize,
    pub is_start: bool,
    pub line: ratatui::text::Line<'static>,
}

/// Prompt tags cospan formats, per harness. Deliberately fixed registries, not
/// "any paired tag": a message *discussing* tags (or an assistant emitting
/// `<Foo>` generics) must not be reformatted as if the tags were real — and each
/// harness injects a different, distinctive set. Generic structural sub-tags
/// (`<path>`, `<entry>`, `<cwd>`) are excluded to avoid colliding with code.
const CLAUDE_PROMPT_TAGS: &[&str] = &[
    "system-reminder",
    "command-message",
    "command-name",
    "command-args",
    "command-stdout",
    "local-command-stdout",
    "task-notification",
    "task-reminder",
    "user-prompt-submit-hook",
];

/// Codex's injected-context wrappers (see `~/.codex/sessions` rollouts),
/// including the `<heartbeat>` block and its nested sub-tags so an automation
/// heartbeat renders as a structured, indented block.
const CODEX_PROMPT_TAGS: &[&str] = &[
    "app-context",
    "environment_context",
    "INSTRUCTIONS",
    "user_instructions",
    "skills_instructions",
    "apps_instructions",
    "plugins_instructions",
    "recommended_plugins",
    "multi_agent_mode",
    "permission_profile",
    "heartbeat",
    "automation_id",
    "current_time_iso",
    "instructions",
];

/// The prompt-tag registry for a harness (opencode's is not yet catalogued).
fn prompt_tags_for(harness: transcripts::Harness) -> &'static [&'static str] {
    match harness {
        transcripts::Harness::ClaudeCode => CLAUDE_PROMPT_TAGS,
        transcripts::Harness::Codex => CODEX_PROMPT_TAGS,
        transcripts::Harness::Opencode => &[],
    }
}

/// Byte ranges of the text that Markdown treats as code — inline code spans and
/// fenced/indented code blocks — computed by the real parser (`pulldown-cmark`),
/// so a tag written inside `` `…` `` or a ``` fence is never mistaken for a live
/// prompt tag. Robust to the code-fence edge cases a hand-rolled scan would miss.
fn code_ranges(text: &str) -> Vec<(usize, usize)> {
    use pulldown_cmark::{Event, Parser, Tag, TagEnd};
    let mut out: Vec<(usize, usize)> = Vec::new();
    let mut block_start: Option<usize> = None;
    for (ev, range) in Parser::new(text).into_offset_iter() {
        match ev {
            Event::Code(_) => out.push((range.start, range.end)),
            Event::Start(Tag::CodeBlock(_)) => block_start = Some(range.start),
            Event::End(TagEnd::CodeBlock) => {
                let s = block_start.take().unwrap_or(range.start);
                out.push((s, range.end));
            }
            _ => {}
        }
    }
    out
}

/// Whether a prompt tag opens, closes, or is self-closing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TagKind {
    Open,
    Close,
    SelfClose,
}

/// One piece of a message body: plain text, or a recognized prompt tag.
enum Seg<'a> {
    Text(&'a str),
    Tag(&'a str, TagKind),
}

/// Parse a tag starting at the `<` of `s`, returning `(byte_len, name, kind)` if
/// `s` opens with a well-formed `<name …>` / `</name>` / `<name/>`. Rejects
/// `<` that is not a tag (e.g. `a < b`), and stops if a nested `<` appears first.
fn parse_tag(s: &str) -> Option<(usize, &str, TagKind)> {
    let b = s.as_bytes();
    if b.first() != Some(&b'<') {
        return None;
    }
    let mut i = 1;
    let mut kind = TagKind::Open;
    if b.get(i) == Some(&b'/') {
        kind = TagKind::Close;
        i += 1;
    }
    let name_start = i;
    if !b.get(i).is_some_and(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    while b
        .get(i)
        .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'-' || *c == b'_')
    {
        i += 1;
    }
    let name = &s[name_start..i];
    // Attributes (or nothing) up to `>`, bailing on a stray `<`.
    while let Some(&c) = b.get(i) {
        match c {
            b'>' => {
                let self_close = i > 0 && b[i - 1] == b'/';
                if self_close && kind == TagKind::Open {
                    kind = TagKind::SelfClose;
                }
                return Some((i + 1, name, kind));
            }
            b'<' => return None,
            _ => i += 1,
        }
    }
    None
}

/// Split a message body into text and *paired* prompt-tag segments. A `<name>`
/// counts as a prompt tag only if the text also contains a matching `</name>`
/// (or it is self-closing) — so real prompt tags like `<system-reminder>…` are
/// formatted, while generics like `Vec<Line>` (no `</Line>`) stay plain text.
fn scan_prompt_tags<'a>(text: &'a str, tags_registry: &[&str]) -> Vec<Seg<'a>> {
    // Pass 1: collect every well-formed tag and which names are opened + closed.
    let mut tags: Vec<(usize, usize, TagKind)> = Vec::new();
    let mut opened: HashSet<&str> = HashSet::new();
    let mut closed: HashSet<&str> = HashSet::new();
    let mut i = 0;
    while let Some(off) = text[i..].find('<') {
        let start = i + off;
        if let Some((len, name, kind)) = parse_tag(&text[start..]) {
            match kind {
                TagKind::Open => {
                    opened.insert(name);
                }
                TagKind::Close => {
                    closed.insert(name);
                }
                TagKind::SelfClose => {}
            }
            tags.push((start, start + len, kind));
            i = start + len;
        } else {
            i = start + 1;
        }
    }

    // Pass 2: accept a tag only when it is a KNOWN prompt tag, is paired (or
    // self-closing), and lies OUTSIDE any Markdown code span/block.
    let code = code_ranges(text);
    let in_code = |a: usize, b: usize| code.iter().any(|&(cs, ce)| a < ce && cs < b);
    let mut segs: Vec<Seg> = Vec::new();
    let mut cursor = 0;
    for (start, end, kind) in tags {
        let name = {
            let inner = &text[start..end];
            // name = the run of tag-name chars after `<`/`</`.
            let after = inner.trim_start_matches('<').trim_start_matches('/');
            let n: usize = after
                .char_indices()
                .find(|(_, c)| !(c.is_ascii_alphanumeric() || *c == '-' || *c == '_'))
                .map(|(k, _)| k)
                .unwrap_or(after.len());
            &after[..n]
        };
        let paired = kind == TagKind::SelfClose || (opened.contains(name) && closed.contains(name));
        let accept = tags_registry.contains(&name) && paired && !in_code(start, end);
        if !accept {
            continue;
        }
        if cursor < start {
            segs.push(Seg::Text(&text[cursor..start]));
        }
        segs.push(Seg::Tag(&text[start..end], kind));
        cursor = end;
    }
    if cursor < text.len() {
        segs.push(Seg::Text(&text[cursor..]));
    }
    if segs.is_empty() {
        segs.push(Seg::Text(text));
    }
    segs
}

/// Render a User/Assistant message body: markdown for prose, and recognized
/// prompt tags broken onto their own colored lines with their contents indented
/// by nesting depth — the common "structured prompt" formatting.
fn render_message_body(text: &str, tags_registry: &[&str]) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};

    let segs = scan_prompt_tags(text, tags_registry);
    // No recognized tags → plain markdown, exactly as before.
    if segs.iter().all(|s| matches!(s, Seg::Text(_))) {
        return crate::markdown::render(text);
    }

    let tag_style = Style::new().fg(Color::Blue).add_modifier(Modifier::BOLD);
    let mut out: Vec<Line<'static>> = Vec::new();
    let mut depth = 0usize;
    let indent = |d: usize| "  ".repeat(d);
    for seg in segs {
        match seg {
            Seg::Text(t) => {
                if t.trim().is_empty() {
                    continue;
                }
                for mut l in crate::markdown::render(t) {
                    if depth > 0 {
                        l.spans.insert(0, Span::raw(indent(depth)));
                    }
                    out.push(l);
                }
            }
            Seg::Tag(t, TagKind::Open) => {
                out.push(Line::from(vec![
                    Span::raw(indent(depth)),
                    Span::styled(t.to_string(), tag_style),
                ]));
                depth += 1;
            }
            Seg::Tag(t, TagKind::Close) => {
                depth = depth.saturating_sub(1);
                out.push(Line::from(vec![
                    Span::raw(indent(depth)),
                    Span::styled(t.to_string(), tag_style),
                ]));
            }
            Seg::Tag(t, TagKind::SelfClose) => {
                out.push(Line::from(vec![
                    Span::raw(indent(depth)),
                    Span::styled(t.to_string(), tag_style),
                ]));
            }
        }
    }
    out
}

/// Render one non-grouped event — a message (header + markdown/prompt-tag body)
/// or a single collapsible turn (thinking / tool / sidechain) as a one-line
/// summary — into `out`, marking its first row with `is_start`.
fn push_single_event(
    out: &mut Vec<ChatRow>,
    e: &transcripts::Event,
    msg: usize,
    is_start: bool,
    expanded: &HashSet<usize>,
    rule_w: usize,
    tags_registry: &[&str],
) {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    use transcripts::{EventKind, Role};

    let (name, accent) = match e.role {
        Role::User => ("you", Color::Cyan),
        Role::Assistant => ("assistant", Color::Green),
        Role::Tool => ("tool", Color::Yellow),
        Role::System => ("system", Color::Magenta),
    };
    // A colored left bar runs down the whole message; content sits two columns
    // out (after "▌ "), flush with the header label.
    let bar = || Span::styled("▌ ".to_string(), Style::new().fg(accent));

    // User messages get a faint background band to set them apart. `banded`
    // paints every span's background and pads the line to full width so the band
    // is a solid strip, not just behind the text.
    let user_bg = (e.role == Role::User).then_some(Color::Indexed(236));
    let banded = |spans: Vec<Span<'static>>| -> Line<'static> {
        let Some(bg) = user_bg else {
            return Line::from(spans);
        };
        let mut spans: Vec<Span<'static>> = spans
            .into_iter()
            .map(|s| Span::styled(s.content, s.style.bg(bg)))
            .collect();
        let used: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        if used < rule_w {
            spans.push(Span::styled(" ".repeat(rule_w - used), Style::new().bg(bg)));
        }
        Line::from(spans)
    };

    if (e.kind.collapses() || e.is_sidechain) && !expanded.contains(&msg) {
        let tag = if e.is_sidechain {
            "sidechain"
        } else {
            match e.kind {
                EventKind::Thinking => "thinking",
                EventKind::ToolCall => "tool",
                EventKind::ToolResult => "result",
                _ => "detail",
            }
        };
        let first = e.text.lines().next().unwrap_or("");
        out.push(ChatRow {
            msg,
            is_start,
            line: Line::from(vec![
                bar(),
                Span::styled(format!("⤷ {tag} "), Style::new().fg(Color::DarkGray)),
                Span::styled(
                    truncate(first, rule_w.saturating_sub(18)),
                    Style::new().fg(Color::DarkGray).add_modifier(Modifier::DIM),
                ),
                Span::styled("  [↵ expand]", Style::new().fg(Color::DarkGray)),
            ]),
        });
        return;
    }

    // Header: the role label, after the bar, with a faded time on the right.
    let mut head = vec![
        bar(),
        Span::styled(
            name.to_string(),
            Style::new().fg(accent).add_modifier(Modifier::BOLD),
        ),
    ];
    if let Some(when) = e.ts.as_deref().and_then(iso_short) {
        head.push(Span::styled(
            format!("  {when}"),
            Style::new().fg(Color::DarkGray),
        ));
    }
    out.push(ChatRow {
        msg,
        is_start,
        line: banded(head),
    });

    // Body: markdown (with prompt-tag formatting) for message turns — including
    // System, since Codex's injected-context wrappers arrive as `developer`
    // (System) messages full of prompt tags; dim raw text for tool turns.
    let body: Vec<Line<'static>> = match (e.role, e.kind) {
        (Role::User | Role::Assistant | Role::System, EventKind::Message) if !e.text.is_empty() => {
            render_message_body(&e.text, tags_registry)
        }
        _ if e.text.is_empty() => vec![Line::from(Span::styled(
            "(empty)",
            Style::new().add_modifier(Modifier::DIM),
        ))],
        _ => e
            .text
            .lines()
            .map(|l| {
                Line::from(Span::styled(
                    l.to_string(),
                    Style::new().fg(Color::Gray).add_modifier(Modifier::DIM),
                ))
            })
            .collect(),
    };
    for l in body {
        for wl in wrap_line(&l, rule_w.saturating_sub(2)) {
            let mut spans = vec![bar()];
            spans.extend(wl.spans);
            out.push(ChatRow {
                msg,
                is_start: false,
                line: banded(spans),
            });
        }
    }
}

/// The kinds of back-to-back turns that fold into one collapsible group.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FoldKind {
    Tool,
    Thinking,
}

/// The fold category of an event, or `None` if it never folds (messages, and —
/// deliberately — sidechain turns, which belong to a subagent's own context).
fn fold_kind(e: &transcripts::Event) -> Option<FoldKind> {
    use transcripts::EventKind;
    if e.is_sidechain {
        return None;
    }
    match e.kind {
        EventKind::ToolCall | EventKind::ToolResult => Some(FoldKind::Tool),
        EventKind::Thinking => Some(FoldKind::Thinking),
        _ => None,
    }
}

/// Render a run of ≥2 back-to-back turns of one fold category as a single
/// collapsible group — "N tool calls" / "N thinking blocks" — that `Enter`
/// expands: a per-call summary list for tools, the full reasoning for thinking.
/// Keyed for expand/collapse by the run's first event index (`msg`), so it is
/// one jump unit and `Enter` on it toggles the whole run.
fn push_group(
    out: &mut Vec<ChatRow>,
    run: &[transcripts::Event],
    kind: FoldKind,
    msg: usize,
    is_start: bool,
    expanded: &HashSet<usize>,
    rule_w: usize,
) {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    use transcripts::EventKind;

    let (accent, noun) = match kind {
        FoldKind::Tool => (Color::Yellow, "tool calls"),
        FoldKind::Thinking => (Color::Gray, "thinking blocks"),
    };
    let bar = || Span::styled("▌ ".to_string(), Style::new().fg(accent));
    let open = expanded.contains(&msg);
    let (glyph, hint) = if open {
        ("▾", "[↵ collapse]")
    } else {
        ("▸", "[↵ expand]")
    };
    out.push(ChatRow {
        msg,
        is_start,
        line: Line::from(vec![
            bar(),
            Span::styled(
                format!("{glyph} {} {noun} ", run.len()),
                Style::new().fg(accent).add_modifier(Modifier::BOLD),
            ),
            Span::styled(hint.to_string(), Style::new().fg(Color::DarkGray)),
        ]),
    });
    if !open {
        return;
    }
    // A little helper to push a body line under the bar.
    let push_body = |out: &mut Vec<ChatRow>, line: Line<'static>| {
        for wl in wrap_line(&line, rule_w.saturating_sub(2)) {
            let mut spans = vec![bar()];
            spans.extend(wl.spans);
            out.push(ChatRow {
                msg,
                is_start: false,
                line: Line::from(spans),
            });
        }
    };
    match kind {
        // Tools: one compact "tool/result  brief" line per call.
        FoldKind::Tool => {
            for e in run {
                let label = if matches!(e.kind, EventKind::ToolResult) {
                    "result"
                } else {
                    "tool"
                };
                let first = e.text.lines().next().unwrap_or("");
                push_body(
                    out,
                    Line::from(vec![
                        Span::styled(format!("{label}  "), Style::new().fg(accent)),
                        Span::styled(
                            truncate(first, rule_w.saturating_sub(12)),
                            Style::new().add_modifier(Modifier::DIM),
                        ),
                    ]),
                );
            }
        }
        // Thinking: the full reasoning of each block, dim, with a divider between.
        FoldKind::Thinking => {
            for (k, e) in run.iter().enumerate() {
                if k > 0 {
                    push_body(
                        out,
                        Line::from(Span::styled("· · ·", Style::new().fg(Color::DarkGray))),
                    );
                }
                let body = if e.text.trim().is_empty() {
                    vec![Line::from("(empty)")]
                } else {
                    e.text
                        .lines()
                        .map(|l| {
                            Line::from(Span::styled(
                                l.to_string(),
                                Style::new().add_modifier(Modifier::DIM | Modifier::ITALIC),
                            ))
                        })
                        .collect::<Vec<_>>()
                };
                for l in body {
                    push_body(out, l);
                }
            }
        }
    }
}

/// Lay a session's turns out as styled rows: color-coded role headers, a
/// separator between turns, markdown bodies with harness-specific prompt-tag
/// formatting, and runs of ≥2 back-to-back tool/thinking turns folded into one
/// collapsible group. `width` sizes the wrap and separators. Pure and testable.
pub fn chat_layout(
    session: &transcripts::Session,
    expanded: &HashSet<usize>,
    width: usize,
) -> Vec<ChatRow> {
    use ratatui::style::{Color, Style};
    use ratatui::text::{Line, Span};

    let mut out: Vec<ChatRow> = Vec::new();
    let rule_w = width.clamp(8, 200);
    let events = &session.events;
    let n = events.len();
    let tags_registry = prompt_tags_for(session.harness);

    let mut i = 0usize;
    let mut first_unit = true;
    while i < n {
        // A run of consecutive turns of one fold category; folded only when ≥2.
        let kind = fold_kind(&events[i]);
        let mut j = i;
        while j < n && kind.is_some() && fold_kind(&events[j]) == kind {
            j += 1;
        }
        let group = kind.filter(|_| j - i >= 2);

        let is_first = first_unit;
        first_unit = false;
        // A dim separator rule opens every unit but the first (its start row);
        // when there is no separator the content's first row is the start.
        if !is_first {
            out.push(ChatRow {
                msg: i,
                is_start: true,
                line: Line::from(Span::styled(
                    "─".repeat(rule_w),
                    Style::new().fg(Color::DarkGray),
                )),
            });
        }
        if let Some(k) = group {
            push_group(&mut out, &events[i..j], k, i, is_first, expanded, rule_w);
            i = j;
        } else {
            push_single_event(
                &mut out,
                &events[i],
                i,
                is_first,
                expanded,
                rule_w,
                tags_registry,
            );
            i += 1;
        }
    }

    if out.is_empty() {
        out.push(ChatRow {
            msg: 0,
            is_start: true,
            line: Line::from("(no turns)"),
        });
    }
    out
}

/// Truncate `s` to `n` chars, appending an ellipsis when it was cut.
/// Word-wrap a styled `Line` to `width`, preserving each character's style, so
/// the conversation's on-screen line count equals its logical line count and the
/// scroll/message-jump offsets stay exact. Breaks at the last space that fits,
/// hard-breaking a word longer than `width`.
pub fn wrap_line(
    line: &ratatui::text::Line<'static>,
    width: usize,
) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::text::Line;
    if width == 0 {
        return vec![line.clone()];
    }
    let chars: Vec<(char, ratatui::style::Style)> = line
        .spans
        .iter()
        .flat_map(|s| s.content.chars().map(move |c| (c, s.style)))
        .collect();
    if chars.is_empty() {
        return vec![Line::from("")];
    }
    let mut out: Vec<Line<'static>> = Vec::new();
    let mut start = 0usize;
    while start < chars.len() {
        let hard = (start + width).min(chars.len());
        let mut end = hard;
        let mut skip_space = false;
        if hard < chars.len() {
            // Prefer a break at the last space within the window (but not at the
            // very start, which would be leading indentation and loop forever).
            if let Some(sp) = (start + 1..hard).rev().find(|&k| chars[k].0 == ' ') {
                end = sp;
                skip_space = true;
            }
        }
        out.push(coalesce_runs(&chars[start..end]));
        start = if skip_space { end + 1 } else { end };
    }
    out
}

/// Merge consecutive same-style chars into `Span`s to rebuild a `Line`.
fn coalesce_runs(seg: &[(char, ratatui::style::Style)]) -> ratatui::text::Line<'static> {
    use ratatui::text::{Line, Span};
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut buf = String::new();
    let mut cur: Option<ratatui::style::Style> = None;
    for &(c, st) in seg {
        if cur != Some(st) {
            if let Some(s) = cur {
                spans.push(Span::styled(std::mem::take(&mut buf), s));
            }
            cur = Some(st);
        }
        buf.push(c);
    }
    if let Some(s) = cur {
        spans.push(Span::styled(buf, s));
    }
    Line::from(spans)
}

/// A compact `MM-DD HH:MM` from an ISO-8601 timestamp string
/// (`YYYY-MM-DDTHH:MM:SS…` / `… …`), or `None` if it doesn't look like one — for
/// the faded time label on a message.
pub fn iso_short(ts: &str) -> Option<String> {
    let b = ts.as_bytes();
    if b.len() >= 16 && b[4] == b'-' && b[7] == b'-' && (b[10] == b'T' || b[10] == b' ') {
        Some(format!("{} {}", &ts[5..10], &ts[11..16]))
    } else {
        None
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() > n {
        format!("{}…", s.chars().take(n).collect::<String>())
    } else {
        s.to_string()
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
    use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

    if !repo.join(".kan").is_dir() {
        eprintln!(
            "warning: {} has no .kan/ — is this a kan repo?",
            repo.display()
        );
    }

    let mut terminal = ratatui::init();
    // Enable mouse reporting for scroll-wheel support (hold Shift to select text).
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture);
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
        // Second gate, same tick: refresh the file tree's git status on an index
        // change, and re-localize the open file on a content change.
        if state.view == View::Comments {
            state.refresh_files();
            state.refresh_comments();
        }
        // Footer gate: refresh day's status line on a cache/width change.
        let size = terminal.size().unwrap_or_default();
        state.refresh_footer(size.width, true);
        // Publish the body viewport so paging and wrapping have real dimensions
        // (header line + a footer of 1..=6 lines bracket the body).
        let footer_h = {
            let mut n = state.fold.errors.len();
            n += if state.footer.is_empty() {
                1
            } else {
                state.footer.len()
            };
            n.clamp(1, 6)
        } as u16;
        state.set_viewport(size.width, size.height.saturating_sub(1 + footer_h));
        // Chat gate: re-read transcripts when the aggregate signal advances, then
        // relayout the conversation so scroll/jump metadata match the render.
        if state.view == View::Chat {
            state.refresh_chat();
            state.chat_relayout();
        }

        if let Err(e) = terminal.draw(|frame| draw(frame, &state)) {
            break Err(e);
        }

        match event::poll(tick) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                    // Ctrl-C always quits — the one hard escape hatch that works
                    // even mid-compose (where every other key types into the buffer).
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break Ok(());
                    }
                    // While composing a comment, all other keys go to the editor (so
                    // `q` types rather than quits); it commits/cancels on its own.
                    if state.view == View::Comments && state.editing.is_some() {
                        state.handle_editing_key(key);
                        continue;
                    }
                    // The thread popup overlays the panes and takes keys ahead of
                    // the normal arms: it reads the full thread and still acts on
                    // that comment. r/e (which open the full-screen composer) and
                    // d close it first; x resolves in place and keeps it open.
                    if state.view == View::Comments && state.popup_open {
                        state.handle_popup_key(key.code);
                        continue;
                    }
                    match key.code {
                        KeyCode::Char('q') => break Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            break Ok(())
                        }
                        KeyCode::Char(d @ '1'..='4') => {
                            if let Some(v) = View::from_digit(d) {
                                state.view = v;
                                if v == View::Comments {
                                    state.enter_comments();
                                } else if v == View::Chat {
                                    state.reload_chat_sessions();
                                }
                            }
                        }
                        KeyCode::Tab => {
                            state.view = state.view.next();
                            if state.view == View::Comments {
                                state.enter_comments();
                            } else if state.view == View::Chat {
                                state.reload_chat_sessions();
                            }
                        }
                        // Enter (Comments/tree focus): toggle a directory or open a
                        // file into the gutter. Esc returns focus to the tree.
                        KeyCode::Enter
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Tree =>
                        {
                            state.file_activate()
                        }
                        // Enter (gutter focus): open the selected comment's full
                        // thread in the popup, if there is one to read.
                        KeyCode::Enter
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.open_thread_popup()
                        }
                        KeyCode::Esc
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.comment_focus = CommentFocus::Tree
                        }
                        // `t`: toggle the file tray held open while reading. In Tree
                        // focus the rail always shows, so this only bites once a file
                        // is open (Comments focus).
                        KeyCode::Char('t') if state.view == View::Comments => state.toggle_tray(),
                        KeyCode::Char('D') if state.view == View::Comments => state.toggle_diff(),
                        // Comment authoring (gutter focus): add / reply / edit / delete / resolve.
                        KeyCode::Char('a' | 'i')
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.begin_new_comment()
                        }
                        KeyCode::Char('r')
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.begin_reply()
                        }
                        KeyCode::Char('e')
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.begin_edit()
                        }
                        KeyCode::Char('d')
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.delete_selected()
                        }
                        KeyCode::Char('x')
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.toggle_resolve_selected()
                        }
                        // Promote to a durable kan claim: `p` the selected comment,
                        // `P` the whole file's set.
                        KeyCode::Char('p')
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.promote_selected()
                        }
                        KeyCode::Char('P')
                            if state.view == View::Comments
                                && state.comment_focus == CommentFocus::Comments =>
                        {
                            state.promote_file()
                        }
                        KeyCode::Char('[') | KeyCode::Left if state.view == View::Chat => {
                            state.select_chat_session(-1)
                        }
                        KeyCode::Char(']') | KeyCode::Right if state.view == View::Chat => {
                            state.select_chat_session(1)
                        }
                        KeyCode::Char('[' | ']') | KeyCode::Left | KeyCode::Right
                            if state.view == View::Process =>
                        {
                            state.process_pane = state.process_pane.toggled();
                            state.process_detail = false; // leave any drill-down on a pane switch
                        }
                        // Fold/unfold the selected session's subagent group (Chat).
                        KeyCode::Char('z') if state.view == View::Chat => state.chat_toggle_fold(),
                        // Skip to the previous/next message (Chat): Shift+↑/↓ or { / }.
                        KeyCode::Char('{') if state.view == View::Chat => state.chat_msg_jump(-1),
                        KeyCode::Char('}') if state.view == View::Chat => state.chat_msg_jump(1),
                        KeyCode::Up | KeyCode::Down
                            if state.view == View::Chat
                                && key.modifiers.contains(KeyModifiers::SHIFT) =>
                        {
                            state.chat_msg_jump(if key.code == KeyCode::Down { 1 } else { -1 })
                        }
                        // Page up/down: a screen of motion in the active view.
                        KeyCode::PageDown => state.nav_step(state.page_rows()),
                        KeyCode::PageUp => state.nav_step(-state.page_rows()),
                        // Line/step motion in the active view.
                        KeyCode::Char('j') | KeyCode::Down => state.nav_step(1),
                        KeyCode::Char('k') | KeyCode::Up => state.nav_step(-1),
                        KeyCode::Enter if state.view == View::Chat => state.chat_toggle_expand(),
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
                    }
                }
                // Mouse wheel scrolls the active view a few lines at a time.
                Ok(Event::Mouse(m)) => match m.kind {
                    MouseEventKind::ScrollDown if state.editing_active() => state.compose_scroll(1),
                    MouseEventKind::ScrollUp if state.editing_active() => state.compose_scroll(-1),
                    MouseEventKind::ScrollDown => state.nav_step(3),
                    MouseEventKind::ScrollUp => state.nav_step(-3),
                    _ => {}
                },
                Ok(_) => {}
                Err(e) => break Err(e),
            },
            Ok(false) => {} // tick elapsed with no input — loop re-checks mtime
            Err(e) => break Err(e),
        }
    };

    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture);
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
    // The Comments view's transient status (e.g. "not your comment") rides the
    // footer now that the bottom strip is gone.
    if state.view == View::Comments {
        if let Some(m) = &state.comment_msg {
            footer_lines.push(format!("· {m}"));
        }
    }
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
        View::Chat => draw_chat(frame, state, area.width, body),
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
        View::Chat => "· ←→ session · z fold · j/k scroll · ⇧↑↓ msg · ↵ expand ",
        View::Comments => {
            "· j/k browse · ↵ fold/thread · t tray · D diff · Esc tree · a add · r reply · e/d edit/del · x resolve · p promote "
        }
        View::Process => "· ←→ atoms/telos · j/k scroll ",
        View::Ledger => "",
    };
    format!(
        "cospan  {}{}{}{}  {keys}· PgUp/PgDn · Tab switch · q quit",
        tab(View::Chat, "1 chat"),
        tab(View::Comments, "2 comments"),
        tab(View::Ledger, "3 ledger"),
        tab(View::Process, "4 process"),
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
    promoted: bool,
) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::{Modifier, Style};
    use ratatui::text::{Line, Span};
    const BODY_CAP: usize = 3;
    let mut label = format!("@{} · {:?}", c.author.id, loc.state);
    if promoted {
        label.push_str(" · kan");
    }
    if c.resolved {
        label.push_str(" [resolved]");
    }
    let header_style = if selected {
        Style::new().add_modifier(Modifier::REVERSED)
    } else {
        Style::new()
    };
    // A filled diamond (◆) marks a promoted comment; a hollow dot (●) an
    // ephemeral one — same signal as the code gutter.
    let bullet = if promoted { "◆ " } else { "● " };
    let mut lines = vec![Line::from(vec![
        Span::styled(bullet, state_style(loc.state)),
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

/// Whether the notes need reflow — i.e. any note (sorted by `start`) would either
/// run past the last code line or overlap the next note's start. When false, every
/// note fits beside the code without pushing it down, so the cheaper, less-jumpy
/// [`side_by_side_rows`] can be used instead of [`reflow_rows`].
pub fn notes_need_reflow(
    notes: &[(usize, usize, Vec<ratatui::text::Line<'static>>)],
    code_len: usize,
) -> bool {
    for (i, (_, start, lines)) in notes.iter().enumerate() {
        let end = start + lines.len(); // exclusive
        if end > code_len {
            return true; // note spills past the file — needs appended rows
        }
        if let Some((_, next_start, _)) = notes.get(i + 1) {
            if *next_start < end {
                return true; // this note's lines collide with the next note
            }
        }
    }
    false
}

/// Place each note's lines into the right column beside the code — no reflow, so
/// the code column does not shift. Safe only when [`notes_need_reflow`] is false
/// (every note fits within the code without colliding). Same return shape as
/// [`reflow_rows`]: `(left,right)` rows and `(localized_index, first_row)` per note.
#[allow(clippy::type_complexity)]
pub fn side_by_side_rows(
    code_lines: Vec<ratatui::text::Line<'static>>,
    notes: &[(usize, usize, Vec<ratatui::text::Line<'static>>)],
) -> (
    Vec<(ratatui::text::Line<'static>, ratatui::text::Line<'static>)>,
    Vec<(usize, usize)>,
) {
    use ratatui::text::Line;
    let mut rows: Vec<(Line, Line)> = code_lines
        .into_iter()
        .map(|c| (c, Line::from("")))
        .collect();
    let mut note_rows: Vec<(usize, usize)> = Vec::new();
    for (loc_idx, start, note_lines) in notes {
        note_rows.push((*loc_idx, *start)); // the note's first row is its code line
        for (j, nl) in note_lines.iter().enumerate() {
            if let Some(row) = rows.get_mut(start + j) {
                row.1 = nl.clone();
            }
        }
    }
    (rows, note_rows)
}

/// The Chat tab: a cross-harness session rail beside the selected session's
/// conversation. Read-only projection of the harnesses' own transcripts.
fn draw_chat(
    frame: &mut ratatui::Frame,
    state: &AppState,
    width: u16,
    body: ratatui::layout::Rect,
) {
    use ratatui::layout::{Constraint, Layout};
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::widgets::{List, ListItem, ListState, Paragraph};

    if state.chat_sessions.is_empty() {
        frame.render_widget(
            Paragraph::new(
                "no agent sessions for this repo yet — start one in claude, codex, or opencode",
            )
            .block(pane_block(" chat ".to_string(), true)),
            body,
        );
        return;
    }

    // Wide: a session rail beside the conversation; narrow: conversation only.
    let (rail_area, main_area) = match layout_mode(width) {
        Fit::Wide => {
            let [l, r] =
                Layout::horizontal([Constraint::Percentage(32), Constraint::Percentage(68)])
                    .areas(body);
            (Some(l), r)
        }
        Fit::Narrow => (None, body),
    };

    if let Some(area) = rail_area {
        use ratatui::text::{Line, Span};
        let rail = state.chat_rail();
        let items: Vec<ListItem> = rail
            .iter()
            .map(|row| {
                let h = &state.chat_sessions[row.idx];
                let branch = h
                    .git_branch
                    .as_deref()
                    .map(|b| format!(" · {b}"))
                    .unwrap_or_default();
                let flag = if h.body_available {
                    ""
                } else {
                    " ·(list-only)"
                };
                // A "new activity" dot for a session with unread turns.
                let dot = if state.chat_session_stale(h) {
                    Span::styled("●", Style::new().fg(Color::Yellow))
                } else {
                    Span::raw(" ")
                };
                // A fold caret for a director with subagents; a nesting mark for a
                // child; the harness tag only at top level.
                let (lead, label) = if row.depth > 0 {
                    (
                        Span::styled("  ↳ ", Style::new().fg(Color::DarkGray)),
                        format!("{}{branch}{flag}", h.title),
                    )
                } else {
                    let caret = if row.is_parent {
                        if row.expanded {
                            "▾ "
                        } else {
                            "▸ "
                        }
                    } else {
                        "  "
                    };
                    let count = if row.is_parent {
                        format!("  ({})", row.child_count)
                    } else {
                        String::new()
                    };
                    (
                        Span::styled(caret.to_string(), Style::new().fg(Color::DarkGray)),
                        format!("[{}] {}{branch}{flag}{count}", h.harness.label(), h.title),
                    )
                };
                // A faded last-active stamp helps tell sessions apart by recency.
                let when = h
                    .last_active
                    .map(|t| format!("  {}", substrate::stamp_short(t)))
                    .unwrap_or_default();
                ListItem::new(Line::from(vec![
                    dot,
                    lead,
                    Span::raw(label),
                    Span::styled(when, Style::new().fg(Color::DarkGray)),
                ]))
            })
            .collect();
        let mut ls = ListState::default();
        ls.select(rail.iter().position(|r| r.idx == state.chat_selected));
        // The rail is a picker, not the scroll target — dim (unfocused) border.
        frame.render_stateful_widget(
            List::new(items)
                .block(pane_block(
                    format!(" sessions · {} · z fold ", state.chat_sessions.len()),
                    false,
                ))
                .highlight_style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .highlight_symbol("▌"),
            area,
            &mut ls,
        );
    }

    // The conversation: pre-wrapped styled rows, scrolled by line. The active
    // scroll target — focused (bold) border.
    let title = state
        .chat_sessions
        .get(state.chat_selected)
        .map(|h| {
            let branch = h
                .git_branch
                .as_deref()
                .map(|b| format!(" · {b}"))
                .unwrap_or_default();
            format!(" {} · {}{branch} ", h.harness.label(), h.title)
        })
        .unwrap_or_else(|| " chat ".to_string());
    // Render the visible window of the cached rows (built by `chat_relayout`),
    // so a frame clones only what fits, never re-parses markdown.
    let rows = state.chat_rows();
    let inner_h = main_area.height.saturating_sub(2) as usize;
    let top = state.chat_scroll.min(rows.len().saturating_sub(1));
    let end = (top + inner_h).min(rows.len());
    let lines: Vec<ratatui::text::Line> = if rows.is_empty() {
        vec![ratatui::text::Line::from("(loading…)")]
    } else {
        rows[top..end].iter().map(|r| r.line.clone()).collect()
    };
    frame.render_widget(
        Paragraph::new(lines).block(pane_block(title, true)),
        main_area,
    );
}

/// A pane border that signals focus: the active (scroll/selection) pane gets a
/// bold accent border, inactive panes a dim one — the "where does my navigation
/// act" hint the multi-pane views share.
fn pane_block(
    title: impl Into<ratatui::text::Line<'static>>,
    focused: bool,
) -> ratatui::widgets::Block<'static> {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::widgets::{Block, BorderType};
    let b = Block::bordered().title(title);
    if focused {
        b.border_type(BorderType::Thick)
            .border_style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    } else {
        b.border_style(Style::new().fg(Color::DarkGray))
    }
}

/// A one-char git marker and its color for a file row (blank/gray when clean).
fn git_marker_style(s: filetree::GitStatus) -> (char, ratatui::style::Color) {
    use filetree::GitStatus::*;
    use ratatui::style::Color;
    let color = match s {
        Clean => Color::Gray,
        Modified => Color::Yellow,
        Added => Color::Green,
        Untracked => Color::Cyan,
        Deleted => Color::Red,
    };
    (filetree::marker(s), color)
}

/// One rail row: the dim ancestry guide, then a collapsible directory (fold
/// arrow + bold name) or a file (git marker + name), aligned by the guide width.
fn file_row_item(row: &FileRow) -> ratatui::widgets::ListItem<'static> {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    use ratatui::widgets::ListItem;
    let base = |p: &str| p.rsplit('/').next().unwrap_or(p).to_string();
    let guide_style = Style::new().fg(Color::DarkGray);
    match row {
        FileRow::Dir {
            path,
            guide,
            collapsed,
        } => {
            let arrow = if *collapsed { "▸" } else { "▾" };
            ListItem::new(Line::from(vec![
                Span::styled(guide.clone(), guide_style),
                Span::styled(
                    format!("{arrow} {}/", base(path)),
                    Style::new().add_modifier(Modifier::BOLD),
                ),
            ]))
        }
        FileRow::File {
            path,
            guide,
            status,
        } => {
            let (m, color) = git_marker_style(*status);
            ListItem::new(Line::from(vec![
                Span::styled(guide.clone(), guide_style),
                Span::styled(format!("{m} "), Style::new().fg(color)),
                Span::styled(base(path), Style::new().fg(color)),
            ]))
        }
    }
}

fn draw_comments(
    frame: &mut ratatui::Frame,
    state: &AppState,
    width: u16,
    body: ratatui::layout::Rect,
) {
    use ratatui::layout::{Constraint, Layout};
    use ratatui::style::{Modifier, Style};
    use ratatui::text::Line;
    use ratatui::widgets::{Block, Clear, List, ListItem, ListState, Paragraph, Wrap};

    if state.file_rows.is_empty() {
        frame.render_widget(
            Paragraph::new("no files to browse (not a git repo, or an empty one)")
                .block(Block::bordered().title(" comments ")),
            body,
        );
        return;
    }

    // Compose mode: a full-width editor view — the file rail collapses, the target
    // line is highlighted, and the popup is placed clear of it.
    if let Some(Editing::Compose { kind, buf }) = &state.editing {
        draw_compose(frame, state, body, kind, buf);
        return;
    }

    // The file-tree rail sits beside the content pane while browsing (Tree focus)
    // and, while reading, only if the tray is toggled open (`rail_visible`); a
    // narrow terminal never shows it. Width is 32% capped at RAIL_MAX, so a wide
    // terminal gives reading width to the code instead of empty gutter.
    const RAIL_MAX: u16 = 40;
    let wide = matches!(layout_mode(width), Fit::Wide);
    let (files_area, main_area) = if rail_visible(state.comment_focus, state.tray_open, wide) {
        let rail_w = ((body.width as u32 * 32 / 100) as u16).min(RAIL_MAX);
        let [l, r] =
            Layout::horizontal([Constraint::Length(rail_w), Constraint::Min(0)]).areas(body);
        (Some(l), r)
    } else {
        (None, body)
    };

    if let Some(area) = files_area {
        let items: Vec<ListItem> = state.file_rows.iter().map(file_row_item).collect();
        let mut ls = ListState::default();
        ls.select(Some(
            state
                .file_selected
                .min(state.file_rows.len().saturating_sub(1)),
        ));
        let focused = state.comment_focus == CommentFocus::Tree;
        frame.render_stateful_widget(
            List::new(items)
                .block(pane_block(
                    format!(" files · {} · ↵ fold/comment ", state.file_entries.len()),
                    focused,
                ))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol("> "),
            area,
            &mut ls,
        );
    }

    // The bottom strip is gone: the content pane owns the full height. Its two
    // jobs are rehomed — the full thread to the Enter popup, the unresolvable
    // list to a pinned band atop the note column (both below).
    let content_area = main_area;

    let gutter_focused = state.comment_focus == CommentFocus::Comments;

    // Nothing opened yet: prompt the operator to pick a file from the tree.
    if state.open_file.is_none() {
        frame.render_widget(
            Paragraph::new("select a file in the tree (↑/↓ · Enter) to read and comment on it")
                .block(pane_block(" file ", gutter_focused)),
            content_area,
        );
        return;
    }

    let promoted = state.promoted_ids();
    let (code_lines, unresolved) = gutter_lines(
        &state.comment_content,
        &state.selected_ext(),
        state.highlight_upto(content_area.height),
        &state.comment_localized,
        state.comment_selected,
        &promoted,
        &state.file_diff,
        state.diff_on,
    );
    let file_title = state
        .open_file
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    // Pick-line mode: one code pane with the target line highlighted; the note
    // columns and compose popup are suppressed until a line is chosen.
    if let Some(Editing::PickLine { cursor }) = &state.editing {
        let mut lines = code_lines;
        if let Some(l) = lines.get_mut(*cursor) {
            for sp in &mut l.spans {
                sp.style = sp.style.add_modifier(Modifier::REVERSED);
            }
        }
        // Follow the pick cursor stickily too: scroll only when it leaves the
        // viewport, so choosing a line does not re-scroll the text on every step.
        let max_top = lines.len().saturating_sub(1);
        let view_h = content_area.height.saturating_sub(2) as usize;
        let scroll = sticky_top(
            state.note_scroll.get().min(max_top),
            *cursor,
            view_h,
            max_top,
        );
        state.note_scroll.set(scroll);
        frame.render_widget(
            Paragraph::new(lines.split_off(scroll)).block(Block::bordered().title(format!(
                " pick line {} · ↑/↓ · PgUp/PgDn · Enter here · Esc ",
                cursor + 1
            ))),
            content_area,
        );
        return;
    }
    match layout_mode(width) {
        Fit::Wide => {
            // Code column beside a right comment column; the code reflows down so
            // a multi-line note never overlaps code or a neighbour. Sizing priority:
            // the note column holds a fixed small width, the code column grows to
            // TEXT_MAX first, and only then does the extra width go to the notes —
            // so wide terminals give reading width to the code, not empty gutter.
            const NOTE_MIN: u16 = 30;
            const TEXT_MAX: u16 = 100;
            let code_w = content_area.width.saturating_sub(NOTE_MIN).min(TEXT_MAX);
            let note_w = content_area.width.saturating_sub(code_w).saturating_sub(2) as usize;
            // Pinned "unresolvable" band for the comments with no line to anchor to
            // (honest-ambiguity). The code column keeps its FULL height — no blanked
            // top — and only the note column is split: the line-anchored notes on
            // top (so they still align row-for-row with the code, both starting at
            // the content top) and the band pinned at the bottom.
            let sel_id = state
                .comment_localized
                .get(state.comment_selected)
                .map(|(c, _)| c.id.as_str());
            let group = unresolvable_group(&unresolved, note_w, sel_id);
            let g = if group.is_empty() {
                0
            } else {
                (group.len() as u16 + 2).min(content_area.height.saturating_sub(3))
            };
            let [code_area, note_col] =
                Layout::horizontal([Constraint::Length(code_w), Constraint::Min(0)])
                    .areas(content_area);
            let [note_area, band_area] =
                Layout::vertical([Constraint::Min(0), Constraint::Length(g)]).areas(note_col);
            if g > 0 {
                frame.render_widget(
                    Paragraph::new(group)
                        .block(
                            Block::bordered()
                                .title(format!(" unresolvable ({}) ", unresolved.len())),
                        )
                        .wrap(Wrap { trim: false }),
                    band_area,
                );
            }
            let notes: Vec<(usize, usize, Vec<Line>)> = state
                .comment_localized
                .iter()
                .enumerate()
                .filter_map(|(idx, (c, loc))| {
                    loc.span.map(|(s, _)| {
                        (
                            idx,
                            s,
                            note_block(
                                c,
                                loc,
                                note_w,
                                idx == state.comment_selected,
                                promoted.contains(&c.id),
                            ),
                        )
                    })
                })
                .collect();
            // Only push the code down (reflow) when notes actually collide; when
            // they fit beside the code, render side-by-side so the code stays put.
            let (rows, note_rows) = if notes_need_reflow(&notes, code_lines.len()) {
                reflow_rows(code_lines, &notes)
            } else {
                side_by_side_rows(code_lines, &notes)
            };
            // Follow the selected note stickily: scroll only when it leaves the
            // viewport, so stepping between visible comments does not snap to top.
            let sel_row = note_rows
                .iter()
                .find(|(idx, _)| *idx == state.comment_selected)
                .map(|(_, r)| *r)
                .unwrap_or(0);
            let max_top = rows.len().saturating_sub(1);
            let view_h = code_area.height.saturating_sub(2) as usize;
            let scroll = sticky_top(
                state.note_scroll.get().min(max_top),
                sel_row,
                view_h,
                max_top,
            );
            state.note_scroll.set(scroll);
            // Fill each comment-banded code line to the pane width so the highlight
            // spans the whole row, not just the characters.
            let inner_w = code_area.width.saturating_sub(2) as usize;
            let left: Vec<Line> = rows
                .iter()
                .skip(scroll)
                .map(|(l, _)| fill_line_bg(l.clone(), inner_w))
                .collect();
            let right: Vec<Line> = rows.iter().skip(scroll).map(|(_, r)| r.clone()).collect();
            frame.render_widget(
                Paragraph::new(left).block(pane_block(
                    format!(" {file_title} · Esc → tree "),
                    gutter_focused,
                )),
                code_area,
            );
            frame.render_widget(
                Paragraph::new(right).block(Block::bordered().title(" comments ")),
                note_area,
            );
        }
        Fit::Narrow => {
            // No room for a note column, but the Unresolvable comments must still
            // be visible (honest-ambiguity) — pin them in a band below the code.
            let sel_id = state
                .comment_localized
                .get(state.comment_selected)
                .map(|(c, _)| c.id.as_str());
            let band_w = content_area.width.saturating_sub(2) as usize;
            let group = unresolvable_group(&unresolved, band_w, sel_id);
            let g = if group.is_empty() {
                0
            } else {
                (group.len() as u16 + 2).min(content_area.height.saturating_sub(3))
            };
            let [code_area, band_area] =
                Layout::vertical([Constraint::Min(3), Constraint::Length(g)]).areas(content_area);
            let scroll = state.comment_scroll.min(code_lines.len().saturating_sub(1));
            let inner_w = code_area.width.saturating_sub(2) as usize;
            let lines: Vec<Line> = code_lines[scroll..]
                .iter()
                .map(|l| fill_line_bg(l.clone(), inner_w))
                .collect();
            frame.render_widget(
                Paragraph::new(lines).block(pane_block(
                    format!(" {file_title} · Esc → tree "),
                    gutter_focused,
                )),
                code_area,
            );
            if g > 0 {
                frame.render_widget(
                    Paragraph::new(group)
                        .block(
                            Block::bordered()
                                .title(format!(" unresolvable ({}) ", unresolved.len())),
                        )
                        .wrap(Wrap { trim: false }),
                    band_area,
                );
            }
        }
    }

    // The thread popup: a centered overlay reading the selected comment's full
    // untruncated thread — the strip's old read job, now on demand (Enter). It is
    // drawn last so it sits atop the panes, and still acts on the comment (the key
    // handler routes r/e/d/x into it).
    if state.popup_open {
        if let Some((c, loc)) = state.comment_localized.get(state.comment_selected) {
            let lines = thread_lines(c, loc, promoted.contains(&c.id));
            let area = centered_rect(body, 70, 60);
            frame.render_widget(Clear, area);
            frame.render_widget(
                Paragraph::new(lines)
                    .block(
                        Block::bordered()
                            .title(" comment thread · r reply · e/d edit/del · x resolve · Esc "),
                    )
                    .wrap(Wrap { trim: false }),
                area,
            );
        }
    }
}

/// A rectangle centered in `area` at the given width/height percentages — the
/// frame for a modal overlay like the thread popup. The percentage math goes
/// through u32 so a very wide terminal cannot overflow the u16 multiply.
fn centered_rect(area: ratatui::layout::Rect, pct_w: u16, pct_h: u16) -> ratatui::layout::Rect {
    let w = (area.width as u32 * pct_w as u32 / 100) as u16;
    let h = (area.height as u32 * pct_h as u32 / 100) as u16;
    ratatui::layout::Rect {
        x: area.x + area.width.saturating_sub(w) / 2,
        y: area.y + area.height.saturating_sub(h) / 2,
        width: w,
        height: h,
    }
}

/// Follow `sel_row` with the viewport top: move only when the selection leaves
/// the visible `[top, top+view_h)` window — up to it when above, just far enough
/// down when below — so stepping between visible comments never snaps the
/// selection to the top. Pure, so the sticky rule is testable.
fn sticky_top(prev_top: usize, sel_row: usize, view_h: usize, max_top: usize) -> usize {
    let view_h = view_h.max(1);
    let top = if sel_row < prev_top {
        sel_row
    } else if sel_row >= prev_top + view_h {
        sel_row + 1 - view_h
    } else {
        prev_top
    };
    top.min(max_top)
}

/// If `line` carries a background band (its leading span sets a bg), pad it with
/// a trailing space span in that colour out to `width` cells, so a comment's
/// highlight fills the whole row rather than stopping at the end of the text.
fn fill_line_bg(
    mut line: ratatui::text::Line<'static>,
    width: usize,
) -> ratatui::text::Line<'static> {
    use ratatui::style::Style;
    use ratatui::text::Span;
    let bg = line.spans.first().and_then(|s| s.style.bg);
    if let Some(bg) = bg {
        let used: usize = line.spans.iter().map(|s| s.content.chars().count()).sum();
        if used < width {
            line.spans
                .push(Span::styled(" ".repeat(width - used), Style::new().bg(bg)));
        }
    }
    line
}

/// The pinned "unresolvable" note-column band: one styled row per comment with no
/// line to anchor to, the selected one reversed, capped with a "+N more" tail.
/// Kept visible rather than hidden so a lost anchor is never silent
/// (telos/honest-ambiguity). Returns the inner rows; the caller frames them.
fn unresolvable_group(
    unresolved: &[&Comment],
    width: usize,
    selected_id: Option<&str>,
) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::Modifier;
    use ratatui::text::{Line, Span};
    const CAP: usize = 4;
    // Show the first CAP, but if the selected comment is one of the unresolvable
    // ones past the cap, swap it into the last slot so it stays visible and marked
    // — "each is selectable" (REQ-4) must not hide behind "+N more".
    let mut shown: Vec<usize> = (0..unresolved.len().min(CAP)).collect();
    if let Some(si) = selected_id.and_then(|id| unresolved.iter().position(|c| c.id == id)) {
        if !shown.contains(&si) {
            if let Some(last) = shown.last_mut() {
                *last = si;
            }
        }
    }
    let mut rows: Vec<Line> = Vec::new();
    for &i in &shown {
        let c = unresolved[i];
        let head = c.body.lines().next().unwrap_or("");
        let mut text = format!("● {head}");
        if text.chars().count() > width {
            text = text
                .chars()
                .take(width.saturating_sub(1))
                .collect::<String>()
                + "…";
        }
        let mut style = state_style(State::Unresolvable);
        if selected_id == Some(c.id.as_str()) {
            style = style.add_modifier(Modifier::REVERSED);
        }
        rows.push(Line::from(Span::styled(text, style)));
    }
    let hidden = unresolved.len().saturating_sub(shown.len());
    if hidden > 0 {
        rows.push(Line::from(format!("  +{hidden} more")));
    }
    rows
}

/// Where to place the compose popup vertically within `body`: just below the
/// target line when the popup fits there, else just above it, else clamped into
/// `body` — so the editor tracks the line being commented rather than snapping to
/// a fixed half. With no anchor line (`target_row` None) it rests at the bottom.
/// Pure, so the placement is testable.
fn compose_popup_y(body: ratatui::layout::Rect, target_row: Option<u16>, want_h: u16) -> u16 {
    let bottom = body.y + body.height;
    let max_y = bottom.saturating_sub(want_h).max(body.y);
    match target_row {
        None => max_y,
        Some(tr) => {
            // The target line's screen row (code sits inside a 1-row top border).
            let line_y = (body.y + 1)
                .saturating_add(tr)
                .min(bottom.saturating_sub(1));
            let below = line_y.saturating_add(1);
            let py = if bottom.saturating_sub(below) >= want_h {
                below
            } else {
                line_y.saturating_sub(want_h).max(body.y)
            };
            py.min(max_y)
        }
    }
}

/// Compose view: the file body full-width (rail collapsed) with the commented
/// line highlighted, and the editor popup placed adjacent to that line (below it
/// when it fits, else above) — so you can see what you are commenting on while
/// you type. The editor scrolls vertically to keep the caret in view and shows a
/// `L<row>/<total>` position with `↑`/`↓` when there is more above or below.
fn draw_compose(
    frame: &mut ratatui::Frame,
    state: &AppState,
    body: ratatui::layout::Rect,
    kind: &ComposeKind,
    buf: &TextBuf,
) {
    use ratatui::layout::Rect;
    use ratatui::style::Modifier;
    use ratatui::text::Line;
    use ratatui::widgets::{Block, Clear, Paragraph};

    let file_title = state
        .open_file
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    // The source line this compose is about: the chosen line for a new comment,
    // else the target comment's anchored line.
    let target = match kind {
        ComposeKind::NewComment { line } => Some(*line),
        ComposeKind::Reply { id } | ComposeKind::Edit { id } => state
            .comment_localized
            .iter()
            .find(|(c, _)| &c.id == id)
            .and_then(|(_, loc)| loc.span.map(|(s, _)| s)),
    };

    // Full-width code with the target line highlighted, scrolled toward the top
    // third so it stays on screen beside the popup. Highlight through the target
    // line plus a screen, so composing on a deep line still shows it colored.
    let upto = target.unwrap_or(0) + body.height as usize + 32;
    let (mut code_lines, _) = gutter_lines(
        &state.comment_content,
        &state.selected_ext(),
        upto,
        &state.comment_localized,
        state.comment_selected,
        &state.promoted_ids(),
        &state.file_diff,
        state.diff_on, // diff shows in the compose view too (all views consistent)
    );
    if let Some(t) = target {
        if let Some(l) = code_lines.get_mut(t) {
            for sp in &mut l.spans {
                sp.style = sp.style.add_modifier(Modifier::REVERSED);
            }
        }
    }
    let view_h = body.height as usize;
    let scroll = target
        .map(|t| t.saturating_sub(view_h / 3))
        .unwrap_or(0)
        .min(code_lines.len().saturating_sub(1));
    frame.render_widget(
        Paragraph::new(code_lines.split_off(scroll))
            .block(Block::bordered().title(format!(" {file_title} · composing "))),
        body,
    );

    // Place the popup adjacent to the target line (below it when it fits, else
    // above), and cap its width at a comfortable reading measure.
    let target_row = target.map(|t| t.saturating_sub(scroll) as u16);
    let lines: Vec<String> = buf.text.split('\n').map(str::to_string).collect();
    let want_h = ((lines.len() as u16).saturating_add(2))
        .clamp(5, body.height.saturating_sub(2).max(5))
        .min(16);
    const COMPOSE_MAX_W: u16 = 80;
    let pw = ((body.width as u32 * 72 / 100) as u16)
        .min(COMPOSE_MAX_W)
        .clamp(20.min(body.width), body.width);
    let px = body.x + body.width.saturating_sub(pw) / 2;
    let py = compose_popup_y(body, target_row, want_h.min(body.height));
    let popup = Rect {
        x: px,
        y: py,
        width: pw,
        height: want_h.min(body.height),
    };

    let label = match kind {
        ComposeKind::NewComment { line } => format!("new comment · line {}", line + 1),
        ComposeKind::Reply { .. } => "reply".to_string(),
        ComposeKind::Edit { .. } => "edit".to_string(),
    };

    // Caret at (row, col); scroll the editor so the caret line is visible. No wrap
    // in the box, so one logical line is one row and the scroll math is exact.
    let (caret_row, caret_col) = buf.row_col();
    let inner_h = popup.height.saturating_sub(2).max(1) as usize;
    let scroll_v = caret_row.saturating_sub(inner_h.saturating_sub(1));
    let mut disp: Vec<Line> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if i == caret_row {
            let chars: Vec<char> = l.chars().collect();
            let cc = caret_col.min(chars.len());
            let a: String = chars[..cc].iter().collect();
            let b: String = chars[cc..].iter().collect();
            disp.push(Line::from(format!("{a}│{b}")));
        } else {
            disp.push(Line::from(l.clone()));
        }
    }
    let end = (scroll_v + inner_h).min(disp.len());
    let visible = disp[scroll_v..end].to_vec();
    let title = format!(
        " {label} · Ctrl-S save · Esc · L{}/{}{}{} ",
        caret_row + 1,
        lines.len(),
        if scroll_v > 0 { " ↑" } else { "" },
        if end < disp.len() { " ↓" } else { "" },
    );

    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(visible).block(Block::bordered().title(title)),
        popup,
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
                    lines.push(format!("  {}", tension.pair()));
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

/// The telos detail as styled lines: slug/title header, the statement rendered
/// as markdown, each witness with the probe description from `schema/witness`
/// (colored, dim probe), and each tension naming the telos — its pair plus the
/// recorded rationale (the "why") rendered as markdown. Live per-witness state
/// still needs machine-readable day, so this is declared structure only.
/// Pure/testable.
pub fn telos_detail(
    t: &substrate::TelosView,
    tensions: &[substrate::Tension],
    probes: &std::collections::BTreeMap<String, String>,
    width: usize,
) -> Vec<ratatui::text::Line<'static>> {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};

    let dim = Style::new().add_modifier(Modifier::DIM);
    let header_style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

    let mut out: Vec<Line<'static>> = Vec::new();
    // Push `line` at `indent` columns, wrapping to the pane width with a hanging
    // indent — every wrapped continuation keeps the indent instead of falling
    // back to column 0.
    let push = |out: &mut Vec<Line<'static>>, indent: usize, line: Line<'static>| {
        let inner = width.saturating_sub(indent).max(1);
        for wl in wrap_line(&line, inner) {
            let mut spans: Vec<Span<'static>> = Vec::new();
            if indent > 0 {
                spans.push(Span::raw(" ".repeat(indent)));
            }
            spans.extend(wl.spans);
            out.push(Line::from(spans));
        }
    };

    push(
        &mut out,
        0,
        Line::from(Span::styled(
            format!("telos/{}", t.slug),
            Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        )),
    );
    if !t.title.is_empty() && t.title != t.slug {
        push(
            &mut out,
            0,
            Line::from(Span::styled(
                t.title.clone(),
                Style::new().add_modifier(Modifier::BOLD),
            )),
        );
    }

    out.push(Line::from(""));
    push(
        &mut out,
        0,
        Line::from(Span::styled("statement", header_style)),
    );
    for l in crate::markdown::render(&t.statement) {
        push(&mut out, 2, l);
    }

    out.push(Line::from(""));
    push(
        &mut out,
        0,
        Line::from(Span::styled(
            format!("witnesses ({})", t.witnesses.len()),
            header_style,
        )),
    );
    if t.witnesses.is_empty() {
        push(
            &mut out,
            2,
            Line::from(Span::styled("(none declared)", dim)),
        );
    }
    for w in &t.witnesses {
        let mut spans = vec![
            Span::styled("· ", dim),
            Span::styled(w.clone(), Style::new().fg(Color::Green)),
        ];
        if let Some(p) = probes.get(w) {
            spans.push(Span::styled(format!("   {p}"), dim));
        }
        push(&mut out, 2, Line::from(spans));
    }

    out.push(Line::from(""));
    let related: Vec<&substrate::Tension> = tensions.iter().filter(|x| x.names(&t.slug)).collect();
    push(
        &mut out,
        0,
        Line::from(Span::styled(
            format!("tensions ({})", related.len()),
            header_style,
        )),
    );
    if related.is_empty() {
        push(&mut out, 2, Line::from(Span::styled("(none)", dim)));
    }
    for x in &related {
        let other = if x.between.0 == t.slug {
            &x.between.1
        } else {
            &x.between.0
        };
        push(
            &mut out,
            2,
            Line::from(vec![
                Span::styled("· ", dim),
                Span::styled(
                    format!("{} <-> {}", t.slug, other),
                    Style::new().fg(Color::Yellow),
                ),
            ]),
        );
        if !x.why.is_empty() {
            for l in crate::markdown::render(&x.why) {
                push(&mut out, 4, l);
            }
        }
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
    use ratatui::widgets::{Block, Paragraph, Wrap};
    let scroll = scroll.min(lines.len().saturating_sub(1));
    frame.render_widget(
        Paragraph::new(lines[scroll..].join("\n"))
            .wrap(Wrap { trim: false })
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
    use ratatui::layout::{Constraint, Layout};
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::widgets::{List, ListItem, ListState, Paragraph};
    let p = &state.fold.process;

    if p.teloi.is_empty() {
        render_scrolled(
            frame,
            area,
            " process · telos · ←→ atoms ",
            &["(no teloi declared)".to_string()],
            0,
        );
        return;
    }
    // A list of teloi (left) beside the selected telos's detail (right); `Enter`
    // moves focus into the detail to scroll it, `Esc` back to the list. A narrow
    // terminal shows only the focused pane.
    let detail_focused = state.process_detail;
    let sel = state.telos_selected.min(p.teloi.len().saturating_sub(1));

    let render_list = |frame: &mut ratatui::Frame, a: ratatui::layout::Rect| {
        let items: Vec<ListItem> = p
            .teloi
            .iter()
            .map(|t| {
                ListItem::new(ratatui::text::Line::from(vec![
                    ratatui::text::Span::raw(t.title.clone()),
                    ratatui::text::Span::styled(
                        format!("  ({})", t.slug),
                        Style::new().fg(Color::DarkGray),
                    ),
                ]))
            })
            .collect();
        let mut ls = ListState::default();
        ls.select(Some(sel));
        frame.render_stateful_widget(
            List::new(items)
                .block(pane_block(
                    " telos · ←→ atoms · ↵ detail ".to_string(),
                    !detail_focused,
                ))
                .highlight_style(Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .highlight_symbol("> "),
            a,
            &mut ls,
        );
    };
    // Lay the detail out to the actual pane width so hanging indents wrap exactly
    // (no double-wrap: the lines are already wrapped, so no `Wrap` on the widget).
    let render_detail = |frame: &mut ratatui::Frame, a: ratatui::layout::Rect| {
        let inner_w = (a.width as usize).saturating_sub(2).max(1);
        let lines = telos_detail(&p.teloi[sel], &p.tensions, &p.witnesses, inner_w);
        let scroll = state.telos_scroll.min(lines.len().saturating_sub(1));
        let title = if detail_focused {
            " detail · Esc back "
        } else {
            " detail "
        };
        frame.render_widget(
            Paragraph::new(lines[scroll..].to_vec())
                .block(pane_block(title.to_string(), detail_focused)),
            a,
        );
    };

    match layout_mode(area.width) {
        Fit::Wide => {
            let [l, r] =
                Layout::horizontal([Constraint::Percentage(38), Constraint::Percentage(62)])
                    .areas(area);
            render_list(frame, l);
            render_detail(frame, r);
        }
        Fit::Narrow => {
            if detail_focused {
                render_detail(frame, area);
            } else {
                render_list(frame, area);
            }
        }
    }
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
    use ratatui::widgets::{List, ListItem, ListState};

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
            .block(pane_block(
                format!(" subjects · {} ", state.fold.subjects.len()),
                state.focus == Focus::Subjects,
            ))
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
    use ratatui::widgets::{List, ListItem, ListState};

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
            .block(pane_block(
                format!(" {subject} · claims "),
                state.focus == Focus::Claims,
            ))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> "),
        area,
        &mut ls,
    );
}

fn draw_claim_detail(frame: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    use ratatui::style::Style;
    use ratatui::text::{Line, Span};
    use ratatui::widgets::{Paragraph, Wrap};

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
            .block(pane_block(
                Span::styled(format!(" {title} "), style),
                state.focus == Focus::Detail,
            )),
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
        // (AC-1) Chat · Comments · Ledger · Process, Chat first.
        assert_eq!(View::Chat.next(), View::Comments);
        assert_eq!(View::Comments.next(), View::Ledger);
        assert_eq!(View::Ledger.next(), View::Process);
        assert_eq!(View::Process.next(), View::Chat);
        assert_eq!(View::from_digit('1'), Some(View::Chat));
        assert_eq!(View::from_digit('2'), Some(View::Comments));
        assert_eq!(View::from_digit('3'), Some(View::Ledger));
        assert_eq!(View::from_digit('4'), Some(View::Process));
        assert_eq!(View::from_digit('5'), None);
        assert_eq!(View::from_digit('9'), None);
    }

    #[test]
    fn tab_bar_names_the_new_tabs_with_legends() {
        // (AC-1) Chat first; Ledger/Process tabs; Process names its sub-pane keys.
        assert!(view_header(View::Chat).contains("1 chat"));
        assert!(view_header(View::Comments).contains("2 comments"));
        assert!(view_header(View::Ledger).contains("3 ledger"));
        let p = view_header(View::Process);
        assert!(p.contains("4 process"), "{p}");
        assert!(p.contains("←→ atoms/telos"), "no sub-pane keys: {p}");
        assert!(p.contains("j/k scroll"), "{p}");
        assert!(!view_header(View::Comments).contains("browser"));
    }

    #[test]
    fn chat_header_shows_the_navigation_legend() {
        // (AC-1) the session-switch, scroll, message-skip, expand, and page keys.
        let h = view_header(View::Chat);
        assert!(h.contains("←→ session"), "no session-switch hint: {h}");
        assert!(h.contains("j/k scroll"), "no scroll hint: {h}");
        assert!(h.contains("⇧↑↓ msg"), "no message-skip hint: {h}");
        assert!(h.contains("↵ expand"), "no expand hint: {h}");
        assert!(h.contains("PgUp/PgDn"), "no paging hint: {h}");
    }

    /// The plain text of a rendered chat row (its spans concatenated).
    fn row_text(r: &ChatRow) -> String {
        r.line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    /// The plain text of styled lines, joined by newlines (for assertions).
    fn lines_text(lines: &[ratatui::text::Line<'static>]) -> String {
        lines
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
    fn chat_reread_plan_switches_appends_or_holds() {
        // (AC-7 / review #3) The gate re-reads only on change, and resets the
        // reading position on a *switch* while preserving it on an *append*.
        use std::time::Duration;
        let t1 = Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1));
        let t2 = Some(SystemTime::UNIX_EPOCH + Duration::from_secs(2));
        // First load (nothing loaded yet) reads and resets.
        assert_eq!(chat_reread_plan(None, None, "a", t1), ChatReread::Switch);
        // A different session id resets cursor/expansions.
        assert_eq!(chat_reread_plan(Some("a"), t1, "b", t2), ChatReread::Switch);
        // Same session, newer mtime (a turn appended) re-reads but preserves.
        assert_eq!(chat_reread_plan(Some("a"), t1, "a", t2), ChatReread::Append);
        // Same session, unchanged mtime does nothing.
        assert_eq!(chat_reread_plan(Some("a"), t1, "a", t1), ChatReread::None);
    }

    #[test]
    fn chat_render_collapses_thinking_tool_and_sidechain_by_default() {
        // (AC-6) collapsible turns show a one-line summary; expanding reveals the
        // body; a plain message renders in full; a tool call is one summary line.
        use crate::transcripts::{Event, EventKind, Harness, Role, Session};
        let mk = |role, kind, is_sidechain, text: &str| Event {
            role,
            kind,
            ts: None,
            id: None,
            parent: None,
            is_sidechain,
            text: text.to_string(),
        };
        let session = Session {
            harness: Harness::ClaudeCode,
            id: "s".into(),
            title: "t".into(),
            git_branch: None,
            events: vec![
                mk(Role::User, EventKind::Message, false, "please fix"),
                mk(
                    Role::Assistant,
                    EventKind::Thinking,
                    false,
                    "reasoning teaser\nSECRET_DETAIL_LINE",
                ),
                mk(Role::Tool, EventKind::ToolCall, false, "Edit(x.rs)"),
                mk(
                    Role::Assistant,
                    EventKind::Message,
                    true,
                    "chatter head\nHIDDEN_SIDECHAIN_BODY",
                ),
            ],
        };
        let none = HashSet::new();
        let joined = chat_layout(&session, &none, 80)
            .iter()
            .map(row_text)
            .collect::<Vec<_>>()
            .join("\n");
        // The plain user message renders in full, under its role bar.
        assert!(joined.contains("▌ you"), "{joined}");
        assert!(joined.contains("please fix"), "{joined}");
        // Thinking is collapsed to a one-line teaser; the rest of its body hides.
        assert!(joined.contains("⤷ thinking"), "{joined}");
        assert!(!joined.contains("SECRET_DETAIL_LINE"), "{joined}");
        // The tool call is a single summary line; the sidechain body collapses.
        assert!(joined.contains("⤷ tool"), "{joined}");
        assert!(joined.contains("⤷ sidechain"), "{joined}");
        assert!(!joined.contains("HIDDEN_SIDECHAIN_BODY"), "{joined}");

        // Expanding the thinking turn (index 1) reveals its hidden body.
        let mut exp = HashSet::new();
        exp.insert(1usize);
        let joined2 = chat_layout(&session, &exp, 80)
            .iter()
            .map(row_text)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined2.contains("SECRET_DETAIL_LINE"), "{joined2}");
    }

    #[test]
    fn render_message_body_formats_paired_prompt_tags() {
        let lines = render_message_body(
            "<command-message>day:wakeup</command-message>",
            CLAUDE_PROMPT_TAGS,
        );
        let texts: Vec<String> = lines
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect();
        // Each tag on its own line, content between them, indented.
        assert!(
            texts.iter().any(|l| l.trim() == "<command-message>"),
            "{texts:?}"
        );
        assert!(texts.iter().any(|l| l.contains("day:wakeup")), "{texts:?}");
        assert!(
            texts.iter().any(|l| l.trim() == "</command-message>"),
            "{texts:?}"
        );
        // The tag line is colored (a foreground was set on the tag span).
        let tagline = lines
            .iter()
            .find(|l| {
                l.spans
                    .iter()
                    .any(|s| s.content.contains("command-message"))
            })
            .unwrap();
        assert!(
            tagline.spans.iter().any(|s| s.style.fg.is_some()),
            "prompt tag should be colored"
        );
    }

    #[test]
    fn render_message_body_ignores_unknown_and_in_code_tags() {
        // An unknown (non-registry) paired tag stays plain text…
        let unknown = render_message_body("<inner>deep</inner> and more", CLAUDE_PROMPT_TAGS);
        assert!(
            !unknown.iter().any(|l| {
                let t: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                t.trim() == "<inner>"
            }),
            "an unknown tag must not be broken out"
        );

        // …and a *known* tag written inside a code fence stays literal code —
        // this is the failure mode where discussing tags rendered as if real.
        let text = "example:\n\n```\n<system-reminder>fake</system-reminder>\n```\n";
        let lines = render_message_body(text, CLAUDE_PROMPT_TAGS);
        let broken_out = lines.iter().any(|l| {
            let t: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
            t.trim() == "<system-reminder>" && l.spans.iter().any(|s| s.style.fg.is_some())
        });
        assert!(
            !broken_out,
            "a tag inside a code fence must not be formatted"
        );
        let joined = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref().to_string()))
            .collect::<Vec<_>>()
            .join("");
        assert!(
            joined.contains("system-reminder"),
            "the literal text survives"
        );
    }

    #[test]
    fn prompt_tags_are_per_harness() {
        use crate::transcripts::Harness;
        // Codex's <environment_context> formats under the Codex registry…
        let codex = render_message_body(
            "<environment_context>\ncwd: /x\n</environment_context>",
            CODEX_PROMPT_TAGS,
        );
        assert!(
            codex.iter().any(|l| {
                let t: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                t.trim() == "<environment_context>"
            }),
            "Codex tag should format under the Codex registry"
        );
        // …but not under Claude's, and a Claude tag is absent from Codex's.
        assert_eq!(prompt_tags_for(Harness::ClaudeCode), CLAUDE_PROMPT_TAGS);
        assert_eq!(prompt_tags_for(Harness::Codex), CODEX_PROMPT_TAGS);
        assert!(!CLAUDE_PROMPT_TAGS.contains(&"environment_context"));
        assert!(!CODEX_PROMPT_TAGS.contains(&"system-reminder"));
        assert!(prompt_tags_for(Harness::Opencode).is_empty());
    }

    #[test]
    fn render_message_body_leaves_generics_alone() {
        // `Vec<Line>` has no matching `</Line>`, so it is not a prompt tag: the
        // body renders as plain markdown, nothing broken onto its own line.
        let lines = render_message_body("it returns `Vec<Line>` from render", CLAUDE_PROMPT_TAGS);
        let joined = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref().to_string()))
            .collect::<Vec<_>>()
            .join("");
        assert!(joined.contains("Vec<Line>"), "{joined}");
        assert!(
            !lines.iter().any(|l| {
                let t: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                t.trim() == "<Line>"
            }),
            "a generic must not be broken out as a tag"
        );
    }

    fn rail_handle(
        id: &str,
        title: &str,
        group: Option<&str>,
        is_subagent: bool,
    ) -> transcripts::SessionHandle {
        transcripts::SessionHandle {
            harness: transcripts::Harness::Codex,
            id: id.into(),
            title: title.into(),
            git_branch: None,
            last_active: None,
            locator: transcripts::Locator::File(std::path::PathBuf::from("/x")),
            body_available: true,
            group: group.map(str::to_string),
            is_subagent,
        }
    }

    #[test]
    fn chat_rail_nests_subagents_under_their_director() {
        let sessions = vec![
            rail_handle("dir", "01a0", Some("G"), false),
            rail_handle("s1", "Lorentz", Some("G"), true),
            rail_handle("s2", "Kant", Some("G"), true),
            rail_handle("solo", "claude", None, false),
        ];
        // Collapsed: the director (a parent with 2 kids) and the standalone; the
        // subagents are hidden.
        let collapsed = chat_rail_rows(&sessions, &HashSet::new());
        assert_eq!(collapsed.len(), 2);
        assert!(collapsed[0].is_parent && collapsed[0].child_count == 2 && !collapsed[0].expanded);
        assert_eq!(collapsed[0].idx, 0);
        assert_eq!(collapsed[1].idx, 3);
        // Expanded: the two subagents appear nested at depth 1.
        let mut g = HashSet::new();
        g.insert("G".to_string());
        let exp = chat_rail_rows(&sessions, &g);
        assert_eq!(exp.len(), 4);
        assert_eq!((exp[1].idx, exp[1].depth), (1, 1));
        assert_eq!((exp[2].idx, exp[2].depth), (2, 1));
        assert_eq!(exp[3].idx, 3);
    }

    #[test]
    fn chat_fold_toggles_group_and_snaps_selection() {
        let mut a = app(&["x"]);
        a.view = View::Chat;
        a.chat_sessions = vec![
            rail_handle("dir", "01a0", Some("G"), false),
            rail_handle("s1", "Lorentz", Some("G"), true),
        ];
        a.chat_selected = 0; // director
        a.chat_toggle_fold();
        assert!(a.chat_expanded_groups.contains("G"), "expands the group");
        // Select the subagent, then collapse — selection snaps to the director.
        a.chat_selected = 1;
        a.chat_toggle_fold();
        assert!(!a.chat_expanded_groups.contains("G"), "collapses the group");
        assert_eq!(
            a.chat_selected, 0,
            "selection snaps off the now-hidden child"
        );
    }

    #[test]
    fn chat_session_stale_flags_new_activity() {
        use crate::transcripts::{Harness, Locator, SessionHandle};
        use std::time::Duration;
        let t1 = SystemTime::UNIX_EPOCH + Duration::from_secs(10);
        let t2 = SystemTime::UNIX_EPOCH + Duration::from_secs(20);
        let handle = |active| SessionHandle {
            harness: Harness::ClaudeCode,
            id: "s1".into(),
            title: "t".into(),
            git_branch: None,
            last_active: Some(active),
            locator: Locator::File(std::path::PathBuf::from("/x")),
            body_available: true,
            group: None,
            is_subagent: false,
        };
        let mut a = app(&["x"]);
        // Never seen → the dot shows (unseen activity).
        assert!(a.chat_session_stale(&handle(t1)));
        // Caught up at t1 → no dot.
        a.chat_seen.insert("s1".into(), t1);
        assert!(!a.chat_session_stale(&handle(t1)));
        // Newer activity than caught-up → the dot returns.
        assert!(a.chat_session_stale(&handle(t2)));
    }

    #[test]
    fn chat_tail_follow_arms_at_bottom_releases_above() {
        let mut a = app(&["x"]);
        a.view = View::Chat;
        a.chat_total_lines = 50;
        a.set_viewport(80, 12); // visible = 10 → bottom scroll = 40
        a.chat_scroll = 40;
        a.chat_follow = true;
        a.chat_scroll_by(-5);
        assert_eq!(a.chat_scroll, 35);
        assert!(!a.chat_follow, "scrolling up releases tail-follow");
        a.chat_scroll_by(100);
        assert_eq!(a.chat_scroll, 40, "clamps to the bottom");
        assert!(a.chat_follow, "returning to the bottom re-arms tail-follow");
    }

    #[test]
    fn chat_relayout_pins_to_bottom_when_following() {
        use crate::transcripts::{Event, EventKind, Harness, Role, Session};
        let ev = |t: &str| Event {
            role: Role::User,
            kind: EventKind::Message,
            ts: None,
            id: None,
            parent: None,
            is_sidechain: false,
            text: t.to_string(),
        };
        let session = Session {
            harness: Harness::ClaudeCode,
            id: "s".into(),
            title: "t".into(),
            git_branch: None,
            events: (0..30).map(|i| ev(&format!("line {i}"))).collect(),
        };
        let mut a = app(&["x"]);
        a.view = View::Chat;
        a.chat_session = Some(session);
        a.set_viewport(80, 12); // visible = 10
        a.chat_dirty = true;
        a.chat_follow = true;
        a.chat_relayout();
        assert!(
            a.chat_total_lines > 10,
            "the fixture fills more than a screen"
        );
        assert_eq!(
            a.chat_scroll,
            a.chat_total_lines - 10,
            "follow pins the view to the bottom"
        );
        // With follow released, relayout keeps the reader's position.
        a.chat_follow = false;
        a.chat_scroll = 0;
        a.chat_dirty = true;
        a.chat_relayout();
        assert_eq!(a.chat_scroll, 0, "released follow does not jump to bottom");
    }

    #[test]
    fn iso_short_extracts_month_day_time() {
        assert_eq!(
            iso_short("2026-08-21T15:12:09.279Z").as_deref(),
            Some("08-21 15:12")
        );
        assert_eq!(
            iso_short("2026-08-21 15:12:09").as_deref(),
            Some("08-21 15:12")
        );
        assert_eq!(iso_short("not a timestamp"), None);
        assert_eq!(iso_short("short"), None);
    }

    #[test]
    fn chat_layout_shows_a_faded_time_on_message_headers() {
        use crate::transcripts::{Event, EventKind, Harness, Role, Session};
        let session = Session {
            harness: Harness::ClaudeCode,
            id: "s".into(),
            title: "t".into(),
            git_branch: None,
            events: vec![Event {
                role: Role::User,
                kind: EventKind::Message,
                ts: Some("2026-08-21T15:12:09.279Z".into()),
                id: None,
                parent: None,
                is_sidechain: false,
                text: "hi".into(),
            }],
        };
        let joined = chat_layout(&session, &HashSet::new(), 80)
            .iter()
            .map(row_text)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            joined.contains("08-21 15:12"),
            "header carries the time: {joined}"
        );
    }

    #[test]
    fn chat_layout_groups_back_to_back_tool_calls() {
        use crate::transcripts::{Event, EventKind, Harness, Role, Session};
        let tool = |kind, text: &str| Event {
            role: Role::Tool,
            kind,
            ts: None,
            id: None,
            parent: None,
            is_sidechain: false,
            text: text.to_string(),
        };
        let msg = |text: &str| Event {
            role: Role::Assistant,
            kind: EventKind::Message,
            ts: None,
            id: None,
            parent: None,
            is_sidechain: false,
            text: text.to_string(),
        };
        let session = Session {
            harness: Harness::ClaudeCode,
            id: "s".into(),
            title: "t".into(),
            git_branch: None,
            events: vec![
                msg("doing things"),
                tool(EventKind::ToolCall, "Bash(ls)"),
                tool(EventKind::ToolResult, "file1\nfile2"),
                tool(EventKind::ToolCall, "Edit(x.rs)"),
                msg("done"),
            ],
        };
        let joined = |rows: &[ChatRow]| rows.iter().map(row_text).collect::<Vec<_>>().join("\n");

        // Collapsed: one "3 tool calls" fold; no individual calls shown.
        let collapsed = chat_layout(&session, &HashSet::new(), 80);
        let j = joined(&collapsed);
        assert!(j.contains("3 tool calls"), "{j}");
        assert!(
            !j.contains("Bash(ls)"),
            "calls hidden when the group is collapsed: {j}"
        );
        // The whole run is a single message-jump unit (start at its first index).
        let starts: Vec<usize> = collapsed
            .iter()
            .filter(|r| r.is_start)
            .map(|r| r.msg)
            .collect();
        assert_eq!(starts, vec![0, 1, 4], "group is one jump unit: {starts:?}");

        // Expanded (keyed by the run's first event index = 1): the calls appear.
        let mut exp = HashSet::new();
        exp.insert(1usize);
        let j2 = joined(&chat_layout(&session, &exp, 80));
        assert!(j2.contains("Bash(ls)") && j2.contains("Edit(x.rs)"), "{j2}");
    }

    #[test]
    fn chat_layout_groups_back_to_back_thinking() {
        use crate::transcripts::{Event, EventKind, Harness, Role, Session};
        let think = |text: &str| Event {
            role: Role::Assistant,
            kind: EventKind::Thinking,
            ts: None,
            id: None,
            parent: None,
            is_sidechain: false,
            text: text.to_string(),
        };
        let msg = |text: &str| Event {
            role: Role::Assistant,
            kind: EventKind::Message,
            ts: None,
            id: None,
            parent: None,
            is_sidechain: false,
            text: text.to_string(),
        };
        let session = Session {
            harness: Harness::ClaudeCode,
            id: "s".into(),
            title: "t".into(),
            git_branch: None,
            events: vec![
                msg("start"),
                think("first thought\nmore"),
                think("second thought"),
                think("third thought"),
                msg("end"),
            ],
        };
        let joined = |rows: &[ChatRow]| rows.iter().map(row_text).collect::<Vec<_>>().join("\n");

        let collapsed = chat_layout(&session, &HashSet::new(), 80);
        let j = joined(&collapsed);
        assert!(j.contains("3 thinking blocks"), "{j}");
        assert!(
            !j.contains("first thought"),
            "reasoning hidden when collapsed: {j}"
        );

        let mut exp = HashSet::new();
        exp.insert(1usize); // the run's first event index
        let j2 = joined(&chat_layout(&session, &exp, 80));
        assert!(
            j2.contains("first thought")
                && j2.contains("second thought")
                && j2.contains("third thought"),
            "{j2}"
        );
        assert!(
            j2.contains("· · ·"),
            "blocks are divided when expanded: {j2}"
        );
    }

    #[test]
    fn chat_layout_leaves_a_lone_tool_call_ungrouped() {
        use crate::transcripts::{Event, EventKind, Harness, Role, Session};
        let one = |kind, text: &str| Event {
            role: Role::Tool,
            kind,
            ts: None,
            id: None,
            parent: None,
            is_sidechain: false,
            text: text.to_string(),
        };
        let session = Session {
            harness: Harness::ClaudeCode,
            id: "s".into(),
            title: "t".into(),
            git_branch: None,
            // A single tool call between two thinking turns — no run to fold.
            events: vec![
                one(EventKind::Thinking, "pondering"),
                one(EventKind::ToolCall, "Bash(ls)"),
                one(EventKind::Thinking, "more"),
            ],
        };
        let j = chat_layout(&session, &HashSet::new(), 80)
            .iter()
            .map(row_text)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !j.contains("tool calls"),
            "a lone tool call is not grouped: {j}"
        );
        assert!(j.contains("⤷ tool"), "it keeps its single summary: {j}");
    }

    #[test]
    fn chat_layout_marks_message_starts_for_skip_and_current() {
        use crate::transcripts::{Event, EventKind, Harness, Role, Session};
        let mk = |role, text: &str| Event {
            role,
            kind: EventKind::Message,
            ts: None,
            id: None,
            parent: None,
            is_sidechain: false,
            text: text.to_string(),
        };
        let session = Session {
            harness: Harness::ClaudeCode,
            id: "s".into(),
            title: "t".into(),
            git_branch: None,
            events: vec![
                mk(Role::User, "one"),
                mk(Role::Assistant, "two"),
                mk(Role::User, "three"),
            ],
        };
        let rows = chat_layout(&session, &HashSet::new(), 80);
        // Exactly one start row per message, in order.
        let starts: Vec<usize> = rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.is_start)
            .map(|(_, r)| r.msg)
            .collect();
        assert_eq!(starts, vec![0, 1, 2], "one start per message: {starts:?}");
    }

    #[test]
    fn chat_scroll_and_msg_jump_move_and_clamp() {
        // Line scroll clamps at both ends; message-jump lands on start lines.
        let mut a = app(&["x"]);
        a.view = View::Chat;
        a.chat_total_lines = 10;
        a.chat_msg_starts = vec![(0, 0), (4, 1), (8, 2)];
        a.chat_scroll = 0;
        a.chat_scroll_by(3);
        assert_eq!(a.chat_scroll, 3);
        a.chat_scroll_by(-100);
        assert_eq!(a.chat_scroll, 0, "clamps at top");
        a.chat_scroll_by(100);
        assert_eq!(a.chat_scroll, 9, "clamps at last line");
        // Jump back to a message start, then step across messages.
        a.chat_scroll = 5; // inside message 1 (start line 4)
        a.chat_msg_jump(1);
        assert_eq!(a.chat_scroll, 8, "next message start");
        a.chat_msg_jump(-1);
        assert_eq!(a.chat_scroll, 4, "previous message start");
        a.chat_msg_jump(-1);
        assert_eq!(a.chat_scroll, 0, "clamps at first message");
    }

    #[test]
    fn page_and_nav_step_scale_with_the_viewport() {
        let mut a = app(&["x"]);
        a.view = View::Chat;
        a.chat_total_lines = 100;
        a.set_viewport(80, 20);
        assert_eq!(a.page_rows(), 19, "a page is the body height minus one");
        a.chat_scroll = 0;
        a.nav_step(a.page_rows());
        assert_eq!(a.chat_scroll, 19, "PgDn scrolls one page in Chat");
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

    fn tv(slug: &str) -> crate::substrate::TelosView {
        crate::substrate::TelosView {
            slug: slug.into(),
            title: format!("{slug} title"),
            statement: format!("the {slug} statement"),
            witnesses: vec!["published-artifact".into()],
        }
    }

    #[test]
    fn telos_select_moves_and_clamps() {
        // (AC-1) the telos list cursor moves ±1 and clamps at both ends.
        let mut a = app(&["telos/a"]);
        a.view = View::Process;
        a.process_pane = ProcessPane::Telos;
        a.fold.process.teloi = vec![tv("a"), tv("b"), tv("c")];
        a.telos_select(1);
        assert_eq!(a.telos_selected, 1);
        a.telos_select(9);
        assert_eq!(a.telos_selected, 2, "clamps at the last telos");
        a.telos_select(-9);
        assert_eq!(a.telos_selected, 0, "clamps at the first");
    }

    #[test]
    fn process_drill_toggles_detail_for_telos() {
        // (AC-2) Enter/Esc drills the Telos pane too, not only Atoms.
        let mut a = app(&["telos/a"]);
        a.view = View::Process;
        a.process_pane = ProcessPane::Telos;
        a.fold.process.teloi = vec![tv("a")];
        a.process_drill(true);
        assert!(a.process_detail, "Enter drills into the telos detail");
        a.process_drill(false);
        assert!(!a.process_detail, "Esc backs out");
    }

    #[test]
    fn telos_detail_shows_statement_witnesses_and_only_matching_tensions() {
        // (AC-3) the drilled view shows the slug/statement/witnesses and only the
        // tensions whose text names this telos.
        let t = crate::substrate::TelosView {
            slug: "kan-is-truth".into(),
            title: "kan is truth".into(),
            statement: "everything is a projection".into(),
            witnesses: vec!["published-artifact".into()],
        };
        let tensions = vec![
            crate::substrate::Tension {
                between: ("disposable".into(), "kan-is-truth".into()),
                why: "comments own the state cospan holds".into(),
            },
            crate::substrate::Tension {
                between: ("poll".into(), "subscribe".into()),
                why: "an unrelated trade-off".into(),
            },
        ];
        let mut probes = std::collections::BTreeMap::new();
        probes.insert(
            "published-artifact".to_string(),
            "path: .claims/*".to_string(),
        );
        let j = lines_text(&telos_detail(&t, &tensions, &probes, 80));
        assert!(j.contains("telos/kan-is-truth"), "{j}");
        assert!(j.contains("everything is a projection"), "{j}");
        // The witness shows its probe description alongside its type name.
        assert!(j.contains("published-artifact"), "{j}");
        assert!(j.contains("path: .claims/*"), "{j}");
        // The tension names the telos, shows the pair, and — the new detail —
        // the recorded rationale; the unrelated tension is omitted entirely.
        assert!(j.contains("kan-is-truth <-> disposable"), "pair: {j}");
        assert!(j.contains("comments own the state"), "the why: {j}");
        assert!(!j.contains("poll"), "omits unrelated tensions: {j}");
        assert!(!j.contains("unrelated trade-off"), "{j}");
    }

    #[test]
    fn telos_detail_falls_back_when_a_witness_has_no_probe() {
        let t = crate::substrate::TelosView {
            slug: "x".into(),
            title: "x".into(),
            statement: "s".into(),
            witnesses: vec!["mystery-witness".into()],
        };
        let j = lines_text(&telos_detail(
            &t,
            &[],
            &std::collections::BTreeMap::new(),
            80,
        ));
        assert!(j.contains("witnesses (1)"), "{j}");
        assert!(j.contains("· mystery-witness"), "{j}");
    }

    #[test]
    fn comments_header_shows_the_navigation_legend() {
        // The tree-nav, open, and authoring keys are visible in the header.
        let h = view_header(View::Comments);
        assert!(h.contains("fold/thread"), "no tree hint: {h}");
        assert!(h.contains("a add"), "no authoring hint: {h}");
        // Other views do not carry the authoring hint.
        assert!(!view_header(View::Ledger).contains("a add"));
    }

    #[test]
    fn build_file_rows_is_collapsed_by_default_and_expands_on_demand() {
        use crate::filetree::{FileEntry, GitStatus};
        let entries = vec![
            FileEntry {
                path: PathBuf::from("src/a.rs"),
                status: GitStatus::Modified,
            },
            FileEntry {
                path: PathBuf::from("src/sub/b.rs"),
                status: GitStatus::Clean,
            },
            FileEntry {
                path: PathBuf::from("README.md"),
                status: GitStatus::Untracked,
            },
        ];
        // Nothing expanded: only top level, `src` collapsed and its subtree hidden.
        let mut expanded = HashSet::new();
        let rows = build_file_rows(&entries, &expanded);
        assert!(matches!(&rows[0], FileRow::File { path, .. } if path == "README.md"));
        assert!(
            matches!(&rows[1], FileRow::Dir { path, collapsed, .. } if path == "src" && *collapsed)
        );
        assert_eq!(rows.len(), 2, "a collapsed dir shows no children: {rows:?}");

        // Expand `src`: its direct children appear; `src/sub` is still collapsed.
        expanded.insert("dir:src".into());
        let rows = build_file_rows(&entries, &expanded);
        assert!(rows
            .iter()
            .any(|r| matches!(r, FileRow::File { path, .. } if path == "src/a.rs")));
        assert!(rows
            .iter()
            .all(|r| !matches!(r, FileRow::File { path, .. } if path == "src/sub/b.rs")));

        // Expand `src/sub` too: the nested file appears, and its guide is deeper.
        expanded.insert("dir:src/sub".into());
        let rows = build_file_rows(&entries, &expanded);
        let b = rows
            .iter()
            .find(|r| matches!(r, FileRow::File { path, .. } if path == "src/sub/b.rs"))
            .expect("b.rs shows when its dirs are expanded");
        let FileRow::File { guide, .. } = b else {
            unreachable!()
        };
        assert!(
            guide.contains("└─") || guide.contains("├─"),
            "guide draws a connector: {guide:?}"
        );
    }

    #[test]
    fn file_move_previews_and_activate_opens_a_file() {
        // Over a real temp git repo: dirs are collapsed by default, moving onto a
        // file previews it (no focus change), and Enter focuses the gutter.
        let repo = comments_tmp("tree-open");
        std::fs::create_dir_all(repo.join("src")).unwrap();
        std::fs::write(repo.join("src/a.rs"), "fn a() {}\n").unwrap();
        let git = |args: &[&str]| {
            std::process::Command::new("git")
                .arg("-C")
                .arg(&repo)
                .args(args)
                .output()
                .unwrap();
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "t@t"]);
        git(&["config", "user.name", "t"]);
        git(&["add", "-A"]);

        let mut a = AppState::new(repo.clone(), fold_of(&[]), None);
        a.enter_comments();
        assert!(!a.file_rows.is_empty(), "the tree lists the repo's files");
        // `src` is collapsed by default, so `src/a.rs` is not visible yet.
        assert!(
            a.file_rows
                .iter()
                .all(|r| !matches!(r, FileRow::File { path, .. } if path == "src/a.rs")),
            "dirs are collapsed by default: {:?}",
            a.file_rows
        );

        // Expand `src` (its Dir row is the only row on this fixture).
        let dir_idx = a
            .file_rows
            .iter()
            .position(|r| matches!(r, FileRow::Dir { path, .. } if path == "src"))
            .expect("src dir row");
        a.file_selected = dir_idx;
        a.file_activate(); // expands, stays in Tree focus
        assert_eq!(a.comment_focus, CommentFocus::Tree);

        // Move the cursor onto src/a.rs -> it previews without focus change.
        let idx = a
            .file_rows
            .iter()
            .position(|r| matches!(r, FileRow::File { path, .. } if path == "src/a.rs"))
            .expect("src/a.rs visible once src is expanded");
        a.file_move(idx as isize - a.file_selected as isize);
        assert_eq!(a.open_file.as_deref(), Some(Path::new("src/a.rs")));
        assert_eq!(a.comment_content, "fn a() {}\n");
        assert_eq!(
            a.comment_focus,
            CommentFocus::Tree,
            "preview does not steal focus"
        );

        // Enter focuses the gutter for commenting.
        a.file_activate();
        assert_eq!(a.comment_focus, CommentFocus::Comments);
        assert_eq!(a.open_file.as_deref(), Some(Path::new("src/a.rs")));
        std::fs::remove_dir_all(&repo).ok();
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
        let (lines, unresolved) = gutter_lines(
            content,
            "",
            usize::MAX,
            &localized,
            0,
            &HashSet::new(),
            &crate::diff::FileDiff::empty(),
            false,
        );
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
        let lines = note_block(&c, &loc, 12, false, false);
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
    fn reflow_only_when_notes_collide() {
        use ratatui::text::Line;
        let code: Vec<Line> = (0..10).map(|i| Line::from(format!("l{i}"))).collect();
        let two = |s: &str| vec![Line::from(format!("{s}0")), Line::from(format!("{s}1"))];
        // Two 2-line notes at lines 1 and 5 — no collision (1..3 and 5..7 disjoint),
        // and neither runs past the 10 code lines: no reflow needed.
        let spaced = [(0usize, 1usize, two("a")), (1usize, 5usize, two("b"))];
        assert!(!notes_need_reflow(&spaced, code.len()));
        let (rows, note_rows) = side_by_side_rows(code.clone(), &spaced);
        assert_eq!(rows.len(), 10, "code is NOT pushed down");
        assert_eq!(line_text(&rows[1].0), "l1"); // code stays put
        assert_eq!(line_text(&rows[1].1), "a0"); // note beside its line
        assert_eq!(line_text(&rows[2].1), "a1"); // continuation beside the next code
        assert_eq!(line_text(&rows[2].0), "l2"); // …which is unchanged
        assert_eq!(note_rows, vec![(0, 1), (1, 5)]);

        // Notes at lines 1 and 2 — the first (2 lines) spills onto line 2 where the
        // second starts: they collide, so reflow IS needed.
        let colliding = [(0usize, 1usize, two("a")), (1usize, 2usize, two("b"))];
        assert!(notes_need_reflow(&colliding, code.len()));

        // A note running past the last code line needs reflow (appended rows).
        let past_eof = [(0usize, 9usize, two("a"))]; // 9..11 > 10
        assert!(notes_need_reflow(&past_eof, code.len()));
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
        let (_lines, unresolved) = gutter_lines(
            content,
            "",
            usize::MAX,
            &localized,
            0,
            &HashSet::new(),
            &crate::diff::FileDiff::empty(),
            false,
        );
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
        let texts: Vec<String> = thread_lines(&c, &loc, false)
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
    fn promoted_comment_gets_a_diamond_marker_and_kan_tag() {
        let content = "l0\nl1\n";
        let c = mk_comment(content, 0, "note"); // id "c_note"
        let loc = Localization {
            state: State::Anchored,
            span: Some((0, 0)),
            confidence: 1.0,
        };
        let localized = vec![(c.clone(), loc.clone())];

        // Not promoted: the hollow dot, and no kan tag in the strip.
        let empty = HashSet::new();
        let (lines, _) = gutter_lines(
            content,
            "",
            usize::MAX,
            &localized,
            0,
            &empty,
            &crate::diff::FileDiff::empty(),
            false,
        );
        assert_eq!(lines[0].spans[0].content.as_ref(), "●");
        let strip = thread_lines(&c, &loc, false);
        assert!(!strip
            .iter()
            .flat_map(|l| l.spans.iter())
            .any(|s| s.content.contains("kan")));

        // Promoted: a filled diamond, and a `◆ kan` tag in the strip header.
        let mut promoted = HashSet::new();
        promoted.insert("c_note".to_string());
        let (lines, _) = gutter_lines(
            content,
            "",
            usize::MAX,
            &localized,
            0,
            &promoted,
            &crate::diff::FileDiff::empty(),
            false,
        );
        assert_eq!(lines[0].spans[0].content.as_ref(), "◆");
        let strip = thread_lines(&c, &loc, true);
        assert!(strip
            .iter()
            .flat_map(|l| l.spans.iter())
            .any(|s| s.content.contains("kan")));
    }

    #[test]
    fn promoted_ids_reads_promotions_from_the_fold() {
        // A comment is "promoted" when a claim on comment/<file> carries its id.
        let mut a = app(&[]);
        a.open_file = Some(PathBuf::from("src/a.rs"));
        let content = "l0\n";
        a.comment_localized = vec![(
            mk_comment(content, 0, "note"), // id "c_note"
            Localization {
                state: State::Anchored,
                span: Some((0, 0)),
                confidence: 1.0,
            },
        )];
        // A promoted claim on the file's subject, carrying the comment id.
        a.fold.claims.insert(
            "comment/src/a.rs".into(),
            vec![mk_claim(
                "Observation",
                "note\n\n```cospan-comment\n{\"id\":\"c_note\"}\n```",
            )],
        );
        assert!(a.promoted_ids().contains("c_note"));

        // A claim for a different id does not mark this comment promoted.
        a.fold.claims.insert(
            "comment/src/a.rs".into(),
            vec![mk_claim("Observation", "x\n\n{\"id\":\"c_other\"}")],
        );
        assert!(a.promoted_ids().is_empty());
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
    fn refresh_comments_is_safe_with_no_open_file() {
        // The per-file refresh no-ops cleanly when no file is open.
        let mut a = app(&["telos/a"]);
        a.open_file = None;
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
        a.open_path(PathBuf::from("src/a.rs"));
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
        a.open_path(PathBuf::from("src/a.rs"));
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

        // Open the real file first, then switch to the deleted-source one.
        let mut a = AppState::new(repo, fold_of(&[]), None);
        a.open_path(PathBuf::from("src/real.rs"));
        assert_eq!(a.comment_content, content);

        // Switch to the deleted-source file.
        a.open_path(PathBuf::from("src/gone.rs"));
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
        let (lines, unresolved) = gutter_lines(
            &a.comment_content,
            "",
            usize::MAX,
            &a.comment_localized,
            a.comment_selected,
            &HashSet::new(),
            &crate::diff::FileDiff::empty(),
            false,
        );
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
            ..Default::default()
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
            ..Default::default()
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

    // --- S1: interactive comment authoring ---

    /// The author id the authoring code stamps (`$USER`, else `local`), so a
    /// fixture can be owned by "me" regardless of the CI environment.
    fn me_id() -> String {
        std::env::var("USER").unwrap_or_else(|_| "local".into())
    }

    fn mk_comment_by(content: &str, line0: usize, body: &str, author_id: &str) -> Comment {
        let mut c = mk_comment(content, line0, body);
        c.id = format!("id_{body}");
        c.author = crate::comments::Author {
            who: "human".into(),
            id: author_id.into(),
        };
        c
    }

    /// An `AppState` over a temp repo whose `src/a.rs` carries `seed`, with the
    /// Comments view loaded and ready to author.
    fn authoring_state(tag: &str, content: &str, seed: &[Comment]) -> (AppState, PathBuf) {
        let repo = comments_tmp(tag);
        std::fs::create_dir_all(repo.join("src")).unwrap();
        std::fs::write(repo.join("src/a.rs"), content).unwrap();
        write_sidecar(&repo, "src/a.rs", seed);
        let mut a = AppState::new(repo.clone(), fold_of(&[]), None);
        a.open_path(PathBuf::from("src/a.rs"));
        (a, repo)
    }

    fn sidecar_of(repo: &Path) -> Vec<Comment> {
        comments::load(&repo.join(comments::sidecar_path("src/a.rs"))).unwrap()
    }

    #[test]
    fn text_buf_inserts_newlines_moves_and_deletes() {
        // (AC-6) newline is stored verbatim; the cursor inserts mid-buffer and
        // backspaces the char before it.
        let mut b = TextBuf::default();
        for ch in "ab".chars() {
            b.insert(ch);
        }
        b.insert('\n');
        b.insert('c');
        assert_eq!(b.text, "ab\nc");
        // Move left twice (before the newline) and insert.
        b.left();
        b.left();
        b.insert('X'); // after "ab", before "\nc"
        assert_eq!(b.text, "abX\nc");
        // Backspace removes the char before the cursor.
        b.backspace();
        assert_eq!(b.text, "ab\nc");
        // Home/End bound the cursor.
        b.home();
        assert_eq!(b.cursor, 0);
        b.end();
        assert_eq!(b.cursor, b.text.chars().count());
    }

    #[test]
    fn compose_new_comment_appends_with_line_anchor() {
        // (AC-1/AC-7) composing a new comment on a line appends one record whose
        // anchor targets that line's text, authored human, thread empty — the same
        // structure `cospan comment add` (ctx 2) writes; then it surfaces on refresh.
        let content = "l0\nl1\nl2\n";
        let (mut a, repo) =
            authoring_state("author-new", content, &[mk_comment(content, 0, "seed")]);
        a.editing = Some(Editing::Compose {
            kind: ComposeKind::NewComment { line: 2 },
            buf: TextBuf::prefilled("new note"),
        });
        a.commit_editing();
        assert!(a.editing.is_none(), "commit clears the compose");

        let cs = sidecar_of(&repo);
        assert_eq!(cs.len(), 2);
        let added = cs.iter().find(|c| c.body == "new note").unwrap();
        assert_eq!(added.anchor.target, "l2");
        assert_eq!(added.anchor, comments::StoredAnchor::capture(content, 2, 2));
        assert_eq!(added.author.who, "human");
        assert!(added.thread.is_empty());
        // Refresh surfaced it with a localization, and the cursor is on it.
        assert!(a
            .comment_localized
            .iter()
            .any(|(c, _)| c.body == "new note"));
        assert_eq!(
            a.comment_localized[a.comment_selected].0.body, "new note",
            "the new comment is selected"
        );
    }

    #[test]
    fn compose_reply_grows_the_thread() {
        // (AC-2)
        let content = "l0\nl1\n";
        let (mut a, repo) =
            authoring_state("author-reply", content, &[mk_comment(content, 0, "seed")]);
        let id = a.comment_localized[0].0.id.clone();
        a.editing = Some(Editing::Compose {
            kind: ComposeKind::Reply { id },
            buf: TextBuf::prefilled("a reply"),
        });
        a.commit_editing();
        let seed = sidecar_of(&repo)
            .into_iter()
            .find(|c| c.body == "seed")
            .unwrap();
        assert_eq!(seed.thread.len(), 1);
        assert_eq!(seed.thread[0].body, "a reply");
        assert_eq!(seed.thread[0].author.who, "human");
    }

    #[test]
    fn compose_preserves_internal_newlines() {
        // (AC-6) a paragraph body keeps its embedded newline through the sidecar.
        let content = "l0\n";
        let (mut a, repo) = authoring_state(
            "author-multiline",
            content,
            &[mk_comment(content, 0, "seed")],
        );
        a.editing = Some(Editing::Compose {
            kind: ComposeKind::NewComment { line: 0 },
            buf: TextBuf::prefilled("line one\nline two"),
        });
        a.commit_editing();
        let added = sidecar_of(&repo)
            .into_iter()
            .find(|c| c.body.contains("line one"))
            .unwrap();
        assert_eq!(added.body, "line one\nline two");
    }

    #[test]
    fn toggle_resolve_flips_both_ways_through_the_sidecar() {
        // (AC-3)
        let content = "l0\nl1\n";
        let (mut a, repo) =
            authoring_state("author-resolve", content, &[mk_comment(content, 0, "seed")]);
        a.comment_selected = 0;
        a.toggle_resolve_selected();
        assert!(sidecar_of(&repo)[0].resolved);
        a.toggle_resolve_selected();
        assert!(!sidecar_of(&repo)[0].resolved, "second toggle un-resolves");
    }

    #[test]
    fn edit_and_delete_are_gated_to_the_author() {
        // (AC-4/AC-5 at the TUI level) you can edit and delete your own comment;
        // a comment owned by someone else is refused with a status, untouched.
        let content = "l0\nl1\n";
        let mine = mk_comment_by(content, 0, "mine", &me_id());
        let (mut a, repo) = authoring_state("author-own", content, &[mine]);
        a.comment_selected = 0;

        // Edit own: begin_edit opens a pre-filled compose; commit rewrites it.
        a.begin_edit();
        assert!(
            matches!(a.editing, Some(Editing::Compose { .. })),
            "own comment is editable"
        );
        a.editing = Some(Editing::Compose {
            kind: ComposeKind::Edit {
                id: "id_mine".into(),
            },
            buf: TextBuf::prefilled("edited"),
        });
        a.commit_editing();
        assert_eq!(sidecar_of(&repo)[0].body, "edited");

        // Delete own: gone from the sidecar.
        a.comment_selected = 0;
        a.delete_selected();
        assert!(sidecar_of(&repo).is_empty(), "own comment deleted");

        // A foreign comment: neither edit nor delete touches it.
        let theirs = mk_comment_by(content, 0, "theirs", "someone-else");
        let (mut b, repo2) = authoring_state("author-foreign", content, &[theirs]);
        b.comment_selected = 0;
        b.begin_edit();
        assert!(b.editing.is_none(), "a foreign comment is not editable");
        assert_eq!(b.comment_msg.as_deref(), Some("not your comment"));
        b.delete_selected();
        assert_eq!(
            sidecar_of(&repo2).len(),
            1,
            "a foreign comment is not deleted"
        );
        assert_eq!(b.comment_msg.as_deref(), Some("not your comment"));
    }

    #[test]
    fn editing_keys_route_pick_then_compose() {
        // Enter in pick-line opens a NewComment compose on that line; a letter
        // types into the buffer (does not quit); Ctrl-S commits; Esc cancels.
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let content = "l0\nl1\nl2\n";
        let (mut a, repo) =
            authoring_state("author-keys", content, &[mk_comment(content, 0, "seed")]);
        a.begin_new_comment();
        assert!(matches!(a.editing, Some(Editing::PickLine { .. })));
        // Move the pick cursor down one, then confirm the line.
        a.handle_editing_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        a.handle_editing_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        match &a.editing {
            Some(Editing::Compose {
                kind: ComposeKind::NewComment { line },
                ..
            }) => assert_eq!(*line, 1),
            _ => panic!("expected a NewComment compose on line 1"),
        }
        // Type a 'q' — it must land in the buffer, not quit.
        a.handle_editing_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        a.handle_editing_key(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE));
        a.handle_editing_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
        assert!(a.editing.is_none(), "Ctrl-S commits");
        assert!(
            sidecar_of(&repo).iter().any(|c| c.body == "q!"),
            "typed body was saved"
        );
    }

    #[test]
    fn commit_in_pick_line_is_a_noop() {
        // A stray Ctrl-S while still choosing the line must not cancel the pick.
        let content = "l0\nl1\n";
        let (mut a, _repo) = authoring_state(
            "author-picknoop",
            content,
            &[mk_comment(content, 0, "seed")],
        );
        a.begin_new_comment();
        assert!(matches!(a.editing, Some(Editing::PickLine { .. })));
        a.commit_editing();
        assert!(
            matches!(a.editing, Some(Editing::PickLine { .. })),
            "pick-line must survive a commit with no compose active"
        );
    }

    #[test]
    fn text_buf_vertical_movement_keeps_column_and_clamps() {
        let mut b = TextBuf::prefilled("ab\ncdef\ng");
        assert_eq!(b.row_col(), (2, 1)); // cursor at end, on "g"
        b.up();
        assert_eq!(b.row_col(), (1, 1)); // column preserved on the longer line
        b.up();
        assert_eq!(b.row_col(), (0, 1));
        b.up();
        assert_eq!(b.row_col(), (0, 1), "clamps at the top");
        b.down();
        b.down();
        assert_eq!(b.row_col(), (2, 1)); // column clamped to the short last line
        b.down();
        assert_eq!(b.row_col(), (2, 1), "clamps at the bottom");
    }

    #[test]
    fn compose_scroll_moves_the_caret_vertically() {
        let content = "l0\n";
        let (mut a, _repo) =
            authoring_state("compose-scroll", content, &[mk_comment(content, 0, "seed")]);
        a.editing = Some(Editing::Compose {
            kind: ComposeKind::NewComment { line: 0 },
            buf: TextBuf::prefilled("one\ntwo\nthree\nfour"),
        });
        let row = |a: &AppState| match &a.editing {
            Some(Editing::Compose { buf, .. }) => buf.row_col().0,
            _ => 99,
        };
        a.compose_scroll(-1); // wheel up: caret toward the top
        assert_eq!(row(&a), 0);
        a.compose_scroll(1); // wheel down: back to the bottom
        assert_eq!(row(&a), 3);
    }

    #[test]
    fn highlighted_gutter_marks_marker_and_backgrounds_the_covered_line() {
        // (AC-13) with a real grammar the code text is highlighted (>1 style across
        // the pane); the covered line carries a full-line background band and the
        // marker cell, and an uncommented line carries neither.
        use ratatui::style::Color;
        let content = "fn main() {\n    let x = 1;\n}\n";
        let c = mk_comment(content, 1, "note");
        let loc = Localization {
            state: State::Anchored,
            span: Some((1, 1)),
            confidence: 1.0,
        };
        let localized = vec![(c, loc)];
        let (lines, _u) = gutter_lines(
            content,
            "rs",
            usize::MAX,
            &localized,
            0,
            &HashSet::new(),
            &crate::diff::FileDiff::empty(),
            false,
        );
        // The commented line begins with a ● marker; an uncommented line does not.
        assert_eq!(lines[1].spans[0].content.as_ref(), "●");
        assert_eq!(lines[0].spans[0].content.as_ref(), " ");
        // The selected covered line has the stronger background on every span…
        assert!(
            lines[1]
                .spans
                .iter()
                .all(|s| s.style.bg == Some(Color::Indexed(240))),
            "the whole covered line is backgrounded"
        );
        // …and an uncommented line has no background band.
        assert!(lines[0].spans.iter().all(|s| s.style.bg.is_none()));
        // Highlighting produced more than one distinct text color across the pane.
        let styles: std::collections::HashSet<String> = lines
            .iter()
            .flat_map(|l| l.spans.iter().skip(2))
            .map(|s| format!("{:?}", s.style.fg))
            .collect();
        assert!(styles.len() > 1, "expected syntax colors, got {styles:?}");
    }

    #[test]
    fn arrow_keys_move_the_caret_between_lines() {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        let content = "l0\n";
        let (mut a, _repo) =
            authoring_state("compose-arrows", content, &[mk_comment(content, 0, "seed")]);
        a.editing = Some(Editing::Compose {
            kind: ComposeKind::NewComment { line: 0 },
            buf: TextBuf::prefilled("aaa\nbbb"),
        });
        a.handle_editing_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        match &a.editing {
            Some(Editing::Compose { buf, .. }) => assert_eq!(buf.row_col().0, 0, "Up moved a row"),
            _ => panic!("compose ended unexpectedly"),
        }
    }

    // --- Comments-tab editor-view layout redesign (Slice A) ---

    /// Render the whole TUI into a test backend and return the rows as strings.
    fn render_view(a: &AppState, w: u16, h: u16) -> Vec<String> {
        use ratatui::{backend::TestBackend, Terminal};
        let mut term = Terminal::new(TestBackend::new(w, h)).unwrap();
        term.draw(|f| draw(f, a)).unwrap();
        let buf = term.backend().buffer().clone();
        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| buf[(x, y)].symbol().to_string())
                    .collect::<String>()
            })
            .collect()
    }

    /// Give a test state a one-file tree, so `draw_comments` renders the panes
    /// rather than the empty "no files to browse" prompt (the temp repo is not a
    /// git checkout, so `reload_files` finds nothing).
    fn with_file_tree(a: &mut AppState) {
        a.file_entries = vec![filetree::FileEntry {
            path: PathBuf::from("src/a.rs"),
            status: filetree::GitStatus::Clean,
        }];
        a.rebuild_file_rows();
    }

    #[test]
    fn rail_shows_while_browsing_and_toggles_while_reading() {
        // (AC-1) Tree focus always shows the rail; Comments focus shows it only when
        // the tray is toggled open; a narrow terminal never shows it; `t` flips the
        // tray without changing focus.
        assert!(rail_visible(CommentFocus::Tree, false, true));
        assert!(!rail_visible(CommentFocus::Comments, false, true));
        assert!(rail_visible(CommentFocus::Comments, true, true));
        assert!(!rail_visible(CommentFocus::Tree, true, false)); // narrow

        let (mut a, _repo) = authoring_state("tray", "a\nb\n", &[]);
        assert_eq!(
            a.comment_focus,
            CommentFocus::Comments,
            "opening a file reads it"
        );
        assert!(!a.tray_open, "the tray starts closed while reading");
        a.toggle_tray();
        assert!(a.tray_open);
        assert_eq!(
            a.comment_focus,
            CommentFocus::Comments,
            "toggling the tray leaves focus"
        );
        // Opening another file re-collapses the tray (REQ-1) even after a toggle —
        // it must not stay stuck open across files.
        a.open_path(PathBuf::from("src/a.rs"));
        assert!(!a.tray_open, "opening a file re-collapses the tray");
    }

    #[test]
    fn unresolvable_group_lists_marks_selected_and_caps() {
        // (AC-3) one row per unplaceable comment, the selected one reversed, and a
        // "+N more" tail once past the cap.
        let content = "x\n";
        let a = mk_comment(content, 0, "lost one");
        let b = mk_comment(content, 0, "lost two");
        let rows = unresolvable_group(&[&a, &b], 40, Some("c_lost two"));
        assert_eq!(rows.len(), 2);
        let txt0: String = rows[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(txt0.contains("lost one"));
        assert!(
            rows[1].spans[0]
                .style
                .add_modifier
                .contains(ratatui::style::Modifier::REVERSED),
            "the selected unresolvable row is reversed"
        );

        let many: Vec<Comment> = (0..6)
            .map(|i| mk_comment(content, 0, &format!("u{i}")))
            .collect();
        let refs: Vec<&Comment> = many.iter().collect();
        let capped = unresolvable_group(&refs, 40, None);
        assert_eq!(capped.len(), 5, "CAP=4 rows plus a tail");
        let tail: String = capped[4].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            tail.contains("+2 more"),
            "tail counts the hidden ones: {tail:?}"
        );
    }

    #[test]
    fn enter_opens_thread_popup_and_esc_closes() {
        // (AC-4) Enter (gutter focus) opens the popup over the selected comment; Esc
        // closes it; with no comment there is nothing to open.
        let content = "l0\nl1\n";
        let (mut a, _r) = authoring_state("popup", content, &[mk_comment(content, 0, "hi")]);
        a.refresh_comments();
        assert!(!a.comment_localized.is_empty());
        a.comment_selected = 0;
        a.open_thread_popup();
        assert!(a.popup_open);
        a.handle_popup_key(crossterm::event::KeyCode::Esc);
        assert!(!a.popup_open);

        let (mut b, _r2) = authoring_state("popup-empty", content, &[]);
        b.refresh_comments();
        b.open_thread_popup();
        assert!(
            !b.popup_open,
            "no comment to read -> the popup stays closed"
        );
    }

    #[test]
    fn popup_actions_reply_resolve_and_close() {
        // (AC-7) inside the popup: `r` closes it and opens reply-compose; `x` toggles
        // resolved in place and keeps it open; Esc closes without mutating.
        let content = "l0\n";
        let mine = me_id();
        let (mut a, repo) = authoring_state(
            "popupact",
            content,
            &[mk_comment_by(content, 0, "seed", &mine)],
        );
        a.refresh_comments();
        a.comment_selected = 0;

        a.popup_open = true;
        a.handle_popup_key(crossterm::event::KeyCode::Char('r'));
        assert!(
            !a.popup_open,
            "r hands off to the composer, closing the popup"
        );
        assert!(
            matches!(
                a.editing,
                Some(Editing::Compose {
                    kind: ComposeKind::Reply { .. },
                    ..
                })
            ),
            "r began a reply"
        );
        a.editing = None;

        a.popup_open = true;
        a.handle_popup_key(crossterm::event::KeyCode::Char('x'));
        assert!(a.popup_open, "x keeps the popup open");
        assert!(
            sidecar_of(&repo)[0].resolved,
            "x resolved the comment in place"
        );

        a.handle_popup_key(crossterm::event::KeyCode::Esc);
        assert!(!a.popup_open);
    }

    #[test]
    fn comments_view_has_no_bottom_strip() {
        // (AC-2) with the strip gone, the code pane runs to the body bottom rather
        // than stopping 6 rows short for a strip beneath it.
        let content = "l0\nl1\nl2\n";
        let (mut a, _r) = authoring_state("nostrip", content, &[mk_comment(content, 0, "hi")]);
        a.refresh_comments();
        with_file_tree(&mut a);
        let rows = render_view(&a, 120, 24);
        let title = rows
            .iter()
            .position(|r| r.contains("src/a.rs"))
            .expect("code pane titled");
        // The focused code pane draws a thick border ('┗'); a plain one would be '└'.
        let bottom = (title + 1..rows.len())
            .find(|&y| rows[y].starts_with('┗') || rows[y].starts_with('└'))
            .expect("code pane has a bottom border");
        assert!(
            bottom >= rows.len() - 3,
            "code pane bottom border at row {bottom} of {} — no 6-row strip below it",
            rows.len()
        );
    }

    #[test]
    fn thread_popup_renders_only_when_open() {
        // (AC-4) the full thread lives in the popup, not an always-on strip: its
        // title appears only once the popup is open.
        let content = "l0\nl1\n";
        let (mut a, _r) = authoring_state("popuprender", content, &[mk_comment(content, 0, "hi")]);
        a.refresh_comments();
        with_file_tree(&mut a);
        a.comment_selected = 0;

        let closed = render_view(&a, 120, 24).join("\n");
        assert!(!closed.contains("comment thread"), "no popup while closed");

        a.popup_open = true;
        let open = render_view(&a, 120, 24).join("\n");
        assert!(
            open.contains("comment thread"),
            "the popup titles itself when open"
        );
    }

    #[test]
    fn narrow_layout_drops_note_column_and_strip() {
        // (AC-6) narrow shows only the code pane (no note column, no strip); the
        // popup still opens over it.
        let content = "l0\nl1\n";
        let (mut a, _r) = authoring_state("narrow", content, &[mk_comment(content, 0, "hi")]);
        a.refresh_comments();
        with_file_tree(&mut a);
        // Wide has two bordered panes (code + note column); narrow (80 < WIDE_COLS)
        // has only the code pane — no note column, no strip.
        let wide = render_view(&a, 120, 20).join("\n");
        let narrow = render_view(&a, 80, 20).join("\n");
        // Count top-left corners of both border styles: the focused code pane is
        // thick ('┏'), the note column plain ('┌').
        let corners = |s: &str| s.matches('┌').count() + s.matches('┏').count();
        assert_eq!(corners(&wide), 2, "wide: code + note boxes");
        assert_eq!(corners(&narrow), 1, "narrow: code box only");
        a.popup_open = true;
        let open = render_view(&a, 80, 20).join("\n");
        assert!(
            open.contains("comment thread"),
            "the popup still opens in narrow"
        );
    }

    #[test]
    fn pick_line_scrolls_stickily() {
        // Adding a comment: the line picker follows the cursor stickily instead of
        // re-centering it on every step.
        let content = (0..40)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        let (mut a, _r) = authoring_state("pick", &content, &[]);
        with_file_tree(&mut a);
        a.begin_new_comment();
        if let Some(Editing::PickLine { cursor }) = &mut a.editing {
            *cursor = 0;
        }
        a.note_scroll.set(0);
        // Cursor near the top stays visible -> no scroll.
        let _ = render_view(&a, 120, 12);
        assert_eq!(a.note_scroll.get(), 0, "cursor visible -> no scroll");
        // Move it well past the viewport bottom -> scroll just enough, not snap.
        if let Some(Editing::PickLine { cursor }) = &mut a.editing {
            *cursor = 20;
        }
        let _ = render_view(&a, 120, 12);
        let top = a.note_scroll.get();
        assert!(top > 0, "cursor below the viewport -> scrolled");
        assert!(
            top <= 20,
            "scrolled just far enough, not snapped to cursor-at-top"
        );
    }

    #[test]
    fn sticky_top_follows_only_off_screen() {
        // The viewport (5 rows: [top, top+5)) stays put while the selection is
        // visible, scrolls up to it when above, scrolls just enough when below, and
        // clamps to max_top.
        assert_eq!(sticky_top(0, 3, 5, 100), 0, "visible -> no move");
        assert_eq!(sticky_top(0, 4, 5, 100), 0, "last visible row -> no move");
        assert_eq!(sticky_top(2, 1, 5, 100), 1, "above top -> up to it");
        assert_eq!(sticky_top(0, 6, 5, 100), 2, "past bottom -> 6+1-5");
        assert_eq!(sticky_top(50, 8, 5, 4), 4, "clamped to max_top");
    }

    #[test]
    fn fill_line_bg_pads_banded_lines_only() {
        use ratatui::style::{Color, Style};
        use ratatui::text::{Line, Span};
        // A banded line (leading span carries a bg) is padded to width with that bg.
        let banded = Line::from(vec![Span::styled(
            "ab",
            Style::new().bg(Color::Indexed(240)),
        )]);
        let out = fill_line_bg(banded, 6);
        let width: usize = out.spans.iter().map(|s| s.content.chars().count()).sum();
        assert_eq!(width, 6, "padded to the full width");
        assert_eq!(
            out.spans.last().unwrap().style.bg,
            Some(Color::Indexed(240)),
            "the pad carries the band colour"
        );
        // A plain line (no bg) is left untouched.
        let plain = Line::from(vec![Span::raw("ab")]);
        assert_eq!(
            fill_line_bg(plain, 6).spans.len(),
            1,
            "no pad on an unbanded line"
        );
    }

    #[test]
    fn narrow_pins_unresolvable_band() {
        // (honest-ambiguity) an Unresolvable comment stays visible in narrow too,
        // where there is no note column or strip.
        let content = "l0\nl1\n";
        let (mut a, _r) = authoring_state("narrow-un", content, &[]);
        a.comment_localized = vec![(
            mk_comment("zzz\n", 0, "lost note"),
            Localization {
                state: State::Unresolvable,
                span: None,
                confidence: 0.0,
            },
        )];
        with_file_tree(&mut a);
        let rows = render_view(&a, 80, 20).join("\n");
        assert!(
            rows.contains("unresolvable (1)"),
            "narrow pins the band: {rows:?}"
        );
        assert!(
            rows.contains("lost note"),
            "the unresolvable comment is shown"
        );
    }

    #[test]
    fn unresolvable_group_keeps_selected_visible_past_cap() {
        // (REQ-4) selecting the 6th unresolvable still shows it, reversed, rather
        // than hiding it behind "+N more".
        let content = "x\n";
        let many: Vec<Comment> = (0..6)
            .map(|i| mk_comment(content, 0, &format!("u{i}")))
            .collect();
        let refs: Vec<&Comment> = many.iter().collect();
        let rows = unresolvable_group(&refs, 40, Some("c_u5"));
        assert_eq!(rows.len(), 5, "4 shown rows + a tail");
        let last_shown: String = rows[3].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            last_shown.contains("u5"),
            "the selected one is swapped in: {last_shown:?}"
        );
        assert!(rows[3].spans[0]
            .style
            .add_modifier
            .contains(ratatui::style::Modifier::REVERSED));
    }

    #[test]
    fn pick_line_pages_by_a_screenful() {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        // PgUp/PgDn move the pick cursor by a screenful (body_h - 1), clamped.
        let content = (0..60)
            .map(|i| format!("l{i}"))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        let (mut a, _r) = authoring_state("pgpick", &content, &[]);
        a.body_h = 12; // page = 11
        a.begin_new_comment();
        if let Some(Editing::PickLine { cursor }) = &mut a.editing {
            *cursor = 0;
        }
        a.handle_editing_key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE));
        match &a.editing {
            Some(Editing::PickLine { cursor }) => assert_eq!(*cursor, 11, "PageDown jumps a page"),
            _ => panic!("still picking a line"),
        }
        a.handle_editing_key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE));
        match &a.editing {
            Some(Editing::PickLine { cursor }) => assert_eq!(*cursor, 0, "PageUp jumps back"),
            _ => panic!("still picking a line"),
        }
    }

    #[test]
    fn unresolvable_band_does_not_hole_the_code_pane() {
        // Regression (operator eyeball): the pinned unresolvable band must not blank
        // the top of the code column. The code pane spans the full height with the
        // band pinned at the bottom of the note column instead.
        let content = "a\nb\nc\n";
        let (mut a, _r) = authoring_state("hole", content, &[]);
        a.comment_localized = vec![(
            mk_comment("zzz\n", 0, "lost note"),
            Localization {
                state: State::Unresolvable,
                span: None,
                confidence: 0.0,
            },
        )];
        with_file_tree(&mut a);
        let rows = render_view(&a, 120, 20);
        // The code pane's titled top border is on the first body row (row 1; row 0
        // is the tab header) — not pushed down by the band.
        let title_row = rows
            .iter()
            .position(|r| r.contains("src/a.rs"))
            .expect("code pane titled");
        assert_eq!(title_row, 1, "code pane starts at the content top, no hole");
        assert!(
            rows.join("\n").contains("unresolvable (1)"),
            "the band is still shown (at the bottom of the note column)"
        );
    }

    #[test]
    fn gutter_shows_diff_signs_when_on() {
        // (AC-4) +/~ and the two-sided deletion framing (`▁` above, `▔` below)
        // appear in the sign column when diff is on, and the column is absent off.
        use crate::diff::FileDiff;
        let content = "a\nb\nc\nd\ne\nf\n";
        let localized: Vec<(Comment, Localization)> = vec![];
        let mut fd = FileDiff::empty();
        fd.added.insert(1);
        fd.changed.insert(2);
        fd.deletions.insert(4, 2); // a removal between lines 3 and 4
        let (on, _) = gutter_lines(
            content,
            "",
            usize::MAX,
            &localized,
            0,
            &HashSet::new(),
            &fd,
            true,
        );
        let sign = |i: usize| on[i].spans[1].content.to_string();
        assert_eq!(sign(0), " ");
        assert_eq!(sign(1), "+");
        assert_eq!(sign(2), "~");
        assert_eq!(sign(3), "▁", "line above the deletion");
        assert_eq!(sign(4), "▔", "line below the deletion");
        // The red is a gutter highlight (bg on the sign cell), never a row tint:
        // the marker span carries no bg, the sign cell does.
        assert_eq!(
            on[3].spans[0].style.bg, None,
            "no row highlight on a deletion line"
        );
        assert_eq!(on[4].spans[0].style.bg, None);
        assert_eq!(
            on[3].spans[1].style.bg,
            Some(ratatui::style::Color::Indexed(52)),
            "the deletion sign cell is red-highlighted"
        );
        assert_eq!(
            on[3].spans[2].style.bg,
            Some(ratatui::style::Color::Indexed(52)),
            "the line-number cell is red-highlighted too"
        );
        assert_eq!(
            on[4].spans[1].style.bg,
            Some(ratatui::style::Color::Indexed(52))
        );
        // Off: no sign column, so span[1] is the line-number cell.
        let (off, _) = gutter_lines(
            content,
            "",
            usize::MAX,
            &localized,
            0,
            &HashSet::new(),
            &fd,
            false,
        );
        assert!(
            off[1].spans[1].content.contains('2'),
            "no sign column when off; span[1] is the number: {:?}",
            off[1].spans[1].content
        );
    }

    #[test]
    fn diff_tint_yields_to_comment_band() {
        // (AC-5) a line both changed and comment-covered keeps its comment band as
        // the row background while still showing the `~` sign.
        use crate::diff::FileDiff;
        let content = "a\nb\nc\n";
        let localized = vec![(
            mk_comment(content, 1, "note"),
            Localization {
                state: State::Anchored,
                span: Some((1, 1)),
                confidence: 1.0,
            },
        )];
        let mut fd = FileDiff::empty();
        fd.changed.insert(1);
        let (lines, _) = gutter_lines(
            content,
            "",
            usize::MAX,
            &localized,
            0,
            &HashSet::new(),
            &fd,
            true,
        );
        assert_eq!(
            lines[1].spans[0].style.bg,
            Some(ratatui::style::Color::Indexed(240)),
            "comment band wins the row background, not the diff tint"
        );
        assert_eq!(
            lines[1].spans[1].content.to_string(),
            "~",
            "diff sign still shows"
        );
    }

    #[test]
    fn diff_toggles_with_d() {
        // (AC-6) `D` flips diff_on without changing focus.
        let (mut a, _r) = authoring_state("difftoggle", "x\n", &[]);
        assert!(a.diff_on, "diff is on by default");
        a.toggle_diff();
        assert!(!a.diff_on);
        assert_eq!(
            a.comment_focus,
            CommentFocus::Comments,
            "toggle leaves focus"
        );
    }

    #[test]
    fn diff_render_respects_the_toggle() {
        // (AC-3) the cached diff drives the render, and toggling it off clears the
        // signs (no recompute needed).
        let content = "a\nb\nc\n";
        let (mut a, _r) = authoring_state("diffrender", content, &[]);
        with_file_tree(&mut a);
        a.file_diff.added.insert(1);
        let on = render_view(&a, 120, 12).join("\n");
        assert!(on.contains('+'), "diff sign rendered when on");
        a.diff_on = false;
        let off = render_view(&a, 120, 12).join("\n");
        assert!(!off.contains('+'), "no diff sign when toggled off");
    }

    #[test]
    fn compose_popup_y_tracks_the_target_line() {
        use ratatui::layout::Rect;
        let body = Rect {
            x: 0,
            y: 1,
            width: 100,
            height: 20,
        }; // rows 1..21
        let want_h = 6;
        // Room below the line -> popup sits just under it.
        assert_eq!(
            compose_popup_y(body, Some(2), want_h),
            5,
            "fits below -> just under the line"
        );
        // Near the bottom -> no room below -> goes just above.
        assert_eq!(
            compose_popup_y(body, Some(17), want_h),
            13,
            "no room below -> just above the line"
        );
        // No anchor line -> rests at the bottom.
        assert_eq!(
            compose_popup_y(body, None, want_h),
            15,
            "no target -> bottom"
        );
    }
}
