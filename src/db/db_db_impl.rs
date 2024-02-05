use super::db_error::DbError;
use super::Database;
use crate::db::db_trait::DatabaseTrait;
use crate::types::stock::StockHeld;
use crate::types::tx::Tx;
use crate::types::user::User;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

impl DatabaseTrait for Database {
    type Db = Database;

    fn get_user_stocks_held(&self, user_id: &String) -> Result<Vec<StockHeld>, DbError> {
        self.get_user_stocks_held(user_id)
    }

    fn delete_stock_holding(&self, user_id: &str, ticker: &str) -> Result<(), DbError> {
        self.delete_stock_holding(user_id, ticker)
    }
    fn add_stock_to_user(
        &self,
        user_id: &str,
        ticker: &str,
        cost_basis: f64,
        number_of_shares: i32,
    ) -> Result<(), DbError> {
        self.add_stock_to_user(
            &user_id.to_string(),
            &ticker.to_string(),
            cost_basis,
            number_of_shares,
        )
    }

    fn add_tx_to_history(
        &self,
        user_id: &String,
        ticker: &String,
        number_of_shares: i32,
        value: f64,
    ) -> Result<(), DbError> {
        self.add_tx_to_history(user_id, ticker, number_of_shares, value)
    }

    fn get_tx_for_user(&self, user_id: &str) -> Result<Vec<Tx>, DbError> {
        self.get_tx_for_user(user_id)
    }
    fn add_user(&self, user: &User) -> Result<(), DbError> {
        self.add_user(user)
            .map_err(|e| DbError::Simple(e.to_string()))
    }
    fn update_user_bal(&self, user: &User, new_bal: f64) -> Result<(), DbError> {
        self.update_user_bal(user, new_bal)
    }
    fn get_user_by_username(&self, username: &str) -> Result<User, DbError> {
        self.get_user_by_username(username)
            .map_err(|e| DbError::Simple(e.to_string()))
    }
    fn user_exists(&self, username: &str) -> Result<bool, DbError> {
        self.user_exists(username)
            .map_err(|e| DbError::Simple(e.to_string()))
    }
    fn new(db_file: &str) -> Result<Self::Db, DbError> {
        let conn = Connection::open(db_file)?;
        Ok(Database { conn })
    }
    fn create_tables(&self) -> Result<(), DbError> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS user (
            user_id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            balance REAL NOT NULL
        )",
                [],
            )
            .map_err(|e| e.to_string())?;
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS tx_history (
            tx_id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            ticker TEXT NOT NULL,
            number_of_shares INTEGER NOT NULL,
            date TEXT NOT NULL,
            value REAL NOT NULL
        )",
                [],
            )
            .map_err(|e| e.to_string())?;
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS stock_held (
            user_id TEXT NOT NULL,
            ticker TEXT NOT NULL,
            cost_basis REAL NOT NULL,
            number_of_shares INTEGER NOT NULL,
            PRIMARY KEY (user_id, ticker)
        )",
                params![],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
