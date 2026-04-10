import { resolve, basename, dirname } from "path";
import { getExerciseFiles, parseHxt } from "./verify";

interface ModuleStats {
  name: string;
  total: number;
  completed: number;
}

export async function showProgress(exercisesDir: string): Promise<void> {
  const files = await getExerciseFiles(exercisesDir);

  if (files.length === 0) {
    console.log("No .hxt exercise files found.");
    return;
  }

  const modules = new Map<string, ModuleStats>();

  for (const file of files) {
    const rel = file.replace(exercisesDir + "/", "");
    const moduleName = dirname(rel);
    const result = await parseHxt(file);

    if (!modules.has(moduleName)) {
      modules.set(moduleName, { name: moduleName, total: 0, completed: 0 });
    }

    const stats = modules.get(moduleName)!;
    stats.total++;
    if (result.passed) stats.completed++;
  }

  let totalCompleted = 0;
  let totalExercises = 0;

  console.log("\n  HELIX TRAINER — Progress\n");
  console.log("  Module                       Progress");
  console.log("  ─────────────────────────────────────────");

  for (const [, stats] of [...modules.entries()].sort()) {
    totalCompleted += stats.completed;
    totalExercises += stats.total;

    const pct = Math.round((stats.completed / stats.total) * 100);
    const barLen = 15;
    const filled = Math.round((stats.completed / stats.total) * barLen);
    const bar = "█".repeat(filled) + "░".repeat(barLen - filled);

    const status = stats.completed === stats.total
      ? "\x1b[32m✓\x1b[0m"
      : " ";

    const name = stats.name.padEnd(28);
    const count = `${stats.completed}/${stats.total}`.padStart(5);

    console.log(`  ${status} ${name} ${bar} ${count}  ${pct}%`);
  }

  const overallPct = totalExercises > 0
    ? Math.round((totalCompleted / totalExercises) * 100)
    : 0;

  console.log("  ─────────────────────────────────────────");
  console.log(`  Total: ${totalCompleted}/${totalExercises} exercises completed (${overallPct}%)\n`);
}
