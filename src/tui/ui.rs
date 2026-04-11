use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::Frame;

use crate::tui::app::{App, ExerciseStatus, Panel};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Header (with breathing room)
            Constraint::Min(10),  // Main content
            Constraint::Length(2), // Footer (with breathing room)
        ])
        .split(frame.area());

    render_header(frame, chunks[0], app);
    render_main(frame, chunks[1], app);
    render_footer(frame, chunks[2], app);

    if app.show_help {
        render_help_popup(frame, app);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let (mod_idx, mod_total) = app.current_module_index();
    let (ex_idx, ex_total) = app.current_exercise_in_module();

    let header = Line::from(vec![
        Span::styled(
            " 🏋️ Helix Trainer",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("    "),
        Span::styled(
            format!("📦 Module {}/{}   ", mod_idx, mod_total),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(
            format!("📝 Exercise {}/{}   ", ex_idx, ex_total),
            Style::default().fg(Color::Gray),
        ),
        Span::styled("[?] help", Style::default().fg(Color::DarkGray)),
    ]);

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

/// Build the flat list of display items, tracking which row index corresponds
/// to each exercise index (needed for scroll calculations).
struct ListLayout {
    items: Vec<ListItem<'static>>,
    /// Maps exercise index → row index in items vec
    exercise_row: Vec<usize>,
}

fn build_exercise_list(app: &App, width: u16) -> ListLayout {
    let mut items: Vec<ListItem<'static>> = Vec::new();
    let mut exercise_row: Vec<usize> = Vec::new();
    let mut current_category = String::new();
    let content_width = width.saturating_sub(4) as usize; // padding + border

    for (i, exercise) in app.exercises.iter().enumerate() {
        // Category header with blank line above (except first)
        if exercise.meta.category != current_category {
            if !current_category.is_empty() {
                items.push(ListItem::new(Line::raw("")));
            }
            current_category = exercise.meta.category.clone();
            items.push(ListItem::new(Line::from(Span::styled(
                format!(" 🗂 {}", current_category),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ))));
            items.push(ListItem::new(Line::raw(""))); // space after header
        }

        let is_selected = i == app.selected;

        let (icon, style) = if is_selected {
            (
                "▶",
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            match exercise.status {
                ExerciseStatus::Passed => ("✅", Style::default().fg(Color::Green)),
                ExerciseStatus::Failed => ("  ", Style::default().fg(Color::White)),
                ExerciseStatus::NotStarted => ("  ", Style::default().fg(Color::DarkGray)),
            }
        };

        let title = &exercise.meta.title;
        let label = format!("  {} {}", icon, title);
        // Truncate or pad to fill the row (for background highlight)
        let display: String = if is_selected {
            if label.chars().count() < content_width {
                format!("{:width$}", label, width = content_width)
            } else {
                label.chars().take(content_width).collect()
            }
        } else {
            label
        };

        exercise_row.push(items.len());
        items.push(ListItem::new(Line::from(Span::styled(display, style))));
    }

    ListLayout {
        items,
        exercise_row,
    }
}

fn render_exercise_list(frame: &mut Frame, area: Rect, app: &App) {
    let layout = build_exercise_list(app, area.width);

    let visible_height = area.height.saturating_sub(2) as usize; // block borders
    let selected_row = layout
        .exercise_row
        .get(app.selected)
        .copied()
        .unwrap_or(0);

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
    let mut list_state = ratatui::widgets::ListState::default()
        .with_offset(scroll_offset);

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
        let mut scrollbar_state =
            ScrollbarState::new(total_items).position(selected_row);
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
    let exercise = app.selected_exercise();
    let meta = exercise.meta;

    let difficulty_stars = match meta.difficulty {
        1 => "⭐",
        2 => "⭐⭐",
        _ => "⭐⭐⭐",
    };

    let mut lines: Vec<Line> = Vec::new();

    // Title with extra top padding
    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled(
        format!("  {} {}", difficulty_stars, meta.title),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!("  Category: {}   Difficulty: {}/3", meta.category, meta.difficulty),
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
                Span::styled(&cmd.description, Style::default().fg(Color::White)),
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
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::raw(""));
                for d in exercise.diff.iter().take(5) {
                    lines.push(Line::from(Span::styled(
                        format!("     line {}: got \"{}\"", d.line_num, d.got),
                        Style::default().fg(Color::Red),
                    )));
                    lines.push(Line::from(Span::styled(
                        format!("            expected \"{}\"", d.expected),
                        Style::default().fg(Color::Green),
                    )));
                }
                if diff_count > 5 {
                    lines.push(Line::from(Span::styled(
                        format!("     ... and {} more", diff_count - 5),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
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
                " Exercise Detail ",
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
        let mut scrollbar_state =
            ScrollbarState::new(total_lines).position(scroll_offset);
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

    // Keybinding hints in footer
    let footer = Line::from(vec![
        Span::styled("  🏆 ", Style::default()),
        Span::styled(&bar, Style::default().fg(Color::Green)),
        Span::styled(
            format!(" {}/{} ({}%)", completed, total, pct),
            Style::default().fg(Color::White),
        ),
        Span::styled("      ", Style::default()),
        Span::styled("💡", Style::default()),
        Span::styled("[space]", Style::default().fg(Color::Cyan)),
        Span::styled(" hint  ", Style::default().fg(Color::DarkGray)),
        Span::styled("🔄", Style::default()),
        Span::styled("[r]", Style::default().fg(Color::Cyan)),
        Span::styled(" reset  ", Style::default().fg(Color::DarkGray)),
        Span::styled("⏭️", Style::default()),
        Span::styled(" [n]", Style::default().fg(Color::Cyan)),
        Span::styled(" next  ", Style::default().fg(Color::DarkGray)),
        Span::styled("🚪", Style::default()),
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::styled(" quit", Style::default().fg(Color::DarkGray)),
        Span::styled("      👀 Watching...", Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(
        Paragraph::new(vec![Line::raw(""), footer]),
        area,
    );
}

fn render_help_popup(frame: &mut Frame, _app: &App) {
    let area = centered_rect(50, 60, frame.area());

    let help_text = vec![
        Line::raw(""),
        Line::from(Span::styled(
            "  🏋️ Helix Trainer — Help",
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
