// The reference counting or Rc<T> smart pointer, is used to
// keep track of the number of references to a value,
// so that value can be droped only when all the references
// are unused.

// This is the recursive data type used in the Box<T> example
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// Let's create two lists (b and c) that share a common list (a).
// b -> [3] --/
//      a -> [5] -> [10] -> [Nil]
// c -> [4] --^
fn main() {
    // We use Rc<T> instead of Box<T> because we want the a list
    // to have two owners
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Doesn't clone the data, just increases the reference counting
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
    
    counting_references();
}

// Example to demonstrate how the Rc<T> smart pointer counts references
fn counting_references() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
