use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
    style::{Color, Modifier, Style},
    Frame,
};

pub struct AccountMain;

impl AccountMain {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame) {
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ]
                    .as_ref(),
            )
            .split(f.area());

        let title = Paragraph::new("ACCOUNTS")
            .style(Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);

        f.render_widget(title, chunks[0]);
    }
}