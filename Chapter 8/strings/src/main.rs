fn main() {
    let mut new_string = String::new();
    new_string = String::from("Hello ");
    new_string.push_str("World!");

    println!("{}", new_string);

    let another_string = String::from(" I'm learning Rust");
    let result: String = new_string + &another_string;
    println!("{}", result);
    // new_string is not available because result has taken ownership on it
    //println!("{}", new_string);
    let like: String = String::from("and I like it");
    // format!() is a way to concatenate Strings and it doesn' take
    // ownership on the parameters, so we can use like or result after the
    // assingment
    let second_result: String = format!("{}, {}", result, like);
    println!("{}", second_result);
    println!("{}", like);

    // Iterate a String
    for c in like.chars() {
        println!("{}", c);
    }
}
