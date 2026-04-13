# Helixir 🧪

**A practice elixir for Helix keybindings — 65 hands-on exercises distilled into your real editor.**

Helixir is a TUI that walks you through 65 structured exercises, watches your exercise files for changes, shows instructions and hints, and auto-advances as you complete each one. You edit in your real editor (Helix, Zed, or any editor with Helix keybindings) — no simulations, no quizzes. Just deliberate practice, one dose at a time.

```
helixir init
cd helixir-exercises
helixir                # Brew the TUI
# Open exercise files in your editor in a split pane
```

---

## Why

Reading a keybinding reference is like reading a phrasebook — you recognize words but can't speak the language. Muscle memory comes from repetition in context.

Helixir gives you that context: structured exercises that progressively distill your fluency, from basic motion (`h`/`j`/`k`/`l`) to multi-selection workflows that feel like a superpower.

## How It Works

1. **Install** the tool and run `helixir init`
2. **Launch** the TUI with `helixir` from the project directory
3. **Open** exercise `.hxt` files in your editor in a split pane alongside the TUI
4. **Edit** the PRACTICE section to match the EXPECTED section using the commands shown in the TUI
5. **Save** — the TUI detects your changes, verifies them, and auto-advances on success
6. **Use the TUI** to navigate exercises, reveal hints, and reset exercises

The TUI shows instructions and commands. Your editor is where you brew.

## Install

```sh
# From crates.io
cargo install helixir

# Or build from source
git clone https://github.com/whilestevego/helixir
cd helixir
cargo install --path .
```

Pre-built binaries for macOS (Intel + Apple Silicon), Linux, and Windows are available on the [Releases](../../releases) page.

## Quick Start

```sh
# Distill the exercise project
helixir init
cd helixir-exercises

# Brew the TUI
helixir

# In another terminal/pane, open exercise files in your editor
hx exercises/01-movement/01-basic-motion.hxt
```

The TUI shows the exercise instructions and commands. Edit the PRACTICE section in your editor to match EXPECTED. Save, and the TUI auto-verifies and advances.

## Exercise Format

Exercise `.hxt` files are minimal — just the content you need to edit:

```
────────────────────────── PRACTICE ──────────────────────────────

The color of the sky changes throughout the day. At dawn, a warm
color spreads across the horizon.

────────────────────────── EXPECTED ──────────────────────────────

The colour of the sky changes throughout the day. At dawn, a warm
colour spreads across the horizon.
```

- **PRACTICE** — the text you edit with real Helix commands
- **EXPECTED** — what PRACTICE should look like when you're done

Everything else — title, commands to learn, instructions, and hints — lives in the TUI, not in the file. This keeps the editing surface clean and focused.

## Curriculum

65 exercises across 16 modules, organized in 5 progressive tiers.

### Tier 1 — Apprentice

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **01 Movement** | 5 | `h`/`j`/`k`/`l`, `w`/`b`/`e`, `f`/`t` (multiline!), line navigation, scrolling |
| **02 Selection** | 4 | `x` (line select), `v` (extend mode), `;` (collapse), `%` (select all) |
| **03 Changes** | 5 | `d`/`c` (delete/change), `y`/`p` (yank/paste), `u`/`U` (undo/redo), indent, case |

### Tier 2 — Adept

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **04 Text Objects** | 4 | `m i`/`m a` + delimiters, words, functions, arguments (tree-sitter) |
| **05 Surround** | 4 | `m s` (add), `m r` (replace), `m d` (delete) surround characters |
| **06 Multi-Selection** | 5 | `s` (regex select), `S` (split), `C` (cursors), `K`/`Alt-K` (keep/remove) |
| **07 Search** | 3 | `/` search, `*` (use selection as pattern), global find-and-replace workflow |

### Tier 3 — Journeyman

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **08 Goto Mode** | 4 | `g d`/`g r` (LSP), `g n`/`g p` (buffers), `g o`/`g u` (Zed git ops — not in native Helix) |
| **09 Space Mode** | 3 | `Space f` (files), `Space s` (symbols), `Space r` (rename), clipboard |
| **10 Unimpaired** | 3 | `]d`/`[d` (diagnostics), `]f`/`[f` (functions), indent navigation |

### Tier 4 — Alchemist

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **11 Registers** | 3 | Unnamed register, named registers (`"a`-`"z`), system clipboard (`"+`) |
| **12 Macros** | 3 | `Q` record, `q` stop, `@` replay, practical macro workflows |
| **13 Window Management** | 3 | `Ctrl-w` splits, navigation, close/swap panes |
| **14 View and Numbers** | 3 | `z` view mode, `mm` match bracket, `Ctrl-a`/`Ctrl-x` increment/decrement |

