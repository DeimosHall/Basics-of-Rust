#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
fn main() {
    let value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value: {} cents", value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // UsState::Alaska is bind to state
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}