use std::env;
use std::process;

use hello_rust::Config;

fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = hello_rust::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
