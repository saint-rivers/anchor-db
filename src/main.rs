mod cli;
mod command;

use std::process::exit;

use crate::command::MetaCommand;

const EXIT_SUCCESS: i32 = 0;

fn main() {
    let mut user_input;

    loop {
        let raw_input = cli::read_user_input();
        user_input = raw_input.trim();

        if user_input.starts_with(".") {
            user_input = &user_input[1..];

            let meta_cmd = MetaCommand::get_meta_command(user_input);
            match meta_cmd {
                MetaCommand::Exit => exit(EXIT_SUCCESS),
                MetaCommand::Version => println!("AnchorDB version: 1.0.0"),
                _ => println!("unrecognized command: '{}'", user_input),
            }
        } else {
            // parse SQL
        }


    }
}
