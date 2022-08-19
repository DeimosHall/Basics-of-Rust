use std::fs::File;
use std::io::ErrorKind;

#[allow(dead_code)]
fn main() {
    /*
    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // ErrorKind contains posible error of io operations
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file due to: {:?}", e),
            },
            other_error => {
                panic!("Couldn't open file due to error: {:?}", other_error);
            },
        },
    };

    // Another way to do the same is with closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    }); */

    // If result is Ok, unwrap will return the ok, but if result is Err,
    // unwrap will call the panic! for us
    //let f = File::open("hello.txt").unwrap();
    // We can show the error message we want with expect
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
