pub mod app;
pub mod event;
pub mod ui;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::{App, ExerciseStatus};
use event::{AppEvent, EventHandler};

pub async fn run(exercises_dir: PathBuf) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(exercises_dir.clone())?;
    let mut events = EventHandler::new(exercises_dir);

    // Main loop
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        match events.next().await? {
            AppEvent::Key(key) => {
                if app.show_help {
                    match key.code {
                        KeyCode::Char('?') | KeyCode::Esc => app.show_help = false,
                        _ => {}
                    }
                    continue;
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
                    continue;
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
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => app.quit = true,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit = true;
                    }
                    // Cheat sheet overlay
                    KeyCode::Char('c') => {
                        app.show_cheatsheet = true;
                        app.cheatsheet_scroll = 0;
                    }
                    // Z-prefix chord (collapse/open/all)
                    KeyCode::Char('z') => app.pending_chord = Some('z'),
                    // Tab toggles current module's collapsed state
                    KeyCode::Tab => app.toggle_current_module(),
                    // Panel focus
                    KeyCode::Char('h') | KeyCode::Left => app.focus_left(),
                    KeyCode::Char('l') | KeyCode::Right => app.focus_right(),
                    // Scroll focused panel
                    KeyCode::Char('j') | KeyCode::Down => app.move_down(),
                    KeyCode::Char('k') | KeyCode::Up => app.move_up(),
                    // Actions
                    KeyCode::Char(' ') => app.reveal_hint(),
                    KeyCode::Char('n') => app.jump_next_incomplete(),
                    KeyCode::Char('r') => app.reset_current()?,
                    KeyCode::Char('u') => app.install_missing_exercises()?,
                    KeyCode::Char('?') => app.show_help = !app.show_help,
                    _ => {}
                }
            }
            AppEvent::FileChanged(path) => {
                let was_focused = app.current_exercise_index();
                if let Ok(Some(changed_idx)) = app.reverify_by_path(&path) {
                    // Auto-advance only when the currently focused exercise
                    // (cursor on Exercise, same index) just passed.
                    if was_focused == Some(changed_idx)
                        && app.exercises[changed_idx].status == ExerciseStatus::Passed
                    {
                        app.flash_message = Some((
                            "🎉 PASSED! Auto-advancing...".to_string(),
                            std::time::Instant::now(),
                        ));
                    }
                }
            }
            AppEvent::Tick => {
                // Clear flash message after 2 seconds
                if let Some((_, created)) = &app.flash_message
                    && created.elapsed() > Duration::from_secs(2)
                {
                    // Auto-advance after flash
                    let was_flash = app.flash_message.take();
                    if was_flash
                        .as_ref()
                        .is_some_and(|(msg, _)| msg.contains("PASSED"))
                    {
                        app.jump_next_incomplete();
                    }
                }
            }
        }

        if app.quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
