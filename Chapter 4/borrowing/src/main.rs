fn main() {
    let mut s1 = String::from("hello");
    // &s1 is a reference to the value of the String
    // References don't take ownership of the value
    // Creating a reference is called borrowing
    let mut len = calculate_length(&s1);

    println!("The lenght of {} is {}", s1, len);

    change(&s1);
    change_mut(&mut s1);
    len = calculate_length(&s1);
    println!("The lenght of {} is {}", s1, len);

    // There can be only one mutable reference
    let r1 = &mut s1;
    //let _r2 = &mut s1;
    println!("r1 = {}", r1);
    //println!("r2 = {}", _r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// References are inmutable, so this function can't change the value of the reference
fn change(_s: &String) {
    //_s.push_str(" world");
}

// If we want to change the reference value, we need to make it mutable
fn change_mut(s: &mut String) {
    s.push_str(" world");
}
