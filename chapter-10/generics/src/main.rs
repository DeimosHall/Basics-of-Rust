mod no_generics;
mod generics;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    
    // Find the largest number in the list
    let mut largest = &number_list[0];
    
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    
    println!("The largest number is {}", largest);
    
    // If we want to do the same for other lists, we
    // probably have to repeat the code or better,
    // write a function to do so
    
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("The largest number is {}", no_generics::find_largest_number(&number_list));
    
    // Let's say we want to find the largest of two different types,
    // [i32] and [char], we could do something like this in the
    // no_generics module
    let i32_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['h', 'o', 'l', 'a'];
    println!("Without generics:");
    println!("The largest number is {}", no_generics::largest_i32(&i32_list));
    println!("The largest char is {}", no_generics::largest_char(&char_list));
    
    println!("With generics:");
    println!("The largest number is {}", generics::find_largest(&i32_list));
    println!("The largest char is {}", generics::find_largest(&char_list));
}
