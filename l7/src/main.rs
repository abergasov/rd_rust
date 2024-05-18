use std::{
    io::stdin,
    process,
    str::FromStr,
    thread,
};
use std::error::Error;
use std::sync::{Arc, mpsc::channel};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::functions::process_command;
use crate::types::Command;

mod types;
mod functions;

struct CommandContainer {
    command: Command,
    input: String,
}

impl CommandContainer {
    pub fn new(input: &str) -> Result<CommandContainer, Box<dyn Error>> {
        let data: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
        let cmd = Command::from_str(data[0].trim())?;
        Ok(CommandContainer {
            command: cmd,
            input: data[1..].join(" "),
        })
    }
}

fn main() {
    let (tx, rx) = channel::<CommandContainer>();
    let running = Arc::new(AtomicBool::new(true));

    // graceful shutdown
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Shutting down...");
        r.store(false, Ordering::SeqCst);
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");


    let sender_thread = thread::spawn({
        let running = running.clone();
        move || {
            while running.load(Ordering::SeqCst) {
                let mut input = String::new();
                stdin().read_line(&mut input).expect("bad input");
                let cmd = CommandContainer::new(&input);
                match cmd {
                    Ok(value) => {
                        if tx.send(value).is_err() {
                            break;
                        }
                    }
                    Err(error) => eprintln!("error processing command: {}, ignoring", error),
                }
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    });

    let processing_thread = thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            if let Ok(value) = rx.recv() {
                let res = process_command(value);
                if let Ok(data) = res {
                    println!("{}", data);
                } else {
                    eprintln!("error processing command: {}", res.err().unwrap());
                }
            } else {
                break;
            }
        }
    });

    sender_thread.join().expect("The sender thread has panicked");
    processing_thread.join().expect("The receiver thread has panicked");
    println!("all exited!")
}
