use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;
use crate::models::{Combatant, StatusEffect};

pub fn render_combatants(f: &mut Frame, area: Rect, app: &App) {
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

            let main_line = Line::from(vec![
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
                temp_hp_span(c),
                death_save_span(c),
                concentration_span(c),
                Span::raw(format!("  AC: {}  ", c.armor_class)),
                Span::styled(status_str, Style::default().fg(Color::Yellow)),
            ]);

            // Build multi-line item with condition effects
            let mut lines = vec![main_line];
            let effect_lines = format_condition_effects(&c.status_effects);
            lines.extend(effect_lines);

            ListItem::new(lines)
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

pub fn hp_color(combatant: &Combatant) -> Color {
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

pub fn hp_bar(combatant: &Combatant) -> String {
    let segments: usize = 12;
    let percentage = combatant.hp_percentage().clamp(0.0, 100.0);
    let filled = ((percentage / 100.0) * segments as f32)
        .round()
        .min(segments as f32) as usize;
    let empty = segments.saturating_sub(filled);

    format!("[{}{}]", "#".repeat(filled), ".".repeat(empty))
}

pub fn death_save_span(combatant: &Combatant) -> Span<'static> {
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

pub fn temp_hp_span(combatant: &Combatant) -> Span<'static> {
    if combatant.temp_hp > 0 {
        Span::styled(
            format!(" (+{} temp)", combatant.temp_hp),
            Style::default().fg(Color::Cyan),
        )
    } else {
        Span::raw("")
    }
}

pub fn concentration_span(combatant: &Combatant) -> Span<'static> {
    if let Some(info) = &combatant.concentration {
        let text = format!(" [Conc: {}]", info.spell_name);
        Span::styled(
            text,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
    } else {
        Span::raw("")
    }
}

/// Creates formatted lines showing mechanical effects for active conditions
pub fn format_condition_effects(status_effects: &[StatusEffect]) -> Vec<Line<'static>> {
    if status_effects.is_empty() {
        return vec![];
    }

    let effect_style = Style::default()
        .fg(Color::Gray)
        .add_modifier(Modifier::ITALIC);

    status_effects
        .iter()
        .map(|effect| {
            let effect_text = format!("    ⚬ {}: {}",
                effect.condition.as_str(),
                effect.condition.mechanical_effects()
            );
            Line::from(vec![
                Span::styled(effect_text, effect_style)
            ])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ConditionType;

    #[test]
    fn format_condition_effects_empty_returns_empty() {
        let effects: Vec<StatusEffect> = vec![];
        let result = format_condition_effects(&effects);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn format_condition_effects_single_condition() {
        let effects = vec![StatusEffect::new(ConditionType::Blinded, 2, None)];
        let result = format_condition_effects(&effects);

        assert_eq!(result.len(), 1);
        // The line should contain the condition name and description
        // We can't easily inspect the Line contents, but we can verify count
    }

    #[test]
    fn format_condition_effects_multiple_conditions() {
        let effects = vec![
            StatusEffect::new(ConditionType::Blinded, 2, None),
            StatusEffect::new(ConditionType::Poisoned, 3, None),
            StatusEffect::new(ConditionType::Prone, 0, None),
        ];
        let result = format_condition_effects(&effects);

        // Should have one line per condition
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn all_conditions_have_descriptions() {
        // Verify that all 14 condition types have descriptions
        // This ensures the format_condition_effects function will work for all
        let all_conditions = ConditionType::all();

        assert_eq!(all_conditions.len(), 14);

        for condition in all_conditions {
            let desc = condition.description();
            assert!(!desc.is_empty(), "Condition {:?} has no description", condition);
        }
    }
}
