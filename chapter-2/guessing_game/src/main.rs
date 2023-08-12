use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // println!("You guessed: '{guess}'"); // guess contains "\n" here
    
        // trim spaces and parse to u8, it is inferred from the variable type
        // let guess: u8 = guess.trim().parse().expect("Please type a positive number!");
        let guess: u8 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("'{}' is not a positive number, please type one!", guess.trim());
                continue;
            },
        };
    
        println!("You guessed: '{guess}'");
    
        // Compare guess and secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
               break; // Exit the game
            },
        }
    }
}
