use std::env;
use std::error::Error;
use std::fs::File;

use slug::slugify;

use crate::CommandContainer;
use crate::types::{Command, CommandError};

pub fn process_command(data: CommandContainer) -> Result<String, Box<dyn Error>> {
    return match data.command {
        Command::Lowercase => lowercase_input(data.input),
        Command::Uppercase => uppercase_input(data.input),
        Command::Nospaces => nospaces_input(data.input),
        Command::Slugify => slugify_input(data.input),
        Command::CSV => parse_csv(data.input),
    };
}

pub fn lowercase_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(input.to_lowercase());
}

pub fn uppercase_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(input.to_uppercase());
}

pub fn nospaces_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(input.replace(" ", ""));
}

pub fn slugify_input(input: String) -> Result<String, Box<dyn Error>> {
    validate_string(input.clone())?;
    return Ok(slugify(input));
}

pub fn validate_string(input: String) -> Result<(), Box<dyn Error>> {
    if input.len() == 0 {
        return Err(Box::new(CommandError::new("Empty input")));
    }
    return Ok(());
}

pub fn parse_csv(input: String) -> Result<String, Box<dyn Error>> {
    let path = env::current_dir()?;
    let file_name = format!("{}/{}.csv", path.display(), input.trim().replace(".csv", ""));
    println!("try read file: {}", file_name);

    let file = File::open(file_name)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut result: Vec<String> = Vec::with_capacity(1_000);
    
    for row in rdr.records() {
        let record = row?;
        let rows: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        result.push(rows.join(","));
    }
    return Ok(result.join("\n"));
}