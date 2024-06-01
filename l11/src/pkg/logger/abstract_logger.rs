use std::io::Error;

#[derive(Clone)]
pub struct StringWith {
    pub key: String,
    pub value: String,
}

impl StringWith {
    pub fn new(key: &str, value: &str) -> StringWith {
        return StringWith {
            key: key.to_string(),
            value: value.to_string(),
        };
    }
    pub fn string(&self) -> String {
        return format!("{}: {}", self.key, self.value);
    }
}


pub trait AppLogger {
    fn info(&self, message: &str, args: &[StringWith]);
    fn error(&self, message: &str, err: Error, args: &[StringWith]);
    fn fatal(&self, message: &str, err: &dyn std::error::Error, args: &[StringWith]);
    fn with(&self, args: &[StringWith]) -> Box<dyn AppLogger>;
}