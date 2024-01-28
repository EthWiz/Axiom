use crate::db::Database;
use crate::types::user::User;
use clap::{App, Arg, SubCommand};
pub fn new_user_subcommand() -> App<'static> {
    SubCommand::with_name("new").about("Create a new user").arg(
        Arg::new("username")
            .help("The username for the new user")
            .required(true)
            .takes_value(true),
    )
}
pub fn handle_new_command(matches: &clap::ArgMatches, db: &Database) {
    if let Some(username) = matches.value_of("username") {
        match db.user_exists(username) {
            Ok(exists) => {
                if exists {
                    println!("User '{}' already exists.", username);
                } else {
                    println!("Creating new user: {}", username);
                    User::new(db, username.to_string());
                }
            }
            Err(e) => eprintln!("Error checking if user exists: {}", e),
        }
    }
}
