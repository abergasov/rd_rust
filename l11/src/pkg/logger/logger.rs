use std::io::Error;

use crate::pkg::logger::abstract_logger::{AppLogger, StringWith};

pub struct SimpleLogger {
    context: Vec<StringWith>,
}

impl SimpleLogger {
    pub(crate) fn new(context: &[StringWith]) -> SimpleLogger {
        SimpleLogger {
            context: context.to_vec()
        }
    }
}

impl AppLogger for SimpleLogger {
    fn info(&self, message: &str, args: &[StringWith]) {
        println!("INFO: {}, {}", message, prepare_params(&self.context, None, args).join(", "));
    }

    fn error(&self, message: &str, err: Error, args: &[StringWith]) {
        println!("INFO: {}, {}", message, prepare_params(&self.context, Some(err), args).join(", "));
    }

    fn fatal(&self, message: &str, err: &dyn std::error::Error, args: &[StringWith]) {
        // Implementation of the fatal method
    }

    fn with(&self, args: &[StringWith]) -> Box<dyn AppLogger> {
        let mut ctx = self.context.clone();
        ctx.extend_from_slice(args);
        return Box::new(SimpleLogger::new(ctx.as_slice()));
    }
}

fn prepare_params(ctx: &Vec<StringWith>, err: Option<Error>, args: &[StringWith]) -> Vec<String> {
    let mut ctx = ctx.clone();
    ctx.extend_from_slice(args);
    if !err.is_none() {
        ctx.push(StringWith::new("error", &err.unwrap().to_string()));
    }
    return ctx.iter().map(|x| x.string()).collect();
}
