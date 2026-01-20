// Smart pointers are date structures that act like a pointer,
// but with additional metadata and capabilties.
//
// References usually only borrow data.
// Smart pointers usually own their data.
//
// We use a struct that implement the `Deref` and `Drop`
// traits.

pub fn run() {
    // Variables are stored in the stack
    let var_on_stack = 5;
    println!("var on stack: {}", var_on_stack);

    // But we can use the Box smart pointer to store them
    // on the heap. For example, for large arrays.
    let var_on_heap = Box::new(5);
    println!("var on heap: {}", var_on_heap);
    
    recursive_data_type();
}

// A use case for storing data on the heap can be on
// recursive data types. This is because Rust can't
// know how much memory a recursive data type can
// take, because it could be infinite.

// If we uncomment this code, the compiler will tell us
// it has an infinite size.
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// The solution is to store it on the heap, because it
// will take a know fixed size.
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),  // Recursive data type
    Nil, // Empty list
}

use crate::List::{Cons, Nil};

fn recursive_data_type() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);
}