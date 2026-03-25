fn main() {
    // Dereferencing a raw pointer
    let mut num = 5;
    let r1 = &raw const num; // Inmutable reference
    let r2 = &raw mut num; // Mutable reference

    // We have to use an unsafe block to access raw pointers
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // To call an unsafe function, we have to use an unsafe block!
    // dangerous();
    unsafe {
        dangerous();
    }

    // Calling a C function called abs
    unsafe {
        println!("The absolute value of -3 is {}", abs(-3));
    }
    
    // Accesing or modiying a mutable static variable
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
    
    
}

unsafe fn dangerous() {}

// Sometimes your Rust code might need to interact with code written in another language.
// For this, Rust has the keyword extern that facilitates the creation and use of a
// Foreign Function Interface (FFI), which is a way for a programming language to define
// functions and enable a different (foreign) programming language to call those functions.
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
