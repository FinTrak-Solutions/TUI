#[allow(unused_imports)]
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Masked, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
//use serde_json::json;

#[derive(Deserialize, Debug, Serialize)]
pub struct CategorySummary {
    pub nickname: String,
    pub budget: f64,
    pub budget_freq: String,
    pub overbudget: bool,
    pub total: f64,
    // a vector of all the relevant transactions within budget freq frame
    pub cat_trans: Vec<String>,
}

// https://ratatui.rs/examples/widgets/block/
// Create a bordered block with a title.
fn title_block(cat_name: &str, overbudget: bool) -> Block {
    match overbudget {
        false => Block::bordered()
            .title(cat_name.blue().on_white().bold())
            .title("OK".green().on_white().bold()),
        true => Block::bordered()
            .title(cat_name.blue().on_white().bold())
            .title("OVERBUDGET!!".red().on_white().bold()),
    }
}

fn generate_report_block(
    budget: f64,
    budget_freq: String,
    spent: f64,
    txns: Vec<String>,
) -> Vec<Line<'static>> {
    let mut all_lines: Vec<Line<'static>> = vec![];
    // a line on the budget information:
    let budget_str = budget.to_string();
    let spent_str = spent.to_string();
    let spent_str_span = match spent <= budget {
        true => spent_str.green().bold(),
        false => spent_str.red().bold(),
    };
    let mut budget_freq_str = budget_freq.to_string();
    budget_freq_str.make_ascii_uppercase();
    let mut budget_line = vec![];
    for budget_info in [
        budget_freq_str.blue().bold(),
        " budget: ".blue().bold(),
        spent_str_span,
        " / ".black().bold(),
        budget_str.black().bold(),
        " spent already.".black(),
    ] {
        budget_line.push(budget_info);
    }
    all_lines.push(Line::from(budget_line));
    // add a split line for transactions
    all_lines.push(Line::raw("Relevant transactions: ").bold());
    // print relevant transactions
    for txn in &txns {
        all_lines.push(Line::raw(txn.clone()));
    }
    all_lines
}

pub struct ReportMain {
    pub email: String,
    // each element in vector is a block to be rendered
    pub summary_blocks: Vec<CategorySummary>,
    pub client: Client,
}

impl ReportMain {
    pub fn new(email: String) -> Self {
        let instance = Self {
            summary_blocks: Vec::new(),
            email: email.clone(),
            client: Client::new(),
        };
        instance
    }

    // mimicking what account_main does: not sure how this works, hopefully just magically.
    pub async fn initialize(&mut self) {
        self.get_categorical_summary().await;
    }

    async fn get_categorical_summary(&mut self) {
        let url = format!("http://localhost:8000/report_details?email={}", self.email);
        match self.client.get(&url).send().await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => {
                    if let Ok(cat_sum) = response.json::<Vec<CategorySummary>>().await {
                        self.summary_blocks = cat_sum;
                    }
                }
                _ => {}
            },
            Err(_e) => {}
        }
    }

    pub fn render(&self, f: &mut Frame) {
        let background = Block::default().style(Style::default().bg(Color::White));
        f.render_widget(background, f.area());

        // divide the page into 3 chunks: 1=title, 2=categorical summary, 3=help message
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

        let title = Paragraph::new("REPORT (Category Based)")
            .style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        f.render_widget(title, chunks[0]);

        // divide the second chunk into one block per category
        let block_percent = 100 / (self.summary_blocks.len() as u16);
        let mut constraint_vec: Vec<Constraint> = vec![];
        for _i in 0..self.summary_blocks.len() {
            constraint_vec.push(Constraint::Percentage(block_percent));
        }
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraint_vec)
            .split(chunks[1]);

        // render each chunk with summary
        for i in 0..self.summary_blocks.len() {
            let cat_title_str = self.summary_blocks[i].nickname.as_str();
            let overbudget_status = self.summary_blocks[i].overbudget.clone();
            let budget = self.summary_blocks[i].budget.clone();
            let budget_freq = self.summary_blocks[i].budget_freq.clone();
            let spent = self.summary_blocks[i].total.clone();
            let transactions: Vec<String> = self.summary_blocks[i].cat_trans.clone();
            let cat_summary = Paragraph::new(generate_report_block(
                budget,
                budget_freq,
                spent,
                transactions,
            ))
            .block(title_block(cat_title_str, overbudget_status));

            f.render_widget(cat_summary, main_chunks[i]);
        }

        // Bottom notice for navigation instructions (Esc to quit, etc.)
        let notice = Paragraph::new("Esc to return to Homepage")
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[2]);
    }
}
