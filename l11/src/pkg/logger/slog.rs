use std::io::Error;

use slog::*;
use slog_term::{FullFormat, PlainSyncDecorator};

use crate::pkg::logger::abstract_logger::{AppLogger, prepare_params, StringWith};

pub struct SLogger {
    logger: Logger,
    context: Vec<StringWith>,
}

impl SLogger {
    pub(crate) fn new(context: &[StringWith]) -> SLogger {
        SLogger {
            context: context.to_vec(),
            logger: Logger::root(
                FullFormat::new(
                    PlainSyncDecorator::new(std::io::stdout())
                ).build().fuse(), o!(),
            ),
        }
    }
}

impl AppLogger for SLogger {
    fn info(&self, message: &str, args: &[StringWith]) {
        let payload = prepare_params(&self.context, None, args);
        if payload.len() > 0 {
            info!(self.logger, "{} - {:?}", message, payload);
        } else {
            info!(self.logger, "{}", message);
        }
    }

    fn error(&self, message: &str, err: Error, args: &[StringWith]) {
        error!(self.logger, "{} - {:?}", message, prepare_params(&self.context, Some(err), args));
    }

    fn fatal(&self, message: &str, err: Error, args: &[StringWith]) {
        crit!(self.logger, "{} - {:?}", message, prepare_params(&self.context, Some(err), args));
    }

    fn with(&self, args: &[StringWith]) -> Box<dyn AppLogger + Send + Sync> {
        let mut ctx = self.context.clone();
        ctx.extend_from_slice(args);
        return Box::new(SLogger::new(ctx.as_slice()));
    }
}

