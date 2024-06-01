use std::{io, thread};
use std::net::{TcpListener, TcpStream};

use pkg::logger::abstract_logger::AppLogger;
use pkg::logger::logger::SimpleLogger;

use crate::pkg::logger::abstract_logger::StringWith;
use crate::pkg::utils::env;

mod pkg;

fn main() {
    let logger: Box<dyn AppLogger> = Box::new(SimpleLogger::new(&[]));
    let address = env::get_connect_params();
    let logger = logger.with(&[StringWith::new("address", &address)]);

    logger.info("server starting", &[]);
    let listener = TcpListener::bind(&address).expect("Could not bind");
    logger.info("server listening", &[]);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                logger.error("Failed to accept connection", err, &[]);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {}