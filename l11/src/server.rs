use std::{io, io::Read, thread};
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
                let peer = stream.peer_addr();
                if peer.is_err() {
                    logger.error("failed to get peer address", peer.err().unwrap(), &[]);
                    continue;
                }
                let peer_logger = logger.with(&[StringWith::new(
                    "peer", &peer.unwrap().to_string(),
                )]);
                peer_logger.info("accepted connection", &[]);
                thread::spawn(|| handle_client(peer_logger, stream));
            }
            Err(err) => {
                logger.error("failed to accept connection", err, &[]);
            }
        }
    }
}

fn handle_client(log: Box<dyn AppLogger + Send + Sync>, mut stream: TcpStream) -> io::Result<()> {
    loop {
        let mut buffer = Vec::new();
        let mut temp_buffer = [0; 512];
        match stream.read(&mut temp_buffer) { // Read by mask FILE%FILE_TYPE%FILE_LENGTH%
            Ok(0) => {
                log.info("connection closed", &[]);
                break;
            }
            Err(e) => {
                log.error("failed to read from socket", e, &[]);
                break;
            }
            Ok(n) => {
                buffer.extend_from_slice(&temp_buffer[..n]);
                log.info("received", &[]);
            }
        }
    }
    Ok(())
}