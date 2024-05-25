use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::Path;

fn main() {
    let default_host = &String::from("localhost");
    let default_port = &String::from("11111");

    let args: Vec<String> = env::args().collect();
    let hostname = args.get(1).unwrap_or(default_host);
    let port = args.get(2).unwrap_or(default_port);
    let address = format!("{}:{}", hostname, port);

    match TcpStream::connect(&address) {
        Ok(mut stream) => {
            println!("Connected to the server at {}", address);
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
                    let file_path = get_file_from_input(trimmed_input);
                    match send_file(&stream, "file", &file_path) {
                        Ok(_) => println!("File sent successfully."),
                        Err(e) => eprintln!("Failed to send file: {}", e),
                    }
                } else if trimmed_input.starts_with("image") {
                    let file_path = get_file_from_input(trimmed_input);
                    match send_file(&stream, "image", &file_path) {
                        Ok(_) => println!("File sent successfully."),
                        Err(e) => eprintln!("Failed to send file: {}", e),
                    }
                } else {
                    if let Err(e) = stream.write_all(input.as_bytes()) {
                        eprintln!("Failed to send message: {}", e);
                        break;
                    }
                }

                println!("Waiting for server response...");
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        println!("Connection closed by server");
                        break;
                    }
                    Ok(n) => {
                        println!("Server response: {}", String::from_utf8_lossy(&buffer[..n]));
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

fn get_file_from_input(input: String) -> String {
    if input.starts_with("file") {
        return input.trim_start_matches("file").trim().to_string();
    }
    if input.starts_with("image") {
        return input.trim_start_matches("image").trim().to_string();
    }
    // return String::new();
    return "/home/alejandro/MainAfter.jpg".to_string(); // for debug simplicity
}

fn send_file(mut stream: &TcpStream, file_type: &str, file_path: &String) -> io::Result<()> {
    if !Path::new(file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;
    let file_length = file_content.len();
    let message = format!("FILE%{}%{}%", file_type, file_length);
    stream.write_all(message.as_bytes())?;
    stream.write_all(&file_content)?;
    Ok(())
}