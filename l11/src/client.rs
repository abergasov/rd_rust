use std::io::{Read, Write};
use std::net::TcpStream;

use pkg::logger::abstract_logger::AppLogger;
use pkg::logger::logger::SimpleLogger;
use pkg::utils;

use crate::pkg::logger::abstract_logger::StringWith;
use crate::pkg::service::client::handler::handle_connection;

mod pkg;

fn main() {
    let logger: Box<dyn AppLogger> = Box::new(SimpleLogger::new(&[]));
    let address = utils::env::get_connect_params();
    let logger = logger.with(&[StringWith::new("remote_address", &address)]);

    logger.info("spawning client", &[]);
    match TcpStream::connect(&address) {
        Ok(stream) => handle_connection(logger, stream),
        Err(err) => logger.fatal("could not connect", err, &[]),
    }
}