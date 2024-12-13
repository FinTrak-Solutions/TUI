use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
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
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(8),  // ASCII Title
                    Constraint::Length(3),  // Username Input
                    Constraint::Length(3),  // Email Input
                    Constraint::Length(3),  // Password Input
                    Constraint::Length(3),  // Confirm Password Input
                    Constraint::Length(3),  // Response message
                    Constraint::Length(3),  // Bottom Notice
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

        self.username.render(f, chunks[1], self.active_field == 0);
        self.email.render(f, chunks[2], self.active_field == 1);
        self.password.render(f, chunks[3], self.active_field == 2);
        self.confirm_password.render(f, chunks[4], self.active_field == 3);

        // Response message displayed between the inputs and the bottom notice
        let response_paragraph = Paragraph::new(self.response_message.clone())
            .style(Style::default().fg(Color::Red).bg(Color::White)) // Response message in red
            .alignment(Alignment::Center);
        f.render_widget(response_paragraph, chunks[5]);

        // Bottom notice
        let notice_text = "Esc to quit | Hit Enter to create new user";
        let notice_paragraph = Paragraph::new(notice_text)
            .style(Style::default().fg(Color::DarkGray).bg(Color::White)) // Grey text, white background
            .alignment(Alignment::Center);
        f.render_widget(notice_paragraph, chunks[6]);
    }

    pub async fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> bool {
        if key == KeyCode::Esc {
            return false; // Navigate back to Cover
        }

        match key {
            KeyCode::Tab => {
                self.active_field = (self.active_field + 1) % 4; // Cycle through input fields
            }
            KeyCode::BackTab => {
                self.active_field = if self.active_field == 0 { 3 } else { self.active_field - 1 };
            }
            KeyCode::Enter => {
                return self.submit().await;
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

    pub async fn submit(&mut self) -> bool {
        if self.password.content != self.confirm_password.content {
            self.response_message = "Passwords do not match".to_string();
            return false;
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

                match status.as_u16() {
                    201 => {
                        self.response_message = "Signup successful! Redirecting to login...".to_string();
                        return true; // Return true to navigate to login
                    }
                    200 if message.contains("Login successful") => {
                        self.response_message = "Email already registered. Please try another one.".to_string();
                    }
                    400 => {
                        self.response_message = "Failed to sign up. Please check your inputs.".to_string();
                    }
                    _ => {
                        self.response_message = format!("ERROR_CODE: {}\nMessage: {}", status, message);
                    }
                }
            }
            Err(e) => {
                self.response_message = format!("Request failed: {}", e);
            }
        }
        false
    }
}
