use std::io::Read;
use std::{fs::File, io, io::ErrorKind};

fn main() {
    println!("Hello, world!");
    // Intentional panic
    // panic!("Crashing the program");

    // Undesired panic
    let vector = vec![1, 2, 3];
    // let _ = vector[4];  // Uncomment

    // Recoverable errors with Result
    let file = File::open("hello.txt");

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => panic!("error: {}", error),
    // };

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("error: {}", e),
            },
            error => panic!("Problem opening the file: {}", error),
        },
    };

    println!("File: {:?}", file);

    // An alternative to match is the unwrap_or_else closure
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap calls panic if it fails
    // let file = File::open("error.txt").unwrap();

    // expect is the same as unwrap but with a custom message
    // let file = File::open("error.txt").expect("Expected error occured");

    // let result = propagate_error();
    // let result = propagate_error_with_question_mark();
    // println!("Result: {:?}", result);

    let result = read_username_from_file();
    println!("Result: {:?}", result);
}

// Propagating errors instead of panic
fn propagate_error() -> Result<u32, std::io::Error> {
    let vector = vec![1, 2, 3];
    match vector.get(4) {
        Some(value) => Ok(*value),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Index out of bounds",
        )),
    }
}

// Propagating errors using ?
fn read_username_from_file() -> Result<String, io::Error> {
    // ? is similar to a match, returns the value, but it 
    // propagates the error instead of handling it
    let mut username_file = File::open("hello.txt")?;
    // let mut username_file = File::open("error.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
