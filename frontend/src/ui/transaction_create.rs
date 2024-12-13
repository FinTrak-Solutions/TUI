use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};
use crossterm::event::{KeyCode, KeyModifiers};
use reqwest::Client;
use serde::Serialize;

use crate::ui::components::InputField;

#[derive(Serialize)]
struct TransactionData {
    email: String,
    category_name: String,
    amount: f64,
    notes: String,
    account_name: String,
}

pub struct TransactionCreate {
    pub category_name: InputField,
    pub amount: InputField,
    pub account_name: InputField,
    pub notes: InputField,
    pub active_field: usize,
    pub response_message: String,
    pub email: String, // The email is passed from the homepage
}

impl TransactionCreate {
    pub fn new(email: String) -> Self {
        Self {
            category_name: InputField::new("Category Name", false),
            amount: InputField::new("Amount", false),
            account_name: InputField::new("Account Name", false),
            notes: InputField::new("Notes", false),
            active_field: 0,
            response_message: String::new(),
            email,
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
                    Constraint::Length(8),   // Title section
                    Constraint::Length(3),   // Category Name
                    Constraint::Length(3),   // Amount
                    Constraint::Length(3),   // Account Name
                    Constraint::Length(3),   // Notes (larger)
                    Constraint::Min(3),      // Response message
                    Constraint::Length(3),   // Navigation notice
                ]
                    .as_ref(),
            )
            .split(f.area());

        // Title
        let title = Paragraph::new("CREATE NEW TRANSACTION")
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        // Input fields
        self.category_name.render(f, chunks[1], self.active_field == 0);
        self.amount.render(f, chunks[2], self.active_field == 1);
        self.account_name.render(f, chunks[3], self.active_field == 2);
        self.notes.render(f, chunks[4], self.active_field == 3);

        // Response message
        let response_paragraph = Paragraph::new(self.response_message.clone())
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(response_paragraph, chunks[5]);

        // Bottom notice
        let notice_text = "Esc to quit | Enter to submit";
        let notice_paragraph = Paragraph::new(notice_text)
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice_paragraph, chunks[6]);
    }

    pub async fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> bool {
        if key == KeyCode::Esc {
            return true; // Return to homepage
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
                    0 => self.category_name.handle_input(key),
                    1 => self.amount.handle_input(key),
                    2 => self.account_name.handle_input(key),
                    3 => self.notes.handle_input(key),
                    _ => {}
                }
            }
        }
        false
    }

    pub async fn submit(&mut self) -> bool {
        let client = Client::new();
        let transaction_data = TransactionData {
            email: self.email.clone(),
            category_name: self.category_name.content.clone(),
            amount: self.amount.content.parse().unwrap_or(0.0),
            notes: self.notes.content.clone(),
            account_name: self.account_name.content.clone(),
        };

        match client.post("http://0.0.0.0:8000/add_trans")
            .json(&transaction_data)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to parse response body".to_string());

                if status == 201 {
                    self.response_message = "Transaction successfully created!".to_string();
                    return true; // Success, return to homepage
                } else {
                    self.response_message = format!("ERROR_CODE: {}\nMessage: {}", status, message);
                }
            }
            Err(e) => {
                self.response_message = format!("Request failed: {}", e);
            }
        }
        false
    }
}
