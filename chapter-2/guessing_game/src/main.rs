use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: u8,
}

// Only returns the type if the condition is met
impl Guess {
    pub fn new(value: u8) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

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
        // let guess: u8 = match guess.trim().parse() {
        //     Ok(number) => number,
        //     Err(_) => {
        //         println!("'{}' is not a valid number, please type one!", guess.trim());
        //         continue;
        //     },
        // };
        
        let guess: Guess = Guess::new(guess.trim().parse().expect("Please type a valid number"));
    
        println!("You guessed: '{}'", guess.value());
    
        // Compare guess and secret number
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
               break; // Exit the game
            },
        }
    }
}
