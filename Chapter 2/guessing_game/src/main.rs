// Input / Output library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // What happen if can't read line
            
        // This is used when it's needed to change the type, no need to create a new variable
        let guess: u32 = match guess.trim().parse() {
            // num is produced by parse()
            Ok(num) => num, // Returns num
            Err(_) => {
                println!("Invalid number! Please try again!");
                continue; // Starts a new iteration ignoring the rest of the code
            }
        };

        // Takes a reference and a value to compare
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You guessed: {guess}");
                break;
            }
        }
    }
}
