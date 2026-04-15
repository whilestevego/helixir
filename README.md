# Helixir 🧪

**72 Helix keybinding exercises. You do them in your real editor.**

You open the TUI in one pane and your editor (Helix, Zed, or anything with a Helix mode) in another. The TUI tells you what to do. When you save a file, Helixir checks your work and moves you to the next exercise.

![Helixir in action — collapsible module tree on the left, exercise detail with commands, instructions, and hints on the right](docs/screenshot.png)

```
helixir init
cd helixir-exercises
helixir                # Brew the TUI
# Open exercise files in your editor in a split pane
```

---

## Why

A cheatsheet teaches you what `wd` does. It doesn't teach your fingers to reach for `wd` without thinking. That part comes from doing it a few hundred times in context.

Helixir is the few hundred times. The first exercises are `hjkl`. The last ones are multi-selection workflows that, once they click, change how you edit.

## How It Works

1. **Install** the tool and run `helixir init`
2. **Launch** the TUI with `helixir` from the project directory
3. **Open** exercise `.hxt` files in your editor in a split pane alongside the TUI
4. **Edit** the PRACTICE section to match the EXPECTED section using the commands shown in the TUI
5. **Save** — the TUI detects your changes, verifies them, and auto-advances on success
6. **Use the TUI** to navigate exercises, reveal hints, and reset exercises

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

Everything else (title, instructions, hints, commands) lives in the TUI, so the file stays out of your way while you edit.

## Curriculum

72 exercises in 16 modules, grouped into 5 tiers.

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
| **06 Multi-Selection** | 6 | `s` (regex select), `S` (split), `C` (cursors), `,` (collapse), `K`/`Alt-K` (keep/remove) |
| **07 Search** | 5 | `/` search, `*` (use selection as pattern), regex patterns, global find-and-replace |

### Tier 3 — Journeyman

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **08 Goto Mode** | 5 | `g g`/`g e` (top/end), `g d`/`g r` (LSP), `g n`/`g p` (buffers), Zed git ops |
| **09 Space Mode** | 3 | `Space f` (files), `Space s` (symbols), `Space r` (rename), clipboard |
| **10 Unimpaired** | 3 | `]d`/`[d` (diagnostics), `]f`/`[f` (functions), indent navigation |

### Tier 4 — Alchemist

| Module | Exercises | What You'll Learn |
|--------|-----------|-------------------|
| **11 Registers** | 5 | Default register, named registers (`"a`-`"z`), system clipboard (`"+`), append (`"A`) |
| **12 Macros** | 4 | `Q` record, `q` stop, `@` replay, macros with search, practical workflows |
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
| `Tab` | Collapse/expand current module |
| `zc` / `zo` / `za` | Collapse / open / toggle current module |
| `zM` / `zR` | Collapse all / expand all modules |
| `Space` | 💡 Reveal next hint |
| `r` | 🔄 Reset current exercise |
| `n` | ⏭️ Next incomplete (or next search match when query active) |
| `N` | Previous search match |
| `c` | Open grimoire (cheatsheet of commands you've learned) |
| `/` | 🔍 Search exercises (incremental) |
| `F` | Cycle status filter (none → ⬜ → 🟡 → ✅) |
| `C` | Cycle completion filter (none → never → once → many) |
| `Esc` | Clear active search/filter |
| `u` | 📦 Install new exercises (when banner is showing) |
| `?` | Toggle help overlay |
| `q` | Quit |

### Search and filter

Large curricula get noisy. Three ways to cut the list down:

- **Search**: press `/`, type any part of a title or category. Matches highlight live in the list; cursor snaps to the first match. `n` and `N` cycle through matches with wrap.
- **Status filter**: press `F` to cycle through "not started", "failed", "passed". Combines with search.
- **Completion filter**: press `C` to filter by how many times you've completed an exercise (never / once / many) using the persisted progress file.

Press `Esc` at any time to clear every active filter.

### Persistent progress

Completion history is saved to `<exercises_dir>/.progress.json` and survives restarts. Each exercise records its first- and last-completed timestamps plus a count that increments every time you reset and redo. The detail pane shows `🏁 Completed 3× · first 2026-04-13` when history exists.

## The Helix Mental Model

Helix reverses Vim's editing grammar:

| | Vim | Helix |
|---|---|---|
| **Model** | Verb → Object (`dw`) | Selection → Action (`wd`) |
| **Delete word** | `dw` | `wd` |
| **Change inside quotes** | `ci"` | `mi"c` |
| **Delete line** | `dd` | `xd` |
| **Yank paragraph** | `yap` | `mapy` |

Every motion creates a visible selection. You see exactly what the next command will affect, refine it if you need to, then commit.

Multi-selection takes this further. Instead of `:%s/old/new/g`, you:

```
%           Select entire file
s old       Split into selections on "old"
c new       Change all selections simultaneously
```

Module 06 drills this until it's automatic.

## Tips

The exercises build on each other. Module 04 assumes you know Module 03; Module 06 assumes you know Module 04. If something feels impossible, you probably skipped ahead.

Don't grind 65 in a day. Do a module, go use those commands in real work for a day, come back.

Press `Space` to reveal hints — but try without first. If a selection goes wrong, `;` collapses it back to a cursor; build that habit early.

When you forget a chord, pause after the prefix key (`g`, `m`, `Space`, `z`, `]`, `[`). Helix shows every option available. That popup is the cheatsheet you actually want.

Module 16 (challenges) is built for repetition. Press `r` to reset and run them again.

## Compatibility

The exercises use standard Helix keybindings and work in any editor that supports them:

- **[Helix](https://helix-editor.com)** — native support
- **[Zed](https://zed.dev)** — built-in Helix mode (`"helix_mode": true` in settings.json)
- **Neovim** — with a Helix emulation plugin

The CLI is a single static binary with no runtime dependencies. The exercises themselves are plain text files embedded in the binary.

## How It's Built

A single-binary Rust TUI built on ratatui.

- **`src/tui/`** — TUI app: event loop, layout, widgets, file watching
- **`src/hxt.rs`** — Pure parser for `.hxt` files: extracts PRACTICE/EXPECTED sections, diffs them
- **`src/metadata.rs`** — Exercise metadata (titles, instructions, hints) deserialized from embedded TOML
- **`src/commands/init.rs`** — Extracts embedded exercises to a new directory
- **`src/progress.rs`** — Persistent completion tracking (JSON store co-located with `.hxt` files)
- **`exercises.toml`** — All 72 exercises' metadata (embedded in binary at compile time)

Exercise templates and metadata are compiled into the binary via `include_dir!` and `include_str!`. The TUI watches `.hxt` files for changes using the `notify` crate.

## License

MIT

## Acknowledgments

- [Helix](https://helix-editor.com) for the selection-first editing model.
- [Kakoune](https://kakoune.org), which inspired Helix.
- [Zed](https://zed.dev) — first GUI editor I know of with a serious Helix mode.
- [ratatui](https://ratatui.rs) — the TUI framework.
- Vim's `vimtutor` — the original "learn by editing" idea.
