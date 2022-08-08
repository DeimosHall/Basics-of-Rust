// This file corresponds to Control Flow section in chapter 3

use std::io;

fn main() {    
    //is_adult();
    //loop_loop();
    //another_loop();
    //while_loop();
    //array_with_while();
    //array_with_for();
    range_with_for();
}

fn is_adult() {
    if get_age() >= 18 {
        println!("You are an adult");
    } else {
        println!("You are a young");
    }
}

fn get_age() -> u32 {
    println!("Enter your age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Unable to read age");
    
    match age.trim().parse() {
        Ok(age) => return age,
        Err(_) => return 0,
    };
}

fn loop_loop() {
    println!("First loop");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Returned value
        }
    };

    println!("Result: {}", result);
    println!("");
}

fn another_loop() {
    println!("Second loop");
    let mut count = 0;

    // This is a label to edentify the loop
    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {}", count);
    println!("");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn array_with_while() {
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < array.len() {
        println!("Value: {}", array[index]);
        index += 1;
    }
}

fn array_with_for() {
    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("Value: {element}");
    }
}

fn range_with_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}