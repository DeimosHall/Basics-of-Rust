use std::env;
// std::env::args_os can receive arguments with invalid Unicode
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let query = &args[1];
    let file_path = &args[2];

    println!("{}, {}", query, file_path);
}
