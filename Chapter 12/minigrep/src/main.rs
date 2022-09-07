use std::env;
// std::env::args_os can receive arguments with invalid Unicode
// fs is to manage file system
use std::fs;

fn main() {
    // collect() converts an iterator into a vector
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    println!("{:?}", args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    println!("With text: \n{}", contents);
}
