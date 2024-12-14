use std::borrow::BorrowMut;

use crossterm::event::{KeyCode, KeyModifiers};
#[allow(unused_imports)]
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Masked, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, StatefulWidget, Wrap,
    },
    Frame,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct CategorySummary {
    pub nickname: String,
    pub budget: f64,
    pub budget_freq: String,
    pub overbudget: bool,
    pub total: f64,
    // a vector of corresponding transaction ids of the entries below
    pub transaction_idz: Vec<i32>,
    // a vector of all the relevant transactions within budget freq frame
    pub cat_trans: Vec<String>,
}

// https://ratatui.rs/examples/widgets/block/
// Create a bordered block with a title.
fn title_block(
    cat_name: &str,
    overbudget: bool,
    budget: f64,
    budget_freq: String,
    spent: f64,
    is_active: bool,
) -> Block {
    // a line on the budget information:
    let budget_str = format!("{:.2}", budget);
    let spent_str = format!("{:.2}", spent);
    let mut budget_freq_str = budget_freq.to_string();
    budget_freq_str.make_ascii_uppercase();
    let budget_suffix = format!("{} budget: {}", budget_freq_str, budget_str);
    let new_block = match overbudget {
        false => Block::bordered()
            .title(cat_name.blue().on_white().bold())
            .title(spent_str.green().on_white().bold())
            .title(budget_suffix.black())
            .border_style(Style::default().fg(if is_active {
                Color::Yellow
            } else {
                Color::Black
            })),
        true => Block::bordered()
            .title(cat_name.blue().on_white().bold())
            .title(spent_str.red().on_white().bold())
            .title(budget_suffix.black())
            .border_style(Style::default().fg(if is_active {
                Color::Yellow
            } else {
                Color::Black
            })),
    };
    new_block
}

/*fn generate_report_block(budget: f64, budget_freq: String, spent: f64) -> Vec<Line<'static>> {
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
    all_lines
}*/

pub struct ReportMain {
    pub email: String,
    // each element in vector is a block to be rendered
    pub summary_blocks: Vec<CategorySummary>,
    pub client: Client,
    // we need multiple list states as we have multiple summary blocks
    pub list_states: Vec<ListState>,
    // store currently selected category
    pub active_cat: usize,
    // trans_mapping[active_cat][list_states[active_cat].selected] = transaction_id
    pub trans_mapping: Vec<Vec<i32>>,
}

impl ReportMain {
    pub fn new(email: String) -> Self {
        let instance = Self {
            summary_blocks: Vec::new(),
            email: email.clone(),
            client: Client::new(),
            list_states: Vec::new(),
            active_cat: 0,
            trans_mapping: Vec::new(),
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
                    if let Ok(mut cat_sum) = response.json::<Vec<CategorySummary>>().await {
                        // sort the categories by nickname
                        cat_sum.sort_unstable_by_key(|item| (item.nickname.clone()));
                        self.summary_blocks = cat_sum;
                    }
                }
                _ => {}
            },
            Err(_e) => {}
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
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
        //let block_percent = 100 / (self.summary_blocks.len() as u16);
        let mut constraint_vec: Vec<Constraint> = vec![];
        for _i in 0..self.summary_blocks.len() {
            // constraint_vec.push(Constraint::Percentage(block_percent));
            constraint_vec.push(Constraint::Min(3));
        }
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraint_vec)
            .split(chunks[1]);

        // render each chunk with summary
        for i in 0..self.summary_blocks.len() {
            self.render_nested_blocks(f, main_chunks[i], i);
        }

        // Bottom notice for navigation instructions (Esc to quit, etc.)
        let notice = Paragraph::new("Esc: Back | Tab: Switch between Categories | ↑↓: Switch between Transactions | D: Delete Transaction")
            .style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        f.render_widget(notice, chunks[2]);
    }

    fn render_nested_blocks(&mut self, frame: &mut Frame, area: Rect, i: usize) {
        let cat_title_str = self.summary_blocks[i].nickname.as_str();
        let overbudget_status = self.summary_blocks[i].overbudget.clone();
        let budget = self.summary_blocks[i].budget.clone();
        let budget_freq = self.summary_blocks[i].budget_freq.clone();
        let spent = self.summary_blocks[i].total.clone();
        let trans_ids = self.summary_blocks[i].transaction_idz.clone();
        let transactions: Vec<String> = self.summary_blocks[i].cat_trans.clone();
        // divide each summary chunk into budget information line + transaction list
        // outer block is the category information
        // inner block is the transaction details
        let mut outer_block = title_block(
            cat_title_str,
            overbudget_status,
            budget,
            budget_freq,
            spent,
            self.active_cat == i,
        );
        let inner_block = Block::default()
            .borders(Borders::NONE)
            .title("Relevant Transactions: ");
        let inner = outer_block.borrow_mut().inner(area);
        frame.render_widget(outer_block, area);
        // render inner block
        // create list state for this block, and select the first entry
        let mut new_list_state: ListState = ListState::default();
        new_list_state.select(Some(0));
        self.list_states.push(new_list_state);
        let mut items: Vec<ListItem> = vec![];
        // set up transaction mapping
        let mut id_mapping: Vec<i32> = vec![];
        for i in 0..transactions.len() {
            let new_item = ListItem::new(format!(
                "{}: {}",
                trans_ids[i].clone(),
                transactions[i].clone(),
            ));
            items.push(new_item);
            id_mapping.push(trans_ids[i]);
        }
        self.trans_mapping.push(id_mapping);
        let list = List::new(items)
            .block(inner_block)
            .style(Style::default().fg(Color::Black))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            );

        frame.render_stateful_widget(list, inner, &mut self.list_states[i]);
    }

    async fn handle_list_input(&mut self, key: KeyCode) {
        match key {
            // tab switches between category blocks
            KeyCode::Tab => {
                self.active_cat = (self.active_cat + 1) % self.summary_blocks.len();
            }
            KeyCode::Up => {
                let selected = self.list_states[self.active_cat].selected().unwrap_or(0);
                self.list_states[self.active_cat].select(Some(if selected == 0 {
                    self.summary_blocks[self.active_cat].cat_trans.len() - 1
                } else {
                    selected - 1
                }));
            }
            KeyCode::Down => {
                let selected = self.list_states[self.active_cat].selected().unwrap_or(0);
                self.list_states[self.active_cat].select(Some(
                    if selected >= self.summary_blocks[self.active_cat].cat_trans.len() - 1 {
                        0
                    } else {
                        selected + 1
                    },
                ));
            }
            KeyCode::Char('d') => {
                if let Some(selected) = self.list_states[self.active_cat].selected() {
                    let to_delete_id = self.trans_mapping[self.active_cat][selected].clone();
                    // pop the transaction from the mapping
                    self.trans_mapping[self.active_cat].remove(selected);
                    // delete the transaction
                    self.delete_transaction(to_delete_id).await;
                }
            }
            _ => {}
        }
    }

    pub async fn handle_input(&mut self, key: KeyCode, _modifiers: KeyModifiers) -> bool {
        if key == KeyCode::Esc {
            return true;
        }
        self.handle_list_input(key).await;
        false
    }

    async fn delete_transaction(&mut self, trans_id: i32) {
        let url = format!("http://localhost:8000/delete_trans?trans_id={}", trans_id);

        match self.client.delete(&url).send().await {
            Ok(response) => {
                let status = response.status();
                match status {
                    reqwest::StatusCode::OK => {
                        self.get_categorical_summary().await;
                    }
                    _ => {}
                }
            }
            Err(_e) => {}
        }
    }
}
