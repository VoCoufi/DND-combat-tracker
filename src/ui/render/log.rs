use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

pub fn render_log(f: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = app
        .log
        .iter()
        .rev()
        .take(5)
        .rev()
        .map(|entry| {
            Line::from(vec![
                Span::styled(
                    format!("R{}: ", entry.round),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&entry.message),
            ])
        })
        .collect();

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "Log empty",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let block = Block::default()
        .title(" Log ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
