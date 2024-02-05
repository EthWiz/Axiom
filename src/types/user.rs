use crate::api::{self, get_cur_price};
use crate::db::db_error::DbError;
use crate::db::db_trait::{self, DatabaseTrait};
use crate::db::Database;
use crate::types::stock::{Stock, StockHeld};
use tokio::runtime;
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub user_id: String,
    pub balance: f64,
    pub holdings: Vec<StockHeld>,
}

impl User {
    pub fn new<T: DatabaseTrait>(db: &T, username: String) -> User {
        let user = User {
            username: username,
            user_id: Uuid::new_v4().to_string(),
            balance: 100000.0,
            holdings: Vec::new(),
        };
        db.add_user(&user).expect("failed to add new user to db");
        user
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_holdings(&self) -> &Vec<StockHeld> {
        &self.holdings
    }
    pub fn get_holding_for_user<T: DatabaseTrait>(
        &self,
        db: &T,
        ticker: &str,
    ) -> Result<StockHeld, String> {
        let holdings = db
            .get_user_stocks_held(&self.user_id)
            .map_err(|e| format!("Failed to get user holdings: {:?}", e))?; // Converts DbError to String

        match holdings
            .into_iter()
            .find(|holding| holding.stock.ticker == ticker)
        {
            Some(holding) => Ok(holding),
            None => Err(format!(
                "Stock {} not found for user {}",
                ticker, self.user_id
            )),
        }
    }

    pub async fn buy_stocks_by_stock_num<T: DatabaseTrait>(
        &mut self,
        db: &T,
        num_stocks: i32,
        ticker: String,
    ) -> Result<(), String> {
        let price_per_stock = match api::get_cur_price(&ticker).await {
            Ok(price) => price,
            Err(e) => {
                eprintln!("Error retrieving stock price: {}", e);
                return Err("Failed to retrieve stock price".to_string());
            }
        };

        let total_price = price_per_stock * num_stocks as f64;
        if self.balance < total_price {
            return Err("User balance not enough".to_string());
        }

        let mut stock_found = false;

        for stock in self.holdings.iter_mut() {
            if stock.stock.ticker == ticker {
                let total_cost = stock.cost_basis * stock.number_of_shares as f64 + total_price;
                stock.number_of_shares += num_stocks;
                stock.cost_basis = total_cost / stock.number_of_shares as f64;
                stock_found = true;
                break;
            }
        }

        if !stock_found {
            self.holdings.push(StockHeld {
                stock: Stock {
                    ticker: ticker.clone(),
                },
                cost_basis: price_per_stock,
                number_of_shares: num_stocks,
            });
        }

        self.balance -= total_price;
        db.add_stock_to_user(
            &self.get_user_id().to_string(),
            &ticker,
            self.holdings
                .iter()
                .find(|s| s.stock.ticker == ticker)
                .unwrap()
                .cost_basis,
            self.holdings
                .iter()
                .find(|s| s.stock.ticker == ticker)
                .unwrap()
                .number_of_shares,
        )
        .expect("failed to update stock in db");

        db.add_tx_to_history(&self.user_id, &ticker, num_stocks, total_price)
            .expect("failed to add transaction to history");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::db_trait::DatabaseTrait;
    use crate::db::MockDB;
    use crate::types::user::User;

    // Mocking the api module
    mod api {
        pub async fn get_cur_price(ticker: &str) -> Result<f64, String> {
            Ok(100.0)
        }
    }

    #[tokio::test]
    async fn test_user_init_bal() {
        let db = MockDB::new("dumy").expect("failied to init mock db");
        let mut test_user = User::new(&db, "testuser".to_string());

        let result = test_user.get_balance();

        assert_eq!(test_user.balance, 100000.0);
    }

    #[tokio::test]
    async fn test_buy_tsla_stock() {
        let db = MockDB::new("dumy").expect("failed to init mock db");
        let mut test_user = User::new(&db, "testuser".to_string());
        let tsla_ticker = "tsla";
        test_user.buy_stocks_by_stock_num(&db, 3, tsla_ticker.to_string());
        let end_tsla_amount = test_user
            .get_holding_for_user(&db, tsla_ticker)
            .expect("failed to get shares in the end of test");
        assert_eq!(end_tsla_amount.number_of_shares, 3);
    }
}
