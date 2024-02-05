use super::db_error::DbError;
use super::Database;
use crate::types::tx::Tx;
use rusqlite::{params, Result};

impl Database {
    pub fn add_tx_to_history(
        &self,
        user_id: &String,
        ticker: &String,
        number_of_shares: i32,
        value: f64,
    ) -> Result<(), DbError> {
        let tx = Tx::new(user_id, number_of_shares, value, ticker);

        println!("Inserting transaction: tx_id={}, user_id={}, ticker={}, number_of_shares={}, date={}, value={}",
            tx.tx_id(), user_id, ticker, number_of_shares, tx.date(), value);

        match self.conn.execute(
            "INSERT INTO tx_history (tx_id, user_id, ticker, number_of_shares, date, value) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![tx.tx_id(), user_id, ticker, number_of_shares, tx.date(), value],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(DbError::Simple(e.to_string())),
        }
    }

    pub fn get_tx_for_user(&self, user_id: &str) -> Result<Vec<Tx>, DbError> {
        let mut stmt = self.conn.prepare("SELECT tx_id, user_id, ticker, number_of_shares, date, value FROM tx_history WHERE user_id = ?1")
            .map_err(|e| e.to_string())?;

        let tx_iter = stmt
            .query_map(params![user_id], |row| {
                Ok(Tx {
                    tx_id: row.get(0)?,
                    user_id: row.get(1)?,
                    ticker: row.get(2)?,
                    number_of_shares: row.get(3)?,
                    date: row.get(4)?,
                    value: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut transactions = Vec::new();
        for tx_result in tx_iter {
            match tx_result {
                Ok(tx) => transactions.push(tx),
                Err(e) => return Err(DbError::Simple(e.to_string())),
            }
        }
        Ok(transactions)
    }
}
