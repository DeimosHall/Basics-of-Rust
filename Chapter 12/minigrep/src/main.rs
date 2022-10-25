// std::env::args_os can receive arguments with invalid Unicode
use std::{env, process};

use minigrep::Config;

fn main() {
    // collect() converts an iterator into a vector
    //let args: Vec<String> = env::args().collect();
    //dbg!(args);
    //println!("{:?}", args);

    /* If build returns an Ok value, it is wrapping a Config instance,
     * and the unwrap_or_else function can unwrap the content to return
     * the Config.
     * If build returns and Err value, the code in the closure will be
     * exected.
    */
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Expected: word file.txt");
        // A nonzero value means the program finishes with a problem
        process::exit(1);
    });

    /*
     * The code will be executed only if run returns an Err value.
     * We do not use unwrap_or_else here because we do not care about
     * if the result was an Ok, because it is () that indicates we only
     * care if there was a problem.
     */
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}