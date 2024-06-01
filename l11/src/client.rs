use std::io::{self, Read, Write};
use std::net::TcpStream;

use pkg::logger::abstract_logger::AppLogger;
use pkg::logger::logger::SimpleLogger;
use pkg::utils;

use crate::pkg::logger::abstract_logger::StringWith;

mod pkg;

fn main() {
    let logger: Box<dyn AppLogger> = Box::new(SimpleLogger::new(&[]));
    let address = utils::env::get_connect_params();
    let logger = logger.with(&[StringWith::new("remote_address", &address)]);

    logger.info("spawning client", &[]);
    match TcpStream::connect(&address) {
        Ok(mut stream) => {
            logger.info("connected", &[]);
            loop {
                let mut input = String::new();
                print!("Enter message: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                let trimmed_input = input.trim().to_string();
                if trimmed_input == "quit" || trimmed_input == "q" {
                    println!("Quitting...");
                    break;
                } else if trimmed_input.starts_with("file") {
                    // let file_path = get_file_from_input(trimmed_input);
                    // match send_file(&stream, "file", &file_path) {
                    //     Ok(_) => println!("File sent successfully."),
                    //     Err(e) => eprintln!("Failed to send file: {}", e),
                    // }
                } else if trimmed_input.starts_with("image") {
                    // let file_path = get_file_from_input(trimmed_input);
                    // match send_file(&stream, "image", &file_path) {
                    //     Ok(_) => println!("File sent successfully."),
                    //     Err(e) => eprintln!("Failed to send file: {}", e),
                    // }
                } else {
                    if let Err(e) = stream.write_all(input.as_bytes()) {
                        logger.error("failed to send message: {}", e, &[]);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            logger.error("could not connect", e, &[]);
        }
    }
}