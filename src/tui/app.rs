use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::hxt;
use crate::metadata::{self, ExerciseMeta};
use crate::progress::{ExerciseProgress, Progress};

#[derive(Debug, Clone, PartialEq)]
pub enum ExerciseStatus {
    Passed,
    Failed,
    NotStarted,
}

pub struct ExerciseState {
    pub meta: &'static ExerciseMeta,
    pub status: ExerciseStatus,
    pub diff: Vec<hxt::DiffLine>,
    pub file_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Panel {
    List,
    Detail,
}

/// Where the tree cursor currently sits — either on a module header or a
/// specific exercise.
#[derive(Debug, Clone, PartialEq)]
pub enum TreeCursor {
    Module(String),
    Exercise(usize),
}

pub struct CheatsheetCommand {
    pub key: String,
    pub description: String,
}

pub struct CheatsheetModule {
    pub name: String,
    pub passed: usize,
    pub total: usize,
    pub commands: Vec<CheatsheetCommand>,
}

/// Current keyboard input mode. `Searching` redirects keystrokes into the
/// search query buffer instead of the normal navigation map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Searching,
}

/// User-controlled filter applied on top of the full exercise list.
/// Multiple dimensions combine with AND semantics.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Filter {
    /// Case-insensitive substring match against `title` + `category`.
    pub query: String,
    /// When `Some(s)`, only show exercises whose status equals `s`.
    pub status: Option<ExerciseStatus>,
}

impl Filter {
    pub fn is_active(&self) -> bool {
        !self.query.is_empty() || self.status.is_some()
    }
}

pub struct App {
    pub exercises: Vec<ExerciseState>,
    pub cursor: TreeCursor,
    pub scroll_offset: usize,
    pub detail_scroll: usize,
    pub detail_scroll_max: usize,
    pub hint_level: usize,
    pub show_help: bool,
    pub focused_panel: Panel,
    pub exercises_dir: PathBuf,
    pub quit: bool,
    pub flash_message: Option<(String, std::time::Instant)>,
    pub missing_exercises: usize,
    /// Module names (categories) that are currently collapsed in the tree.
    pub collapsed_modules: BTreeSet<String>,
    /// Cheat-sheet overlay visibility.
    pub show_cheatsheet: bool,
    /// Vertical scroll offset within the cheat-sheet overlay.
    pub cheatsheet_scroll: usize,
    /// Pending z-prefix chord (for zc/zo/za/zM/zR).
    pub pending_chord: Option<char>,
    /// Persistent completion history, loaded from `<exercises_dir>/.progress.json`.
    pub progress: Progress,
    /// Active search/status filter; empty = no filter.
    pub filter: Filter,
    /// Current input mode (Normal vs search-query entry).
    pub input_mode: InputMode,
}

impl App {
    pub fn new(exercises_dir: PathBuf) -> anyhow::Result<Self> {
        let missing_exercises = crate::commands::init::count_missing_exercises(&exercises_dir);

        let db = metadata::load_exercises();
        let mut exercises = Vec::with_capacity(db.exercises.len());

        for meta in &db.exercises {
            let file_path = exercises_dir.join(format!("{}.hxt", meta.id));
            let (status, diff) = if file_path.exists() {
                let content = fs::read_to_string(&file_path)?;
                let result = hxt::verify_content(&content);
                if result.passed {
                    (ExerciseStatus::Passed, vec![])
                } else {
                    (ExerciseStatus::Failed, result.diff)
                }
            } else {
                (ExerciseStatus::NotStarted, vec![])
            };

            exercises.push(ExerciseState {
                meta,
                status,
                diff,
                file_path,
            });
        }

        // Initial cursor: jump to the first non-Passed exercise so users
        // resume where they left off.
        let initial_selected = exercises
            .iter()
            .position(|e| e.status != ExerciseStatus::Passed)
            .unwrap_or(0);

        // Default-collapse every module *except* the one containing the
        // initial selection — keeps the panel scannable from launch.
        let initial_module = exercises
            .get(initial_selected)
            .map(|e| e.meta.category.clone());
        let mut collapsed_modules: BTreeSet<String> =
            exercises.iter().map(|e| e.meta.category.clone()).collect();
        if let Some(m) = initial_module {
            collapsed_modules.remove(&m);
        }

        let progress = Progress::load(&exercises_dir);

        Ok(App {
            exercises,
            cursor: TreeCursor::Exercise(initial_selected),
            scroll_offset: 0,
            detail_scroll: 0,
            detail_scroll_max: 0,
            hint_level: 0,
            show_help: false,
            focused_panel: Panel::List,
            exercises_dir,
            quit: false,
            flash_message: None,
            missing_exercises,
            collapsed_modules,
            show_cheatsheet: false,
            cheatsheet_scroll: 0,
            pending_chord: None,
            progress,
            filter: Filter::default(),
            input_mode: InputMode::Normal,
        })
    }

