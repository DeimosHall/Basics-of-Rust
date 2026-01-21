use std::ops::Deref;

pub fn run() {
    let x = 5;
    let y = &x; // It doesn't equals 5, it is a reference to the value
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // We derefence here to get the value
    
    let x = 5;
    let y = Box::new(x); // Instance of a box of a copied value of x
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Follow the box pointer, just as a reference
    
    using_my_box();
    
    println!("Works!");
}

// Defining our smart pointer (it doesn't store data on heap)
struct MyBox<T>(T); // Tuple type

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementing the deref trait
impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type (check chapter 20)
    
    fn deref(&self) -> &Self::Target {
        &self.0 // Firs item of the tuple, our value
    }
}

fn using_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // Rust call this behind the scenes: *(y.deref())
}