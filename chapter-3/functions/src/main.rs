fn main() {
    println!("Hello, world!");
    another_function();
    print_number(15);
    print_number(67);
    print_user_and_age("Pablo".to_string(), 25);

    // Creating a variable is an statement, it doesn't return a value
    let _x = 5;
    // That's why it's not possible to create variables in one line like in C
    // let _y: u32 = let _z = 5;

    // Expressions return a value, for example a function call or a block of {} code
    let y = {
        let x = 3;
        x + 1 // No semicolon at the end of the expression
    };
    
    // println! is a macro, not a function, so it doesn't return a value
    println!("The value of y is: {}", y);

    // Functions with return values
    let six = six();
    println!("The value of six is: {}", six);

    // Function with a void parameter
    say_hello(());
}

fn another_function() {
    println!("Another function text!");
}

fn print_number(number: u32) {
    println!("Your number is {number}");
}

fn print_user_and_age(user: String, age: u32) {
    println!("User: {user}, Age: {age}");
}

fn six() -> u32 {
    6
}

// () is the unit type, it's like void in C
fn say_hello(_: ()) -> () {
    println!("Hello!");
}
