import { Glob } from "bun";
import { resolve } from "path";

const PRACTICE_MARKER = /^─+\s*PRACTICE\s*─+$/;
const EXPECTED_MARKER = /^─+\s*EXPECTED\s*─+$/;
const END_MARKER = /^─+$/;

export interface VerifyResult {
  file: string;
  passed: boolean;
  practiceText: string;
  expectedText: string;
  diff: string[];
}

function extractSections(content: string): { practice: string; expected: string } | null {
  const lines = content.split("\n");

  let practiceStart = -1;
  let expectedStart = -1;
  let expectedEnd = -1;

  for (let i = 0; i < lines.length; i++) {
    const trimmed = lines[i].trim();
    if (PRACTICE_MARKER.test(trimmed)) {
      practiceStart = i + 1;
    } else if (EXPECTED_MARKER.test(trimmed)) {
      expectedStart = i + 1;
    } else if (expectedStart > 0 && END_MARKER.test(trimmed)) {
      expectedEnd = i; // Keep updating — last ─+ line wins
    }
  }

  if (practiceStart < 0 || expectedStart < 0) return null;

  // Practice runs from practiceStart to the line before EXPECTED marker
  const practiceLines = lines.slice(practiceStart, expectedStart - 1);
  // Expected runs from expectedStart to the end marker (or end of section)
  const expectedLines = expectedEnd > 0
    ? lines.slice(expectedStart, expectedEnd)
    : lines.slice(expectedStart);

  // Trim leading/trailing empty lines from both sections
  const trim = (arr: string[]) => {
    while (arr.length > 0 && arr[0].trim() === "") arr.shift();
    while (arr.length > 0 && arr[arr.length - 1].trim() === "") arr.pop();
    return arr;
  };

  return {
    practice: trim([...practiceLines]).join("\n"),
    expected: trim([...expectedLines]).join("\n"),
  };
}

function computeDiff(practice: string, expected: string): string[] {
  const pLines = practice.split("\n");
  const eLines = expected.split("\n");
  const diff: string[] = [];
  const maxLen = Math.max(pLines.length, eLines.length);

  for (let i = 0; i < maxLen; i++) {
    const p = pLines[i] ?? "";
    const e = eLines[i] ?? "";
    // Normalize trailing whitespace for comparison
    if (p.trimEnd() !== e.trimEnd()) {
      diff.push(`  line ${i + 1}:`);
      diff.push(`    got:      "${p}"`);
      diff.push(`    expected: "${e}"`);
    }
  }

  return diff;
}

export async function parseHxt(filePath: string): Promise<VerifyResult> {
  const content = await Bun.file(filePath).text();
  const sections = extractSections(content);

  if (!sections) {
    return {
      file: filePath,
      passed: false,
      practiceText: "",
      expectedText: "",
      diff: ["Could not parse PRACTICE/EXPECTED sections"],
    };
  }

  const diff = computeDiff(sections.practice, sections.expected);

  return {
    file: filePath,
    passed: diff.length === 0,
    practiceText: sections.practice,
    expectedText: sections.expected,
    diff,
  };
}

export async function verify(filePath: string): Promise<VerifyResult> {
  const result = await parseHxt(filePath);

  if (result.passed) {
    console.log(`\x1b[32m✓\x1b[0m ${filePath}`);
  } else {
    console.log(`\x1b[31m✗\x1b[0m ${filePath}`);
    if (result.diff.length > 0) {
      console.log("\n  Differences:");
      for (const line of result.diff) {
        console.log(`  ${line}`);
      }
    }
  }

  return result;
}

export async function getExerciseFiles(exercisesDir: string): Promise<string[]> {
  const glob = new Glob("**/*.hxt");
  const files: string[] = [];
  for await (const file of glob.scan({ cwd: exercisesDir })) {
    files.push(resolve(exercisesDir, file));
  }
  return files.sort();
}

export async function verifyAll(exercisesDir: string): Promise<boolean> {
  const files = await getExerciseFiles(exercisesDir);

  if (files.length === 0) {
    console.log("No .hxt exercise files found.");
    return true;
  }

  let passed = 0;
  let failed = 0;

  for (const file of files) {
    const result = await parseHxt(file);
    const rel = file.replace(exercisesDir + "/", "");
    if (result.passed) {
      console.log(`  \x1b[32m✓\x1b[0m ${rel}`);
      passed++;
    } else {
      console.log(`  \x1b[31m✗\x1b[0m ${rel}`);
      failed++;
    }
  }

  console.log(`\n  ${passed} passed, ${failed} remaining\n`);
  return failed === 0;
}
