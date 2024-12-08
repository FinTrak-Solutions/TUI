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
        // Mask content if it's a password field
        let display_content = if self.is_password {
            let stars = "*".repeat(self.content.len());
            if is_active {
                format!("{}|", stars) // Add a blinking cursor to the active field
            } else {
                stars
            }
        } else {
            if is_active {
                format!("{}|", self.content)
            } else {
                self.content.clone()
            }
        };

        // Set active field style (yellow border for active, black otherwise)
        let border_style = if is_active {
            Style::default().fg(Color::Yellow) // Highlight active box with yellow border
        } else {
            Style::default().fg(Color::Black) // Default box edge (border) color is black
        };

        // Paragraph with black text for input visibility
        let paragraph = Paragraph::new(display_content)
            .style(Style::default().fg(Color::Black)) // Text color is black
            .block(
                Block::default()
                    .title(self.label.as_str())
                    .borders(Borders::ALL)
                    .style(border_style), // Apply border color
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