    /// Test/library constructor: builds an `App` from pre-constructed
    /// `ExerciseState` values, bypassing disk I/O. Useful for state-machine
    /// and UI snapshot tests. The initial cursor jumps to the first non-passed
    /// exercise (matching `App::new`), and the module containing that cursor
    /// is expanded while all others are collapsed.
    pub fn from_exercises(exercises: Vec<ExerciseState>, exercises_dir: PathBuf) -> Self {
        let initial_selected = exercises
            .iter()
            .position(|e| e.status != ExerciseStatus::Passed)
            .unwrap_or(0);

        let initial_module = exercises
            .get(initial_selected)
            .map(|e| e.meta.category.clone());
        let mut collapsed_modules: BTreeSet<String> =
            exercises.iter().map(|e| e.meta.category.clone()).collect();
        if let Some(m) = initial_module {
            collapsed_modules.remove(&m);
        }

        let cursor = if exercises.is_empty() {
            TreeCursor::Module(String::new())
        } else {
            TreeCursor::Exercise(initial_selected)
        };

        let progress = Progress::load(&exercises_dir);

        App {
            exercises,
            cursor,
            scroll_offset: 0,
            detail_scroll: 0,
            detail_scroll_max: 0,
            hint_level: 0,
            show_help: false,
            focused_panel: Panel::List,
            exercises_dir,
            quit: false,
            flash_message: None,
            missing_exercises: 0,
            collapsed_modules,
            show_cheatsheet: false,
            cheatsheet_scroll: 0,
            pending_chord: None,
            progress,
            filter: Filter::default(),
            input_mode: InputMode::Normal,
        }
    }

    /// Record a not-passed → passed transition for the given exercise and
    /// persist to disk. Save errors are logged but never propagated — we'd
    /// rather drop a single update than crash mid-session.
    pub fn record_pass(&mut self, index: usize) {
        let id = self.exercises[index].meta.id.clone();
        self.progress.record_pass(&id, chrono::Utc::now());
        if let Err(e) = self.progress.save() {
            eprintln!("warning: could not save progress: {}", e);
        }
    }

    /// Look up persisted completion history for an exercise by index.
    pub fn progress_for(&self, index: usize) -> Option<&ExerciseProgress> {
        self.progress.get(&self.exercises[index].meta.id)
    }

    /// The module name the cursor is currently on (whether on the header or
    /// on one of its exercises).
    pub fn cursor_module(&self) -> &str {
        match &self.cursor {
            TreeCursor::Module(m) => m,
            TreeCursor::Exercise(i) => &self.exercises[*i].meta.category,
        }
    }

    pub fn current_exercise_index(&self) -> Option<usize> {
        match &self.cursor {
            TreeCursor::Exercise(i) => Some(*i),
            TreeCursor::Module(_) => None,
        }
    }

    pub fn current_exercise(&self) -> Option<&ExerciseState> {
        self.current_exercise_index().map(|i| &self.exercises[i])
    }

    pub fn is_module_collapsed(&self, module: &str) -> bool {
        self.collapsed_modules.contains(module)
    }

    /// Toggle the collapsed state of the module the cursor is on.
    pub fn toggle_current_module(&mut self) {
        let module = self.cursor_module().to_string();
        if self.collapsed_modules.contains(&module) {
            self.collapsed_modules.remove(&module);
        } else {
            self.collapsed_modules.insert(module);
        }
        self.fix_stranded_cursor();
    }

    pub fn collapse_current_module(&mut self) {
        let module = self.cursor_module().to_string();
        self.collapsed_modules.insert(module);
        self.fix_stranded_cursor();
    }

