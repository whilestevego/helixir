use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExerciseDb {
    pub exercises: Vec<ExerciseMeta>,
}

#[derive(Debug, Deserialize)]
pub struct ExerciseMeta {
    pub id: String,
    pub title: String,
    pub category: String,
    pub difficulty: u8,
    #[serde(default)]
    pub notes: String,
    pub instructions: String,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default)]
    pub commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub key: String,
    pub description: String,
}

static EXERCISES_TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/exercises.toml"));

pub fn load_exercises() -> &'static ExerciseDb {
    use std::sync::OnceLock;
    static DB: OnceLock<ExerciseDb> = OnceLock::new();
    DB.get_or_init(|| toml::from_str(EXERCISES_TOML).expect("exercises.toml is invalid"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_exercises() {
        let db = load_exercises();
        assert_eq!(db.exercises.len(), 72);
    }

    #[test]
    fn test_first_exercise_has_metadata() {
        let db = load_exercises();
        let first = &db.exercises[0];
        assert_eq!(first.id, "01-movement/01-basic-motion");
        assert_eq!(first.title, "Basic Motion");
        assert_eq!(first.category, "Movement");
        assert_eq!(first.difficulty, 1);
        assert!(!first.commands.is_empty());
        assert!(!first.instructions.is_empty());
    }

    #[test]
    fn test_all_exercises_have_required_fields() {
        let db = load_exercises();
        for ex in &db.exercises {
            assert!(!ex.id.is_empty(), "exercise has empty id");
            assert!(!ex.title.is_empty(), "exercise {} has empty title", ex.id);
            assert!(
                !ex.category.is_empty(),
                "exercise {} has empty category",
                ex.id
            );
            assert!(
                ex.difficulty >= 1 && ex.difficulty <= 3,
                "exercise {} has invalid difficulty",
                ex.id
            );
            assert!(
                !ex.instructions.is_empty(),
                "exercise {} has empty instructions",
                ex.id
            );
        }
    }

    #[test]
    fn test_every_exercise_id_has_matching_hxt_file() {
        let db = load_exercises();
        for ex in &db.exercises {
            let path = format!("{}.hxt", ex.id);
            assert!(
                crate::exercises::EXERCISES.get_file(&path).is_some(),
                "exercise id '{}' has no embedded .hxt file at '{}'",
                ex.id,
                path
            );
        }
    }

    #[test]
    fn test_no_duplicate_exercise_ids() {
        let db = load_exercises();
        let mut seen = std::collections::HashSet::new();
        for ex in &db.exercises {
            assert!(seen.insert(&ex.id), "duplicate exercise id: {}", ex.id);
        }
    }

    #[test]
    fn test_categories_are_contiguous() {
        // Exercises sharing a category must appear consecutively in the
        // TOML. Violations would make the grouped UI misrender.
        let db = load_exercises();
        let mut seen = std::collections::HashSet::new();
        let mut current: Option<&str> = None;
        for ex in &db.exercises {
            if current != Some(ex.category.as_str()) {
                assert!(
                    seen.insert(ex.category.clone()),
                    "category '{}' reappears after another category; must be contiguous",
                    ex.category
                );
                current = Some(ex.category.as_str());
            }
        }
    }
}
