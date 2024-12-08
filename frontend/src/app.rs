use crossterm::event::{self, Event, KeyCode};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::ui::{signup::SignupPage, cover::CoverPage, login::LoginPage};

/// Enum to manage app state
pub enum State {
    Cover,      // Cover page
    Signup,     // Signup page
    Login,      // Login page
}

pub struct App {
    pub state: State,           // Current page/state
    pub cover_page: CoverPage,  // Cover page
    pub signup_page: SignupPage, // Signup page
    pub login_page: LoginPage,  // Login page
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Cover,
            cover_page: CoverPage::new(),
            signup_page: SignupPage::new(),
            login_page: LoginPage::new(),
        }
    }
}

pub async fn run_app<B: ratatui::backend::Backend>(mut terminal: ratatui::Terminal<B>, app: Arc<Mutex<App>>) -> std::io::Result<()> {
    loop {
        {
            let app_guard = app.lock().await;
            terminal.draw(|f| {
                match app_guard.state {
                    State::Cover => app_guard.cover_page.render(f), // Render Cover Page
                    State::Signup => app_guard.signup_page.render(f), // Render Signup Page
                    State::Login => app_guard.login_page.render(f), // Render Login Page
                }
            })?;
        }

        if let Event::Key(key_event) = event::read()? {
            let mut app_guard = app.lock().await;

            match app_guard.state {
                State::Cover => {
                    if app_guard.cover_page.handle_input(key_event.code).await {
                        break; // Quit on Esc
                    }

                    // Navigate to Signup or Login
                    match key_event.code {
                        KeyCode::Char('1') => app_guard.state = State::Signup, // Navigate to Signup
                        KeyCode::Char('2') => app_guard.state = State::Login,  // Navigate to Login
                        _ => {}
                    }
                }
                State::Signup => {
                    if app_guard.signup_page.handle_input(key_event.code, key_event.modifiers).await {
                        app_guard.state = State::Cover; // Navigate back to Cover on Esc
                    }
                }
                State::Login => {
                    if app_guard.login_page.handle_input(key_event.code, key_event.modifiers).await {
                        app_guard.state = State::Cover; // Navigate back to Cover on Esc
                    }
                }
            }
        }
    }
    Ok(())
}
