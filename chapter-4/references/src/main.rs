fn main() {
    // Modify a value without return
    let mut number = 5;
    println!("number: {number}");
    plus_two(&mut number);
    println!("number: {number}");

    // Avoid function to taking ownership
    let mut s1 = String::from("Hello World!");
    let len = calculate_length(&mut s1);
    println!("{s1} length is {len}");

    let only_one_borrow = String::from("One");
    let first_borrow = &only_one_borrow;
    let second_borrow = &only_one_borrow;
    println!("{first_borrow}, {second_borrow}");

    let mut only_one_borrow = String::from("Two");
    let first_borrow = &mut only_one_borrow;
    // let second_borrow = &mut only_one_borrow; // There can only be one mutable borrow, so this is not valid
    println!("{first_borrow}, {second_borrow}");
}

fn plus_two(number: &mut i32) {
    *number = *number + 2;
}

fn calculate_length(s: &mut String) -> usize {
    *s = s.to_uppercase();
    s.len()
}