    pub fn expand_current_module(&mut self) {
        let module = self.cursor_module().to_string();
        self.collapsed_modules.remove(&module);
    }

    /// If the cursor is on an exercise whose module is collapsed, promote
    /// the cursor to the module header so navigation stays usable.
    fn fix_stranded_cursor(&mut self) {
        if let TreeCursor::Exercise(idx) = &self.cursor {
            let module = self.exercises[*idx].meta.category.clone();
            if self.collapsed_modules.contains(&module) {
                self.cursor = TreeCursor::Module(module);
                self.hint_level = 0;
                self.detail_scroll = 0;
            }
        }
    }

    /// Case-insensitive substring + status check for a single exercise against
    /// the current filter.
    pub fn exercise_matches_filter(&self, idx: usize) -> bool {
        let ex = &self.exercises[idx];
        if let Some(want) = &self.filter.status
            && ex.status != *want
        {
            return false;
        }
        if !self.filter.query.is_empty() {
            let needle = self.filter.query.to_lowercase();
            let haystack = format!("{} {}", ex.meta.title, ex.meta.category).to_lowercase();
            if !haystack.contains(&needle) {
                return false;
            }
        }
        true
    }

    /// True when any exercise in the module passes the filter.
    fn module_has_match(&self, module: &str) -> bool {
        self.exercises
            .iter()
            .enumerate()
            .any(|(i, ex)| ex.meta.category == module && self.exercise_matches_filter(i))
    }

    /// The full ordered list of currently visible tree nodes (modules always
    /// shown; exercises shown only when their module is expanded). Respects
    /// the active filter: non-matching exercises are hidden, and modules with
    /// zero matches drop their header too.
    pub fn visible_tree(&self) -> Vec<TreeCursor> {
        let mut nodes = Vec::new();
        let mut current_module = String::new();
        for (i, ex) in self.exercises.iter().enumerate() {
            if ex.meta.category != current_module {
                current_module = ex.meta.category.clone();
                if self.filter.is_active() && !self.module_has_match(&current_module) {
                    continue;
                }
                nodes.push(TreeCursor::Module(current_module.clone()));
            }
            if self.is_module_collapsed(&ex.meta.category) {
                continue;
            }
            if self.filter.is_active() && !self.exercise_matches_filter(i) {
                continue;
            }
            nodes.push(TreeCursor::Exercise(i));
        }
        nodes
    }

    /// Move cursor to first visible node if current cursor is no longer in
    /// the visible tree (e.g. after filter change). No-op if already visible.
    pub fn fix_cursor_visibility(&mut self) {
        let tree = self.visible_tree();
        if tree.contains(&self.cursor) {
            return;
        }
        if let Some(first) = tree.into_iter().next() {
            self.cursor = first;
            self.hint_level = 0;
            self.detail_scroll = 0;
        }
    }

    /// Cycle the status filter: None → NotStarted → Failed → Passed → None.
    pub fn cycle_status_filter(&mut self) {
        self.filter.status = match self.filter.status {
            None => Some(ExerciseStatus::NotStarted),
            Some(ExerciseStatus::NotStarted) => Some(ExerciseStatus::Failed),
            Some(ExerciseStatus::Failed) => Some(ExerciseStatus::Passed),
            Some(ExerciseStatus::Passed) => None,
        };
        // When filter narrows, expand all modules so any remaining matches are
        // reachable without extra keystrokes.
        if self.filter.is_active() {
            self.expand_all_modules();
        }
        self.fix_cursor_visibility();
    }

    /// Clear all filter state and leave search mode if active.
    pub fn clear_filters(&mut self) {
        self.filter = Filter::default();
        self.input_mode = InputMode::Normal;
        self.fix_cursor_visibility();
    }

    /// Enter search mode. Starts with a fresh empty query.
    pub fn enter_search(&mut self) {
        self.filter.query.clear();
        self.input_mode = InputMode::Searching;
        // Expand everything so incremental search can show matches anywhere.
        self.expand_all_modules();
    }

    /// Append one character to the search query. Auto-updates visibility.
    pub fn search_push(&mut self, c: char) {
        self.filter.query.push(c);
        self.fix_cursor_visibility();
    }

    /// Pop the last character from the search query.
    pub fn search_pop(&mut self) {
        self.filter.query.pop();
        self.fix_cursor_visibility();
    }

