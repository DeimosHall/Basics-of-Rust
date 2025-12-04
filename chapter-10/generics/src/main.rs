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
    
    println!("Using generic structs");
    let integer = generics::Point { x: 5, y: 10 };
    println!("x: {}, y: {}", integer.x, integer.y);
    let float = generics::Point { x: 5.1, y: 10.3 };
    println!("x: {}, y: {}", float.x, float.y);
    
    // We can't create a generics point with different
    // data types because they share the same T type
    // let wont_work = Point { x: 5, y: 4.0 };
    
    // To acheive the previous behavior, we can use a Point<T, U>
    let will_work = generics::Another::Point { x: 5, y: 4.0 };
    println!("x: {}, y: {}", will_work.x, will_work.y);
    
    // We can also implement functions to generic data types, for example
    // a getter to a Point field
    println!("x value: {}", will_work.x());
    
    // We can have specific behavior for specific types, for example, a method
    // that returns hello only on Point<f32, f32> types
    let will_work = generics::Another::Point { x: 1.2, y: 2.1 };
    println!("Say {}", will_work.hello());
    let wont_work = generics::Another::Point { x: 1.5, y: 2 };
    // println!("Say {}", wont_work.hello());
}
