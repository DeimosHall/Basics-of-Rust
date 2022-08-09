fn main() {
    // integers are allocated in the stack because they have a known size
    let x = 3;
    let y = x;
    println!("x = {x}");
    println!("y = {y}");

    // Strings don't have a known size at compile time and can't be allocated directly in the stack
    let s1 = String::from("Hello");
    let s2 = s1;
    // s1 has been dropped and it's not available after it's no longer the owner of "Hello"
    //println!("s1 = {s1}");
    println!("s2 = {s2}");

    let s3 = String::from("Hello");
    // Is needed to specify we want to clone the content of s3 to s4
    let s4 = s3.clone();
    println!("s3 = {s3}");
    println!("s4 = {s4}");

    // Ownership with functions
    let s5 = String::from("Bye");
    takes_ownership(s5);
    // takes_ownership now is the owner of "Bye" and it's not posible to use it later
    //println!("s5 = {s5}");
}

fn takes_ownership(s: String) {
    println!("s5 = {}", s);
}
