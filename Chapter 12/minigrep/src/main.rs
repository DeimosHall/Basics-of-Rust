use std::env;
// std::env::args_os can receive arguments with invalid Unicode
// fs is to manage file system
use std::fs;
use std::process;

fn main() {
    // collect() converts an iterator into a vector
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    //println!("{:?}", args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    
    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    /*
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    } */

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}