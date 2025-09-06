#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Florida,
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
