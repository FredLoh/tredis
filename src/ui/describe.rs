use crate::app::App;
use crate::model::KeyValue;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

fn format_string_for_display(value: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_else(|_| value.to_string()),
        Err(_) => value.to_string(),
    }
}

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let key_info = if !app.scan_result.is_empty() {
        Some(&app.scan_result[app.selected_key_index])
    } else {
        None
    };

    let title = if let Some(info) = key_info {
        format!(" Describe: {} ({}) ", info.key, info.key_type)
    } else {
        " Describe ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner_area);

    let content_text = match &app.describe_data {
        KeyValue::String(s) => format_string_for_display(s),
        KeyValue::List(l) => serde_json::to_string_pretty(l).unwrap_or_default(),
        KeyValue::Set(s) => serde_json::to_string_pretty(s).unwrap_or_default(),
        KeyValue::ZSet(z) => serde_json::to_string_pretty(z).unwrap_or_default(),
        KeyValue::Hash(h) => serde_json::to_string_pretty(h).unwrap_or_default(),
        KeyValue::Stream(_) => "Stream data...".to_string(),
        KeyValue::None => "No data loaded.".to_string(),
        KeyValue::Error(e) => format!("Error: {}", e),
    };

    let help_text = if app.active_resource == "keys" {
        "e edit  Enter edit/save  Esc back"
    } else {
        "Esc back"
    };
    let help = Paragraph::new(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(help, layout[0]);

    let lines: Vec<Line> = content_text
        .lines()
        .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::White))))
        .collect();

    let scroll = app.describe_scroll as u16;
    let paragraph = Paragraph::new(lines).scroll((scroll, 0));

    f.render_widget(paragraph, layout[1]);
}
