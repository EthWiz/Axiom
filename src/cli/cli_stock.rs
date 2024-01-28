use crate::db::Database;
use crate::types::user::User;
use clap::{App, Arg, SubCommand};
use prettytable::{cell, format, row, Table};

pub fn stock_subcommand() -> App<'static> {
    SubCommand::with_name("stock")
        .about("Stock operations")
        .subcommand(buy_subcommand())
        .subcommand(list_subcommand())
}

fn buy_subcommand() -> App<'static> {
    SubCommand::with_name("buy")
        .about("Buy stocks by number of shares")
        .arg(
            Arg::new("username")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .arg(Arg::new("ticker").required(true).takes_value(true).index(2))
        .arg(
            Arg::new("share_num")
                .required(true)
                .takes_value(true)
                .index(3),
        )
}
fn list_subcommand() -> App<'static> {
    SubCommand::with_name("list")
        .about("List user's stock holdings")
        .arg(
            Arg::new("username")
                .required(true)
                .takes_value(true)
                .index(1),
        )
}

pub async fn handle_stock_command(matches: &clap::ArgMatches, db: &Database) {
    if let Some(buy_matches) = matches.subcommand_matches("list") {
        let username = buy_matches.value_of("username").unwrap();
        let user = db
            .get_user_by_username(username)
            .expect("couldn't get user");
        let holdings = user.get_holdings();

        if holdings.is_empty() {
            println!("No stock holdings found for user: {}", username);
        } else {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["Ticker", "Shares", "Cost Basis", "Total Value"]);

            for holding in holdings.iter() {
                let total_value = holding.cost_basis * holding.number_of_shares as f64;
                table.add_row(row![
                    holding.stock.ticker,
                    holding.number_of_shares.to_string(),
                    format!("${:.2}", holding.cost_basis),
                    format!("${:.2}", total_value)
                ]);
            }

            println!("Stock holdings for user: {}", username);
            table.printstd();
        }
    }

    if let Some(buy_matches) = matches.subcommand_matches("buy") {
        let username = buy_matches.value_of("username").unwrap();
        let ticker = buy_matches.value_of("ticker").unwrap();
        let num_shares_str = buy_matches.value_of("share_num").unwrap();

        let num_shares: i32 = match num_shares_str.parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Error: Share num must be a integer number.");
                return;
            }
        };
        println!("Buying stock: {} {} {}", username, ticker, num_shares);

        let mut user = db
            .get_user_by_username(username)
            .expect("couldn't get user");
        match user
            .buy_stocks_by_stock_num(db, num_shares, ticker.to_string())
            .await
        {
            Ok(_) => {
                let db_holdings = db
                    .get_user_stocks_held(&user.get_user_id())
                    .expect("failed to get user holdings");
                let user_holdings = user.get_holdings();
                println!("db holdings: {:#?}", db_holdings);
                println!("user holdings: {:#?}", user_holdings);
            }
            Err(e) => {
                eprintln!("Error buying stock: {}", e)
            }
        }
    }
}
