use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Frame,
};
use crossterm::event::{KeyCode, KeyModifiers};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Account {
    pub account_id: i32,
    pub email: String,
    pub account_type: String,
    pub account_name: String,
}

#[derive(Serialize)]
pub struct NewAccount {
    pub email: String,
    pub account_type: String,
    pub account_name: String,
}

pub struct AccountMain {
    accounts: Vec<Account>,
    list_state: ListState,
    email: String,
    message: String,
    creating_account: bool,
    new_account: NewAccount,
    active_field: usize,
    client: Client,
}

impl AccountMain {
    pub fn new(email: String) -> Self {
        let mut instance = Self {
            accounts: Vec::new(),
            list_state: ListState::default(),
            email: email.clone(),
            message: String::new(),
            creating_account: false,
            new_account: NewAccount {
                email,
                account_type: String::new(),
                account_name: String::new(),
            },
            active_field: 0,
            client: Client::new(),
        };

        // Manually call fetch_accounts since we can't use async in new
        instance.message = "Loading accounts...".to_string();
        instance
    }

    pub fn render(&mut self, f: &mut Frame) {
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(10),    // Content
                Constraint::Length(3),  // Message/Status
                Constraint::Length(3),  // Navigation help
            ].as_ref())
            .split(f.area());

        let title = Paragraph::new("ACCOUNT MANAGEMENT")
            .style(Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        if self.creating_account {
            self.render_create_account(f, chunks[1]);
        } else {
            self.render_account_list(f, chunks[1]);
        }

        let message_style = if self.message.contains("Error") || self.message.contains("Failed") {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };
        let message = Paragraph::new(self.message.clone())
            .style(message_style)
            .alignment(Alignment::Center);
        f.render_widget(message, chunks[2]);

        let help_text = if self.creating_account {
            "ESC: Back | TAB: Next Field | ENTER: Submit"
        } else {
            "ESC: Back | N: New Account | D: Delete Account | ↑↓: Navigate"
        };
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[3]);
    }

    fn render_account_list(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.accounts
            .iter()
            .map(|account| {
                ListItem::new(format!(
                    "{}: {}",
                    account.account_name,
                    account.account_type
                ))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Black))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow));

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_create_account(&self, f: &mut Frame, area: Rect) {
        let create_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Account Name
                Constraint::Length(3),  // Account Type
            ].as_ref())
            .split(area);

        let name_block = Block::default()
            .title("Account Name")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.active_field == 0 { Color::Yellow } else { Color::Black }));
        let name_text = Paragraph::new(self.new_account.account_name.clone())
            .style(Style::default().fg(Color::Black));
        f.render_widget(name_text.block(name_block), create_chunks[0]);

        let type_block = Block::default()
            .title("Account Type (credit/debit)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if self.active_field == 1 { Color::Yellow } else { Color::Black }));
        let type_text = Paragraph::new(self.new_account.account_type.clone())
            .style(Style::default().fg(Color::Black));
        f.render_widget(type_text.block(type_block), create_chunks[1]);
    }

    pub async fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> bool {
        if key == KeyCode::Esc {
            if self.creating_account {
                self.creating_account = false;
                return false;
            }
            return true;
        }

        if self.creating_account {
            self.handle_create_input(key).await;
        } else {
            self.handle_list_input(key).await;
        }
        false
    }


    async fn handle_create_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.active_field = (self.active_field + 1) % 2;
            }
            KeyCode::Enter => {
                self.submit_new_account().await;
            }
            KeyCode::Char(c) => {
                match self.active_field {
                    0 => self.new_account.account_name.push(c),
                    1 => self.new_account.account_type.push(c),
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                match self.active_field {
                    0 => { self.new_account.account_name.pop(); }
                    1 => { self.new_account.account_type.pop(); }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    async fn handle_list_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('n') => {
                self.creating_account = true;
                self.new_account.account_name.clear();
                self.new_account.account_type.clear();
                self.active_field = 0;
            }
            KeyCode::Char('d') => {
                if let Some(selected) = self.list_state.selected() {
                    if selected < self.accounts.len() {
                        let account_name = self.accounts[selected].account_name.clone();
                        self.delete_account(&account_name).await;
                    }
                }
            }
            KeyCode::Up => {
                let selected = self.list_state.selected().unwrap_or(0);
                if !self.accounts.is_empty() {
                    self.list_state.select(Some(
                        if selected == 0 { self.accounts.len() - 1 } else { selected - 1 }
                    ));
                }
            }
            KeyCode::Down => {
                let selected = self.list_state.selected().unwrap_or(0);
                if !self.accounts.is_empty() {
                    self.list_state.select(Some(
                        if selected >= self.accounts.len() - 1 { 0 } else { selected + 1 }
                    ));
                }
            }
            _ => {}
        }
    }

    pub async fn initialize(&mut self) {
        self.fetch_accounts().await;
    }

    async fn fetch_accounts(&mut self) {
        let url = format!("http://localhost:8000/account_summary?email={}", self.email);
        match self.client.get(&url).send().await {
            Ok(response) => {
                match response.status() {
                    reqwest::StatusCode::OK => {
                        if let Ok(accounts) = response.json::<Vec<Account>>().await {
                            self.accounts = accounts;
                            if !self.accounts.is_empty() && self.list_state.selected().is_none() {
                                self.list_state.select(Some(0));
                            }
                            self.message = format!("Loaded {} accounts", self.accounts.len());
                        } else {
                            self.message = "Failed to parse account data".to_string();
                        }
                    }
                    _ => {
                        self.message = "Failed to fetch accounts".to_string();
                    }
                }
            }
            Err(e) => {
                self.message = format!("Error fetching accounts: {}", e);
            }
        }
    }

    async fn submit_new_account(&mut self) {
        if self.new_account.account_name.is_empty() || self.new_account.account_type.is_empty() {
            self.message = "Please fill in all fields".to_string();
            return;
        }

        match self.client
            .post("http://localhost:8000/account_create")
            .json(&self.new_account)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                let message = response.text().await.unwrap_or_default();

                match status {
                    reqwest::StatusCode::CREATED => {
                        self.message = "Account created successfully".to_string();
                        self.creating_account = false;
                        self.fetch_accounts().await;
                    }
                    reqwest::StatusCode::BAD_REQUEST => {
                        self.message = message;
                    }
                    _ => {
                        self.message = format!("Failed to create account: {}", message);
                    }
                }
            }
            Err(e) => {
                self.message = format!("Error creating account: {}", e);
            }
        }
    }

    async fn delete_account(&mut self, account_name: &str) {
        let url = format!(
            "http://localhost:8000/delete_account?email={}&account_name={}",
            self.email, account_name
        );

        match self.client.delete(&url).send().await {
            Ok(response) => {
                let status = response.status();
                let message = response.text().await.unwrap_or_default();

                match status {
                    reqwest::StatusCode::OK => {
                        self.message = "Account deleted successfully".to_string();
                        self.fetch_accounts().await;
                    }
                    _ => {
                        self.message = format!("Failed to delete account: {}", message);
                    }
                }
            }
            Err(e) => {
                self.message = format!("Error deleting account: {}", e);
            }
        }
    }
}