use std::process::exit;

use crate::constant::constant;

pub enum MetaCommandStatus {
    Success,
    UnrecognizedCommand,
}

pub enum MetaCommand {
    Exit,
    Version,
    Invalid,
}

impl MetaCommand {
    pub fn get_meta_command(str: &str) -> MetaCommand {
        match str.as_bytes() {
            b"exit" => MetaCommand::Exit,
            b"version" => MetaCommand::Version,
            _ => MetaCommand::Invalid,
        }
    }
}

pub fn do_meta_command(input_buffer: &str) -> MetaCommandStatus {
    let parsed_input = &input_buffer[1..];

    let meta_cmd = MetaCommand::get_meta_command(parsed_input);
    match meta_cmd {
        MetaCommand::Exit => exit(constant::EXIT_SUCCESS),
        MetaCommand::Version => {
            println!("AnchorDB version: 1.0.0");
            MetaCommandStatus::Success
        }
        _ => MetaCommandStatus::UnrecognizedCommand,
    }
}
