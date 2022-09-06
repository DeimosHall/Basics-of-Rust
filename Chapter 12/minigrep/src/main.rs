use std::env;
// std::env::args_os can receive arguments with invalid Unicode

fn main() {
    // collect() converts an iterator into a vector
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
