use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style},
    Frame,
};

pub struct Homepage {
    pub username: String,
    #[allow(dead_code)]
    pub email: String,
}

impl Homepage {
    pub fn new(username: String, email: String) -> Self {
        Self { username, email }
    }

    pub fn render(&self, f: &mut Frame) {
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),  // Greeting
                    Constraint::Min(10),   // Main blocks
                    Constraint::Length(3), // Notice
                ]
                    .as_ref(),
            )
            .split(f.area());

        // Greeting: Welcome back <username>
        let greeting = format!("Welcome back, {}", self.username);
        let greeting_paragraph = Paragraph::new(greeting)
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .alignment(Alignment::Left);
        f.render_widget(greeting_paragraph, chunks[0]);

        // Main horizontal blocks: Accounts, Categories, Report
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ]
                    .as_ref(),
            )
            .split(chunks[1]);

        let accounts_block = Block::default().title("Accounts").borders(Borders::ALL);
        f.render_widget(accounts_block, main_chunks[0]);

        let categories_block = Block::default().title("Categories").borders(Borders::ALL);
        f.render_widget(categories_block, main_chunks[1]);

        let report_block = Block::default().title("Report").borders(Borders::ALL);
        f.render_widget(report_block, main_chunks[2]);

        // Bottom notice
        let notice = Paragraph::new("Esc to quit | Use arrow keys to navigate")
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[2]);
    }
}
