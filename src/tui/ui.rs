use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

use crate::hxt::DiffLine;
use crate::tui::app::{App, ExerciseStatus, InputMode, Panel, TreeCursor};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Header (with breathing room)
            Constraint::Min(10),   // Main content
            Constraint::Length(2), // Footer (with breathing room)
        ])
        .split(frame.area());

    render_header(frame, chunks[0], app);
    render_main(frame, chunks[1], app);
    render_footer(frame, chunks[2], app);

    if app.show_cheatsheet {
        render_cheatsheet_popup(frame, app);
    }

    if app.show_help {
        render_help_popup(frame, app);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let (mod_idx, mod_total) = app.current_module_index();

    let exercise_span = match app.current_exercise_in_module() {
        Some((ex_idx, ex_total)) => Span::styled(
            format!("📝 Exercise {}/{}   ", ex_idx, ex_total),
            Style::default().fg(Color::Gray),
        ),
        None => {
            let (passed, total) = app.module_progress(app.cursor_module());
            Span::styled(
                format!("🗂  Module Overview ({}/{})   ", passed, total),
                Style::default().fg(Color::Gray),
            )
        }
    };

    let mut header_spans = vec![
        Span::styled(
            " 🧪 Helixir",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled(
            format!("📦 Module {}/{}   ", mod_idx, mod_total),
            Style::default().fg(Color::Gray),
        ),
        exercise_span,
        Span::styled("[?] help", Style::default().fg(Color::DarkGray)),
    ];

    if app.missing_exercises > 0 {
        header_spans.push(Span::raw("    "));
        header_spans.push(Span::styled(
            format!(
                "📦 {} new exercises available — press [u] to install",
                app.missing_exercises
            ),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let header = Line::from(header_spans);
    frame.render_widget(Paragraph::new(vec![header, Line::raw("")]), area);
}

fn render_main(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    render_exercise_list(frame, chunks[0], app);
    render_exercise_detail(frame, chunks[1], app);
}

/// Build the flat list of display items for the tree view. Tracks the row
/// index of the currently focused tree node so the list can be scrolled to
/// keep the cursor visible.
struct ListLayout {
    items: Vec<ListItem<'static>>,
    /// Row index of the cursor inside `items`, if it's currently rendered.
    cursor_row: Option<usize>,
}

fn build_exercise_list(app: &App, width: u16) -> ListLayout {
    let mut items: Vec<ListItem<'static>> = Vec::new();
    let mut cursor_row: Option<usize> = None;
    let mut current_category = String::new();
    let content_width = width.saturating_sub(4) as usize; // padding + border

    let selected_style = Style::default()
        .fg(Color::Cyan)
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD);

    for (i, exercise) in app.exercises.iter().enumerate() {
        // Module header with blank line above (except first)
        if exercise.meta.category != current_category {
            if !current_category.is_empty() {
                items.push(ListItem::new(Line::raw("")));
            }
            current_category = exercise.meta.category.clone();
            let collapsed = app.is_module_collapsed(&current_category);
            let chevron = if collapsed { "▶" } else { "▼" };
            let (passed, total) = app.module_progress(&current_category);
            let badge = if passed == total {
                format!("({}/{} ✅)", passed, total)
            } else {
                format!("({}/{})", passed, total)
            };
            let header_text = format!(" {} 🗂  {} {}", chevron, current_category, badge);
            let cursor_on_header = matches!(
                &app.cursor,
                TreeCursor::Module(m) if m == &current_category
            );
            let header_line = if cursor_on_header {
                let padded = if header_text.chars().count() < content_width {
                    format!("{:width$}", header_text, width = content_width)
                } else {
                    header_text.chars().take(content_width).collect()
                };
                cursor_row = Some(items.len());
                Line::from(Span::styled(padded, selected_style))
            } else {
                Line::from(vec![
                    Span::styled(
                        format!(" {} 🗂  {} ", chevron, current_category),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(badge, Style::default().fg(Color::DarkGray)),
                ])
            };
            items.push(ListItem::new(header_line));
            if !collapsed {
                items.push(ListItem::new(Line::raw(""))); // space after header
            }
        }

        // Skip exercise rows for collapsed modules
        if app.is_module_collapsed(&exercise.meta.category) {
            continue;
        }

        let is_selected = matches!(&app.cursor, TreeCursor::Exercise(idx) if *idx == i);

        let (icon, base_style) = match exercise.status {
            ExerciseStatus::Passed => ("✅", Style::default().fg(Color::Green)),
            ExerciseStatus::Failed => ("  ", Style::default().fg(Color::White)),
            ExerciseStatus::NotStarted => ("  ", Style::default().fg(Color::DarkGray)),
        };
        let style = if is_selected {
            selected_style
        } else {
            base_style
        };

        let title = exercise.meta.title.clone();
        let prefix = format!("    {} ", icon);
        // Total display width per row (for selected-row background fill).
        let raw_len = prefix.chars().count() + title.chars().count();
        let pad_len = if is_selected && raw_len < content_width {
            content_width - raw_len
        } else {
            0
        };

        if is_selected {
            cursor_row = Some(items.len());
        }

        // Build spans: prefix + (highlighted-or-plain title) + padding.
        let mut spans: Vec<Span<'static>> = Vec::new();
        spans.push(Span::styled(prefix, style));
        spans.extend(title_spans(&title, &app.filter.query, style));
        if pad_len > 0 {
            spans.push(Span::styled(" ".repeat(pad_len), style));
        }
        items.push(ListItem::new(Line::from(spans)));
    }

    ListLayout { items, cursor_row }
}

/// Build spans for an exercise title with the active search query highlighted.
/// Falls back to a single styled span when the query is empty or doesn't match.
fn title_spans(title: &str, query: &str, base: Style) -> Vec<Span<'static>> {
    if query.is_empty() {
        return vec![Span::styled(title.to_string(), base)];
    }
    let lower_title = title.to_lowercase();
    let lower_query = query.to_lowercase();
    let Some(byte_start) = lower_title.find(&lower_query) else {
        return vec![Span::styled(title.to_string(), base)];
    };
    let byte_end = byte_start + lower_query.len();
    // Slice using byte offsets from the lowercase string — safe because
    // `to_lowercase` preserves char boundaries for ASCII titles. For non-ASCII
    // titles the search still functions; highlight may degrade gracefully.
    let before = &title[..byte_start];
    let middle = &title[byte_start..byte_end];
    let after = &title[byte_end..];
    let highlight = base
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        .fg(Color::Yellow);
    let mut out: Vec<Span<'static>> = Vec::new();
    if !before.is_empty() {
        out.push(Span::styled(before.to_string(), base));
    }
    out.push(Span::styled(middle.to_string(), highlight));
    if !after.is_empty() {
        out.push(Span::styled(after.to_string(), base));
    }
    out
}

fn render_exercise_list(frame: &mut Frame, area: Rect, app: &App) {
    let layout = build_exercise_list(app, area.width);

    let visible_height = area.height.saturating_sub(2) as usize; // block borders
    let selected_row = layout.cursor_row.unwrap_or(0);

    // Calculate scroll offset to keep selected item visible with context
    let scroll_offset = if selected_row >= app.scroll_offset + visible_height {
        selected_row.saturating_sub(visible_height) + 1
    } else if selected_row < app.scroll_offset {
        selected_row.saturating_sub(2)
    } else {
        app.scroll_offset
    };

    let total_items = layout.items.len();

    // Use ListState for offset-based scrolling
    let mut list_state = ratatui::widgets::ListState::default().with_offset(scroll_offset);

    let border_color = if app.focused_panel == Panel::List {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let list = List::new(layout.items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(
                " 📚 Exercises ",
                Style::default().fg(Color::Cyan),
            )),
    );

    frame.render_stateful_widget(list, area, &mut list_state);

    // Scrollbar
    if total_items > visible_height {
        let mut scrollbar_state = ScrollbarState::new(total_items).position(selected_row);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some("│"))
                .thumb_symbol("█"),
            area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

fn render_exercise_detail(frame: &mut Frame, area: Rect, app: &mut App) {
    let (lines, title) = match &app.cursor {
        TreeCursor::Module(name) => (
            build_module_summary_lines(app, &name.clone()),
            " Module Summary ",
        ),
        TreeCursor::Exercise(_) => (build_exercise_detail_lines(app), " Exercise Detail "),
    };

    render_detail_pane(frame, area, app, lines, title);
}

fn build_module_summary_lines(app: &App, module: &str) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let (passed, total) = app.module_progress(module);
    let bar_width = 24usize;
    let filled = if total > 0 {
        (passed as f64 / total as f64 * bar_width as f64).round() as usize
    } else {
        0
    };
    let bar = format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(bar_width.saturating_sub(filled))
    );

    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("  🗂  {}", module),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(bar, Style::default().fg(Color::Green)),
        Span::styled(
            format!("  {}/{}", passed, total),
            Style::default().fg(Color::White),
        ),
        Span::styled(
            if passed == total { "  ✅" } else { "" },
            Style::default().fg(Color::Green),
        ),
    ]));
    lines.push(Line::raw(""));

    // Exercises in this module
    lines.push(Line::from(Span::styled(
        "  📝 EXERCISES",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));
    for idx in app.exercises_in_module(module) {
        let ex = &app.exercises[idx];
        let icon = match ex.status {
            ExerciseStatus::Passed => "✅",
            ExerciseStatus::Failed => "🟡",
            ExerciseStatus::NotStarted => "⬜",
        };
        let stars = match ex.meta.difficulty {
            1 => "⭐",
            2 => "⭐⭐",
            _ => "⭐⭐⭐",
        };
        lines.push(Line::from(vec![
            Span::styled(format!("     {} ", icon), Style::default().fg(Color::White)),
            Span::styled(ex.meta.title.clone(), Style::default().fg(Color::White)),
            Span::styled(
                format!("   {}", stars),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }
    lines.push(Line::raw(""));

    // Commands learned so far in this module (deduped, from passed exercises)
    let mut learned: Vec<(String, String)> = Vec::new();
    for idx in app.exercises_in_module(module) {
        let ex = &app.exercises[idx];
        if ex.status != ExerciseStatus::Passed {
            continue;
        }
        for cmd in &ex.meta.commands {
            if !learned.iter().any(|(k, _)| k == &cmd.key) {
                learned.push((cmd.key.clone(), cmd.description.clone()));
            }
        }
    }

    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("  ⌨️  COMMANDS LEARNED ({})", learned.len()),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));
    if learned.is_empty() {
        lines.push(Line::from(Span::styled(
            "     Pass an exercise in this module to start learning commands.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (key, desc) in learned {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("     {:10}", key),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(desc, Style::default().fg(Color::White)),
            ]));
        }
    }
    lines.push(Line::raw(""));

    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        "  Press j/Tab on an exercise to focus it · Tab on this header to collapse",
        Style::default().fg(Color::DarkGray),
    )));

    lines
}

