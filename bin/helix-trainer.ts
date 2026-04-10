#!/usr/bin/env bun

import { resolve, dirname } from "path";
import { verify, verifyAll } from "../src/verify";
import { showProgress } from "../src/progress";
import { resetExercise, resetAll } from "../src/reset";

const PROJECT_ROOT = resolve(dirname(new URL(import.meta.url).pathname), "..");
const EXERCISES_DIR = resolve(PROJECT_ROOT, "exercises");

const [command, ...args] = process.argv.slice(2);

const USAGE = `
helix-trainer — Helix keybinding practice for Zed

Usage:
  helix-trainer verify [file]    Check exercise against expected output
  helix-trainer verify-all       Check all exercises
  helix-trainer progress         Show completion stats per module
  helix-trainer reset <file>     Reset an exercise to its original state
  helix-trainer reset-all        Reset all exercises
  helix-trainer next             Show the next incomplete exercise

Examples:
  helix-trainer verify exercises/04-text-objects/01-delimiter-objects.hxt
  helix-trainer progress
  helix-trainer reset exercises/01-movement/01-basic-motion.hxt
`.trim();

async function findNext(): Promise<void> {
  const { getExerciseFiles, parseHxt } = await import("../src/verify");
  const files = await getExerciseFiles(EXERCISES_DIR);

  for (const file of files) {
    const result = await parseHxt(file);
    if (!result.passed) {
      const rel = file.replace(PROJECT_ROOT + "/", "");
      console.log(rel);
      return;
    }
  }
  console.log("All exercises completed!");
}

switch (command) {
  case "verify": {
    const file = args[0];
    if (!file) {
      console.error("Usage: helix-trainer verify <file>");
      process.exit(1);
    }
    const filePath = resolve(file);
    const result = await verify(filePath);
    process.exit(result.passed ? 0 : 1);
    break;
  }
  case "verify-all": {
    const allPassed = await verifyAll(EXERCISES_DIR);
    process.exit(allPassed ? 0 : 1);
    break;
  }
  case "progress": {
    await showProgress(EXERCISES_DIR);
    break;
  }
  case "reset": {
    const file = args[0];
    if (!file) {
      console.error("Usage: helix-trainer reset <file>");
      process.exit(1);
    }
    await resetExercise(resolve(file));
    break;
  }
  case "reset-all": {
    await resetAll(EXERCISES_DIR);
    break;
  }
  case "next": {
    await findNext();
    break;
  }
  default: {
    console.log(USAGE);
    process.exit(command ? 1 : 0);
  }
}
