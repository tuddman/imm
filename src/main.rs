use std::env;
use std::process;

use pimm::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = pimm::run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}
