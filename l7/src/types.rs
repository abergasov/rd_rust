use std::error::Error;
use std::fmt;
use std::fmt::Debug;

pub enum Command {
    Lowercase,
    Uppercase,
    Nospaces,
    Slugify,
    CSV,
}

impl Command {
    pub fn from_str(input: &str) -> Result<Command, Box<dyn Error>> {
        match input.to_lowercase().trim() {
            "lowercase" => Ok(Command::Lowercase),
            "l" => Ok(Command::Lowercase),
            "uppercase" => Ok(Command::Uppercase),
            "u" => Ok(Command::Uppercase),
            "nospaces" => Ok(Command::Nospaces),
            "n" => Ok(Command::Nospaces),
            "slugify" => Ok(Command::Slugify),
            "s" => Ok(Command::Slugify),
            "csv" => Ok(Command::CSV),
            _ => Err(Box::new(CommandError::new("Unknown command"))),
        }
    }
}

pub struct CommandError {
    details: String,
}

impl Debug for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl CommandError {
    pub fn new(msg: &str) -> CommandError {
        CommandError { details: msg.to_string() }
    }
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

