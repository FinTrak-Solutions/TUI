mod app;
mod ui;

use app::{run_app, App};
use crossterm::{execute, terminal::{self, Clear, ClearType}};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    // Enable raw mode for TUI interaction
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(&mut stdout);
    let terminal = Terminal::new(backend)?;
    let app = App::new();

    // Run the TUI app
    let result = run_app(terminal, app).await;

    // Disable raw mode before clearing the screen
    terminal::disable_raw_mode()?;

    // Clear the screen after the TUI ends
    execute!(stdout, Clear(ClearType::All))?;

    result
}