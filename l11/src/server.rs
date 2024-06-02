use std::{io::Read, thread};
use std::net::TcpListener;

use pkg::logger::abstract_logger::AppLogger;
use pkg::logger::slog::SLogger;

use crate::pkg::logger::abstract_logger::StringWith;
use crate::pkg::service::server::handler::handle_client;
use crate::pkg::utils::env;

mod pkg;

fn main() {
    let logger: Box<dyn AppLogger> = Box::new(SLogger::new(&[]));
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
            Err(err) => logger.error("failed to accept connection", err, &[]),
        }
    }
}
