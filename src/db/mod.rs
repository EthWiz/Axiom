pub mod db_stock_held;
pub mod db_user;
use rusqlite::{params, Connection, Result};
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_file: &str) -> Result<Database> {
        let conn = Connection::open(db_file)?;
        Ok(Database { conn })
    }

    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
                user_id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                balance REAL NOT NULL
            )",
            [],
        )?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS stock_held (
                user_id TEXT NOT NULL,
                ticker TEXT NOT NULL,
                cost_basis REAL NOT NULL,
                number_of_shares INTEGER NOT NULL,
                PRIMARY KEY (user_id, ticker)
            )",
            params![],
        )?;

        Ok(())
    }
}
