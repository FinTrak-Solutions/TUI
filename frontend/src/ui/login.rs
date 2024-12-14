use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use reqwest::Client;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

use crate::ui::components::InputField;
use crate::ui::homepage::Homepage;
use crate::ui::report::*;

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
    pub report_overview: Vec<String>,
}

impl LoginPage {
    pub fn new() -> Self {
        Self {
            email: InputField::new("Email", false),
            password: InputField::new("Password", true),
            active_field: 0,
            response_message: String::new(),
            report_overview: vec![],
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
                    Constraint::Length(8), // ASCII Title
                    Constraint::Length(3), // Email Input
                    Constraint::Length(3), // Password Input
                    Constraint::Length(3), // Response message
                    Constraint::Length(3), // Bottom Notice
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

        // Response message displayed between the password and the bottom notice
        let response_paragraph = Paragraph::new(self.response_message.clone())
            .style(Style::default().fg(Color::Red).bg(Color::White)) // Red message for errors
            .alignment(Alignment::Center);
        f.render_widget(response_paragraph, chunks[3]);

        // Render the bottom notice
        let notice_text = "Esc to quit | Hit Enter to login";
        let notice_paragraph = Paragraph::new(notice_text)
            .style(Style::default().fg(Color::DarkGray).bg(Color::White)) // Grey text, white background
            .alignment(Alignment::Center);
        f.render_widget(notice_paragraph, chunks[4]);
    }

    pub async fn handle_input(
        &mut self,
        key: KeyCode,
        _modifiers: KeyModifiers,
        homepage: &mut Option<Homepage>,
    ) -> bool {
        if key == KeyCode::Esc {
            return true; // Signal to quit
        }

        match key {
            KeyCode::Tab => {
                self.active_field = (self.active_field + 1) % 2; // Cycle through input fields
            }
            KeyCode::BackTab => {
                self.active_field = if self.active_field == 0 {
                    1
                } else {
                    self.active_field - 1
                };
            }
            KeyCode::Enter => {
                self.submit(homepage).await;

                // Transition to homepage if login is successful
                if homepage.is_some() {
                    return true; // Signal that we should transition state
                }
            }
            _ => match self.active_field {
                0 => self.email.handle_input(key),
                1 => self.password.handle_input(key),
                _ => {}
            },
        }
        false
    }

    pub async fn submit(&mut self, homepage: &mut Option<Homepage>) {
        let client = Client::new();
        let login_data = SignupData {
            username: "_login".to_string(),
            email: self.email.content.clone(),
            password: self.password.content.clone(),
        };

        match client
            .post("http://0.0.0.0:8000/signup")
            .json(&login_data)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                let raw_body = response.text().await.unwrap_or_default();

                if status == 200 && raw_body.contains("Login successful") {
                    self.report_overview = get_report_overview(login_data.email.clone()).await;
                    let category_overview = get_category_overview(login_data.email.clone()).await;
                    let account_overview = get_account_overview(login_data.email.clone()).await;
                    if let Some(username) = raw_body.split_whitespace().next() {
                        *homepage = Some(Homepage::new(
                            username.to_string(),
                            self.email.content.clone(),
                            self.report_overview.clone(),
                            category_overview,
                            account_overview,
                        ));
                        self.response_message =
                            "Login successful! Redirecting to homepage...".to_string();
                    }
                } else if status == 400 {
                    self.response_message = "Invalid password. Please try again.".to_string();
                } else if status == 201 {
                    self.response_message =
                        "Email not registered. Please sign up first.".to_string();
                } else {
                    self.response_message =
                        format!("ERROR_CODE: {}\nMessage: {}", status, raw_body);
                }
            }
            Err(e) => {
                self.response_message = format!("Request failed: {}", e);
            }
        }
    }
}
