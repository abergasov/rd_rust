use std::env;
use std::io::stdin;

use slug::slugify;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("bad input");
    println!("You typed: {}", input);

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
    loop {
        if x > args.len() - 1 {
            break;
        }
        println!("{}: {}, result: {}", args[x], args[x + 1], processInput(args[x].to_string(), args[x + 1].to_string()));
        x += 2;
    }
}

fn processInput(command: String, input: String) -> String {
    return match command.to_lowercase().as_str() {
        "lowercase" => input.to_lowercase(),
        "uppercase" => input.to_uppercase(),
        "nospaces" => input.replace(" ", ""),
        "slugify" => slugify(input),
        _ => "Invalid command".to_string(),
    };
}
