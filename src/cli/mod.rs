pub mod cli_new;
pub mod cli_stock;

use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static> {
    App::new("Your App Name")
        .version("1.0")
        .about("Description of your app")
        .subcommand(cli_new::new_user_subcommand())
        .subcommand(cli_stock::stock_subcommand())
}
