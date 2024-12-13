use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct Homepage {
    pub username: String,
    #[allow(dead_code)]
    pub email: String,
    pub report_overview: String,
}

impl Homepage {
    pub fn new(username: String, email: String, report_overview: String) -> Self {
        Self {
            username,
            email,
            report_overview,
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
                    Constraint::Length(3), // Greeting row
                    Constraint::Min(10),   // Main blocks
                    Constraint::Length(3), // Notice
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

        f.render_widget(greeting_paragraph, horizontal_layout[0]);
        f.render_widget(title, horizontal_layout[1]);

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

        // add report overview details
        let report_block = Block::default().title("Report").borders(Borders::ALL);
        let report_paragraph = Paragraph::new(self.report_overview.clone())
            .wrap(Wrap { trim: true })
            .block(report_block);
        f.render_widget(report_paragraph, main_chunks[2]);

        // Bottom notice
        let notice = Paragraph::new("Esc to quit | 1 to Account | 2 to Category | 3 to Report")
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[2]);
    }
}
