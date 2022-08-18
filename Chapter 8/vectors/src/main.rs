enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // A vector can grow or shrink in size
    // It can be only one type of data
    let vector: Vec<i32> = Vec::new();
    // It's not needeed to specify the type once Rust can infer the type
    // Creating a new vector with a macro
    let mut another_vector: Vec<i32> = vec![1, 2, 3, 4];

    // We can add elements with push
    another_vector.push(5);
    another_vector.push(6);
    another_vector.push(7);

    println!("another_vector = {:#?}", another_vector);

    // There are two ways to access elements in the vector
    let third_element: &i32 = &another_vector[2];
    println!("third_element = {}", third_element);

    match another_vector.get(2) {
        Some(third) => println!("third_element = {}", third),
        None => println!("There is no third element"),
    }

    // Iterating with loop
    let size: i32 = 7;
    print!("another_vector = ");
    for i in &another_vector {
        match i == &size {
            false => print!("{}, ", i),
            true => println!("{}", i),
        }
    }

    // Mutable iteration
    print!("another_vector = ");
    for i in &mut another_vector {
        *i += 1;
        match i == &size {
            false => print!("{}, ", i),
            true => println!("{}", i),
        }
    }
}
