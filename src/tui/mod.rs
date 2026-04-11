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
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

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

                match key.code {
                    KeyCode::Char('q') => app.quit = true,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit = true;
                    }
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
                    KeyCode::Char('?') => app.show_help = !app.show_help,
                    _ => {}
                }
            }
            AppEvent::FileChanged(path) => {
                let was_selected = app.selected;
                if let Ok(Some(changed_idx)) = app.reverify_by_path(&path) {
                    // Auto-advance if the current exercise just passed
                    if changed_idx == was_selected
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
                if let Some((_, created)) = &app.flash_message {
                    if created.elapsed() > Duration::from_secs(2) {
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
