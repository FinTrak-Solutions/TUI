use ratatui::{style::Stylize, text::Line};
use reqwest::Client;

pub async fn get_account_overview(user_email: String) -> Vec<String> {
    let client = Client::new();
    let url = format!(
        "http://localhost:8000/account_summary?email={}",
        user_email
    );

    match client.get(&url).send().await {
        Ok(response) => {
            match response.status() {
                reqwest::StatusCode::OK => {
                    if let Ok(accounts) = response.json::<Vec<crate::ui::account_main::Account>>().await {
                        accounts.iter().map(|acc| {
                            format!(
                                "{}: {}",
                                acc.account_name,
                                acc.account_type
                            )
                        }).collect()
                    } else {
                        vec!["Error parsing account data".to_string()]
                    }
                }
                _ => vec!["Failed to fetch accounts".to_string()]
            }
        }
        Err(_) => vec!["Error connecting to server".to_string()]
    }
}

pub async fn get_category_overview(user_email: String) -> Vec<String> {
    let client = Client::new();
    let url = format!(
        "http://localhost:8000/category_summary?email={}",
        user_email
    );

    match client.get(&url).send().await {
        Ok(response) => {
            match response.status() {
                reqwest::StatusCode::OK => {
                    if let Ok(categories) = response.json::<Vec<crate::ui::category_main::Category>>().await {
                        categories.iter().map(|cat| {
                            format!(
                                "{}: {} (Budget: ${} {})",
                                cat.nickname,
                                cat.category_type,
                                cat.budget,
                                cat.budget_freq
                            )
                        }).collect()
                    } else {
                        vec!["Error parsing category data".to_string()]
                    }
                }
                _ => vec!["Failed to fetch categories".to_string()]
            }
        }
        Err(_) => vec!["Error connecting to server".to_string()]
    }
}

pub async fn get_report_overview(user_email: String) -> Vec<String> {
    let client = Client::new();
    let overview_url = format!(
        "http://0.0.0.0:8000/report_overview?email={}",
        user_email.clone().as_str()
    );
    match client.get(overview_url).send().await {
        Ok(response) => {
            let status = response.status();
            let raw_body = response.text().await.unwrap_or_default();
            let body_vec: Vec<String> = serde_json::from_str(raw_body.as_str()).unwrap();

            if status.is_success() {
                return body_vec;
            } else {
                return vec!["Error querying report overview!".to_string()];
            }
        }
        Err(_) => {
            return vec!["Error querying report overview!".to_string()];
        }
    }
}

// helper function to render summary tab
/// Create some lines to display in the paragraph.
pub fn create_lines(summary_lines: Vec<String>) -> Vec<Line<'static>> {
    let mut formatted_lines: Vec<Line<'static>> = vec![];

    for line in summary_lines.into_iter() {
        if line.contains("Category Summary:") {
            formatted_lines.push(Line::raw(line.clone()).magenta().on_white().bold());
        } else if line.contains("Account Summary:") {
            formatted_lines.push(Line::raw(line.clone()).light_blue().on_white().bold());
        } else {
            formatted_lines.push(Line::raw(line.clone()).black());
        }
    }
    formatted_lines
}
