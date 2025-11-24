use std::{collections::HashMap, vec};

fn main() {
    // Storing Lists of Values with Vectors
    // No values means we have to specify the type
    let vector: Vec<i32> = Vec::new();
    println!("vector: {:?}", vector);

    // Infered i32 as default int type
    let mut vector = vec![2, 3, 5];
    println!("vector: {:?}", vector);
    println!("Adding a value...");
    vector.push(10);
    println!("vector: {:?}", vector);

    println!("Accesing elements by index");
    let first = &vector[0];
    println!("first element: {}", first);

    let second: Option<&i32> = vector.get(1);
    match second {
        Some(second) => println!("second: {}", second),
        None => println!("No value"),
    }
    let last: Option<&i32> = vector.get(vector.len() - 1);
    println!("last: {}", last.unwrap()); // Unsafe, never use unwrap

    let invalid: Option<&i32> = vector.get(vector.len());
    println!("invalid: {}", invalid.unwrap_or(&77)); // This is safe to use

    // The following methods crashes the program
    // let error = &vector[8];
    // let error = vector.get(1000).unwrap();

    // We can't get a value of a vector and later try to modify the vector
    vector.push(11);
    // println!("first element: {}", first);  // Uncoment to see the error

    // Iterating with inmutable references
    let mut vector = vec![10, 20, 30];
    println!("vector: {:?}", vector);
    for value in &vector {
        println!("value: {}", value);
    }

    println!("Mutable references");
    for value in &mut vector {
        *value += 100;
        println!("value: {}", value);
    }

    // Strings
    // Creation can be in different ways
    let text = "Hola".to_string();
    let text = String::from("Hola");
    let mut text = String::new();

    // We can add single chars
    text.push('H');

    // Or an entire string
    text.push_str("ola");
    println!("text: {}", text);

    // Concatenating strings
    let hola = String::from("Hola");
    let mundo = String::from("Mundo");
    let hello = hola + " " + &mundo; // hola is no longer avaible as we borrowed it here
    println!("{}", hello);
    // println!("{}", hola);  // Error here

    // We can also use format! to concatenate
    let hola = String::from("Hola");
    let mundo = String::from("Mundo");
    let hello = format!("{} {}", hola, mundo);
    println!("{}", hello);

    // Iterating a String
    let text = "Hola".to_string();
    for c in text.chars() {
        println!("{}", c);
    }
    
    // Hash maps
    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("red", 20);
    println!("\nHash maps\n");
    println!("scores: {:?}", scores);
    
    // Getting a value
    let score = scores.get("red");  // Returns an Option<&i32>
    println!("score: {:?}", score.copied().unwrap_or(0));
    
    // Overriding a value
    scores.insert("blue", 12);
    
    // Including a value only if it doesn't exist
    scores.entry("yellow").or_insert(31);
    scores.entry("blue").or_insert(5);  // This will not change
    
    // Iterating a HashMap
    scores.insert("green", 15);
    for (key, value) in &scores {
        println!("key: {}, value: {}", key, value);
    }
    
    // Look up at key's value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // Counting word's occurrences
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
