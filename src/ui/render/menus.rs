#![allow(clippy::implicit_saturating_sub)]

use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::App;
use crate::models::ConditionType;

use super::modals::centered_rect;

pub fn render_action_menu(f: &mut Frame, selected: usize) {
    let area = centered_rect(50, 40, f.area());
    let items = [
        "Deal Damage",
        "Heal",
        "Add Status Effect",
        "Roll Death Save",
        "Set Concentration",
        "Clear Concentration/Status",
        "Grant Temp HP",
    ];

    let mut lines = vec![Line::from(Span::styled(
        "Action Menu",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    for (i, label) in items.iter().enumerate() {
        let selected_style = if i == selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if i == selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, label),
            selected_style,
        )));
    }

    let block = Block::default()
        .title(" Actions ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_combatant_menu(f: &mut Frame, selected: usize) {
    let area = centered_rect(50, 50, f.area());
    let items = [
        "Add Combatant",
        "Remove Combatant",
        "Add from Template",
        "Save as Template",
        "Load Encounter Library",
        "Save to Encounter Library",
    ];

    let mut lines = vec![Line::from(Span::styled(
        "Combatant Menu",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    for (i, label) in items.iter().enumerate() {
        let selected_style = if i == selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if i == selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, label),
            selected_style,
        )));
    }

    let block = Block::default()
        .title(" Combatants ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

pub fn render_quick_reference(f: &mut Frame, selected_index: usize, _app: &App) {
    let area = centered_rect(70, 80, f.area());

    let mut lines = vec![Line::from(Span::styled(
        "Condition Reference (↑/↓, Esc to close)",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))];
    lines.push(Line::from(""));

    let conditions = ConditionType::all();
    let max_visible = 8;
    let start = if selected_index + 1 > max_visible {
        selected_index + 1 - max_visible
    } else {
        0
    };
    let end = (start + max_visible).min(conditions.len());

    if start > 0 {
        lines.push(Line::from(Span::styled(
            "… more above …",
            Style::default().fg(Color::DarkGray),
        )));
    }

    for (idx, condition) in conditions.iter().enumerate().skip(start).take(max_visible) {
        let selected = idx == selected_index;
        let title_style = if selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Yellow)
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{}: ", condition.as_str()), title_style),
            Span::raw(condition.description()),
        ]));
        lines.push(Line::from(""));
    }

    if end < conditions.len() {
        lines.push(Line::from(Span::styled(
            "… more below …",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let block = Block::default()
        .title(" Quick Reference (?) ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
