use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::exercises::EXERCISES;

/// Extract embedded exercises to disk. If `skip_existing` is true,
/// only writes files that don't already exist (additive update).
/// Returns (new_count, skipped_count).
fn extract_dir(
    dir: &include_dir::Dir<'_>,
    dest: &Path,
    skip_existing: bool,
) -> Result<(usize, usize)> {
    let mut new_count = 0;
    let mut skipped = 0;

    for file in dir.files() {
        let dest_path = dest.join(file.path());
        if skip_existing && dest_path.exists() {
            skipped += 1;
            continue;
        }
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {}", parent.display()))?;
        }
        fs::write(&dest_path, file.contents())
            .with_context(|| format!("writing {}", dest_path.display()))?;
        new_count += 1;
    }

    for subdir in dir.dirs() {
        let (sub_new, sub_skipped) = extract_dir(subdir, dest, skip_existing)?;
        new_count += sub_new;
        skipped += sub_skipped;
    }

    Ok((new_count, skipped))
}

/// Count how many embedded exercise files are missing from the target directory.
pub fn count_missing_exercises(exercises_dir: &Path) -> usize {
    count_missing_in_dir(&EXERCISES, exercises_dir)
}

fn count_missing_in_dir(dir: &include_dir::Dir<'_>, dest: &Path) -> usize {
    let mut missing = 0;
    for file in dir.files() {
        let dest_path = dest.join(file.path());
        if !dest_path.exists() {
            missing += 1;
        }
    }
    for subdir in dir.dirs() {
        missing += count_missing_in_dir(subdir, dest);
    }
    missing
}

/// Install only the missing exercises into an existing exercises directory.
/// Returns the number of new files written.
pub fn install_missing(exercises_dir: &Path) -> Result<usize> {
    let (new_count, _) = extract_dir(&EXERCISES, exercises_dir, true)?;
    Ok(new_count)
}

pub fn run(target_arg: Option<&Path>) -> Result<()> {
    let target = target_arg
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| Path::new("helixir-exercises").to_path_buf());
    let target = if target.is_relative() {
        std::env::current_dir()?.join(&target)
    } else {
        target
    };

    let exercises_dest = target.join("exercises");

    // If exercises already exist, do an additive update
    if exercises_dest.join("README.md").exists() {
        let missing = count_missing_exercises(&exercises_dest);
        if missing == 0 {
            println!("\n  ✅ All exercises are already up to date.\n");
            return Ok(());
        }

        println!("\n  Updating exercises...\n");
        let (new_count, skipped) = extract_dir(&EXERCISES, &exercises_dest, true)?;
        println!(
            "  ✅ Added {} new exercise files ({} existing unchanged)\n",
            new_count, skipped
        );
        return Ok(());
    }

    // Fresh install
    println!("\n  Distilling Helixir exercises...\n");

    let (count, _) = extract_dir(&EXERCISES, &exercises_dest, false)?;

    // Create .gitignore
    fs::write(target.join(".gitignore"), "*.db\n.DS_Store\n")?;

    println!(
        "  ✅ Created {} exercise files in {}/exercises/",
        count,
        target.display()
    );

    let display_name = target_arg
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "helixir-exercises".to_string());

    println!(
        r#"
  Next steps:
    cd {}
    helixir                    Brew the TUI 🧪
    # Open exercise files in your editor in a split pane
"#,
        display_name
    );

    Ok(())
}