fn build_exercise_detail_lines(app: &App) -> Vec<Line<'static>> {
    let exercise = app.current_exercise().expect("cursor is on exercise");
    let meta = exercise.meta;

    let difficulty_stars = match meta.difficulty {
        1 => "⭐",
        2 => "⭐⭐",
        _ => "⭐⭐⭐",
    };

    let mut lines: Vec<Line<'static>> = Vec::new();

    // Title with extra top padding
    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("  {} {}", difficulty_stars, meta.title),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!(
            "  Category: {}   Difficulty: {}/3",
            meta.category, meta.difficulty
        ),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::raw(""));

    // Commands
    if !meta.commands.is_empty() {
        lines.push(Line::from(Span::styled(
            "  ⌨️  COMMANDS",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::raw(""));
        for cmd in &meta.commands {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("     {:8}", cmd.key),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(cmd.description.clone(), Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::raw(""));
    }

    // Notes (better contrast)
    if !meta.notes.is_empty() {
        lines.push(Line::raw(""));
        for line in meta.notes.lines() {
            lines.push(Line::from(Span::styled(
                format!("     {}", line),
                Style::default().fg(Color::Gray), // Gray instead of DarkGray
            )));
        }
        lines.push(Line::raw(""));
    }

    // Instructions
    lines.push(Line::from(Span::styled(
        "  📋 INSTRUCTIONS",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::raw(""));
    for line in meta.instructions.trim().lines() {
        lines.push(Line::from(Span::styled(
            format!("     {}", line),
            Style::default().fg(Color::White),
        )));
    }
    lines.push(Line::raw(""));
    lines.push(Line::raw(""));

    // Status separator
    lines.push(Line::from(Span::styled(
        "  ───────────────────────────────────────────",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::raw(""));

    // Flash message or status
    if let Some((msg, _)) = &app.flash_message {
        lines.push(Line::from(Span::styled(
            format!("  {}", msg),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
    } else {
        match &exercise.status {
            ExerciseStatus::Passed => {
                lines.push(Line::from(Span::styled(
                    "  ✅ PASSED!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )));
                if let Some(idx) = app.current_exercise_index()
                    && let Some(p) = app.progress_for(idx)
                {
                    let first = p.first_completed_at.format("%Y-%m-%d");
                    lines.push(Line::from(Span::styled(
                        format!("  🏁 Completed {}× · first {}", p.completion_count, first),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            }
            ExerciseStatus::NotStarted => {
                lines.push(Line::from(Span::styled(
                    "  ⬜ Not started — edit the .hxt file to begin",
                    Style::default().fg(Color::DarkGray),
                )));
            }
            ExerciseStatus::Failed => {
                let diff_count = exercise.diff.len();
                lines.push(Line::from(Span::styled(
                    format!(
                        "  ❌ {} difference{} found",
                        diff_count,
                        if diff_count == 1 { "" } else { "s" }
                    ),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::raw(""));
                lines.extend(build_diff_lines(&exercise.diff));
            }
        }
    }

    // Hints
    if app.hint_level > 0 {
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "  💡 HINTS",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::raw(""));
        for (i, hint) in meta.hints.iter().enumerate() {
            if i < app.hint_level {
                lines.push(Line::from(Span::styled(
                    format!("     {}. {}", i + 1, hint),
                    Style::default().fg(Color::Yellow),
                )));
                lines.push(Line::raw(""));
            }
        }
        let remaining = meta.hints.len().saturating_sub(app.hint_level);
        if remaining > 0 {
            lines.push(Line::from(Span::styled(
                format!("     ({} more — press h)", remaining),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines
}

fn render_detail_pane(
    frame: &mut Frame,
    area: Rect,
    app: &mut App,
    lines: Vec<Line<'static>>,
    title: &str,
) {
    // Pre-wrap long lines to fit the panel width
    let content_width = area.width.saturating_sub(4) as usize; // borders + padding
    let mut wrapped_lines: Vec<Line> = Vec::new();
    for line in lines {
        let line_str: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        if line_str.len() <= content_width || line_str.trim().is_empty() {
            wrapped_lines.push(line);
        } else {
            // Grab the style from the first span for wrapped continuation
            let style = line.spans.first().map(|s| s.style).unwrap_or_default();
            let chars: Vec<char> = line_str.chars().collect();
            for chunk in chars.chunks(content_width) {
                let s: String = chunk.iter().collect();
                wrapped_lines.push(Line::from(Span::styled(s, style)));
            }
        }
    }

    // Add bottom padding so "end" is visually obvious
    for _ in 0..5 {
        wrapped_lines.push(Line::raw(""));
    }
    wrapped_lines.push(Line::from(Span::styled(
        "  ── end ──",
        Style::default().fg(Color::DarkGray),
    )));
    for _ in 0..2 {
        wrapped_lines.push(Line::raw(""));
    }

    let total_lines = wrapped_lines.len();
    let visible_height = area.height.saturating_sub(2) as usize; // borders
    let can_scroll = total_lines > visible_height;

    // Clamp scroll offset and update app's max for input clamping
    let max_scroll = total_lines.saturating_sub(visible_height);
    app.detail_scroll_max = max_scroll;
    app.detail_scroll = app.detail_scroll.min(max_scroll);
    let scroll_offset = app.detail_scroll;

    // Manually slice visible window
    let visible_lines: Vec<Line> = wrapped_lines
        .into_iter()
        .skip(scroll_offset)
        .take(visible_height)
        .collect();

    let border_color = if app.focused_panel == Panel::Detail {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let scroll_indicator = if can_scroll {
        if scroll_offset < max_scroll {
            " ↕ j/k to scroll "
        } else {
            " (end) "
        }
    } else {
        ""
    };

    let detail = Paragraph::new(visible_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(
                title.to_string(),
                Style::default().fg(Color::Cyan),
            ))
            .title_bottom(Span::styled(
                scroll_indicator,
                Style::default().fg(Color::DarkGray),
            )),
    );

    frame.render_widget(detail, area);

    // Detail scrollbar
    if can_scroll {
        let mut scrollbar_state = ScrollbarState::new(total_lines).position(scroll_offset);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some("│"))
                .thumb_symbol("█"),
            area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    // While the user is typing a search, replace the footer entirely with
    // the search prompt line so the input location is unambiguous.
    if app.input_mode == InputMode::Searching {
        let search_line = Line::from(vec![
            Span::styled(
                "  🔍 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("/", Style::default().fg(Color::Yellow)),
            Span::styled(
                app.filter.query.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("_", Style::default().fg(Color::Yellow)),
            Span::styled(
                "     Enter: commit  ·  Esc: cancel",
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        frame.render_widget(Paragraph::new(vec![Line::raw(""), search_line]), area);
        return;
    }

    let completed = app.completed_count();
    let total = app.total_count();
    let pct = if total > 0 {
        (completed as f64 / total as f64 * 100.0).round() as usize
    } else {
        0
    };

    let bar_width = 20usize;
    let filled = if total > 0 {
        (completed as f64 / total as f64 * bar_width as f64).round() as usize
    } else {
        0
    };
    let bar = format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(bar_width.saturating_sub(filled))
    );

    let mut spans: Vec<Span<'static>> = vec![
        Span::styled("  🏆 ", Style::default()),
        Span::styled(bar, Style::default().fg(Color::Green)),
        Span::styled(
            format!(" {}/{} ({}%)", completed, total, pct),
            Style::default().fg(Color::White),
        ),
        Span::styled("      ", Style::default()),
    ];

    // Active filter chips (only when something is filtering).
    if app.filter.is_active() {
        if !app.filter.query.is_empty() {
            spans.push(Span::styled(
                format!("🔍\"{}\" ", app.filter.query),
                Style::default().fg(Color::Yellow),
            ));
        }
        if let Some(status) = &app.filter.status {
            let (icon, label) = match status {
                ExerciseStatus::Passed => ("✅", "Passed"),
                ExerciseStatus::Failed => ("🟡", "Failed"),
                ExerciseStatus::NotStarted => ("⬜", "NotStarted"),
            };
            spans.push(Span::styled(
                format!("{}{} ", icon, label),
                Style::default().fg(Color::Yellow),
            ));
        }
        spans.push(Span::styled(
            "[Esc] clear  ",
            Style::default().fg(Color::DarkGray),
        ));
    }

    let query_active = !app.filter.query.is_empty();
    let (next_key, next_label) = if query_active {
        (" [n/N]", " match  ")
    } else {
        (" [n]", " next  ")
    };

    spans.extend([
        Span::styled("💡", Style::default()),
        Span::styled("[/]", Style::default().fg(Color::Cyan)),
        Span::styled(" search  ", Style::default().fg(Color::DarkGray)),
        Span::styled("[F]", Style::default().fg(Color::Cyan)),
        Span::styled(" filter  ", Style::default().fg(Color::DarkGray)),
        Span::styled("🔄", Style::default()),
        Span::styled("[r]", Style::default().fg(Color::Cyan)),
        Span::styled(" reset  ", Style::default().fg(Color::DarkGray)),
        Span::styled("⏭️", Style::default()),
        Span::styled(next_key, Style::default().fg(Color::Cyan)),
        Span::styled(next_label, Style::default().fg(Color::DarkGray)),
        Span::styled("🚪", Style::default()),
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::styled(" quit", Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(Paragraph::new(vec![Line::raw(""), Line::from(spans)]), area);
}

fn render_help_popup(frame: &mut Frame, _app: &App) {
    let area = centered_rect(55, 90, frame.area());

    let help_text = vec![
        Line::raw(""),
        Line::from(Span::styled(
            "  🧪 Helixir — Recipe Book",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::raw(""),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    h / ←     ", Style::default().fg(Color::Green)),
            Span::raw("Focus exercise list"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    l / →     ", Style::default().fg(Color::Green)),
            Span::raw("Focus exercise detail"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    j / ↓     ", Style::default().fg(Color::Green)),
            Span::raw("Scroll down in focused panel"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    k / ↑     ", Style::default().fg(Color::Green)),
            Span::raw("Scroll up in focused panel"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    Space     ", Style::default().fg(Color::Green)),
            Span::raw("Reveal next hint"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    n         ", Style::default().fg(Color::Green)),
            Span::raw("Jump to next incomplete"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    r         ", Style::default().fg(Color::Green)),
            Span::raw("Reset current exercise"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    Tab       ", Style::default().fg(Color::Green)),
            Span::raw("Collapse / expand current module"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    zc/zo/za  ", Style::default().fg(Color::Green)),
            Span::raw("Collapse / open / toggle module"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    zM / zR   ", Style::default().fg(Color::Green)),
            Span::raw("Collapse all / expand all modules"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    c         ", Style::default().fg(Color::Green)),
            Span::raw("Open grimoire (spells you've learned)"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    /         ", Style::default().fg(Color::Green)),
            Span::raw("Search exercises (incremental)"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    n / N     ", Style::default().fg(Color::Green)),
            Span::raw("Next / previous match (when search active)"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    F         ", Style::default().fg(Color::Green)),
            Span::raw("Cycle status filter (none → ⬜ → 🟡 → ✅)"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    Esc       ", Style::default().fg(Color::Green)),
            Span::raw("Clear active search/filter"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    ?         ", Style::default().fg(Color::Green)),
            Span::raw("Toggle this help"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    q         ", Style::default().fg(Color::Green)),
            Span::raw("Quit"),
        ]),
        Line::raw(""),
        Line::raw(""),
        Line::from(Span::styled(
            "    Edit .hxt files in your editor — changes",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "    are detected automatically on save.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::raw(""),
        Line::from(Span::styled(
            "    Press ? or Esc to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let popup = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Help "),
    );

    frame.render_widget(ratatui::widgets::Clear, area);
    frame.render_widget(popup, area);
}

fn build_cheatsheet_module_section(
    module: &crate::tui::app::CheatsheetModule,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let badge = if module.passed == module.total {
        format!("({}/{} ✅)", module.passed, module.total)
    } else {
        format!("({}/{})", module.passed, module.total)
    };
    lines.push(Line::from(vec![
        Span::styled(
            format!("  🗂  {}  ", module.name),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(badge, Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::raw(""));
    for cmd in &module.commands {
        lines.push(Line::from(vec![
            Span::styled(
                format!("     {:10}", cmd.key),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(cmd.description.clone(), Style::default().fg(Color::White)),
        ]));
    }
    lines.push(Line::raw(""));
    lines
}

/// Greedily distribute module sections into two columns by line count so the
/// columns end up roughly balanced in height.
fn distribute_into_columns(
    sections: Vec<Vec<Line<'static>>>,
) -> (Vec<Line<'static>>, Vec<Line<'static>>) {
    let mut left: Vec<Line<'static>> = Vec::new();
    let mut right: Vec<Line<'static>> = Vec::new();
    for section in sections {
        if left.len() <= right.len() {
            left.extend(section);
        } else {
            right.extend(section);
        }
    }
    (left, right)
}

fn render_cheatsheet_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(85, 85, frame.area());
    let modules = app.build_cheatsheet();
    let any_passed = modules.iter().any(|m| m.passed > 0);

    // Outer frame
    frame.render_widget(ratatui::widgets::Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            " 📜 Grimoire — Spells you've learned ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_bottom(Span::styled(
            " j/k to scroll · c or Esc to close ",
            Style::default().fg(Color::DarkGray),
        ));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if !any_passed {
        let msg = Paragraph::new(vec![
            Line::raw(""),
            Line::raw(""),
            Line::from(Span::styled(
                "  Brew your first exercise to begin your grimoire 🧪",
                Style::default().fg(Color::DarkGray),
            )),
        ]);
        frame.render_widget(msg, inner);
        return;
    }

    // Build per-module sections and distribute across two columns.
    let sections: Vec<Vec<Line<'static>>> = modules
        .iter()
        .filter(|m| !m.commands.is_empty())
        .map(build_cheatsheet_module_section)
        .collect();
    let (left_lines, right_lines) = distribute_into_columns(sections);

    // Shared scroll: clamp by the taller column.
    let visible_height = inner.height as usize;
    let max_lines = left_lines.len().max(right_lines.len());
    let max_scroll = max_lines.saturating_sub(visible_height);
    app.cheatsheet_scroll = app.cheatsheet_scroll.min(max_scroll);
    let scroll = app.cheatsheet_scroll;

    let take_window = |lines: Vec<Line<'static>>| -> Vec<Line<'static>> {
        lines
            .into_iter()
            .skip(scroll)
            .take(visible_height)
            .collect()
    };

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(1), // gutter
            Constraint::Percentage(50),
        ])
        .split(inner);

    frame.render_widget(Paragraph::new(take_window(left_lines)), cols[0]);
    frame.render_widget(Paragraph::new(take_window(right_lines)), cols[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Split `line` into `(common_prefix, differing_middle, common_suffix)` using
/// char-level longest-common-prefix / longest-common-suffix against `other`.
/// The middle is the region that actually differs and deserves highlighting.
///
/// If `line == other`, middle is empty. If one side is a prefix of the other,
/// the longer side's middle contains the extra chars. Unicode-safe (chars,
/// not bytes).
fn split_diff_region(line: &str, other: &str) -> (String, String, String) {
    let line_chars: Vec<char> = line.chars().collect();
    let other_chars: Vec<char> = other.chars().collect();

    let prefix_len = line_chars
        .iter()
        .zip(other_chars.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // Suffix comparison must not cross into either string's prefix region.
    let max_suffix = line_chars
        .len()
        .saturating_sub(prefix_len)
        .min(other_chars.len().saturating_sub(prefix_len));
    let suffix_len = line_chars
        .iter()
        .rev()
        .zip(other_chars.iter().rev())
        .take(max_suffix)
        .take_while(|(a, b)| a == b)
        .count();

    let middle_end = line_chars.len() - suffix_len;
    let prefix: String = line_chars[..prefix_len].iter().collect();
    let middle: String = line_chars[prefix_len..middle_end].iter().collect();
    let suffix: String = line_chars[middle_end..].iter().collect();
    (prefix, middle, suffix)
}

/// Render whitespace visibly: space → `·`, tab → `→`. Applied only inside the
/// differing middle so normal text stays readable.
fn visualize_whitespace(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => '·',
            '\t' => '→',
            other => other,
        })
        .collect()
}

/// Build styled spans for a single side of a diff line. Prefix/suffix render
/// in neutral gray; the differing middle renders bold in `accent` with
/// whitespace visualized.
fn diff_line_spans(line: &str, other: &str, accent: Color) -> Vec<Span<'static>> {
    let (prefix, middle, suffix) = split_diff_region(line, other);
    let mut spans: Vec<Span<'static>> = Vec::new();
    if !prefix.is_empty() {
        spans.push(Span::styled(prefix, Style::default().fg(Color::Gray)));
    }
    if !middle.is_empty() {
        spans.push(Span::styled(
            visualize_whitespace(&middle),
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        ));
    }
    if !suffix.is_empty() {
        spans.push(Span::styled(suffix, Style::default().fg(Color::Gray)));
    }
    spans
}

/// Build a full unified-diff-style block for all `DiffLine`s — no truncation,
/// relies on detail-pane scroll for long lists.
fn build_diff_lines(diff: &[DiffLine]) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    for (i, d) in diff.iter().enumerate() {
        if i > 0 {
            lines.push(Line::raw(""));
        }
        lines.push(Line::from(Span::styled(
            format!("     line {}", d.line_num),
            Style::default().fg(Color::DarkGray),
        )));

        let mut got_spans = vec![Span::styled(
            "     - ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )];
        got_spans.extend(diff_line_spans(&d.got, &d.expected, Color::Red));
        lines.push(Line::from(got_spans));

        let mut exp_spans = vec![Span::styled(
            "     + ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )];
        exp_spans.extend(diff_line_spans(&d.expected, &d.got, Color::Green));
        lines.push(Line::from(exp_spans));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_identical_lines_yields_empty_middle() {
        let (p, m, s) = split_diff_region("hello", "hello");
        assert_eq!(p, "hello");
        assert_eq!(m, "");
        assert_eq!(s, "");
    }

    #[test]
    fn split_shared_prefix_and_suffix() {
        // "hello WORLD foo" vs "hello EARTH foo" → differ only in the middle word.
        let (p, m, s) = split_diff_region("hello WORLD foo", "hello EARTH foo");
        assert_eq!(p, "hello ");
        assert_eq!(m, "WORLD");
        assert_eq!(s, " foo");
    }

    #[test]
    fn split_one_is_prefix_of_other() {
        let (p, m, s) = split_diff_region("hello world", "hello");
        assert_eq!(p, "hello");
        assert_eq!(m, " world");
        assert_eq!(s, "");
    }

    #[test]
    fn split_completely_different() {
        let (p, m, s) = split_diff_region("abc", "xyz");
        assert_eq!(p, "");
        assert_eq!(m, "abc");
        assert_eq!(s, "");
    }

    #[test]
    fn split_empty_line() {
        let (p, m, s) = split_diff_region("", "hello");
        assert_eq!(p, "");
        assert_eq!(m, "");
        assert_eq!(s, "");
    }

    #[test]
    fn split_unicode_safe() {
        // Each emoji is multi-byte; algorithm must count by chars, not bytes.
        let (p, m, s) = split_diff_region("✅ok✅", "✅no✅");
        assert_eq!(p, "✅");
        assert_eq!(m, "ok");
        assert_eq!(s, "✅");
    }

    #[test]
    fn visualize_whitespace_replaces_spaces_and_tabs() {
        assert_eq!(visualize_whitespace("a b\tc"), "a·b→c");
    }

    #[test]
    fn build_diff_lines_produces_three_rows_per_diff_plus_separators() {
        let diff = vec![
            DiffLine {
                line_num: 1,
                got: "foo".into(),
                expected: "bar".into(),
            },
            DiffLine {
                line_num: 3,
                got: "x".into(),
                expected: "y".into(),
            },
        ];
        let out = build_diff_lines(&diff);
        // Per diff: label + got + expected (3 rows). Plus 1 blank separator between the two diffs.
        assert_eq!(out.len(), 3 + 1 + 3);
    }
}
