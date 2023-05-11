#[derive(Debug)]
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
