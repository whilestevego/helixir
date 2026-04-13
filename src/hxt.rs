//! Pure .hxt file parser — no I/O, operates on string content.

pub struct Sections {
    pub practice: String,
    pub expected: String,
}

pub struct DiffLine {
    pub line_num: usize,
    pub got: String,
    pub expected: String,
}

#[allow(dead_code)]
pub struct VerifyResult {
    pub passed: bool,
    pub practice: String,
    pub expected: String,
    pub diff: Vec<DiffLine>,
}

fn is_separator(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && trimmed.chars().all(|c| c == '─')
}

fn is_practice_marker(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains("PRACTICE") && trimmed.starts_with('─') && trimmed.ends_with('─')
}

fn is_expected_marker(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains("EXPECTED") && trimmed.starts_with('─') && trimmed.ends_with('─')
}

fn trim_blank_lines<'a>(lines: &'a [&'a str]) -> Vec<&'a str> {
    let start = lines
        .iter()
        .position(|l| !l.trim().is_empty())
        .unwrap_or(lines.len());
    let end = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map(|i| i + 1)
        .unwrap_or(start);
    lines[start..end].to_vec()
}

pub fn extract_sections(content: &str) -> Option<Sections> {
    let lines: Vec<&str> = content.split('\n').collect();

    let mut practice_start: Option<usize> = None;
    let mut expected_start: Option<usize> = None;
    let mut expected_end: Option<usize> = None;

    for (i, line) in lines.iter().enumerate() {
        if is_practice_marker(line) {
            practice_start = Some(i + 1);
        } else if is_expected_marker(line) {
            expected_start = Some(i + 1);
        } else if expected_start.is_some() && is_separator(line) {
            expected_end = Some(i); // Keep updating — last separator wins
        }
    }

    let practice_start = practice_start?;
    let expected_start = expected_start?;

    // Practice runs from practice_start to the line before EXPECTED marker
    let practice_lines = &lines[practice_start..expected_start - 1];
    // Expected runs from expected_start to the end marker (or end of file)
    let expected_lines = match expected_end {
        Some(end) => &lines[expected_start..end],
        None => &lines[expected_start..],
    };

    let practice = trim_blank_lines(practice_lines).join("\n");
    let expected = trim_blank_lines(expected_lines).join("\n");

    Some(Sections { practice, expected })
}

pub fn compute_diff(practice: &str, expected: &str) -> Vec<DiffLine> {
    let p_lines: Vec<&str> = practice.split('\n').collect();
    let e_lines: Vec<&str> = expected.split('\n').collect();
    let max_len = p_lines.len().max(e_lines.len());
    let mut diff = Vec::new();

    for i in 0..max_len {
        let p = p_lines.get(i).copied().unwrap_or("");
        let e = e_lines.get(i).copied().unwrap_or("");
        if p.trim_end() != e.trim_end() {
            diff.push(DiffLine {
                line_num: i + 1,
                got: p.to_string(),
                expected: e.to_string(),
            });
        }
    }

    diff
}

pub fn verify_content(content: &str) -> VerifyResult {
    match extract_sections(content) {
        None => VerifyResult {
            passed: false,
            practice: String::new(),
            expected: String::new(),
            diff: vec![],
        },
        Some(sections) => {
            let diff = compute_diff(&sections.practice, &sections.expected);
            let passed = diff.is_empty();
            VerifyResult {
                passed,
                practice: sections.practice,
                expected: sections.expected,
                diff,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSING_HXT: &str = "\
╔══════════════════════════════════════════════════════════════════╗
║  HELIXIR — Exercise 1.1: Test                                   ║
╚══════════════════════════════════════════════════════════════════╝

INSTRUCTIONS
────────────
  Do the thing.

────────────────────────── PRACTICE ──────────────────────────────

hello world
foo bar

────────────────────────── EXPECTED ──────────────────────────────

hello world
foo bar

──────────────────────────────────────────────────────────────────
HINTS:
  Hint 1: just do it";

    const FAILING_HXT: &str = "\
╔══════════════════════════════════════════════════════════════════╗
║  HELIXIR — Exercise 1.1: Test                                   ║
╚══════════════════════════════════════════════════════════════════╝

────────────────────────── PRACTICE ──────────────────────────────

hello world
WRONG LINE

────────────────────────── EXPECTED ──────────────────────────────

hello world
foo bar

──────────────────────────────────────────────────────────────────";

    #[test]
    fn test_passing_exercise() {
        let result = verify_content(PASSING_HXT);
        assert!(result.passed);
        assert!(result.diff.is_empty());
        assert_eq!(result.practice, "hello world\nfoo bar");
        assert_eq!(result.expected, "hello world\nfoo bar");
    }

    #[test]
    fn test_failing_exercise() {
        let result = verify_content(FAILING_HXT);
        assert!(!result.passed);
        assert_eq!(result.diff.len(), 1);
        assert_eq!(result.diff[0].line_num, 2);
        assert_eq!(result.diff[0].got, "WRONG LINE");
        assert_eq!(result.diff[0].expected, "foo bar");
    }

    #[test]
    fn test_trailing_whitespace_ignored() {
        let content = "\
────────────────────────── PRACTICE ──────────────────────────────

hello

────────────────────────── EXPECTED ──────────────────────────────

hello

──────────────────────────────────────────────────────────────────";
        let result = verify_content(content);
        assert!(result.passed);
    }

    #[test]
    fn test_malformed_returns_not_passed() {
        let content = "no markers here at all";
        let result = verify_content(content);
        assert!(!result.passed);
    }
}
