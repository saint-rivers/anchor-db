use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

const EXIT_COMMAND: &str = ".exit";
const EXIT_SUCCESS: i32 = 0;

fn main() {
    let mut user_input;

    loop {
        let raw_input = read_user_input();
        user_input = raw_input.trim();

        if user_input == EXIT_COMMAND {
            exit(EXIT_SUCCESS)
        } else {
            println!("unrecognized command: '{}'", user_input);
        }
    }
}

fn print_prompt() {
    print!("db > ");
    stdout().flush().unwrap(); // this needs to be flushed, otherwise it will only print after the input is complete
}

fn read_user_input() -> String {
    let mut input_string = String::new();
    print_prompt();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    return input_string.clone();
}
