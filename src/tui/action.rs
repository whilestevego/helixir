//! Pure state-machine dispatcher for the TUI event loop.
//!
//! Takes an `AppEvent` and mutates `App` accordingly, returning an `Action`
//! that tells the outer loop what (if any) I/O-bearing side effect to perform.
//! Keeping the dispatch pure makes it trivially unit-testable: feed a sequence
//! of events into `handle_event` and assert on the resulting `App` state.

use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::{App, ExerciseStatus, InputMode};
use crate::tui::event::AppEvent;

/// A side effect the outer loop must perform on behalf of `handle_event`.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// No side effect needed. State may or may not have changed.
    None,
    /// The user asked to quit; the outer loop should break.
    Quit,
    /// Reset the currently-selected exercise back to its template.
    ResetCurrent,
    /// Install any missing exercises (additive init).
    InstallMissing,
}

/// How long a flash message stays on screen before auto-dismissing.
pub const FLASH_DURATION: Duration = Duration::from_secs(2);

/// Dispatch a single event against the given `App`. Pure state mutation plus
/// a returned `Action` describing any I/O the caller must perform.
///
/// `now` is injected so `Tick` handling (flash-message expiry) is
/// deterministic in tests.
pub fn handle_event(app: &mut App, event: AppEvent, now: Instant) -> Action {
    match event {
        AppEvent::Key(key) => handle_key(app, key),
        AppEvent::FileChanged(path) => {
            let was_focused = app.current_exercise_index();
            // Find the affected exercise by path so we can capture its prior
            // status before reverifying. Recording a "pass" only when the
            // status actually transitions into Passed prevents us from
            // bumping the completion count on every save while green.
            let target_idx = app.exercises.iter().position(|e| e.file_path == path);
            if let Some(idx) = target_idx {
                let prior = app.exercises[idx].status.clone();
                if app.reverify_exercise(idx).is_ok() {
                    let now_passed = app.exercises[idx].status == ExerciseStatus::Passed;
                    let just_passed = now_passed && prior != ExerciseStatus::Passed;
                    if just_passed {
                        app.record_pass(idx);
                    }
                    if was_focused == Some(idx) && now_passed {
                        app.flash_message = Some(("🎉 PASSED! Auto-advancing...".to_string(), now));
                    }
                }
            }
            Action::None
        }
        AppEvent::Tick => {
            if let Some((_, created)) = &app.flash_message
                && now.duration_since(*created) > FLASH_DURATION
            {
                let was_flash = app.flash_message.take();
                if was_flash
                    .as_ref()
                    .is_some_and(|(msg, _)| msg.contains("PASSED"))
                {
                    app.jump_next_incomplete();
                }
            }
            Action::None
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) -> Action {
    // Search input mode: keystrokes edit the query instead of dispatching
    // navigation. Handled before all other modal checks so '/' can't
    // collide and so Esc behaves like a search-cancel, not a quit.
    if app.input_mode == InputMode::Searching {
        match key.code {
            KeyCode::Esc => app.cancel_search(),
            KeyCode::Enter => app.commit_search(),
            KeyCode::Backspace => app.search_pop(),
            KeyCode::Char(c) => app.search_push(c),
            _ => {}
        }
        return Action::None;
    }

    if app.show_help {
        match key.code {
            KeyCode::Char('?') | KeyCode::Esc => app.show_help = false,
            _ => {}
        }
        return Action::None;
    }

    if app.show_cheatsheet {
        match key.code {
            KeyCode::Char('c') | KeyCode::Esc => {
                app.show_cheatsheet = false;
                app.cheatsheet_scroll = 0;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.cheatsheet_scroll = app.cheatsheet_scroll.saturating_add(3);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.cheatsheet_scroll = app.cheatsheet_scroll.saturating_sub(3);
            }
            _ => {}
        }
        return Action::None;
    }

    // Handle pending z-prefix chord
    if let Some('z') = app.pending_chord {
        app.pending_chord = None;
        match key.code {
            KeyCode::Char('c') => app.collapse_current_module(),
            KeyCode::Char('o') => app.expand_current_module(),
            KeyCode::Char('a') => app.toggle_current_module(),
            KeyCode::Char('M') => app.collapse_all_modules(),
            KeyCode::Char('R') => app.expand_all_modules(),
            _ => {}
        }
        return Action::None;
    }

    match key.code {
        KeyCode::Char('q') => {
            app.quit = true;
            Action::Quit
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit = true;
            Action::Quit
        }
        KeyCode::Char('c') => {
            app.show_cheatsheet = true;
            app.cheatsheet_scroll = 0;
            Action::None
        }
        KeyCode::Char('z') => {
            app.pending_chord = Some('z');
            Action::None
        }
        KeyCode::Tab => {
            app.toggle_current_module();
            Action::None
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app.focus_left();
            Action::None
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.focus_right();
            Action::None
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.move_down();
            Action::None
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.move_up();
            Action::None
        }
        KeyCode::Char(' ') => {
            app.reveal_hint();
            Action::None
        }
        KeyCode::Char('n') => {
            app.jump_next_incomplete();
            Action::None
        }
        KeyCode::Char('r') => Action::ResetCurrent,
        KeyCode::Char('u') => Action::InstallMissing,
        KeyCode::Char('/') => {
            app.enter_search();
            Action::None
        }
        KeyCode::Char('F') => {
            app.cycle_status_filter();
            Action::None
        }
        KeyCode::Esc => {
            if app.filter.is_active() {
                app.clear_filters();
            }
            Action::None
        }
        KeyCode::Char('?') => {
            app.show_help = !app.show_help;
            Action::None
        }
        _ => Action::None,
    }
}