### Tier 5 — Grandmaster

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **15 Combined Techniques** | 5 | Rename variable, extract function, reformat data, bulk transform, Vim-to-Helix |
| **16 Challenges** | 8 | Speed edit, code golf, register relay, macro marathon, number cruncher, and more |

## Usage

### `helixir init [dir]`

Distill a new exercise project — or top up an existing one.

```sh
helixir init                    # Creates ./helixir-exercises/
helixir init ~/my-training      # Custom location
```

If the target directory already has exercises, `init` is **additive**: it only writes files that don't exist yet, leaving your edited exercises untouched. This is how you pick up new exercises after upgrading the tool.

### `helixir`

Brew the TUI from inside a project directory. The TUI watches exercise files for changes and auto-verifies on save. When new exercises are available (e.g. after an upgrade), a banner appears in the header — press `u` to install them in-place.

**TUI Keybindings:**

| Key | Action |
|-----|--------|
| `h`/`l` or ←/→ | Focus left/right panel |
| `j`/`k` or ↑/↓ | Scroll focused panel |
| `Space` | 💡 Reveal next hint |
| `r` | 🔄 Reset current exercise |
| `n` | ⏭️ Jump to next incomplete |
| `u` | 📦 Install new exercises (when banner is showing) |
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

**One module per session.** Don't grind through all 65 in a day. Do a module, then use those commands in your real work. Come back tomorrow.

**Use the which-key popup.** Press any prefix key (`g`, `m`, `Space`, `z`, `]`, `[`) and pause — a popup shows all available sub-commands. This is your cheat sheet.

**`;` is your reset button.** If a selection goes wrong, press `;` to collapse it back to a cursor and try again. Build this habit early.

**Use the TUI hints.** Press `Space` to reveal hints one at a time. Try each exercise without hints first. Struggle is where learning happens.

**Repeat the hard ones.** Press `r` in the TUI to reset any exercise. The challenges in Module 16 are designed for repeated practice.

## Compatibility

The exercises use standard Helix keybindings and work in any editor that supports them:

- **[Helix](https://helix-editor.com)** — native support
- **[Zed](https://zed.dev)** — built-in Helix mode (`"helix_mode": true` in settings.json)
- **Neovim** — with a Helix emulation plugin

The CLI is a single static binary with no runtime dependencies. The exercises themselves are plain text files embedded in the binary.

## Contributing

Contributions are welcome! Here's how you can help:

### Adding exercises

An exercise is two things: a minimal `.hxt` file and a metadata entry in `exercises.toml`.

1. **Create the `.hxt` file** in the appropriate module directory under `exercises/`
   - Only the PRACTICE and EXPECTED sections, bounded by `──── PRACTICE ────` and `──── EXPECTED ────` markers
   - See any existing exercise (e.g. `exercises/01-movement/01-basic-motion.hxt`) for the format
2. **Add the metadata** to `exercises.toml`:
   - `id` matching the file path without `.hxt`
   - `title`, `category`, `difficulty` (1-3)
   - `notes`, `instructions`, `hints` (what the TUI shows)
   - `[[exercises.commands]]` entries for the keybindings taught
3. **Build and launch** with `cargo run` to verify your exercise loads and parses

### Exercise quality checklist

- [ ] Instructions are clear enough to follow without prior Helix experience for that tier
- [ ] PRACTICE text is realistic (code, prose, or data — not lorem ipsum)
- [ ] EXPECTED result is achievable with the listed commands
- [ ] Hints are ordered from gentle nudge to explicit keystroke sequence
- [ ] The exercise teaches something that builds on previous modules

### Reporting issues

If an exercise has incorrect expected output, unclear instructions, or a keybinding that doesn't work in your editor's Helix mode, please [open an issue](../../issues).

## How It's Built

A single-binary Rust TUI distilled with ratatui:

- **`src/tui/`** — TUI app: event loop, layout, widgets, file watching
- **`src/hxt.rs`** — Pure parser for `.hxt` files: extracts PRACTICE/EXPECTED sections, diffs them
- **`src/metadata.rs`** — Exercise metadata (titles, instructions, hints) deserialized from embedded TOML
- **`src/commands/init.rs`** — Extracts embedded exercises to a new directory
- **`exercises.toml`** — All 65 exercises' metadata (embedded in binary at compile time)

Exercise templates and metadata are compiled into the binary via `include_dir!` and `include_str!`. The TUI watches `.hxt` files for changes using the `notify` crate.

## License

MIT

## Acknowledgments

- [Helix Editor](https://helix-editor.com) — for the selection-first editing model
- [Zed](https://zed.dev) — for bringing Helix mode to a modern editor
- [Kakoune](https://kakoune.org) — the original inspiration for selection-first editing
- [ratatui](https://ratatui.rs) — for the TUI framework
- Vim's `vimtutor` — the original "learn by editing" concept that inspired this project
