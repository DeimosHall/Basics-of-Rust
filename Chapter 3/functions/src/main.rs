use std::io;
use std::any;

fn main() {
    say_hello(get_name());

    expression();
}

fn get_name() -> String {
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Unable to read name");
    name
}

fn say_hello(name: String) {
    println!("Hello {}", name);
}

fn expression() {
    // Statments have a semicolon and don't return its value
    let _statment = 5;
    let y = {
        let x = 3;
        x + 1 // Expressions don't have a semicolon and return its value
    };
    println!("Expression value: {}", y);
}
