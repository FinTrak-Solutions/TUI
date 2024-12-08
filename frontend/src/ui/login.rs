use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style},
    Frame,
};
use serde::Serialize;
use reqwest::Client;

use crate::ui::components::InputField;

#[derive(Serialize)]
struct SignupData {
    username: String,
    email: String,
    password: String,
}

pub struct LoginPage {
    pub email: InputField,
    pub password: InputField,
    pub active_field: usize,
    pub response_message: String,
}

impl LoginPage {
    pub fn new() -> Self {
        Self {
            email: InputField::new("Email", false),
            password: InputField::new("Password", true),
            active_field: 0,
            response_message: String::new(),
        }
    }

    pub fn render(&self, f: &mut Frame) {
        // Set the global background color to white
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(8),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(3),
                    Constraint::Length(2),
                ]
                    .as_ref(),
            )
            .split(f.area());

        let ascii_title = r#"
 ________ ___  ________   _________  ________  ________  ________  ___  __
|\  _____\\  \|\   ___  \|\___   ___\\   __  \|\   __  \|\   ____\|\  \|\  \
 \ \  \__/\ \  \ \  \\ \  \|___ \  \_\ \  \|\  \ \  \|\  \ \  \___|\ \  \/  /|_
   \ \   __\\ \  \ \  \\ \  \   \ \  \ \ \   _  _\ \   __  \ \  \    \ \   ___  \
      \ \  \_| \ \  \ \  \\ \  \   \ \  \ \ \  \\  \\ \  \ \  \ \  \____\ \  \\ \  \
        \ \__\   \ \__\ \__\\ \__\   \ \__\ \ \__\\ _\\ \__\ \__\ \_______\ \__\\ \__\
         \|__|    \|__|\|__| \|__|    \|__|  \|__|\|__|\|__|\|__|\|_______|\|__| \|__|
"#;

        let title_paragraph = Paragraph::new(ascii_title)
            .style(Style::default().fg(Color::Yellow).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(title_paragraph, chunks[0]);

        self.email.render(f, chunks[1], self.active_field == 0);
        self.password.render(f, chunks[2], self.active_field == 1);

        let response_paragraph = Paragraph::new(self.response_message.clone())
            .block(Block::default().title("Response").borders(Borders::ALL).style(Style::default().bg(Color::White)));
        f.render_widget(response_paragraph, chunks[3]);

        let notice = Paragraph::new("Esc to quit | Hit Enter to log in")
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[4]);
    }

    pub async fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> bool {
        // Detect Escape (Esc) key for quitting
        if key == KeyCode::Esc {
            return true; // Signal to quit
        }

        match key {
            KeyCode::Tab => {
                self.active_field = (self.active_field + 1) % 2; // Cycle through input fields
            }
            KeyCode::BackTab => {
                self.active_field = if self.active_field == 0 { 1 } else { self.active_field - 1 };
            }
            KeyCode::Enter => {
                self.submit().await;
            }
            _ => {
                match self.active_field {
                    0 => self.email.handle_input(key),
                    1 => self.password.handle_input(key),
                    _ => {}
                }
            }
        }
        false
    }

    pub async fn submit(&mut self) {
        // Create a default username for the login request
        let client = Client::new();
        let login_data = SignupData {
            username: "_login".to_string(), // Default username (not used in login)
            email: self.email.content.clone(),
            password: self.password.content.clone(),
        };

        match client.post("http://0.0.0.0:8000/signup")
            .json(&login_data)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to parse response body".to_string());

                if status.is_success() {
                    self.response_message = format!("STATUS_CODE: {}\nMessage: {}", status, message);
                } else {
                    self.response_message = format!("ERROR_CODE: {}\nMessage: {}", status, message);
                }
            }
            Err(e) => {
                self.response_message = format!("Request failed: {}", e);
            }
        }
    }
}
