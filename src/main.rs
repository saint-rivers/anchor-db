
mod cli;
// use crate::cli;

use std::process::exit;

const EXIT_COMMAND: &str = ".exit";
const EXIT_SUCCESS: i32 = 0;

fn main() {
    let mut user_input;

    loop {
        let raw_input = cli::read_user_input();
        user_input = raw_input.trim();

        if user_input == EXIT_COMMAND {
            exit(EXIT_SUCCESS)
        } else {
            println!("unrecognized command: '{}'", user_input);
        }
    }
}
