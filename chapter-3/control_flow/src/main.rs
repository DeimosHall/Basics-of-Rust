use std::io;

fn main() {
    let age = 18;

    if age >= 18 {
        println!("You are an adult!");
    } else {
        println!("You are a minor!");
    }

    let mut input = String::new();
    println!("Please input your age: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let age: u32 = input.trim().parse().expect("Please type a number!");

    println!("You are {} years old", age);

    let condition = age >= 18;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    my_loop();
    nested_loops();
    while_loop();
    for_loop();
}

fn my_loop() {
    let mut counter = 0;

    loop {
        if counter == 10 {
            print!("{}", counter);
            break;
        } else {
            print!("{}, ", counter);
        }
        counter += 1;
    }
    println!("");
}

fn nested_loops() {
    let mut counter = 0;

    'loop_1: loop {
        println!("Loop 1");
        '_loop_2: loop {
            println!("Loop 2");
            if counter == 10 {
                break 'loop_1;
            }

            counter += 1;
        }
    }
}

fn while_loop() {
    let mut counter = 10;

    while counter > 0 {
        println!("{}", counter);
        counter -= 1;
    }
}

fn for_loop() {
    let array = [1, 2, 3, 4, 5];

    for element in array.iter() {
        print!("{}, ", element);
    }

    println!("");

    for number in (1..=4).rev() {
        print!("{}, ", number);
    }
    println!("");
}
