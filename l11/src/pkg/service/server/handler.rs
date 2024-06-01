use std::{fs, fs::File, io::Read, io::self, io::Write};
use std::net::TcpStream;

use crate::pkg::logger::abstract_logger::AppLogger;

pub fn handle_client(log: Box<dyn AppLogger + Send + Sync>, mut stream: TcpStream) -> io::Result<()> {
    loop {
        let mut buffer = Vec::new();
        let mut temp_buffer = [0; 512];

        match stream.read(&mut temp_buffer) { // Read by mask FILE%FILE_TYPE%FILE_LENGTH%
            Ok(0) => {
                log.info("connection was closed", &[]);
                break; // connection was closed
            }
            Ok(n) => {
                buffer.extend_from_slice(&temp_buffer[..n]);

                // FILE first_pos - file_type - second_pos - file_length - third_pos - file_content
                let first_pos = buffer.iter().position(|&c| c == b'%');
                if first_pos.is_none() || first_pos == Some(0) {
                    print!("none file command\n");
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                    if let Err(e) = stream.write_all(&buffer[..n]) {
                        eprintln!("Failed to send response: {}", e);
                    }
                    continue;
                }
                let first_pos = first_pos.unwrap();
                let payload_type = String::from_utf8_lossy(&buffer[..first_pos]).to_string();
                if payload_type != "FILE" {
                    eprintln!("Invalid message format.");
                    continue;
                }

                let second_pos = buffer[first_pos + 1..].iter().position(|&c| c == b'%');
                if second_pos.is_none() || second_pos == Some(0) {
                    continue;
                }
                let second_pos = first_pos + second_pos.unwrap() + 1;

                let third_pos = buffer[second_pos + 1..].iter().position(|&c| c == b'%');
                if third_pos.is_none() || third_pos == Some(0) {
                    continue;
                }
                let third_pos = second_pos + third_pos.unwrap() + 1;


                let file_type = String::from_utf8_lossy(&buffer[first_pos + 1..second_pos]).to_string();
                let payload_size = String::from_utf8_lossy(&buffer[second_pos + 1..third_pos]).to_string();

                let payload_size = payload_size.parse::<usize>();
                if payload_size.is_err() {
                    eprintln!("Invalid message format.");
                    continue;
                }
                let payload_size = payload_size.unwrap();

                let mut payload_content = Vec::with_capacity(payload_size);
                let mut read_bytes = buffer[third_pos + 1..].to_vec();
                payload_content.append(&mut read_bytes);
                while payload_content.len() < payload_size {
                    let mut chunk = vec![0; 512];
                    match stream.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => payload_content.extend_from_slice(&chunk[..n]),
                        Err(err) => {
                            log.error("failed to read from socket: {}", err, &[]);
                            break;
                        }
                    }
                }
                if payload_content.len() == payload_size {
                    fs::create_dir_all(format!("/tmp/{}", file_type))?;
                    let file_name = format!("/tmp/{}/received_{}.txt", file_type, file_type);
                    let mut file = File::create(file_name)?;
                    file.write_all(&payload_content)?;
                    println!("File received successfully.");
                    process_message(&mut stream, &file_type);
                } else {
                    log.info("received file length does not match the specified length.", &[]);
                }
            }
            Err(err) => {
                log.error("failed to read from socket", err, &[]);
                break;
            }
        }
    }
    Ok(())
}

fn process_message(stream: &mut TcpStream, message: &str) {
    let response = if message.starts_with("file") {
        "Received file".to_string()
    } else if message.starts_with("image") {
        "Received image".to_string()
    } else {
        println!("Received: {}", message);
        format!("Received: {}", message)
    };

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}
