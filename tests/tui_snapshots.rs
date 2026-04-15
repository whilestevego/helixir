//! Snapshot tests for `tui::ui::render` at representative `App` states.
//!
//! Uses `ratatui::backend::TestBackend` to draw into an in-memory buffer,
//! then serializes the buffer row-by-row for `insta::assert_snapshot!`.

mod common;

use std::path::PathBuf;
use std::time::Instant;

use helixir::tui::app::{App, ExerciseStatus, InputMode, Panel, TreeCursor};
use helixir::tui::ui;
use ratatui::Terminal;
use ratatui::backend::TestBackend;

use common::test_app;

const WIDTH: u16 = 120;
const HEIGHT: u16 = 40;

fn render_to_string(app: &mut App) -> String {
    let backend = TestBackend::new(WIDTH, HEIGHT);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| ui::render(frame, app)).unwrap();
    let buffer = terminal.backend().buffer().clone();
    let mut out = String::new();
    for y in 0..buffer.area.height {
        let mut line = String::new();
        for x in 0..buffer.area.width {
            line.push_str(buffer[(x, y)].symbol());
        }
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
}

fn app() -> App {
    test_app(PathBuf::from("/tmp/helixir-test"))
}

#[test]
fn snapshot_initial_state() {
    let mut a = app();
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_help_overlay() {
    let mut a = app();
    a.show_help = true;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_cheatsheet_populated() {
    let mut a = app();
    a.show_cheatsheet = true;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_cheatsheet_empty() {
    let mut a = app();
    // Mark everything NotStarted so the cheatsheet has no passed commands.
    for ex in &mut a.exercises {
        ex.status = ExerciseStatus::NotStarted;
    }
    a.show_cheatsheet = true;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_failing_exercise() {
    let mut a = app();
    // Cursor starts on Exercise 1 (Failed).
    a.cursor = TreeCursor::Exercise(1);
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_failing_exercise_many_diffs() {
    // Regression guard for the no-truncation change: when an exercise has
    // more than the old cap of 5 diffs, every one must still render.
    // Also exercises the char-level highlighting with a shared-prefix case
    // ("helloX" vs "helloY") and a whitespace-visualization case ("a b"
    // vs "a  b" where the middle is a space).
    use helixir::hxt::DiffLine;
    let mut a = app();
    a.cursor = TreeCursor::Exercise(1);
    a.exercises[1].diff = vec![
        DiffLine {
            line_num: 1,
            got: "helloA".into(),
            expected: "helloB".into(),
        },
        DiffLine {
            line_num: 2,
            got: "foo bar".into(),
            expected: "foo  bar".into(),
        },
        DiffLine {
            line_num: 3,
            got: "completely different".into(),
            expected: "xyz".into(),
        },
        DiffLine {
            line_num: 4,
            got: "short".into(),
            expected: "shorter".into(),
        },
        DiffLine {
            line_num: 5,
            got: "five".into(),
            expected: "FIVE".into(),
        },
        DiffLine {
            line_num: 6,
            got: "six".into(),
            expected: "SIX".into(),
        },
        DiffLine {
            line_num: 7,
            got: "seven".into(),
            expected: "SEVEN".into(),
        },
    ];
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_passed_exercise() {
    let mut a = app();
    // Expand Selection so Exercise(3) (Passed) is visible, then cursor there.
    a.expand_all_modules();
    a.cursor = TreeCursor::Exercise(3);
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_not_started_exercise() {
    let mut a = app();
    a.expand_all_modules();
    a.cursor = TreeCursor::Exercise(2);
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_all_modules_collapsed() {
    let mut a = app();
    a.collapse_all_modules();
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_flash_message_visible() {
    let mut a = app();
    a.flash_message = Some(("🎉 PASSED! Auto-advancing...".to_string(), Instant::now()));
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_detail_panel_focused() {
    let mut a = app();
    a.focused_panel = Panel::Detail;
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_search_mode_active() {
    // Footer is replaced by the search-prompt line while typing.
    let mut a = app();
    a.input_mode = InputMode::Searching;
    a.filter.query = "sel".to_string();
    a.expand_all_modules();
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_status_filter_active() {
    // With a status filter set and no query, the footer shows a filter chip
    // + [Esc] clear hint, and only matching exercises render.
    let mut a = app();
    a.filter.status = Some(ExerciseStatus::Passed);
    a.expand_all_modules();
    a.fix_cursor_visibility();
    insta::assert_snapshot!(render_to_string(&mut a));
}

#[test]
fn snapshot_module_header_selected() {
    let mut a = app();
    a.cursor = TreeCursor::Module("Movement".to_string());
    insta::assert_snapshot!(render_to_string(&mut a));
}
