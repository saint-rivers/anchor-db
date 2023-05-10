use std::{io::stdin, process::exit};

fn main() {
    // let exit_command = String::from(".exit");
    // let mut user_input;

    // loop {
    //     user_input = read_user_input();

    //     if user_input.trim() == exit_command {
    //         exit(0)
    //     } else {
    //         println!("output {}", user_input);
    //     }
    // }

    print_prompt();
    // user_input = read_user_input();
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

}

fn print_prompt() {
    print!("db > ");
}

fn read_user_input() -> String {
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    return input_string.clone();
}
