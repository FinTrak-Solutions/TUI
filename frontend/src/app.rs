use crossterm::event::{self, Event, KeyCode};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::ui::{signup::SignupPage, cover::CoverPage, login::LoginPage, homepage::Homepage};

/// Enum to manage app state
pub enum State {
    Cover,      // Cover page
    Signup,     // Signup page
    Login,      // Login page
    Homepage,   // Homepage
}

pub struct App {
    pub state: State,           // Current page/state
    pub cover_page: CoverPage,  // Cover page
    pub signup_page: SignupPage, // Signup page
    pub login_page: LoginPage,  // Login page
    pub homepage: Option<Homepage>, // Homepage (initialized after successful login)
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Cover,
            cover_page: CoverPage::new(),
            signup_page: SignupPage::new(),
            login_page: LoginPage::new(),
            homepage: None, // Initially, homepage is not set
        }
    }
}

pub async fn run_app<B: ratatui::backend::Backend>(mut terminal: ratatui::Terminal<B>, app: Arc<Mutex<App>>) -> std::io::Result<()> {
    loop {
        {
            let app_guard = app.lock().await;
            terminal.draw(|f| {
                match app_guard.state {
                    State::Cover => app_guard.cover_page.render(f),
                    State::Signup => app_guard.signup_page.render(f),
                    State::Login => app_guard.login_page.render(f),
                    State::Homepage => {
                        if let Some(ref homepage) = app_guard.homepage {
                            homepage.render(f); // Render Homepage
                        }
                    }
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
                    match key_event.code {
                        KeyCode::Char('1') => app_guard.state = State::Signup,
                        KeyCode::Char('2') => app_guard.state = State::Login,
                        _ => {}
                    }
                }
                State::Signup => {
                    if app_guard.signup_page.handle_input(key_event.code, key_event.modifiers).await {
                        app_guard.state = State::Cover; // Return to Cover
                    }
                }
                State::Login => {
                    // Destructure app_guard to separate the login_page and homepage
                    let App {
                        state,
                        login_page,
                        homepage,
                        ..
                    } = &mut *app_guard;

                    if login_page.handle_input(key_event.code, key_event.modifiers, homepage).await {
                        if homepage.is_some() {
                            *state = State::Homepage; // Transition to Homepage
                        } else {
                            *state = State::Cover; // Return to Cover on Esc
                        }
                    }
                }
                State::Homepage => {
                    if key_event.code == KeyCode::Esc {
                        break; // Quit from Homepage
                    }
                }
            }
        }
    }
    Ok(())
}
