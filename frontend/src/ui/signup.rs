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

pub struct SignupPage {
    pub username: InputField,
    pub email: InputField,
    pub password: InputField,
    pub confirm_password: InputField,
    pub active_field: usize,
    pub response_message: String,
}

impl SignupPage {
    pub fn new() -> Self {
        Self {
            username: InputField::new("Username", false),
            email: InputField::new("Email", false),
            password: InputField::new("Password", true),
            confirm_password: InputField::new("Confirm Password", true),
            active_field: 0,
            response_message: String::new(),
        }
    }

    pub fn render(&self, f: &mut Frame) {
        // Layout with space for the ASCII title, input fields, and response
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1) // Margin outside the whole form
            .constraints(
                [
                    Constraint::Length(8), // ASCII title height
                    Constraint::Length(3), // Username field height
                    Constraint::Length(3), // Email field height
                    Constraint::Length(3), // Password field height
                    Constraint::Length(3), // Confirm Password field height
                    Constraint::Min(3),    // Response box height
                    Constraint::Length(2), // Notice height
                ]
                    .as_ref(),
            )
            .split(f.area());

        // Render the ASCII title at the top
        // https://patorjk.com/software/taag/#p=testall&h=2&v=1&f=Standard&t=FinTrack
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
            .style(Style::default().fg(Color::Yellow)) // Yellow color for the title
            .alignment(Alignment::Center); // Center align the title
        f.render_widget(title_paragraph, chunks[0]);

        // Render each input field
        self.username.render(f, chunks[1], self.active_field == 0);
        self.email.render(f, chunks[2], self.active_field == 1);
        self.password.render(f, chunks[3], self.active_field == 2);
        self.confirm_password.render(f, chunks[4], self.active_field == 3);

        // Render the response box
        let response_paragraph = Paragraph::new(self.response_message.clone())
            .block(Block::default().title("Response").borders(Borders::ALL));
        f.render_widget(response_paragraph, chunks[5]);

        // Render the centered notice
        let notice = Paragraph::new("Esc to quit | Hit Enter to create a new user")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[6]);
    }

    pub async fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> bool {
        // Detect Escape (Esc) key for quitting
        if key == KeyCode::Esc {
            return true; // Signal to quit
        }

        match key {
            KeyCode::Tab => {
                self.active_field = (self.active_field + 1) % 4; // Cycle through input fields
            }
            KeyCode::BackTab => {
                self.active_field = if self.active_field == 0 { 3 } else { self.active_field - 1 };
            }
            KeyCode::Enter => {
                self.submit().await;
            }
            _ => {
                match self.active_field {
                    0 => self.username.handle_input(key),
                    1 => self.email.handle_input(key),
                    2 => self.password.handle_input(key),
                    3 => self.confirm_password.handle_input(key),
                    _ => {}
                }
            }
        }
        false
    }

    pub async fn submit(&mut self) {
        if self.password.content != self.confirm_password.content {
            self.response_message = "Passwords do not match".to_string();
            return;
        }

        let client = Client::new();
        let signup_data = SignupData {
            username: self.username.content.clone(),
            email: self.email.content.clone(),
            password: self.password.content.clone(),
        };

        match client.post("http://0.0.0.0:8000/signup")
            .json(&signup_data)
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
