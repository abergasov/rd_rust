use std::net::TcpStream;

use pkg::logger::abstract_logger::AppLogger;
use pkg::logger::logger::SimpleLogger;
use pkg::utils::env;

use crate::pkg::logger::abstract_logger::StringWith;

mod pkg;

fn main() {
    let logger: Box<dyn AppLogger> = Box::new(SimpleLogger::new(&[]));
    let address = env::get_connect_params();
    let logger = logger.with(&[StringWith::new("address", &address)]);

    logger.info("spawning client", &[]);
    match TcpStream::connect(&address) {
        Ok(mut stream) => {}
        Err(e) => {
            logger.error("could not connect", e, &[]);
        }
    }
}