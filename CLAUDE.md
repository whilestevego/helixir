# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

- `cargo run` — Launch the TUI (expects an `exercises/` directory in cwd; run `cargo run -- init` first if needed).
- `cargo run -- init [dir]` — Generate a fresh exercise project (default `./helix-exercises`). Additive: preserves existing edited `.hxt` files.
- `cargo build --release` — Release build (profile uses `opt-level = "z"`, LTO, strip).
- `cargo test` — Run all tests. Single test: `cargo test <name>`. Integration tests use `assert_cmd` + `tempfile`.
- `cargo clippy` / `cargo fmt` — Standard Rust tooling.

Rust edition: **2024** (keep any new crates/code on edition 2024).

## Architecture

This is a single-binary Rust TUI (ratatui + crossterm + tokio) that teaches Helix keybindings by watching real editor files.

### Runtime flow

1. `main.rs` dispatches to either `commands::init::run` or `tui::run`.
2. `tui::run` (`src/tui/mod.rs`) owns the terminal lifecycle and event loop. It drives three event sources via `EventHandler` (`src/tui/event.rs`):
   - Keyboard input (crossterm)
   - File change notifications (`notify` + `notify-debouncer-mini`) watching the exercises directory
   - Tick events (for flash-message expiry and auto-advance)
3. On `FileChanged`, the app re-parses the changed `.hxt` file and re-verifies it. If the currently-selected exercise just passed, a flash message is set and `Tick` handling auto-advances via `jump_next_incomplete`.

### Data model (compile-time embedded)

- `exercises.toml` — metadata (title, category, difficulty, instructions, hints, commands) for all 65 exercises. Embedded via `include_str!` and deserialized by `src/metadata.rs`.
- `exercises/**/*.hxt` — exercise templates. Embedded via `include_dir!` and written to disk by `commands/init.rs`. `init` is **additive** — never overwrites existing files.
- An exercise's `id` in `exercises.toml` must match its path under `exercises/` without the `.hxt` extension.

### The `.hxt` format

Minimal: just `──── PRACTICE ────` and `──── EXPECTED ────` sections. `src/hxt.rs` is a pure parser — extracts the two sections and diffs them. All other exercise content (title, hints, commands, instructions) lives in `exercises.toml` so the editing surface stays clean.

### Module map

- `src/tui/app.rs` — `App` state, exercise list, selection, status, reverification, reset, install-missing.
- `src/tui/ui.rs` — layout and widget rendering (ratatui).
- `src/tui/event.rs` — async event multiplexing.
- `src/hxt.rs` — `.hxt` parser and PRACTICE/EXPECTED diff.
- `src/metadata.rs` — TOML deserialization for exercise metadata.
- `src/exercises.rs` — loads `.hxt` files from disk and joins with metadata.
- `src/commands/init.rs` — extracts embedded exercises into a target directory (additive).

## Conventions

- **UI text uses emojis** (🎉, 💡, 🔄, ⏭️, 📦) — this is intentional styling, keep it when adding TUI strings.
- When adding an exercise: create the `.hxt` under the appropriate `exercises/<module>/` directory **and** add a matching `[[exercises]]` entry in `exercises.toml` with `id` equal to the relative path minus `.hxt`. Verify with `cargo run`.
- Exercises are embedded at compile time — after editing `exercises.toml` or any `.hxt`, rebuild to see changes in the installed binary.
