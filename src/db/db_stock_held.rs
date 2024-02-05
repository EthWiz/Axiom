use super::db_error::DbError;
use super::Database;
use super::DatabaseTrait;
use crate::types::stock::{Stock, StockHeld};

use rusqlite::{params, Result};

impl Database {
    pub fn delete_stock_holding(&self, user_id: &str, ticker: &str) -> Result<(), DbError> {
        self.conn
            .execute(
                "DELETE FROM stock_held WHERE user_id = ?1 AND ticker = ?2",
                params![user_id, ticker],
            )
            .map_err(DbError::from);
        Ok(())
    }
    pub fn add_stock_to_user(
        &self,
        user_id: &String,
        ticker: &String,
        cost_basis: f64,
        number_of_shares: i32,
    ) -> Result<(), DbError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO stock_held (user_id, ticker, cost_basis, number_of_shares) 
             VALUES (?1, ?2, ?3, ?4)",
            params![user_id, ticker, cost_basis, number_of_shares],
        )?;
        Ok(())
    }
    pub fn get_user_stocks_held(&self, user_id: &str) -> Result<Vec<StockHeld>, DbError> {
        let mut stmt = self.conn.prepare(
            "SELECT ticker, cost_basis, number_of_shares FROM stock_held WHERE user_id = ?1",
        )?;

        let stock_held_iter = stmt.query_map(params![user_id], |row| {
            Ok(StockHeld {
                stock: Stock {
                    ticker: row.get(0)?,
                },
                cost_basis: row.get(1)?,
                number_of_shares: row.get(2)?,
            })
        })?;

        let mut stocks_held = Vec::new();
        for stock_held in stock_held_iter {
            match stock_held {
                Ok(held) => stocks_held.push(held),
                Err(e) => {
                    eprintln!("Error retrieving stock holdings: {}", e);
                    return Ok(Vec::new());
                }
            }
        }

        Ok(stocks_held)
    }
}
