//! Exercise file parser — no I/O, operates on string content.
//!
//! Supports two marker formats:
//! - Legacy dashed lines: `────── PRACTICE ──────` (optionally inside comments)
//! - Markdown headings:   `## PRACTICE` / `## EXPECTED`

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

/// Strip a leading single-line comment prefix (`//`, `#`, `--`, `%`, `;`)
/// so that separator/marker detection works inside commented lines.
fn strip_comment_prefix(line: &str) -> &str {
    let trimmed = line.trim();
    for prefix in &["//", "#", "--", "%", ";"] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return rest.trim();
        }
    }
    trimmed
}

fn is_separator(line: &str) -> bool {
    let stripped = strip_comment_prefix(line);
    !stripped.is_empty() && stripped.chars().all(|c| c == '─')
}

fn is_practice_marker(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed == "## PRACTICE" {
        return true;
    }
    let stripped = strip_comment_prefix(line);
    stripped.contains("PRACTICE") && stripped.starts_with('─') && stripped.ends_with('─')
}

fn is_expected_marker(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed == "## EXPECTED" {
        return true;
    }
    let stripped = strip_comment_prefix(line);
    stripped.contains("EXPECTED") && stripped.starts_with('─') && stripped.ends_with('─')
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

    #[test]
    fn test_last_separator_wins_as_expected_end() {
        // `extract_sections` updates `expected_end` on every separator seen
        // after EXPECTED — the *last* separator wins. Content between the
        // first and last trailing separators is therefore INCLUDED in
        // `expected`. This mirrors the real `.hxt` files, which have exactly
        // one trailing separator.
        let content = "\
────────────────────────── PRACTICE ──────────────────────────────

hello

────────────────────────── EXPECTED ──────────────────────────────

hello

──────────────────────────────────────────────────────────────────
interstitial
──────────────────────────────────────────────────────────────────";
        let sections = extract_sections(content).expect("should parse");
        assert!(
            sections.expected.contains("interstitial"),
            "last separator wins: text between separators is part of expected, got {:?}",
            sections.expected
        );
    }

    #[test]
    fn test_leading_trailing_blank_lines_trimmed() {
        let content = "\
────────────────────────── PRACTICE ──────────────────────────────



  body

────────────────────────── EXPECTED ──────────────────────────────



  body

──────────────────────────────────────────────────────────────────";
        let sections = extract_sections(content).expect("should parse");
        assert_eq!(sections.practice, "  body");
        assert_eq!(sections.expected, "  body");
    }

    #[test]
    fn test_no_trailing_separator_after_expected() {
        // Content that ends immediately after EXPECTED body (no closing
        // separator) should still parse; expected extends to EOF.
        let content = "\
────────────────────────── PRACTICE ──────────────────────────────

hello

────────────────────────── EXPECTED ──────────────────────────────

hello";
        let sections = extract_sections(content).expect("should parse");
        assert_eq!(sections.expected, "hello");
    }

    #[test]
    fn test_markdown_heading_markers() {
        let content = "\
# Exercise Title

## PRACTICE

hello world
foo bar

## EXPECTED

hello world
foo bar";
        let result = verify_content(content);
        assert!(result.passed);
        assert_eq!(result.practice, "hello world\nfoo bar");
        assert_eq!(result.expected, "hello world\nfoo bar");
    }

    #[test]
    fn test_markdown_with_fenced_code_blocks() {
        let content = "\
# Tree-sitter Objects

## PRACTICE

```js
function foo() { return 42; }
```

## EXPECTED

```js
function foo() { return 42; }
```";
        let result = verify_content(content);
        assert!(result.passed);
        assert!(result.practice.contains("```js"));
        assert!(result.practice.contains("function foo"));
    }

    #[test]
    fn test_markdown_fenced_blocks_diff_only_code() {
        let content = "\
# Test

## PRACTICE

```js
function foo() {}
```

## EXPECTED

```js
function bar() {}
```";
        let result = verify_content(content);
        assert!(!result.passed);
        // Only the code line differs, not the fence lines
        assert_eq!(result.diff.len(), 1);
        assert_eq!(result.diff[0].got, "function foo() {}");
        assert_eq!(result.diff[0].expected, "function bar() {}");
    }

    #[test]
    fn test_markdown_multi_language_blocks() {
        let content = "\
# Surround Workflows

## PRACTICE

```css
.header { color: red; }
```

```js
log('hello');
```

## EXPECTED

```css
.header { color: red; }
```

```js
log('hello');
```";
        let result = verify_content(content);
        assert!(result.passed);
    }

    #[test]
    fn test_markdown_plain_text_no_fences() {
        let content = "\
# Basic Motion

## PRACTICE

The quick brown fox.

## EXPECTED

The slow brown fox.";
        let result = verify_content(content);
        assert!(!result.passed);
        assert_eq!(result.diff.len(), 1);
    }
}
