use std::env;
// std::env::args_os can receive arguments with invalid Unicode
// fs is to manage file system
use std::fs;

fn main() {
    // collect() converts an iterator into a vector
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    //println!("{:?}", args);

    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}