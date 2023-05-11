use std::io::{stdin, stdout, Write};

fn print_prompt() {
    print!("db > ");
    stdout().flush().unwrap(); // this needs to be flushed, otherwise it will only print after the input is complete
}

pub fn read_user_input() -> String {
    let mut input_string = String::new();
    print_prompt();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    return input_string.clone();
}
