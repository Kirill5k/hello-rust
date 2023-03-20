use std::env;
use std::process;

use hello_rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = hello_rust::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}