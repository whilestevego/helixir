//! Library-level tests for `commands::init`.

use std::fs;

use helixir::commands::init;
use tempfile::tempdir;

#[test]
fn install_missing_into_empty_dir_writes_all_files() {
    let dir = tempdir().unwrap();
    let dest = dir.path().join("exercises");
    let count = init::install_missing(&dest).unwrap();
    assert!(count > 0, "expected to write some exercise files");
    assert_eq!(
        init::count_missing_exercises(&dest),
        0,
        "nothing should be missing after full install"
    );
}

#[test]
fn install_missing_preserves_user_modifications() {
    let dir = tempdir().unwrap();
    let dest = dir.path().join("exercises");

    // Full install
    init::install_missing(&dest).unwrap();

    // Modify a known file
    let target = dest.join("01-movement/01-basic-motion.md");
    assert!(target.exists());
    fs::write(&target, "USER EDITED CONTENT\n").unwrap();

    // Re-install missing (should be a no-op for this file)
    let installed = init::install_missing(&dest).unwrap();
    assert_eq!(
        installed, 0,
        "no new files should be written when everything is present"
    );

    let contents = fs::read_to_string(&target).unwrap();
    assert_eq!(contents, "USER EDITED CONTENT\n");
}

#[test]
fn count_missing_returns_zero_after_full_install() {
    let dir = tempdir().unwrap();
    let dest = dir.path().join("exercises");
    init::install_missing(&dest).unwrap();
    assert_eq!(init::count_missing_exercises(&dest), 0);
}

#[test]
fn count_missing_detects_deleted_files() {
    let dir = tempdir().unwrap();
    let dest = dir.path().join("exercises");
    init::install_missing(&dest).unwrap();

    // Delete 3 known files
    let victims = [
        "01-movement/01-basic-motion.md",
        "01-movement/02-word-motion.md",
        "02-selection/01-basic-selection.md",
    ];
    let mut deleted = 0;
    for v in &victims {
        let p = dest.join(v);
        if p.exists() {
            fs::remove_file(&p).unwrap();
            deleted += 1;
        }
    }
    assert!(deleted > 0, "could not delete any victim files");

    assert_eq!(init::count_missing_exercises(&dest), deleted);
}

#[test]
fn install_missing_fills_only_gaps() {
    let dir = tempdir().unwrap();
    let dest = dir.path().join("exercises");
    init::install_missing(&dest).unwrap();

    // Modify one file
    let sentinel = dest.join("01-movement/02-word-motion.md");
    fs::write(&sentinel, "SENTINEL\n").unwrap();

    // Delete a different file
    let victim = dest.join("01-movement/01-basic-motion.md");
    if victim.exists() {
        fs::remove_file(&victim).unwrap();
    }

    let installed = init::install_missing(&dest).unwrap();
    assert_eq!(installed, 1, "only the deleted file should be re-written");

    // Victim restored, sentinel untouched
    assert!(victim.exists());
    assert_eq!(fs::read_to_string(&sentinel).unwrap(), "SENTINEL\n");
}
