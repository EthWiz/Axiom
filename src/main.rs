mod api;
mod cli;
mod db;
mod types;
mod util;
use crate::cli::{cli_new, cli_stock};
use crate::db::Database;
use tokio::runtime::Runtime;
fn main() {
    let matches = cli::build_cli().get_matches();
    let db = Database::new("fibelty.sqlite").expect("failed to build db");
    db.create_tables().expect("failed to create tables");

    let rt = Runtime::new().unwrap();

    if let Some(matches) = matches.subcommand_matches("new") {
        cli_new::handle_new_command(matches, &db);
    } else if let Some(matches) = matches.subcommand_matches("stock") {
        rt.block_on(cli_stock::handle_stock_command(matches, &db));
    }
}
