import { resolve, dirname } from "path";
import { getExerciseFiles } from "./verify";

const PROJECT_ROOT = resolve(dirname(new URL(import.meta.url).pathname), "..");

async function gitRestore(filePath: string): Promise<boolean> {
  const proc = Bun.spawn(["git", "checkout", "--", filePath], {
    cwd: PROJECT_ROOT,
    stdout: "pipe",
    stderr: "pipe",
  });
  const exitCode = await proc.exited;

  if (exitCode === 0) {
    const rel = filePath.replace(PROJECT_ROOT + "/", "");
    console.log(`\x1b[32m✓\x1b[0m Reset ${rel}`);
    return true;
  } else {
    const stderr = await new Response(proc.stderr).text();
    console.error(`\x1b[31m✗\x1b[0m Failed to reset: ${stderr.trim()}`);
    return false;
  }
}

export async function resetExercise(filePath: string): Promise<void> {
  await gitRestore(filePath);
}

export async function resetAll(exercisesDir: string): Promise<void> {
  const files = await getExerciseFiles(exercisesDir);

  if (files.length === 0) {
    console.log("No .hxt exercise files found.");
    return;
  }

  let count = 0;
  for (const file of files) {
    if (await gitRestore(file)) count++;
  }

  console.log(`\nReset ${count}/${files.length} exercises.`);
}
