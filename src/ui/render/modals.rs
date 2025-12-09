#![allow(clippy::implicit_saturating_sub)]

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::{
    AddCombatantState, AddConcentrationState, App, ClearAction, ConcentrationCheckState,
    ConditionSelectionState, LoadLibraryState, SaveEncounterState, SaveLibraryState,
    SelectionState, StatusSelectionState,
};
use crate::models::ConditionType;

pub fn render_add_combatant_modal(f: &mut Frame, state: &AddCombatantState) {
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

pub fn render_selection_modal(
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

pub fn render_condition_selection(f: &mut Frame, state: &ConditionSelectionState, app: &App) {
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

pub fn render_status_clear_modal(f: &mut Frame, state: &StatusSelectionState, app: &App) {
    let area = centered_rect(60, 50, f.area());
    let combatant = app.encounter.combatants.get(state.combatant_index).cloned();

    let name = combatant
        .as_ref()
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let mut lines = vec![Line::from(Span::styled(
        format!("Clear status from {}", name),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    if let Some(c) = combatant {
        for (i, effect) in c.status_effects.iter().enumerate() {
            let selected = i == state.selected_status_index;
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if selected { "> " } else { "  " };
            let duration = if effect.duration >= 0 {
                format!("{}r", effect.duration)
            } else {
                "∞".to_string()
            };
            lines.push(Line::from(Span::styled(
                format!(
                    "{}{} (duration: {})",
                    prefix,
                    effect.condition.as_str(),
                    duration
                ),
                style,
            )));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "No combatant selected.",
            Style::default().fg(Color::Red),
        )));
    }

    let block = Block::default()
        .title(" Clear Status ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_add_concentration_modal(f: &mut Frame, state: &AddConcentrationState, app: &App) {
    let area = centered_rect(60, 50, f.area());
    let combatant_name = app
        .encounter
        .combatants
        .get(state.combatant_index)
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let prompts = ["Spell name:", "CON modifier:"];
    let values = [&state.spell_name, &state.con_mod];

    let mut lines = vec![Line::from(Span::styled(
        format!("Set concentration for {}", combatant_name),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

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
        .title(" Set Concentration ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_concentration_check(f: &mut Frame, state: &ConcentrationCheckState, app: &App) {
    let area = centered_rect(60, 40, f.area());
    let combatant_name = app
        .encounter
        .combatants
        .get(state.combatant_index)
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let lines = vec![
        Line::from(Span::styled(
            format!(
                "Concentration check for {} (DC {})",
                combatant_name, state.dc
            ),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("Enter total CON save roll:"),
        Line::from(""),
        Line::from(vec![
            Span::raw("> "),
            Span::styled(state.input.clone(), Style::default().fg(Color::White)),
            Span::styled(
                "_",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]),
    ];

    let block = Block::default()
        .title(" Concentration Check ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_clear_choice_modal(f: &mut Frame, choice: &ClearAction) {
    let area = centered_rect(40, 40, f.area());
    let options = [
        (ClearAction::Concentration, "Clear Concentration"),
        (ClearAction::StatusEffects, "Clear Status Effects"),
    ];

    let mut lines = vec![Line::from(Span::styled(
        "Choose what to clear:",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    for (opt, label) in &options {
        let selected = std::mem::discriminant(opt) == std::mem::discriminant(choice);
        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, label),
            style,
        )));
    }

    let block = Block::default()
        .title(" Clear Menu ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_template_selection_modal(f: &mut Frame, state: &SelectionState, app: &App) {
    let area = centered_rect(60, 50, f.area());

    let filtered: Vec<&crate::models::CombatantTemplate> = app
        .templates
        .iter()
        .filter(|t| t.name.to_lowercase().contains(&state.input.to_lowercase()))
        .collect();
    let selected_index = state.selected_index.min(filtered.len().saturating_sub(1));
    let max_visible = 10;
    let start = if selected_index + 1 > max_visible {
        selected_index + 1 - max_visible
    } else {
        0
    };
    let end = (start + max_visible).min(filtered.len());

    let mut lines = vec![Line::from(Span::styled(
        "Select template to add:",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    if start > 0 {
        lines.push(Line::from(Span::styled(
            "… more above …",
            Style::default().fg(Color::DarkGray),
        )));
    }

    for (visible_idx, t) in filtered.iter().enumerate().skip(start).take(max_visible) {
        let selected = visible_idx == selected_index;
        let style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!(
                "{}{}. {} (HP: {}, AC: {}, {})",
                prefix,
                visible_idx + 1,
                t.name,
                t.hp_max,
                t.armor_class,
                if t.is_player { "PC" } else { "NPC" }
            ),
            style,
        )));
    }

    if end < filtered.len() {
        lines.push(Line::from(Span::styled(
            "… more below …",
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("Filter: "),
        Span::styled(
            state.input.clone(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let block = Block::default()
        .title(" Templates ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_save_encounter_modal(f: &mut Frame, state: &SaveEncounterState) {
    let area = centered_rect(60, 30, f.area());

    let lines = vec![
        Line::from(Span::styled(
            "Save Encounter",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Enter filename (alphanumeric, underscore, hyphen only):"),
        Line::from(""),
        Line::from(vec![
            Span::raw("> "),
            Span::styled(state.input.clone(), Style::default().fg(Color::White)),
            Span::styled(
                "_",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to save, Esc to cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let block = Block::default()
        .title(" Save Encounter ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_load_encounter_modal(f: &mut Frame, state: &SelectionState, app: &App) {
    let area = centered_rect(60, 50, f.area());

    let saved_encounters = app.list_saved_encounters();
    let filtered: Vec<&String> = saved_encounters
        .iter()
        .filter(|name| name.to_lowercase().contains(&state.input.to_lowercase()))
        .collect();

    let selected_index = state.selected_index.min(filtered.len().saturating_sub(1));
    let max_visible = 10;
    let start = if selected_index + 1 > max_visible {
        selected_index + 1 - max_visible
    } else {
        0
    };
    let end = (start + max_visible).min(filtered.len());

    let mut lines = vec![Line::from(Span::styled(
        "Load Encounter",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    if filtered.is_empty() {
        lines.push(Line::from(Span::styled(
            "No saved encounters found",
            Style::default().fg(Color::Red),
        )));
    } else {
        if start > 0 {
            lines.push(Line::from(Span::styled(
                "… more above …",
                Style::default().fg(Color::DarkGray),
            )));
        }

        for (visible_idx, name) in filtered.iter().enumerate().skip(start).take(max_visible) {
            let selected = visible_idx == selected_index;
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if selected { "> " } else { "  " };
            lines.push(Line::from(Span::styled(
                format!("{}{}. {}", prefix, visible_idx + 1, name),
                style,
            )));
        }

        if end < filtered.len() {
            lines.push(Line::from(Span::styled(
                "… more below …",
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("Filter: "),
        Span::styled(
            state.input.clone(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let block = Block::default()
        .title(" Load Encounter ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_save_library_modal(f: &mut Frame, state: &SaveLibraryState) {
    let area = centered_rect(60, 50, f.area());

    let prompts = [
        "Enter encounter name:",
        "Enter description:",
        "Enter difficulty (optional, e.g., Easy, Medium, Hard):",
    ];

    let values = [&state.name, &state.description, &state.difficulty];

    let mut lines = vec![Line::from(Span::styled(
        "Save to Encounter Library",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

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
        .title(" Save to Library ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_loading_library_modal(f: &mut Frame, state: &SelectionState, app: &App) {
    let area = centered_rect(70, 60, f.area());

    let library_templates = app.list_library_templates();
    let filtered: Vec<&String> = library_templates
        .iter()
        .filter(|name| name.to_lowercase().contains(&state.input.to_lowercase()))
        .collect();

    let selected_index = state.selected_index.min(filtered.len().saturating_sub(1));
    let max_visible = 10;
    let start = if selected_index + 1 > max_visible {
        selected_index + 1 - max_visible
    } else {
        0
    };
    let end = (start + max_visible).min(filtered.len());

    let mut lines = vec![Line::from(Span::styled(
        "Load from Encounter Library",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    if filtered.is_empty() {
        lines.push(Line::from(Span::styled(
            "No library templates found",
            Style::default().fg(Color::Red),
        )));
    } else {
        if start > 0 {
            lines.push(Line::from(Span::styled(
                "… more above …",
                Style::default().fg(Color::DarkGray),
            )));
        }

        for (visible_idx, name) in filtered.iter().enumerate().skip(start).take(max_visible) {
            let selected = visible_idx == selected_index;
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let prefix = if selected { "> " } else { "  " };
            lines.push(Line::from(Span::styled(
                format!("{}{}. {}", prefix, visible_idx + 1, name),
                style,
            )));
        }

        if end < filtered.len() {
            lines.push(Line::from(Span::styled(
                "… more below …",
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("Filter: "),
        Span::styled(
            state.input.clone(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let block = Block::default()
        .title(" Load from Library ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_library_initiative_modal(f: &mut Frame, state: &LoadLibraryState, _app: &App) {
    let area = centered_rect(60, 40, f.area());

    let current_combatant = &state.combatants_with_init[state.current_index].0;
    let current_input = &state.combatants_with_init[state.current_index].1;

    let progress = format!(
        "Setting initiative ({}/{})",
        state.current_index + 1,
        state.combatants_with_init.len()
    );

    let lines = vec![
        Line::from(Span::styled(
            format!("Loading: {}", state.template.name),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(progress, Style::default().fg(Color::DarkGray))),
        Line::from(""),
        Line::from(vec![
            Span::raw("Combatant: "),
            Span::styled(
                &current_combatant.name,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(format!(
            "HP: {}, AC: {}",
            current_combatant.hp_max, current_combatant.armor_class
        )),
        Line::from(""),
        Line::from("Enter initiative:"),
        Line::from(""),
        Line::from(vec![
            Span::raw("> "),
            Span::styled(current_input.clone(), Style::default().fg(Color::White)),
            Span::styled(
                "_",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::SLOW_BLINK),
            ),
        ]),
    ];

    let block = Block::default()
        .title(" Set Initiatives ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_confirm_overwrite_modal(f: &mut Frame) {
    let area = centered_rect(50, 30, f.area());

    let lines = vec![
        Line::from(Span::styled(
            "Confirm Overwrite",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("A library entry with this name already exists."),
        Line::from(""),
        Line::from(Span::styled(
            "Overwrite? (y/n)",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let block = Block::default()
        .title(" Confirm ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_confirm_load_modal(f: &mut Frame) {
    let area = centered_rect(50, 30, f.area());

    let lines = vec![
        Line::from(Span::styled(
            "Confirm Load",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "This will clear the current encounter!",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("All combatants, HP, statuses, and log will be lost."),
        Line::from(""),
        Line::from(Span::styled(
            "Continue? (y/n)",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let block = Block::default()
        .title(" Warning ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
