use super::db_error::DbError;
use crate::types::stock::StockHeld;
use crate::types::tx::Tx;
use crate::types::user::User;
use rusqlite::Result as RusqliteResult;
use std::result::Result;

pub trait DatabaseTrait {
    type Db;

    fn get_user_stocks_held(&self, user_id: &String) -> Result<Vec<StockHeld>, DbError>;
    fn new(db_file: &str) -> Result<Self::Db, DbError>;
    fn delete_stock_holding(&self, user_id: &str, ticker: &str) -> Result<(), DbError>;
    fn add_stock_to_user(
        &self,
        user_id: &str,
        ticker: &str,
        cost_basis: f64,
        number_of_shares: i32,
    ) -> Result<(), DbError>;
    fn add_tx_to_history(
        &self,
        user_id: &String,
        ticker: &String,
        number_of_shares: i32,
        value: f64,
    ) -> Result<(), DbError>;
    fn get_tx_for_user(&self, user_id: &str) -> Result<Vec<Tx>, DbError>;
    fn add_user(&self, user: &User) -> Result<(), DbError>;
    fn update_user_bal(&self, user: &User, new_bal: f64) -> Result<(), DbError>;
    fn get_user_by_username(&self, username: &str) -> Result<User, DbError>;
    fn user_exists(&self, username: &str) -> Result<bool, DbError>;
    fn create_tables(&self) -> Result<(), DbError>;
}
