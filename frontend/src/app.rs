#[allow(unused_imports)]
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::ui::signup::SignupPage;

pub struct App {
    pub signup_page: SignupPage,
}

impl App {
    pub fn new() -> Self {
        Self {
            signup_page: SignupPage::new(),
        }
    }
}

pub async fn run_app<B: ratatui::backend::Backend>(mut terminal: ratatui::Terminal<B>, app: Arc<Mutex<App>>) -> std::io::Result<()> {
    loop {
        {
            let app_guard = app.lock().await;
            terminal.draw(|f| {
                app_guard.signup_page.render(f);
            })?;
        }

        if let Event::Key(key_event) = event::read()? {
            let mut app_guard = app.lock().await;
            if app_guard.signup_page.handle_input(key_event.code, key_event.modifiers).await {
                break; // Quit the application if Esc is pressed
            }
        }
    }
    Ok(())
}
