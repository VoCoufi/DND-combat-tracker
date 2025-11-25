use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::{AddCombatantState, App, ConditionSelectionState, InputMode, SelectionState};
use crate::models::{Combatant, ConditionType};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Commands
            Constraint::Length(2), // Message
        ])
        .split(f.area());

    // Render header
    render_header(f, chunks[0], app);

    // Render main content
    render_combatants(f, chunks[1], app);

    // Render commands
    render_commands(f, chunks[2], app);

    // Render message
    if let Some(ref msg) = app.message {
        render_message(f, chunks[3], msg);
    }

    // Render modal if needed
    match &app.input_mode {
        InputMode::AddingCombatant(state) => render_add_combatant_modal(f, state),
        InputMode::DealingDamage(state) => {
            render_selection_modal(f, state, "Deal Damage", "Enter damage amount:", app)
        }
        InputMode::Healing(state) => {
            render_selection_modal(f, state, "Heal", "Enter heal amount:", app)
        }
        InputMode::AddingStatus(state) => {
            render_selection_modal(f, state, "Add Status Effect", "Select combatant:", app)
        }
        InputMode::SelectingCondition(state) => render_condition_selection(f, state, app),
        InputMode::RollingDeathSave(state) => {
            render_selection_modal(f, state, "Death Save", "Enter d20 roll:", app)
        }
        InputMode::Removing(state) => render_selection_modal(
            f,
            state,
            "Remove Combatant",
            "Select combatant to remove:",
            app,
        ),
        InputMode::Normal => {}
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let title = format!(
        " D&D 5e Combat Tracker | Round: {} ",
        app.encounter.round_number
    );
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(block, area);
}

fn render_combatants(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .encounter
        .combatants
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let is_current = i == app.encounter.current_turn_index;
            let arrow = if is_current { "→ " } else { "  " };

            let name_color = if c.is_player {
                Color::Green
            } else {
                Color::Red
            };

            let status_str = if c.status_effects.is_empty() {
                String::new()
            } else {
                let effects: Vec<String> = c
                    .status_effects
                    .iter()
                    .map(|e| format!("{}({})", e.condition.as_str(), e.duration))
                    .collect();
                format!(" [{}]", effects.join(", "))
            };

            let hp_color = hp_color(c);
            let hp_style = Style::default().fg(hp_color);
            let hp_bar = hp_bar(c);

            let line = Line::from(vec![
                Span::raw(arrow),
                Span::raw(format!("[{:2}] ", c.initiative)),
                Span::styled(
                    format!("{:<20}", c.name),
                    Style::default().fg(name_color).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" HP "),
                Span::styled(hp_bar, hp_style),
                Span::raw(" "),
                Span::styled(format!("{}/{}", c.hp_current, c.hp_max), hp_style),
                death_save_span(c),
                Span::raw(format!("  AC: {}  ", c.armor_class)),
                Span::styled(status_str, Style::default().fg(Color::Yellow)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" Initiative Order ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White)),
    );

    f.render_widget(list, area);
}

fn render_commands(f: &mut Frame, area: Rect, app: &App) {
    let commands = match app.input_mode {
        InputMode::Normal => {
            "[n] Next Turn  [d] Damage  [h] Heal  [s] Status  [v] Death Save  [a] Add  [r] Remove  [q] Quit"
        }
        _ => "[Esc] Cancel",
    };

    let block = Block::default()
        .title(" Commands ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(commands)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn render_message(f: &mut Frame, area: Rect, msg: &str) {
    let paragraph = Paragraph::new(msg)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn render_add_combatant_modal(f: &mut Frame, state: &AddCombatantState) {
    let area = centered_rect(60, 40, f.area());

    let prompts = [
        "Enter name:",
        "Enter initiative:",
        "Enter max HP:",
        "Enter AC:",
        "Is player? (y/n):",
    ];

    let values = [
        &state.name,
        &state.initiative,
        &state.hp,
        &state.ac,
        &state.is_player,
    ];

    let mut lines = vec![];
    for (i, prompt) in prompts.iter().enumerate() {
        if i < state.step {
            lines.push(Line::from(vec![
                Span::raw(*prompt),
                Span::raw(" "),
                Span::styled(values[i].clone(), Style::default().fg(Color::Green)),
            ]));
        } else if i == state.step {
            lines.push(Line::from(vec![Span::styled(
                *prompt,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(vec![
                Span::raw("> "),
                Span::styled(values[i].clone(), Style::default().fg(Color::White)),
                Span::styled(
                    "_",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ]));
        } else {
            lines.push(Line::from(Span::styled(
                *prompt,
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let block = Block::default()
        .title(" Add Combatant ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn render_selection_modal(
    f: &mut Frame,
    state: &SelectionState,
    title: &str,
    prompt: &str,
    app: &App,
) {
    let area = centered_rect(60, 50, f.area());

    let mut lines = vec![Line::from(Span::styled(
        prompt,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];

    lines.push(Line::from(""));

    for (i, c) in app.encounter.combatants.iter().enumerate() {
        let style = if i == state.selected_index {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let marker = if i == state.selected_index {
            "> "
        } else {
            "  "
        };
        lines.push(Line::from(Span::styled(
            format!(
                "{}{}. {} (HP: {}/{})",
                marker,
                i + 1,
                c.name,
                c.hp_current,
                c.hp_max
            ),
            style,
        )));
    }

    if !state.input.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("Input: "),
            Span::styled(&state.input, Style::default().fg(Color::Green)),
        ]));
    }

    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn render_condition_selection(f: &mut Frame, state: &ConditionSelectionState, app: &App) {
    let area = centered_rect(50, 60, f.area());

    let combatant_name = app
        .encounter
        .combatants
        .get(state.combatant_index)
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let mut lines = vec![
        Line::from(Span::styled(
            format!("Select condition for {}:", combatant_name),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (i, condition) in ConditionType::all().iter().enumerate() {
        lines.push(Line::from(Span::raw(format!(
            "{}. {}",
            i + 1,
            condition.as_str()
        ))));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::raw(
        "Enter number and duration (e.g., 1 3):",
    )));
    lines.push(Line::from(vec![
        Span::raw("> "),
        Span::styled(state.input.clone(), Style::default().fg(Color::White)),
        Span::styled(
            "_",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]));

    let block = Block::default()
        .title(" Select Condition ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn hp_color(combatant: &Combatant) -> Color {
    if combatant.is_dead() {
        Color::DarkGray
    } else if combatant.is_unconscious() {
        Color::DarkGray
    } else if combatant.hp_percentage() < 25.0 {
        Color::Red
    } else if combatant.hp_percentage() < 50.0 {
        Color::Yellow
    } else {
        Color::Green
    }
}

fn hp_bar(combatant: &Combatant) -> String {
    let segments: usize = 12;
    let percentage = combatant.hp_percentage().clamp(0.0, 100.0);
    let filled = ((percentage / 100.0) * segments as f32)
        .round()
        .min(segments as f32) as usize;
    let empty = segments.saturating_sub(filled);

    format!("[{}{}]", "#".repeat(filled), ".".repeat(empty))
}

fn death_save_span(combatant: &Combatant) -> Span<'static> {
    if combatant.is_dead() {
        return Span::styled(
            " [DEAD]",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        );
    }

    if let Some(ds) = &combatant.death_saves {
        let mut label = format!(" DS S{}/F{}", ds.successes, ds.failures);
        if ds.is_stable {
            label.push_str(" (stable)");
        }
        Span::styled(
            label,
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::raw("")
    }
}
