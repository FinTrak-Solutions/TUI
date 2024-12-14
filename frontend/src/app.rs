use crate::ui::report::{get_report_overview, get_category_overview, get_account_overview};
use crate::ui::transaction_create::TransactionCreate;
use crate::ui::{
    account_main::AccountMain, category_main::CategoryMain, cover::CoverPage, homepage::Homepage,
    login::LoginPage, report_main::ReportMain, signup::SignupPage,
};
#[allow(unused_imports)]
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
#[allow(unused_imports)]
use std::time::Duration;

pub enum State {
    Cover,             // Cover page
    Signup,            // Signup page
    Login,             // Login page
    Homepage,          // Homepage
    AccountMain,       // Account Main page
    CategoryMain,      // Category Main page
    ReportMain,        // Report page
    TransactionCreate, // Transaction Create page
}

pub struct App {
    pub state: State,                                  // Current page/state
    pub cover_page: CoverPage,                         // Cover page
    pub signup_page: SignupPage,                       // Signup page
    pub login_page: LoginPage,                         // Login page
    pub homepage: Option<Homepage>, // Homepage (initialized after successful login)
    pub account_main: Option<AccountMain>, // Account Main (accessed from homepage)
    pub category_main: Option<CategoryMain>, // Category Main (accessed from homepage)
    pub report_main: Option<ReportMain>, // Report page (accessed from homepage)
    pub transaction_create: Option<TransactionCreate>, // Transaction Create page
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Cover,
            cover_page: CoverPage::new(),
            signup_page: SignupPage::new(),
            login_page: LoginPage::new(),
            homepage: None,           // Initially, homepage is not set
            account_main: None,       // Initially, account_main is not set
            category_main: None,      // Initially, category_main is not set
            report_main: None,        // Initially, report page is not set
            transaction_create: None, // Initially, transaction_create is not set
        }
    }
}

pub async fn run_app<B: ratatui::backend::Backend>(
    mut terminal: ratatui::Terminal<B>,
    mut app: App,
) -> std::io::Result<()> {
    loop {
        // Render the current state of the app
        // before rendering, load the overview everytime
        if let Some(ref mut curr_homepage) = app.homepage {
            curr_homepage.report_overview = get_report_overview(curr_homepage.email.clone()).await;
            curr_homepage.category_overview = get_category_overview(curr_homepage.email.clone()).await;
            curr_homepage.account_overview = get_account_overview(curr_homepage.email.clone()).await;
        }
        terminal.draw(|f| match app.state {
            State::Cover => app.cover_page.render(f),
            State::Signup => app.signup_page.render(f),
            State::Login => app.login_page.render(f),
            State::Homepage => {
                if let Some(ref homepage) = app.homepage {
                    homepage.render(f);
                }
            }
            State::AccountMain => {
                if let Some(ref mut account_main) = app.account_main {
                    account_main.render(f);
                }
            }
            State::CategoryMain => {
                if let Some(ref mut category_main) = app.category_main {
                    category_main.render(f);
                }
            }
            State::ReportMain => {
                if let Some(ref mut report_main) = app.report_main {
                    report_main.render(f);
                }
            }
            State::TransactionCreate => {
                if let Some(ref transaction_create) = app.transaction_create {
                    transaction_create.render(f);
                }
            }
        })?;

        // Handle user input (outside of draw)
        if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
            match app.state {
                State::Cover => {
                    if key_event.code == KeyCode::Esc {
                        break; // Quit on Esc
                    }
                    match key_event.code {
                        KeyCode::Char('1') => app.state = State::Signup,
                        KeyCode::Char('2') => app.state = State::Login,
                        _ => {}
                    }
                }
                State::Signup => {
                    if key_event.code == KeyCode::Esc {
                        app.state = State::Cover; // Return to Cover when Esc is pressed
                    } else if app
                        .signup_page
                        .handle_input(key_event.code, key_event.modifiers)
                        .await
                    {
                        app.state = State::Login; // After successful signup, go to Login
                    }
                }
                State::Login => {
                    if key_event.code == KeyCode::Esc {
                        app.state = State::Cover; // Return to Cover on Esc
                    } else if app
                        .login_page
                        .handle_input(key_event.code, key_event.modifiers, &mut app.homepage)
                        .await
                    {
                        if app.homepage.is_some() {
                            app.state = State::Homepage; // Transition to Homepage
                        }
                    }
                }
                State::Homepage => {
                    if key_event.code == KeyCode::Esc {
                        break; // Quit from Homepage
                    }
                    if let Some(ref homepage) = app.homepage {
                        match key_event.code {
                            KeyCode::Char('1') => {
                                let mut account_main = AccountMain::new(homepage.email.clone());
                                account_main.initialize().await;
                                app.account_main = Some(account_main);
                                app.state = State::AccountMain;
                            }
                            KeyCode::Char('2') => {
                                let mut category_main = CategoryMain::new(homepage.email.clone());
                                category_main.initialize().await;
                                app.category_main = Some(category_main);
                                app.state = State::CategoryMain;
                            }
                            KeyCode::Char('3') => {
                                let mut report_main = ReportMain::new(homepage.email.clone());
                                report_main.initialize().await;
                                app.report_main = Some(report_main);
                                app.state = State::ReportMain;
                            }
                            KeyCode::Char('n') => {
                                app.transaction_create =
                                    Some(TransactionCreate::new(homepage.email.clone()));
                                app.state = State::TransactionCreate;
                            }
                            _ => {}
                        }
                    }
                }
                State::AccountMain => {
                    if let Some(ref mut account_main) = app.account_main {
                        if account_main
                            .handle_input(key_event.code, key_event.modifiers)
                            .await
                        {
                            app.state = State::Homepage;
                        }
                    }
                }
                State::CategoryMain => {
                    if let Some(ref mut category_main) = app.category_main {
                        if category_main
                            .handle_input(key_event.code, key_event.modifiers)
                            .await
                        {
                            app.state = State::Homepage;
                        }
                    }
                }
                State::ReportMain => {
                    if let Some(ref mut report_main) = app.report_main {
                        if report_main
                            .handle_input(key_event.code, key_event.modifiers)
                            .await
                        {
                            app.state = State::Homepage;
                        }
                    }
                }
                State::TransactionCreate => {
                    if key_event.code == KeyCode::Esc {
                        app.state = State::Homepage; // Return to Homepage on Esc
                    }
                    if let Some(ref mut transaction_create) = app.transaction_create {
                        if transaction_create
                            .handle_input(key_event.code, key_event.modifiers)
                            .await
                        {
                            app.state = State::Homepage; // Return to Homepage after transaction create
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
