pub mod acls_table;
pub mod channels_table;
pub mod clients_table;
pub mod configs_table;
pub mod describe;
pub mod dialog;
pub mod edit_dialog;
pub mod header;
pub mod info_view;
pub mod keys_table;
pub mod monitor_table;
pub mod pubsub_table;
pub mod resources;
pub mod server_dialog;
pub mod servers_table;
pub mod slowlog_table;
pub mod splash;
pub mod streams_table;

use crate::app::{App, Mode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    if app.mode == Mode::Splash {
        splash::render(f, &app.splash_state);
        return;
    }

    // Server dialog is shown as a full-screen overlay when no servers exist
    if app.mode == Mode::ServerDialog {
        server_dialog::render(f, &app.server_dialog_state);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Header
            Constraint::Min(1),    // Main content
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    header::render(f, app, chunks[0]);

    match app.mode {
        Mode::Describe | Mode::EditValue => {
            describe::render(f, app, chunks[1]);
        }
        _ => match app.active_resource.as_str() {
            "servers" => servers_table::render(f, app, chunks[1]),
            "clients" => clients_table::render(f, app, chunks[1]),
            "info" => info_view::render(f, app, chunks[1]),
            "slowlog" => slowlog_table::render(f, app, chunks[1]),
            "config" => configs_table::render(f, app, chunks[1]),
            "acl" => acls_table::render(f, app, chunks[1]),
            "monitor" => monitor_table::render(f, app, chunks[1]),
            "streams" => streams_table::render(f, app, chunks[1]),
            "channels" => channels_table::render(f, app, chunks[1]),
            "pubsub" => pubsub_table::render(f, app, chunks[1]),
            _ => keys_table::render(f, app, chunks[1]),
        },
    }

    // Render overlays
    if app.mode == Mode::Confirm {
        dialog::render(f, app);
    }

    if app.mode == Mode::Resources {
        resources::render(f, app);
    }

    if app.mode == Mode::EditValue {
        edit_dialog::render(f, app);
    }

    if app.loading_state.active {
        render_loading_overlay(f, app);
    }
}

fn render_loading_overlay(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 5, f.area());
    let spinner_chars = ["⠋", "⠙", "⠹", "⠸"];
    let spinner = spinner_chars[app.loading_state.spinner_frame % spinner_chars.len()];

    let block = Block::default()
        .title(" Loading ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let content = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("{} ", spinner),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &app.loading_state.message,
            Style::default().fg(Color::White),
        ),
    ]))
    .alignment(Alignment::Center)
    .block(block);

    f.render_widget(Clear, area);
    f.render_widget(content, area);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width) / 2),
            Constraint::Percentage(width),
            Constraint::Percentage((100 - width) / 2),
        ])
        .split(vertical[1])[1]
}
