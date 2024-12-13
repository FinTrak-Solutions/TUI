use crossterm::event::KeyCode;
#[allow(unused_imports)]
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
    style::{Color, Style},
    Frame,
};

pub struct CoverPage;

impl CoverPage {
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
                    Constraint::Percentage(20),
                    Constraint::Length(10),
                    Constraint::Percentage(65),
                    Constraint::Length(3),
                ]
                    .as_ref(),
            )
            .split(f.area());

        let ascii_logo = r#"
 ________ ___  ________   _________  ________  ________  ________  ___  __
|\  _____\\  \|\   ___  \|\___   ___\\   __  \|\   __  \|\   ____\|\  \|\  \
 \ \  \__/\ \  \ \  \\ \  \|___ \  \_\ \  \|\  \ \  \|\  \ \  \___|\ \  \/  /|_
   \ \   __\\ \  \ \  \\ \  \   \ \  \ \ \   _  _\ \   __  \ \  \    \ \   ___  \
      \ \  \_| \ \  \ \  \\ \  \   \ \  \ \ \  \\  \\ \  \ \  \ \  \____\ \  \\ \  \
        \ \__\   \ \__\ \__\\ \__\   \ \__\ \ \__\\ _\\ \__\ \__\ \_______\ \__\\ \__\
         \|__|    \|__|\|__| \|__|    \|__|  \|__|\|__|\|__|\|__|\|_______|\|__| \|__|
"#;

        let logo_paragraph = Paragraph::new(ascii_logo)
            .style(Style::default().fg(Color::Yellow).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(logo_paragraph, chunks[1]);

        let notice = Paragraph::new("Esc to quit | 1 to signup | 2 to login")
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[3]);
    }


    #[allow(dead_code)]
    pub async fn handle_input(&self, key: KeyCode) -> bool {
        // Quit on Esc
        if key == KeyCode::Esc {
            return true;
        }
        false
    }
}
