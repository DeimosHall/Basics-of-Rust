use std::env;
// std::env::args_os can receive arguments with invalid Unicode
use std::fs;

fn main() {
    // collect() converts an iterator into a vector
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
<<<<<<< HEAD

    let query = &args[1];
    let file_path = &args[2];

    println!("{}, {}", query, file_path);
=======
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
>>>>>>> 571f78d9479ea0f3913958cc322df95441e1c29a
}
