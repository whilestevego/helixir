# Helix Trainer

**Learn Helix keybindings by doing — 49 hands-on exercises you complete in your real editor.**

Helix Trainer is a TUI app that guides you through 49 structured exercises. It watches your exercise files for changes, shows instructions and hints, and auto-advances as you complete each one. You edit in your real editor (Helix, Zed, or any editor with Helix keybindings) — no simulations, no quizzes, just deliberate practice.

```
helix-trainer init
cd helix-exercises
helix-trainer          # Launch the TUI
# Open exercise files in your editor in a split pane
```

---

## Why

Reading a keybinding reference is like reading a phrasebook — you recognize words but can't speak the language. Muscle memory comes from repetition in context.

Helix Trainer gives you that context: structured exercises that progressively build your fluency, from basic motion (`h`/`j`/`k`/`l`) to multi-selection workflows that feel like a superpower.

## How It Works

1. **Install** the tool and run `helix-trainer init`
2. **Launch** the TUI with `helix-trainer` from the project directory
3. **Open** exercise `.hxt` files in your editor in a split pane alongside the TUI
4. **Edit** the PRACTICE section to match the EXPECTED section using the commands shown in the TUI
5. **Save** — the TUI detects your changes, verifies them, and auto-advances on success
6. **Use the TUI** to navigate exercises, reveal hints, and reset exercises

The TUI shows instructions and commands. Your editor is where you practice.

## Install

```sh
# From crates.io
cargo install helix-trainer

# Or build from source
git clone https://github.com/yourusername/helix-trainer
cd helix-trainer
cargo install --path .
```

Pre-built binaries for macOS (Intel + Apple Silicon), Linux, and Windows are available on the [Releases](../../releases) page.

## Quick Start

```sh
# Generate the exercise project
helix-trainer init
cd helix-exercises

# Launch the TUI
helix-trainer

# In another terminal/pane, open exercise files in your editor
hx exercises/01-movement/01-basic-motion.hxt
```

The TUI shows the exercise instructions and commands. Edit the PRACTICE section in your editor to match EXPECTED. Save, and the TUI auto-verifies and advances.

## Exercise Format

Exercise `.hxt` files are minimal — just the content you need to edit:

```
────────────────────────── PRACTICE ──────────────────────────────

The color of the sky changes throughout the day. At dawn, a warm
color spreads across the horizon. By noon the color shifts to a
brilliant blue. Artists know that color theory is essential for
painting realistic scenes. The right color can set the entire
mood of a composition.

────────────────────────── EXPECTED ──────────────────────────────

The colour of the sky changes throughout the day. At dawn, a warm
colour spreads across the horizon. By noon the colour shifts to a
brilliant blue. Artists know that colour theory is essential for
painting realistic scenes. The right colour can set the entire
mood of a composition.

──────────────────────────────────────────────────────────────────
HINTS (read only if stuck):
  ...
```

- **COMMANDS TO LEARN** — the keybindings the exercise teaches
- **INSTRUCTIONS** — what to do and how
- **PRACTICE** — the text you edit with real Helix commands
- **EXPECTED** — what PRACTICE should look like when you're done
- **HINTS** — placed below EXPECTED so you scroll past the answer first

## Curriculum

49 exercises across 12 modules, organized in 4 progressive tiers.

### Tier 1 — Fundamentals

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **01 Movement** | 5 | `h`/`j`/`k`/`l`, `w`/`b`/`e`, `f`/`t` (multiline!), line navigation, scrolling |
| **02 Selection** | 4 | `x` (line select), `v` (extend mode), `;` (collapse), `%` (select all) |
| **03 Changes** | 5 | `d`/`c` (delete/change), `y`/`p` (yank/paste), `u`/`U` (undo/redo), indent, case |

### Tier 2 — Intermediate

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **04 Text Objects** | 4 | `m i`/`m a` + delimiters, words, functions, arguments (tree-sitter) |
| **05 Surround** | 4 | `m s` (add), `m r` (replace), `m d` (delete) surround characters |
| **06 Multi-Selection** | 5 | `s` (regex select), `S` (split), `C` (cursors), `K`/`Alt-K` (keep/remove) |
| **07 Search** | 3 | `/` search, `*` (use selection as pattern), global find-and-replace workflow |

### Tier 3 — Advanced

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **08 Goto Mode** | 4 | `g d`/`g r` (LSP), `g n`/`g p` (buffers), `g o`/`g u` (Zed git ops — not in native Helix) |
| **09 Space Mode** | 3 | `Space f` (files), `Space s` (symbols), `Space r` (rename), clipboard |
| **10 Unimpaired** | 3 | `]d`/`[d` (diagnostics), `]f`/`[f` (functions), indent navigation |

### Tier 4 — Mastery

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **11 Advanced Workflows** | 5 | Rename variable, extract function, reformat data, bulk transform, Vim-to-Helix |
| **12 Challenges** | 4 | Speed edit, minimal keystrokes, real-world refactor, code golf |

