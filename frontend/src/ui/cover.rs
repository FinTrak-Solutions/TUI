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
        // Calculate vertical space dynamically to center the logo
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20), // Top margin to center logo
                    Constraint::Length(10),     // Logo height (adjust as needed)
                    Constraint::Percentage(65), // Bottom margin
                    Constraint::Length(3),      // Notice area
                ]
                    .as_ref(),
            )
            .split(f.area());

        // ASCII Logo for FinTracker
        // Generated using https://patorjk.com/software/taag/#p=testall&h=2&v=1&f=Standard&t=FinTrack
        let ascii_logo = r#"
 ________ ___  ________   _________  ________  ________  ________  ___  __
|\  _____\\  \|\   ___  \|\___   ___\\   __  \|\   __  \|\   ____\|\  \|\  \
 \ \  \__/\ \  \ \  \\ \  \|___ \  \_\ \  \|\  \ \  \|\  \ \  \___|\ \  \/  /|_
   \ \   __\\ \  \ \  \\ \  \   \ \  \ \ \   _  _\ \   __  \ \  \    \ \   ___  \
      \ \  \_| \ \  \ \  \\ \  \   \ \  \ \ \  \\  \\ \  \ \  \ \  \____\ \  \\ \  \
        \ \__\   \ \__\ \__\\ \__\   \ \__\ \ \__\\ _\\ \__\ \__\ \_______\ \__\\ \__\
         \|__|    \|__|\|__| \|__|    \|__|  \|__|\|__|\|__|\|__|\|_______|\|__| \|__|
"#;

        // Render the logo in the center (vertically and horizontally)
        let logo_paragraph = Paragraph::new(ascii_logo)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(logo_paragraph, chunks[1]);

        // Render the notice at the bottom
        let notice = Paragraph::new("Esc to quit | 1 to signup | 2 to login")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[3]);
    }

    pub async fn handle_input(&self, key: KeyCode) -> bool {
        // Quit on Esc
        if key == KeyCode::Esc {
            return true;
        }
        false
    }
}
