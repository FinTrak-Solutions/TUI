use crate::ui::report::create_lines;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct Homepage {
    pub username: String,
    pub email: String,
    pub report_overview: Vec<String>,
    pub category_overview: Vec<String>,
    pub account_overview: Vec<String>,
}

impl Homepage {
    pub fn new(
        username: String,
        email: String,
        report_overview: Vec<String>,
        category_overview: Vec<String>,
        account_overview: Vec<String>
    ) -> Self {
        Self {
            username,
            email,
            report_overview,
            category_overview,
            account_overview,
        }
    }

    pub fn render(&self, f: &mut Frame) {
        // Set white background for the whole page
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        // Split the frame into four vertical chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Greeting row
                    Constraint::Min(10),   // Main blocks (Accounts, Categories, Report)
                    Constraint::Length(5), // Navigation notice (with extra padding)
                ]
                .as_ref(),
            )
            .split(f.area());

        // Greeting: Welcome back <username> on the left
        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50), // Left side for greeting
                    Constraint::Percentage(50), // Centered HOMEPAGE
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        let greeting = format!("Welcome back, {}", self.username);
        let greeting_paragraph = Paragraph::new(greeting)
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .alignment(Alignment::Left);

        // HOMEPAGE title centered and bold
        let title = Paragraph::new("HOMEPAGE")
            .style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Right);

        // Render the greeting and title
        f.render_widget(greeting_paragraph, horizontal_layout[0]);
        f.render_widget(title, horizontal_layout[1]);

        // Main horizontal blocks: Accounts, Categories, Report
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33), // 33% width for Accounts
                    Constraint::Percentage(33), // 33% width for Categories
                    Constraint::Percentage(34), // 34% width for Report
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        // Accounts block (press 1 to jump) with account overview
        let accounts_block = Block::default().title("Accounts").borders(Borders::ALL);
        let accounts_paragraph = Paragraph::new(create_lines(self.account_overview.clone()))
            .wrap(Wrap { trim: true })
            .block(accounts_block);
        f.render_widget(accounts_paragraph, main_chunks[0]);

        // Categories block (press 2 to jump) with category overview
        let categories_block = Block::default().title("Categories").borders(Borders::ALL);
        let categories_paragraph = Paragraph::new(create_lines(self.category_overview.clone()))
            .wrap(Wrap { trim: true })
            .block(categories_block);
        f.render_widget(categories_paragraph, main_chunks[1]);

        // Report block (press 3 to jump) with report overview
        let report_block = Block::default().title("Report").borders(Borders::ALL);
        let report_paragraph = Paragraph::new(create_lines(self.report_overview.clone()))
            .wrap(Wrap { trim: true })
            .block(report_block);
        f.render_widget(report_paragraph, main_chunks[2]);

        // Bottom notice for navigation instructions (Esc to quit, etc.)
        let notice = Paragraph::new("Esc to quit | 1 to Account | 2 to Category | 3 to Report")
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[2]);

        // Additional notice for transaction creation
        let create_transaction_notice = Paragraph::new("Press 'n' to create a new transaction")
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(
            create_transaction_notice,
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)].as_ref())
                .split(chunks[2])[1],
        ); // Place it right below the first notice
    }
}
