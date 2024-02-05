use crate::db::Database;
use crate::types::user::User;
use clap::{App, Arg, SubCommand};
pub fn summary_subcommand() -> App<'static> {
    SubCommand::with_name("summary")
        .about("Display important info regarding user account")
        .arg(
            Arg::new("username")
                .help("The username for the new user")
                .required(true)
                .takes_value(true),
        )
}
pub fn handle_new_command(matches: &clap::ArgMatches, db: &Database) {
    if let Some(username) = matches.value_of("username") {
        let user = db.get_user_by_username(username);
        println!("here will be sumary");
    }
}
