use super::db_error::DbError;
use super::{Database, MockDB};
use crate::db::db_trait::DatabaseTrait;
use crate::types::stock::{Stock, StockHeld};
use crate::types::tx::Tx;
use crate::types::user::User;
use rusqlite::{params, Connection, Result};
use std::cell::RefCell;
use std::collections::HashMap;
impl DatabaseTrait for MockDB {
    type Db = MockDB;

    fn new(_db_file: &str) -> Result<Self::Db, DbError> {
        Ok(MockDB {
            users: RefCell::new(HashMap::new()),
            tx_history: RefCell::new(HashMap::new()),
            stock_holdings: RefCell::new(HashMap::new()),
            username_to_userid: RefCell::new(HashMap::new()),
        })
    }

    fn get_user_stocks_held(&self, user_id: &String) -> Result<Vec<StockHeld>, DbError> {
        // Attempt to borrow the users HashMap and find the user by user_id
        let users_borrowed = self.users.borrow();
        if let Some(user) = users_borrowed.get(user_id) {
            // If the user is found, clone their stock holdings to return
            // This assumes User has a method or public field to get their holdings
            Ok(user.get_holdings().clone()) // Adjust based on your User struct
        } else {
            // If no user is found with the given user_id, return an error
            Err(DbError::Simple(format!(
                "User with ID {} not found",
                user_id
            )))
        }
    }

    fn delete_stock_holding(&self, _user_id: &str, _ticker: &str) -> Result<(), DbError> {
        Err(DbError::Simple(
            "MockDB method `delete_stock_holding` not implemented yet".to_string(),
        ))
    }
    fn add_stock_to_user(
        &self,
        user_id: &str,
        ticker: &str,
        cost_basis: f64,
        number_of_shares: i32,
    ) -> Result<(), DbError> {
        let mut users_borrowed = self.users.borrow_mut();

        if let Some(user) = users_borrowed.get_mut(user_id) {
            let mut found = false;
            for holding in &mut user.holdings {
                if holding.stock.ticker == ticker {
                    holding.cost_basis = ((holding.cost_basis * holding.number_of_shares as f64)
                        + (cost_basis * number_of_shares as f64))
                        / (holding.number_of_shares + number_of_shares) as f64;
                    holding.number_of_shares += number_of_shares;
                    found = true;
                    break;
                }
            }
            if !found {
                user.holdings.push(StockHeld {
                    stock: Stock {
                        ticker: ticker.to_string(),
                    },
                    cost_basis,
                    number_of_shares,
                });
            }
            Ok(())
        } else {
            Err(DbError::Simple("User not found".to_string()))
        }
    }

    fn get_tx_for_user(&self, _user_id: &str) -> Result<Vec<Tx>, DbError> {
        Err(DbError::Simple(
            "MockDB method `get_tx_for_user` not implemented yet".to_string(),
        ))
    }

    fn add_user(&self, _user: &User) -> Result<(), DbError> {
        let user_id = _user.get_user_id().to_string();
        let mut user: User = _user.clone();
        let sec_user_id = user_id.to_string();
        let username = user.get_username().to_string();
        &self.users.borrow_mut().insert(user_id, user);
        &self
            .username_to_userid
            .borrow_mut()
            .insert(username, sec_user_id);

        Ok(())
    }

    fn update_user_bal(&self, user: &User, _new_bal: f64) -> Result<(), DbError> {
        Err(DbError::Simple(
            "MockDB method `update_user_bal` not implemented yet".to_string(),
        ))
    }

    fn get_user_by_username(&self, username: &str) -> Result<User, DbError> {
        let binding = self.username_to_userid.borrow();
        let user_id_option = binding.get(username);
        match user_id_option {
            Some(user_id) => {
                let sec_binding = self.users.borrow();
                let user_option = sec_binding.get(user_id);
                match user_option {
                    Some(user) => Ok(user.clone()),
                    None => Err(DbError::Simple(format!(
                        "User not found for user_id {}",
                        user_id
                    ))),
                }
            }
            None => Err(DbError::Simple(format!(
                "Username {} does not exist",
                username
            ))),
        }
    }

    fn user_exists(&self, _username: &str) -> Result<bool, DbError> {
        Err(DbError::Simple(
            "MockDB method `user_exists` not implemented yet".to_string(),
        ))
    }

    fn create_tables(&self) -> Result<(), DbError> {
        Err(DbError::Simple(
            "MockDB method `create_tables` not implemented yet".to_string(),
        ))
    }

    fn add_tx_to_history(
        &self,
        _user_id: &String,
        _ticker: &String,
        _number_of_shares: i32,
        _value: f64,
    ) -> Result<(), DbError> {
        Err(DbError::Simple(
            "MockDB method `add_tx_to_history` not implemented yet".to_string(),
        ))
    }
}
