fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("{s}");
    } // this scope is now over, and s is no longer valid
    // println!("{s}"); // Uncoment to see the error

    let mut s = String::from("Hello");
    s.push_str(", world :D");
    println!("{s}");
    let s = s.to_uppercase();
    println!("{s}");

    let x = 5;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");

    let x = String::from("5");
    let y = x; // It does not copy the value, it gives it to `y` and now `x` is dropped
    // println!("x: {x}"); // Uncoment to see the error
    println!("y: {y}");

    let x = String::from("Value");
    println!("x: {x}");
    takes_ownership(x);
    // println!("x: {x}"); // Uncoment to see the error
    let y = 10;
    makes_copy(y);

    let x = String::from("Hello World");
    println!("{x}");
    let y = takes_and_gives(x);
    // println!("{x}"); // x is no longer available since takes_and_gives has taken ownership of its value
    println!("{y}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(number: i32) {
    println!("{number}");
}

fn takes_and_gives(some_string: String) -> String {
    let mut extra = some_string;
    extra.push_str("!");
    extra
}
