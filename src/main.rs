use std::env;
use std::process;

use mapping::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    mapping::process_input(&config.file).unwrap_or_else(|err| {
        println!("Problem processing input {}", err);
        process::exit(1);
    });
}


