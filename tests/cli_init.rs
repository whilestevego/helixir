//! CLI smoke tests for the `helixir init` subcommand.

use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn helixir() -> Command {
    Command::cargo_bin("helixir").unwrap()
}

#[test]
fn init_default_target_creates_exercises_tree() {
    let tmp = tempdir().unwrap();
    helixir()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    let exercises = tmp.path().join("helixir-exercises/exercises");
    assert!(
        exercises.exists(),
        "expected {} to exist",
        exercises.display()
    );
    assert!(exercises.join("01-movement").is_dir());
    assert!(tmp.path().join("helixir-exercises/.gitignore").exists());
}

#[test]
fn init_custom_target_uses_supplied_path() {
    let tmp = tempdir().unwrap();
    helixir()
        .current_dir(tmp.path())
        .args(["init", "my-dir"])
        .assert()
        .success();

    assert!(tmp.path().join("my-dir/exercises/01-movement").is_dir());
}

#[test]
fn init_twice_reports_up_to_date() {
    let tmp = tempdir().unwrap();
    helixir()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();
    helixir()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("up to date"));
}

#[test]
fn init_after_deletion_reports_added_files() {
    let tmp = tempdir().unwrap();
    helixir()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success();

    let victim = tmp
        .path()
        .join("helixir-exercises/exercises/01-movement/01-basic-motion.md");
    fs::remove_file(&victim).unwrap();

    helixir()
        .current_dir(tmp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Added"));
    assert!(victim.exists(), "deleted file should be restored");
}

#[test]
fn no_subcommand_without_exercises_dir_fails() {
    let tmp = tempdir().unwrap();
    helixir()
        .current_dir(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("No exercises/ directory"));
}
