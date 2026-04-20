use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub struct EditDialogState {
    pub key: String,
    pub buffer: String,
    pub cursor: usize,
    pub prefer_json_for_string: bool,
    pub error_message: Option<String>,
}

impl EditDialogState {
    pub fn new(key: String, buffer: String, prefer_json_for_string: bool) -> Self {
        let cursor = buffer.len();
        Self {
            key,
            buffer,
            cursor,
            prefer_json_for_string,
            error_message: None,
        }
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += c.len_utf8();
        self.error_message = None;
    }

    pub fn insert_str(&mut self, text: &str) {
        self.buffer.insert_str(self.cursor, text);
        self.cursor += text.len();
        self.error_message = None;
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let new_cursor = prev_boundary(&self.buffer, self.cursor);
        self.buffer.drain(new_cursor..self.cursor);
        self.cursor = new_cursor;
        self.error_message = None;
    }

    pub fn delete(&mut self) {
        if self.cursor >= self.buffer.len() {
            return;
        }
        let next = next_boundary(&self.buffer, self.cursor);
        self.buffer.drain(self.cursor..next);
        self.error_message = None;
    }

    pub fn move_left(&mut self) {
        self.cursor = prev_boundary(&self.buffer, self.cursor);
    }

    pub fn move_right(&mut self) {
        self.cursor = next_boundary(&self.buffer, self.cursor);
    }

    pub fn move_home(&mut self) {
        self.cursor = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor = self.buffer.len();
    }
}

fn prev_boundary(text: &str, index: usize) -> usize {
    let mut candidate = index.saturating_sub(1);
    while candidate > 0 && !text.is_char_boundary(candidate) {
        candidate -= 1;
    }
    candidate
}

fn next_boundary(text: &str, index: usize) -> usize {
    if index >= text.len() {
        return text.len();
    }
    let mut candidate = index + 1;
    while candidate < text.len() && !text.is_char_boundary(candidate) {
        candidate += 1;
    }
    candidate
}

pub fn render(f: &mut Frame, app: &crate::app::App) {
    let Some(state) = &app.edit_dialog_state else {
        return;
    };

    let area = centered_rect(80, 20, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Edit Value ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    let instructions = Paragraph::new(Line::from(vec![
        Span::styled("<Enter>", Style::default().fg(Color::Yellow)),
        Span::styled(" save  ", Style::default().fg(Color::DarkGray)),
        Span::styled("<Esc>", Style::default().fg(Color::Yellow)),
        Span::styled(" cancel  ", Style::default().fg(Color::DarkGray)),
        Span::styled("<Ctrl-j>", Style::default().fg(Color::Yellow)),
        Span::styled(" newline  ", Style::default().fg(Color::DarkGray)),
        Span::styled("<Arrows>", Style::default().fg(Color::Yellow)),
        Span::styled(" move", Style::default().fg(Color::DarkGray)),
    ]));
    f.render_widget(instructions, chunks[0]);

    let key_line = Paragraph::new(Span::styled(
        format!("Key: {}", state.key),
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(key_line, chunks[1]);

    let mut display = state.buffer.clone();
    if display.is_char_boundary(state.cursor) {
        display.insert(state.cursor, '_');
    }
    let editor = Paragraph::new(display).style(Style::default().fg(Color::White));
    f.render_widget(editor, chunks[2]);

    let footer = if let Some(error) = &state.error_message {
        Paragraph::new(Span::styled(error, Style::default().fg(Color::Red)))
    } else if state.prefer_json_for_string {
        Paragraph::new(Span::styled(
            "String contains JSON: shown formatted here, saved back as a raw string.",
            Style::default().fg(Color::DarkGray),
        ))
    } else {
        Paragraph::new(Span::styled(
            "Strings use plain text. Lists, sets, hashes, and zsets use JSON.",
            Style::default().fg(Color::DarkGray),
        ))
    };
    f.render_widget(footer, chunks[3]);
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(height),
            Constraint::Percentage(20),
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
