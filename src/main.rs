mod command;
mod constant;
mod utils;

use command::{
    meta::{self, MetaCommandStatus},
    statement::{self, PrepareStatus},
};
use constant::constant::META_CMD_INDICATOR;

use crate::utils::cli;

fn main() {
    let mut user_input: &str;

    loop {
        let raw_input = cli::read_user_input();
        user_input = raw_input.trim();

        if user_input == "" {
            continue;
        } else if user_input.starts_with(META_CMD_INDICATOR) {
            let result = meta::do_meta_command(user_input);
            match result {
                MetaCommandStatus::Success => {}
                MetaCommandStatus::UnrecognizedCommand => {
                    println!("unrecognized command: '{}'", user_input);
                }
            }
        } else {
            let result = statement::do_prepared_statement(user_input);
            match result {
                PrepareStatus::Success => {}
                PrepareStatus::UnrecognizedStatement => {
                    println!("unrecognized command: '{}'", user_input);
                }
            }
            // execute_statement(statement);
        }
    }
}
