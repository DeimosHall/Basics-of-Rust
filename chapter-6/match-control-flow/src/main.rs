use std::fmt::format;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Florida,
}


impl UsState {
    // Check the age of a state
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            _ => false,
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin: Coin = Coin::Dime;
    println!("Coin value is {}", value_in_cents(coin));

    let california_coin = Coin::Quarter(UsState::California);
    println!("Coin value is {}", value_in_cents(california_coin));

    // Using match to perform operations
    let number: Option<i32> = Some(5);
    println!("{number:?} + 1 is equals to {:?}", plus_one(number));

    // No operation here
    let number: Option<i32> = None;
    println!("{:?} + 1 is equals to {:?}", number, plus_one(number));

    // If we want to run code only for Some
    println!("Using if let");
    let config_max: Option<u8> = Some(8);
    match config_max {
        Some(value) => println!("We are here, value: {value}"),
        _ => (),
    }

    // Another way is to use `if let`
    let config_max: Option<u8> = Some(9);
    // If config_max is Some, then we have it's value
    if let Some(value) = config_max {
        println!("We are also here, value: {value}");
    }

    let quarter = Coin::Quarter(UsState::Alaska);
    println!("{:?}", describe_state_quarter_v1(quarter));

    println!("Using v2");
    let quarter = Coin::Quarter(UsState::Alabama);
    println!("{:?}", describe_state_quarter_v2(quarter));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("\tThe state of the quarter is {state:?}");
            25
        },
    }
}

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn describe_state_quarter_v1(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old for America"))
        } else {
            Some(format!("{state:?} is relatively new"))
        }
    } else {
        None
    }
}

fn describe_state_quarter_v2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state // Return the state to the variable
    } else {
        return None
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for America"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}
