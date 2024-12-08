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

    pub fn render(&self, f: &mut Frame, area: Rect, is_active: bool) {
        // Display asterisks if it's a password field, otherwise show the content
        let display_content = if self.is_password {
            let stars = "*".repeat(self.content.len());
            if is_active {
                format!("{}|", stars) // Add a cursor at the end of the input
            } else {
                stars
            }
        } else {
            if is_active {
                format!("{}|", self.content) // Add a cursor at the end of the input
            } else {
                self.content.clone()
            }
        };

        // Highlight the border in gold if the field is active
        let border_style = if is_active {
            Style::default().fg(Color::Yellow) // Gold border for active input
        } else {
            Style::default().fg(Color::Black) // Black border for inactive input
        };

        // Render the input field
        let paragraph = Paragraph::new(display_content)
            .style(Style::default().fg(Color::Black).bg(Color::White)) // Set text to black on white background
            .block(
                Block::default()
                    .title(self.label.as_str())
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::White).fg(Color::Black)) // White background, black text
                    .border_style(border_style), // Border changes color based on active status
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
