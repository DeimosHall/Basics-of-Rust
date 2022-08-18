use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Key - value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Another way to create a HashMap
    let teams = vec![String::from("Red"), String::from("Black")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_,_> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Accesing values
    let teams_name = String::from("Red");
    let score = scores.get(&teams_name);

    print_scores(&scores);

    // Overwriting a value
    // We need to insert the same key and a new value
    scores.insert(String::from("Red"), 100);
    print_scores(&scores);
}

fn print_scores(scores: &HashMap<String,i32>) {
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}