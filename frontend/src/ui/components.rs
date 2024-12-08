use crossterm::event::KeyCode;
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style},
    layout::Rect,
    Frame,
};

pub struct InputField {
    pub label: String,
    pub content: String,
    pub is_password: bool,
}

impl InputField {
    pub fn new(label: &str, is_password: bool) -> Self {
        Self {
            label: label.to_string(),
            content: String::new(),
            is_password,
        }
    }

    #[allow(unused_variables)]
    pub fn render(&self, f: &mut Frame, area: Rect, is_active: bool) {
        let display_content = if self.is_password {
            "*".repeat(self.content.len())
        } else {
            self.content.clone()
        };

        let paragraph = Paragraph::new(display_content)
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .block(
                Block::default()
                    .title(self.label.as_str())
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::White).fg(Color::Black)),
            );

        f.render_widget(paragraph, area);
    }


    pub fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Backspace => {
                self.content.pop();
            }
            KeyCode::Char(c) => {
                self.content.push(c);
            }
            _ => {}
        }
    }
}
