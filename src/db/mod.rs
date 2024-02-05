pub mod db_db_impl;
pub mod db_error;
pub mod db_mockdb;
pub mod db_stock_held;
pub mod db_trait;
pub mod db_tx_history;
pub mod db_user;

use crate::db::db_trait::DatabaseTrait;
use crate::types::stock::StockHeld;
use crate::types::tx::Tx;
use crate::types::user::User;
use rusqlite::{params, Connection, Result};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Database {
    conn: Connection,
}

pub struct MockDB {
    users: RefCell<HashMap<String, User>>,
    tx_history: RefCell<HashMap<String, Vec<Tx>>>,
    stock_holdings: RefCell<HashMap<String, Vec<StockHeld>>>,
    username_to_userid: RefCell<HashMap<String, String>>,
}
