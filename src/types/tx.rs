use crate::types::stock::Stock;
use chrono::Local;
use uuid::Uuid;

#[derive(Debug)]
pub struct Tx {
    pub tx_id: String,
    pub user_id: String,
    pub date: String,
    pub number_of_shares: i32,
    pub value: f64,
    pub ticker: String,
}

impl Tx {
    pub fn new(user_id: &String, number_of_shares: i32, value: f64, ticker: &String) -> Tx {
        Tx {
            tx_id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            date: Local::now().format("%Y-%m-%d").to_string(),
            number_of_shares,
            value: value,
            ticker: ticker.to_string(),
        }
    }
    // Getter for tx_id
    pub fn tx_id(&self) -> &String {
        &self.tx_id
    }

    // Getter for date
    pub fn date(&self) -> &String {
        &self.date
    }

    // Getter for number_of_shares
    pub fn number_of_shares(&self) -> i32 {
        self.number_of_shares
    }

    // Getter for value
    pub fn value(&self) -> f64 {
        self.value
    }

    // Getter for ticker
    pub fn ticker(&self) -> &String {
        &self.ticker
    }
}
