use core::fmt::Debug;
use std::{env, fmt};
use std::error::Error;

use slug::slugify;

struct CommandError {
    details: String,
}

impl Debug for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl CommandError {
    fn new(msg: &str) -> CommandError {
        CommandError { details: msg.to_string() }
    }
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

fn main() {
    println!("parse cli args");
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No arguments provided");
        return;
    }

    if (args.len() - 1) % 2 != 0 {
        println!("Invalid number of arguments, each param should have a value");
        return;
    }

    let mut x = 1;
    while x < args.len() - 1 {
        let res = process_input(args[x].to_string(), args[x + 1].to_string());
        match res {
            Ok(value) => println!("{}: {}, result: {}", args[x], args[x + 1], value),
            Err(error) => eprintln!("Error processing command: {}", error),
        }
        x += 2;
    }
}


fn process_input(command: String, input: String) -> Result<String, Box<dyn Error>> {
    return match command.to_lowercase().as_str() {
        "lowercase" => lowercase_input(input),
        "uppercase" => uppercase_input(input),
        "nospaces" => nospaces_input(input),
        "slugify" => slugify_input(input),
        "csv" => parse_csv(input),
        _ => Err(Box::new(CommandError::new("Unknown command"))),
    };
}

fn parse_csv(csv_input: String) -> Result<String, Box<dyn Error>> {
    validate_string(csv_input.clone())?;
    let rows: Vec<&str> = csv_input.split("\n").collect();
    let mut result: Vec<String> = Vec::with_capacity(rows.len());
    let mut headers: Vec<String> = rows[0].split(",").map(|s| s.to_string()).collect();

    let mut max_cols = 0;
    for row in rows.iter() {
        let cols: Vec<&str> = row.split(",").collect();
        if cols.len() > max_cols {
            max_cols = cols.len();
        }
    }

    if headers.len() < max_cols {
        for i in headers.len()..max_cols {
            headers.push(format!("MISSED_HEADER_{}", i + 1));
        }
    }
    for row_index in 1..rows.len() {
        let cols: Vec<&str> = rows[row_index].split(",").collect();
        let mut row = String::new();
        for col_index in 0..max_cols {
            if col_index < cols.len() {
                row.push_str(&format!("{}: {}\n", headers[col_index].replace(" ", ""), cols[col_index].replace(" ", "")));
            } else {
                row.push_str(&format!("{}: {}\n", headers[col_index].replace(" ", ""), "MISSING DATA"));
            }
        }
        result.push(row);
    }
    return Ok(result.join("\n"));
}

fn lowercase_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(input.to_lowercase());
}

fn uppercase_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(input.to_uppercase());
}

fn nospaces_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(input.replace(" ", ""));
}

fn slugify_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(slugify(input));
}

fn validate_string(input: String) -> Result<(), Box<dyn Error>> {
    if input.len() == 0 {
        return Err(Box::new(CommandError::new("Empty input")));
    }
    return Ok(());
}