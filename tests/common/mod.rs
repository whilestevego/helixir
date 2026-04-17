//! Shared test helpers for building synthetic `App` states without disk I/O.

use std::path::{Path, PathBuf};

use helixir::hxt::DiffLine;
use helixir::metadata::{Command, ExerciseMeta};
use helixir::tui::app::{App, ExerciseState, ExerciseStatus};

fn make_meta(
    id: &str,
    title: &str,
    category: &str,
    difficulty: u8,
    hints: &[&str],
    commands: &[(&str, &str)],
) -> &'static ExerciseMeta {
    let meta = ExerciseMeta {
        id: id.to_string(),
        title: title.to_string(),
        category: category.to_string(),
        difficulty,
        notes: String::new(),
        instructions: format!("Instructions for {title}"),
        hints: hints.iter().map(|s| s.to_string()).collect(),
        commands: commands
            .iter()
            .map(|(k, d)| Command {
                key: k.to_string(),
                description: d.to_string(),
            })
            .collect(),
        extension: "md".to_string(),
    };
    // Leak into 'static — acceptable in test binaries.
    Box::leak(Box::new(meta))
}

fn make_state(
    meta: &'static ExerciseMeta,
    status: ExerciseStatus,
    diff: Vec<DiffLine>,
    dir: &Path,
) -> ExerciseState {
    let file_path = dir.join(meta.filename());
    ExerciseState {
        meta,
        status,
        diff,
        file_path,
    }
}

/// Build an `App` with 4 synthetic exercises across 2 modules:
///
/// * `Movement / m1` — Passed
/// * `Movement / m2` — Failed (1 diff line)
/// * `Selection / s1` — NotStarted
/// * `Selection / s2` — Passed
///
/// Both modules start collapsed *except* the one containing the initial
/// cursor, matching the production `App::new` behavior. With this layout the
/// initial cursor lands on `Movement/m2` (first non-passed).
pub fn test_app(dir: PathBuf) -> App {
    let m1 = make_meta(
        "movement/m1",
        "Move 1",
        "Movement",
        1,
        &["hint-a", "hint-b"],
        &[("h", "left"), ("l", "right")],
    );
    let m2 = make_meta(
        "movement/m2",
        "Move 2",
        "Movement",
        2,
        &["mv2-hint"],
        &[("w", "word")],
    );
    let s1 = make_meta(
        "selection/s1",
        "Select 1",
        "Selection",
        1,
        &[],
        &[("v", "select")],
    );
    let s2 = make_meta(
        "selection/s2",
        "Select 2",
        "Selection",
        2,
        &["hint"],
        &[("x", "line"), ("h", "left")], // duplicate 'h' across modules
    );

    let diff = vec![DiffLine {
        line_num: 1,
        got: "wrong".to_string(),
        expected: "right".to_string(),
    }];

    let exercises = vec![
        make_state(m1, ExerciseStatus::Passed, vec![], dir.as_path()),
        make_state(m2, ExerciseStatus::Failed, diff, dir.as_path()),
        make_state(s1, ExerciseStatus::NotStarted, vec![], dir.as_path()),
        make_state(s2, ExerciseStatus::Passed, vec![], dir.as_path()),
    ];

    App::from_exercises(exercises, dir)
}
