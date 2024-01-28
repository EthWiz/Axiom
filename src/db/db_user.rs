use super::Database;
use crate::types::user::User;
use rusqlite::{params, Result};

impl Database {
    pub fn add_user(&self, user: &User) -> Result<()> {
        self.conn.execute(
            "INSERT INTO user (user_id, username, balance) VALUES (?1, ?2, ?3)",
            params![user.get_user_id(), user.get_username(), user.get_balance()],
        )?;
        Ok(())
    }

    pub fn update_user_bal(&self, user: &User, new_bal: f64) -> Result<(), String> {
        self.conn.execute(
            "INSERT INTO user (user_id, username, balance) VALUES (?1, ?2, ?3)",
            params![user.get_user_id(), user.get_username(), user.get_balance()],
        );
        Ok(())
    }

    pub fn user_exists(&self, username: &str) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT EXISTS(SELECT 1 FROM user WHERE username = ?1)")?;
        let exists = stmt.query_row([username], |row| row.get(0))?;
        Ok(exists)
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<User> {
        let mut stmt = self
            .conn
            .prepare("SELECT user_id, username, balance FROM user WHERE username = ?1")?;

        let user = stmt.query_row(params![username], |row| {
            let user_id: String = row.get(0)?;
            let username: String = row.get(1)?;
            let balance: f64 = row.get(2)?;
            let holdings = match self.get_user_stocks_held(&user_id) {
                Ok(holdings) => holdings,
                Err(e) => {
                    eprintln!("Error getting user stocks: {}", e);
                    vec![] // Return an empty vector if there's an error
                }
            };

            Ok(User {
                user_id,
                username,
                balance,
                holdings,
            })
        })?;

        Ok(user)
    }
}
