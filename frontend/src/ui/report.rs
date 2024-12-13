use reqwest::Client;

pub async fn get_report_overview(user_email: String) -> String {
    let client = Client::new();
    let overview_url = format!(
        "http://0.0.0.0:8000/report_overview?email={}",
        user_email.clone().as_str()
    );
    match client.get(overview_url).send().await {
        Ok(response) => {
            let status = response.status();
            let raw_body = response.text().await.unwrap_or_default();

            if status.is_success() {
                return raw_body;
            } else {
                return "Error querying report overview!".to_string();
            }
        }
        Err(_) => {
            return "Error querying report overview!".to_string();
        }
    }
}