    /// Commit the search (leave input mode but keep query active).
    pub fn commit_search(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    /// Cancel search: drop the query, exit input mode.
    pub fn cancel_search(&mut self) {
        self.filter.query.clear();
        self.input_mode = InputMode::Normal;
        self.fix_cursor_visibility();
    }

    pub fn collapse_all_modules(&mut self) {
        self.collapsed_modules = self
            .exercises
            .iter()
            .map(|e| e.meta.category.clone())
            .collect();
        self.fix_stranded_cursor();
    }

    pub fn expand_all_modules(&mut self) {
        self.collapsed_modules.clear();
    }

    /// Build the cheat-sheet view: a list of (module, passed, total, commands)
    /// where commands are deduped by key, drawn only from passed exercises,
    /// and modules appear in the order they first show up in the exercise list.
    pub fn build_cheatsheet(&self) -> Vec<CheatsheetModule> {
        let mut modules: Vec<CheatsheetModule> = Vec::new();
        let mut module_idx: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        // First pass: register every module in display order with totals.
        for ex in &self.exercises {
            if !module_idx.contains_key(&ex.meta.category) {
                module_idx.insert(ex.meta.category.clone(), modules.len());
                let (passed, total) = self.module_progress(&ex.meta.category);
                modules.push(CheatsheetModule {
                    name: ex.meta.category.clone(),
                    passed,
                    total,
                    commands: Vec::new(),
                });
            }
        }

        // Second pass: collect commands from passed exercises only, dedupe by key.
        for ex in &self.exercises {
            if ex.status != ExerciseStatus::Passed {
                continue;
            }
            let idx = module_idx[&ex.meta.category];
            for cmd in &ex.meta.commands {
                if !modules[idx].commands.iter().any(|c| c.key == cmd.key) {
                    modules[idx].commands.push(CheatsheetCommand {
                        key: cmd.key.clone(),
                        description: cmd.description.clone(),
                    });
                }
            }
        }

        modules
    }

    /// (passed, total) for a given module name.
    pub fn module_progress(&self, module: &str) -> (usize, usize) {
        let mut passed = 0;
        let mut total = 0;
        for ex in &self.exercises {
            if ex.meta.category == module {
                total += 1;
                if ex.status == ExerciseStatus::Passed {
                    passed += 1;
                }
            }
        }
        (passed, total)
    }

    pub fn focus_left(&mut self) {
        self.focused_panel = Panel::List;
    }

    pub fn focus_right(&mut self) {
        self.focused_panel = Panel::Detail;
    }

    /// Move down in whichever panel is focused
    pub fn move_down(&mut self) {
        match self.focused_panel {
            Panel::List => self.select_next(),
            Panel::Detail => self.scroll_detail_down(3),
        }
    }

    /// Move up in whichever panel is focused
    pub fn move_up(&mut self) {
        match self.focused_panel {
            Panel::List => self.select_prev(),
            Panel::Detail => self.scroll_detail_up(3),
        }
    }

    /// Move cursor to the next visible tree node (module or exercise).
    pub fn select_next(&mut self) {
        let tree = self.visible_tree();
        if let Some(pos) = tree.iter().position(|n| *n == self.cursor)
            && pos + 1 < tree.len()
        {
            self.cursor = tree[pos + 1].clone();
            self.hint_level = 0;
            self.detail_scroll = 0;
        }
    }

    /// Move cursor to the previous visible tree node (module or exercise).
    pub fn select_prev(&mut self) {
        let tree = self.visible_tree();
        if let Some(pos) = tree.iter().position(|n| *n == self.cursor)
            && pos > 0
        {
            self.cursor = tree[pos - 1].clone();
            self.hint_level = 0;
            self.detail_scroll = 0;
        }
    }

    pub fn jump_next_incomplete(&mut self) {
        let start = self.current_exercise_index().unwrap_or(0);
        for i in 0..self.exercises.len() {
            let idx = (start + 1 + i) % self.exercises.len();
            if self.exercises[idx].status != ExerciseStatus::Passed {
                self.cursor = TreeCursor::Exercise(idx);
                self.hint_level = 0;
                self.detail_scroll = 0;
                self.expand_current_module();
                return;
            }
        }
    }

    pub fn scroll_detail_down(&mut self, amount: usize) {
        self.detail_scroll = self
            .detail_scroll
            .saturating_add(amount)
            .min(self.detail_scroll_max);
    }

    pub fn scroll_detail_up(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_sub(amount);
    }

    pub fn reveal_hint(&mut self) {
        let Some(ex) = self.current_exercise() else {
            return;
        };
        let max_hints = ex.meta.hints.len();
        if self.hint_level < max_hints {
            self.hint_level += 1;
            // Scroll down to show the new hint
            self.detail_scroll = self.detail_scroll.saturating_add(3);
        }
    }

    pub fn reset_current(&mut self) -> anyhow::Result<()> {
        let Some(idx) = self.current_exercise_index() else {
            return Ok(());
        };
        let exercise = &self.exercises[idx];
        let template = crate::exercises::EXERCISES.get_file(format!("{}.hxt", exercise.meta.id));

        if let Some(template_file) = template {
            fs::write(&exercise.file_path, template_file.contents())?;
            self.flash_message =
                Some(("🔄 Exercise reset!".to_string(), std::time::Instant::now()));
            // Re-verify
            self.reverify_exercise(idx)?;
        }
        Ok(())
    }

    pub fn reverify_exercise(&mut self, index: usize) -> anyhow::Result<()> {
        let exercise = &mut self.exercises[index];
        if exercise.file_path.exists() {
            let content = fs::read_to_string(&exercise.file_path)?;
            let result = hxt::verify_content(&content);
            if result.passed {
                exercise.status = ExerciseStatus::Passed;
                exercise.diff = vec![];
            } else {
                exercise.status = ExerciseStatus::Failed;
                exercise.diff = result.diff;
            }
        }
        Ok(())
    }

    pub fn reverify_by_path(&mut self, path: &Path) -> anyhow::Result<Option<usize>> {
        for (i, exercise) in self.exercises.iter().enumerate() {
            if exercise.file_path == path {
                self.reverify_exercise(i)?;
                return Ok(Some(i));
            }
        }
        Ok(None)
    }

    pub fn install_missing_exercises(&mut self) -> anyhow::Result<()> {
        if self.missing_exercises == 0 {
            return Ok(());
        }

        let installed = crate::commands::init::install_missing(&self.exercises_dir)?;
        self.missing_exercises = 0;

        // Re-verify all exercises (new ones will now exist on disk)
        for i in 0..self.exercises.len() {
            self.reverify_exercise(i)?;
        }

        self.flash_message = Some((
            format!("📦 Installed {} new exercises!", installed),
            std::time::Instant::now(),
        ));

        Ok(())
    }

    pub fn completed_count(&self) -> usize {
        self.exercises
            .iter()
            .filter(|e| e.status == ExerciseStatus::Passed)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.exercises.len()
    }

    /// Modules in display order (the order they first appear in `exercises`).
    pub fn modules_in_order(&self) -> Vec<&str> {
        let mut seen = std::collections::HashSet::new();
        let mut out = Vec::new();
        for ex in &self.exercises {
            if seen.insert(ex.meta.category.as_str()) {
                out.push(ex.meta.category.as_str());
            }
        }
        out
    }

    pub fn current_module_index(&self) -> (usize, usize) {
        let current = self.cursor_module();
        let modules = self.modules_in_order();
        let idx = modules.iter().position(|m| *m == current).unwrap_or(0);
        (idx + 1, modules.len())
    }

    /// Returns Some((pos, total)) when cursor is on an exercise; None when on a module.
    pub fn current_exercise_in_module(&self) -> Option<(usize, usize)> {
        let idx = self.current_exercise_index()?;
        let category = &self.exercises[idx].meta.category;
        let module_exercises: Vec<usize> = self
            .exercises
            .iter()
            .enumerate()
            .filter(|(_, e)| &e.meta.category == category)
            .map(|(i, _)| i)
            .collect();
        let pos = module_exercises.iter().position(|&i| i == idx).unwrap_or(0);
        Some((pos + 1, module_exercises.len()))
    }

    /// Indices of exercises belonging to a given module, in order.
    pub fn exercises_in_module(&self, module: &str) -> Vec<usize> {
        self.exercises
            .iter()
            .enumerate()
            .filter(|(_, e)| e.meta.category == module)
            .map(|(i, _)| i)
            .collect()
    }
}
