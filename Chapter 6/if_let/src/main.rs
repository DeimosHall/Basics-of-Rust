fn main() {
    let config_max = Some(3u8);
    // When we use match we need to especify all the posibilities
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // When we only want to run code that matches an speccific option
    // we can use 'if let' instead of match
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
