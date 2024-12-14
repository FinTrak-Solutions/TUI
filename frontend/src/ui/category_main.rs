use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    #[allow(dead_code)]
    pub email: String,
    pub nickname: String,
    pub category_type: String,
    pub budget: f64,
    pub budget_freq: String,
}

#[derive(Serialize, Debug)]
pub struct NewCategory {
    pub email: String,
    pub nickname: String,
    pub category_type: String,
    pub budget: f64,
    pub budget_freq: String,
}

pub struct CategoryMain {
    categories: Vec<Category>,
    list_state: ListState,
    email: String,
    message: String,
    creating_category: bool,
    active_field: usize,
    client: Client,
    input_strings: [String; 5],
    last_operation_nickname: Option<String>,
}

impl CategoryMain {
    pub fn new(email: String) -> Self {
        let mut instance = Self {
            categories: Vec::new(),
            list_state: ListState::default(),
            email: email.clone(),
            message: String::new(),
            creating_category: false,
            active_field: 0,
            client: Client::new(),
            input_strings: Default::default(),
            last_operation_nickname: None,
        };

        instance.message = "Loading categories...".to_string();
        instance
    }

    pub fn render(&mut self, f: &mut Frame) {
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Title
                    Constraint::Min(10),   // Content
                    Constraint::Length(3), // Message/Status
                    Constraint::Length(3), // Navigation help
                ]
                .as_ref(),
            )
            .split(f.area());

        let title = Paragraph::new("CATEGORY MANAGEMENT")
            .style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        if self.creating_category {
            self.render_create_category(f, chunks[1]);
        } else {
            self.render_category_list(f, chunks[1]);
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

        let help_text = if self.creating_category {
            "ESC: Back | TAB: Next Field | ENTER: Submit"
        } else {
            "ESC: Back | N: New Category | D: Delete Category | U: Update Category | ↑↓: Navigate"
        };
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[3]);
    }

    fn render_category_list(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .categories
            .iter()
            .map(|category| {
                ListItem::new(format!(
                    "{}: {} (Budget: ${} {})",
                    category.nickname,
                    category.category_type,
                    category.budget,
                    category.budget_freq
                ))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Black))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            );

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_create_category(&self, f: &mut Frame, area: Rect) {
        let create_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Nickname
                    Constraint::Length(3), // Category Type
                    Constraint::Length(3), // Budget
                    Constraint::Length(3), // Budget Frequency
                ]
                .as_ref(),
            )
            .split(area);

        let fields = [
            ("Nickname", &self.input_strings[0]),
            ("Category Type", &self.input_strings[1]),
            ("Budget", &self.input_strings[2]),
            (
                "Budget Frequency (daily/weekly/monthly)",
                &self.input_strings[3],
            ),
        ];

        for (i, (title, content)) in fields.iter().enumerate() {
            let block = Block::default()
                .title(*title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if self.active_field == i {
                    Color::Yellow
                } else {
                    Color::Black
                }));
            let text = Paragraph::new(content.to_string()).style(Style::default().fg(Color::Black));
            f.render_widget(text.block(block), create_chunks[i]);
        }
    }

    pub async fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> bool {
        if key == KeyCode::Esc {
            if self.creating_category {
                self.creating_category = false;
                return false;
            }
            return true;
        }

        if self.creating_category {
            self.handle_create_input(key).await;
        } else {
            self.handle_list_input(key).await;
        }
        false
    }

    async fn handle_create_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.active_field = (self.active_field + 1) % 4; // Only cycle through 4 fields
            }
            KeyCode::Enter => {
                self.submit_new_category().await;
            }
            KeyCode::Char(c) => {
                if self.active_field < 4 {
                    // Only allow input for first 4 fields
                    self.input_strings[self.active_field].push(c);
                }
            }
            KeyCode::Backspace => {
                if self.active_field < 4 {
                    // Only allow deletion for first 4 fields
                    self.input_strings[self.active_field].pop();
                }
            }
            _ => {}
        }
    }

    async fn handle_list_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('n') => {
                self.creating_category = true;
                self.input_strings = Default::default();
                self.active_field = 0;
            }
            KeyCode::Char('d') => {
                if let Some(selected) = self.list_state.selected() {
                    if selected < self.categories.len() {
                        let nickname = self.categories[selected].nickname.clone();
                        self.delete_category(&nickname).await;
                    }
                }
            }
            KeyCode::Char('u') => {
                // delete first
                if let Some(selected) = self.list_state.selected() {
                    if selected < self.categories.len() {
                        let nickname = self.categories[selected].nickname.clone();
                        self.delete_category(&nickname).await;
                    }
                }
                // then create new lol
                self.creating_category = true;
                self.input_strings = Default::default();
                // self.active_field = 0;
            }
            KeyCode::Up => {
                let selected = self.list_state.selected().unwrap_or(0);
                if !self.categories.is_empty() {
                    self.list_state.select(Some(if selected == 0 {
                        self.categories.len() - 1
                    } else {
                        selected - 1
                    }));
                }
            }
            KeyCode::Down => {
                let selected = self.list_state.selected().unwrap_or(0);
                if !self.categories.is_empty() {
                    self.list_state
                        .select(Some(if selected >= self.categories.len() - 1 {
                            0
                        } else {
                            selected + 1
                        }));
                }
            }
            _ => {}
        }
    }

    pub async fn initialize(&mut self) {
        self.fetch_categories().await;
    }

    async fn fetch_categories(&mut self) {
        let url = format!(
            "http://localhost:8000/category_summary?email={}",
            self.email
        );
        match self.client.get(&url).send().await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => {
                    if let Ok(categories) = response.json::<Vec<Category>>().await {
                        self.categories = categories;

                        // If we have a last operated category, find and select it
                        if let Some(ref nickname) = self.last_operation_nickname {
                            if let Some(index) = self.categories.iter().position(|c| &c.nickname == nickname) {
                                self.list_state.select(Some(index));
                            }
                        } else if !self.categories.is_empty() && self.list_state.selected().is_none() {
                            self.list_state.select(Some(0));
                        }

                        self.message = format!("Loaded {} categories", self.categories.len());
                        self.last_operation_nickname = None; // Clear the last operation
                    } else {
                        self.message = "Failed to parse category data".to_string();
                    }
                }
                _ => {
                    self.message = "Failed to fetch categories".to_string();
                }
            },
            Err(e) => {
                self.message = format!("Error fetching categories: {}", e);
            }
        }
    }

    async fn submit_new_category(&mut self) {
        // Only check the first 4 fields that we actually use
        if self.input_strings[..4].iter().any(|s| s.is_empty()) {
            self.message = "Please fill in all fields".to_string();
            return;
        }

        // Parse budget value
        let budget = match self.input_strings[2].parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                self.message = "Invalid budget value".to_string();
                return;
            }
        };

        // Store the nickname for later selection
        let nickname = self.input_strings[0].clone();
        self.last_operation_nickname = Some(nickname.clone());

        // Create new category
        let new_category = NewCategory {
            email: self.email.clone(),
            nickname: self.input_strings[0].clone(),
            category_type: self.input_strings[1].clone(),
            budget,
            budget_freq: self.input_strings[3].clone(),
        };

        match self
            .client
            .post("http://localhost:8000/category_create")
            .json(&new_category)
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();

                let message = response.text().await.unwrap_or_default();

                match status {
                    reqwest::StatusCode::CREATED => {
                        self.message = "Category created successfully".to_string();
                        self.creating_category = false;
                        self.input_strings = Default::default();
                        self.active_field = 0;
                        self.fetch_categories().await;
                    }
                    reqwest::StatusCode::BAD_REQUEST => {
                        self.message = message;
                    }
                    _ => {
                        self.message = format!("Failed to create category: {}", message);
                    }
                }
            }
            Err(e) => {
                self.message = format!("Error creating category: {}", e);
            }
        }
    }

    async fn delete_category(&mut self, nickname: &str) {
        let url = format!(
            "http://localhost:8000/delete_category?email={}&category_nickname={}",
            self.email, nickname
        );

        match self.client.delete(&url).send().await {
            Ok(response) => {
                let status = response.status();
                let message = response.text().await.unwrap_or_default();

                match status {
                    reqwest::StatusCode::OK => {
                        self.message = "Category deleted successfully".to_string();
                        self.fetch_categories().await;
                    }
                    _ => {
                        self.message = format!("Failed to delete category: {}", message);
                    }
                }
            }
            Err(e) => {
                self.message = format!("Error deleting category: {}", e);
            }
        }
    }
}
