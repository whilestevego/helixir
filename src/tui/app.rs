use std::fs;
use std::path::{Path, PathBuf};

use crate::hxt;
use crate::metadata::{self, ExerciseMeta};

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

pub struct App {
    pub exercises: Vec<ExerciseState>,
    pub selected: usize,
    pub scroll_offset: usize,
    pub detail_scroll: usize,
    pub detail_scroll_max: usize,
    pub hint_level: usize,
    pub show_help: bool,
    pub focused_panel: Panel,
    pub exercises_dir: PathBuf,
    pub quit: bool,
    pub flash_message: Option<(String, std::time::Instant)>,
}

impl App {
    pub fn new(exercises_dir: PathBuf) -> anyhow::Result<Self> {
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

        Ok(App {
            exercises,
            selected: 0,
            scroll_offset: 0,
            detail_scroll: 0,
            detail_scroll_max: 0,
            hint_level: 0,
            show_help: false,
            focused_panel: Panel::List,
            exercises_dir,
            quit: false,
            flash_message: None,
        })
    }

    pub fn selected_exercise(&self) -> &ExerciseState {
        &self.exercises[self.selected]
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

    pub fn select_next(&mut self) {
        if self.selected < self.exercises.len() - 1 {
            self.selected += 1;
            self.hint_level = 0;
            self.detail_scroll = 0;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.hint_level = 0;
            self.detail_scroll = 0;
        }
    }

    pub fn jump_next_incomplete(&mut self) {
        for i in 0..self.exercises.len() {
            let idx = (self.selected + 1 + i) % self.exercises.len();
            if self.exercises[idx].status != ExerciseStatus::Passed {
                self.selected = idx;
                self.hint_level = 0;
                self.detail_scroll = 0;
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
        let max_hints = self.selected_exercise().meta.hints.len();
        if self.hint_level < max_hints {
            self.hint_level += 1;
            // Scroll down to show the new hint
            self.detail_scroll = self.detail_scroll.saturating_add(3);
        }
    }

    pub fn reset_current(&mut self) -> anyhow::Result<()> {
        let exercise = &self.exercises[self.selected];
        let template = crate::exercises::EXERCISES
            .get_file(&format!("{}.hxt", exercise.meta.id));

        if let Some(template_file) = template {
            fs::write(&exercise.file_path, template_file.contents())?;
            self.flash_message = Some((
                "🔄 Exercise reset!".to_string(),
                std::time::Instant::now(),
            ));
            // Re-verify
            self.reverify_exercise(self.selected)?;
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

    pub fn completed_count(&self) -> usize {
        self.exercises
            .iter()
            .filter(|e| e.status == ExerciseStatus::Passed)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.exercises.len()
    }

    pub fn current_module_index(&self) -> (usize, usize) {
        let current_category = &self.selected_exercise().meta.category;
        let categories: Vec<&str> = self
            .exercises
            .iter()
            .map(|e| e.meta.category.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        let module_idx = categories
            .iter()
            .position(|c| c == &current_category.as_str())
            .unwrap_or(0);
        (module_idx + 1, categories.len())
    }

    pub fn current_exercise_in_module(&self) -> (usize, usize) {
        let current_category = &self.selected_exercise().meta.category;
        let module_exercises: Vec<usize> = self
            .exercises
            .iter()
            .enumerate()
            .filter(|(_, e)| e.meta.category == *current_category)
            .map(|(i, _)| i)
            .collect();
        let pos = module_exercises
            .iter()
            .position(|&i| i == self.selected)
            .unwrap_or(0);
        (pos + 1, module_exercises.len())
    }
}