## Usage

### `helix-trainer init [dir]`

Generate a new exercise project.

```sh
helix-trainer init                    # Creates ./helix-exercises/
helix-trainer init ~/my-training      # Custom location
```

### `helix-trainer`

Launch the TUI from inside a project directory. The TUI watches exercise files for changes and auto-verifies on save.

**TUI Keybindings:**

| Key | Action |
|-----|--------|
| `h`/`l` or ←/→ | Focus left/right panel |
| `j`/`k` or ↑/↓ | Scroll focused panel |
| `Space` | 💡 Reveal next hint |
| `r` | 🔄 Reset current exercise |
| `n` | ⏭️ Jump to next incomplete |
| `?` | Toggle help overlay |
| `q` | Quit |

## The Helix Mental Model

Helix reverses Vim's editing grammar:

| | Vim | Helix |
|---|---|---|
| **Model** | Verb → Object (`dw`) | Selection → Action (`wd`) |
| **Delete word** | `dw` | `wd` |
| **Change inside quotes** | `ci"` | `mi"c` |
| **Delete line** | `dd` | `xd` |
| **Yank paragraph** | `yap` | `mapy` |

The core insight: **you always see what will be affected before you act**. Every motion creates a visible selection. You refine it, then commit. No more "oops, I deleted the wrong thing."

Multi-selection takes this further. Instead of `:%s/old/new/g`, you:

```
%           Select entire file
s old       Split into selections on "old"
c new       Change all selections simultaneously
```

This is the workflow you'll master in Module 06.

## Tips for Getting the Most Out of It

**Follow the progression.** The exercises build on each other. Module 04 (text objects) assumes you know Module 03 (changes). Module 06 (multi-selection) assumes you know Module 04.

**One module per session.** Don't grind through all 49 in a day. Do a module, then use those commands in your real work. Come back tomorrow.

**Use the which-key popup.** Press any prefix key (`g`, `m`, `Space`, `z`, `]`, `[`) and pause — a popup shows all available sub-commands. This is your cheat sheet.

**`;` is your reset button.** If a selection goes wrong, press `;` to collapse it back to a cursor and try again. Build this habit early.

**Use the TUI hints.** Press `Space` to reveal hints one at a time. Try each exercise without hints first. Struggle is where learning happens.

**Repeat the hard ones.** Press `r` in the TUI to reset any exercise. The challenges in Module 12 are designed for repeated practice.

## Compatibility

The exercises use standard Helix keybindings and work in any editor that supports them:

- **[Helix](https://helix-editor.com)** — native support
- **[Zed](https://zed.dev)** — built-in Helix mode (`"helix_mode": true` in settings.json)
- **Neovim** — with a Helix emulation plugin

The CLI is a single static binary with no runtime dependencies. The exercises themselves are plain text files embedded in the binary.

## Contributing

Contributions are welcome! Here's how you can help:

### Adding exercises

1. Follow the `.hxt` format (see any existing exercise for reference)
2. Place the file in the appropriate module directory
3. Ensure PRACTICE and EXPECTED sections are bounded by the marker lines
4. Test with `helix-trainer verify` to confirm the parser handles your file
5. Include progressive hints

### Exercise quality checklist

- [ ] Instructions are clear enough to follow without prior Helix experience for that tier
- [ ] PRACTICE text is realistic (code, prose, or data — not lorem ipsum)
- [ ] EXPECTED result is achievable with the listed commands
- [ ] Hints are ordered from gentle nudge to explicit keystroke sequence
- [ ] The exercise teaches something that builds on previous modules

### Reporting issues

If an exercise has incorrect expected output, unclear instructions, or a keybinding that doesn't work in your editor's Helix mode, please [open an issue](../../issues).

## How It's Built

A single-binary Rust TUI built with ratatui:

- **`src/tui/`** — TUI app: event loop, layout, widgets, file watching
- **`src/hxt.rs`** — Pure parser for `.hxt` files: extracts PRACTICE/EXPECTED sections, diffs them
- **`src/metadata.rs`** — Exercise metadata (titles, instructions, hints) deserialized from embedded TOML
- **`src/commands/init.rs`** — Extracts embedded exercises to a new directory
- **`exercises.toml`** — All 49 exercises' metadata (embedded in binary at compile time)

Exercise templates and metadata are compiled into the binary via `include_dir!` and `include_str!`. The TUI watches `.hxt` files for changes using the `notify` crate.

## License

MIT

## Acknowledgments

- [Helix Editor](https://helix-editor.com) — for the selection-first editing model
- [Zed](https://zed.dev) — for bringing Helix mode to a modern editor
- [Kakoune](https://kakoune.org) — the original inspiration for selection-first editing
- [ratatui](https://ratatui.rs) — for the TUI framework
- Vim's `vimtutor` — the original "learn by editing" concept that inspired this project
