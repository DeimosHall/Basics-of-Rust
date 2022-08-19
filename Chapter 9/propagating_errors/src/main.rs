use std::fs::File;
use std::fs;
use std::io::{self, Read};

fn main() {
    
}

// Return type is Result<T, E>
/*
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
} */

// A shortcut with ?
fn read_username_from_file() -> Result<String, io::Error> {
    // With ? if the result is Ok, Ok value will be returned from the expression
    // If the result is Err, Err will be returned to the calling function as we
    // had used return
    // The error returned by the ? will be converted into the type especified in
    // the returned type in the function

    /*
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) */

    // This can be written in a shorter way
    /*
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s) */

    // In a more shorter way
    fs::read_to_string("hello.txt")
}