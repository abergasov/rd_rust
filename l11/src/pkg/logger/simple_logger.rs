use std::io::Error;
use std::process;

use crate::pkg::logger::abstract_logger::{AppLogger, prepare_params, StringWith};

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

    fn fatal(&self, message: &str, err: Error, args: &[StringWith]) {
        println!("INFO: {}, {}", message, prepare_params(&self.context, Some(err), args).join(", "));
        process::exit(1);
    }

    fn with(&self, args: &[StringWith]) -> Box<dyn AppLogger + Send + Sync> {
        let mut ctx = self.context.clone();
        ctx.extend_from_slice(args);
        return Box::new(SimpleLogger::new(ctx.as_slice()));
    }
}

