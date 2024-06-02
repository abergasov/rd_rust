use std::fmt;
use std::io::Error;

#[derive(Clone, Debug)]
pub struct StringWith {
    pub key: String,
    pub value: String,
}

impl fmt::Display for StringWith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
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
    fn fatal(&self, message: &str, err: Error, args: &[StringWith]);
    fn with(&self, args: &[StringWith]) -> Box<dyn AppLogger + Send + Sync>;
}

pub fn prepare_params(ctx: &Vec<StringWith>, err: Option<Error>, args: &[StringWith]) -> Vec<String> {
    let mut ctx = ctx.clone();
    ctx.extend_from_slice(args);
    if !err.is_none() {
        ctx.push(StringWith::new("error", &err.unwrap().to_string()));
    }
    return ctx.iter().map(|x| x.string()).collect();
}