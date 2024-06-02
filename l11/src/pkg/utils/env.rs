use std::env;

pub fn get_connect_params() -> String {
    let default_host = &String::from("localhost");
    let default_port = &String::from("11111");

    let args: Vec<String> = env::args().collect();
    let hostname = args.get(1).unwrap_or(default_host);
    let port = args.get(2).unwrap_or(default_port);
    return format!("{}:{}", hostname, port);
}