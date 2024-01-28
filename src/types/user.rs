use crate::api::{self, get_cur_price};
use crate::db::Database;
use crate::types::stock::{Stock, StockHeld};
use tokio::runtime;
use uuid::Uuid;

pub struct User {
    pub username: String,
    pub user_id: String,
    pub balance: f64,
    pub holdings: Vec<StockHeld>,
}

impl User {
    pub fn new(db: &Database, username: String) -> User {
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

    pub async fn buy_stocks_by_stock_num(
        &mut self,
        db: &Database,
        num_stocks: i32,
        ticker: String,
    ) -> Result<(), String> {
        let mut total_price = 0.0;
        match api::get_cur_price(&ticker).await {
            Ok(price_per_stock) => {
                total_price = price_per_stock * num_stocks as f64;
            }
            Err(e) => {
                eprintln!("Error retrieving stock price: {}", e);
            }
        }
        if self.balance < total_price {
            return Err("User balance not enough".to_string());
        }

        let mut stock_found = false;
        let mut numer_of_cur_shares = 0;

        for stock in self.holdings.iter_mut() {
            if stock.stock.ticker == ticker {
                stock.number_of_shares += num_stocks;
                numer_of_cur_shares = stock.number_of_shares;
                stock_found = true;
                break;
            }
        }

        if !stock_found {
            let new_stock = StockHeld {
                stock: Stock {
                    ticker: ticker.to_string(),
                },
                cost_basis: 0.0,
                number_of_shares: num_stocks,
            };
            numer_of_cur_shares = num_stocks;
            self.holdings.push(new_stock);
        };

        db.add_stock_to_user(
            &self.get_user_id().to_string(),
            &ticker,
            0.0,
            numer_of_cur_shares,
        )
        .expect("failed to register new stock in db");

        self.balance -= total_price;

        Ok(())
    }
}
