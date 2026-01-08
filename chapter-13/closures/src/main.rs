use std::{thread, time::Duration};

fn main() {
    // Closures are the equivalent of callbacks
    
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    println!("Num: {}", expensive_closure(32));
    
    // They don't require to specify the types of
    // parameters or the return type
    fn  _add_one_v1   (x: u32) -> u32 { x + 1 }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 =  |x|             { x + 1 };
    let add_one_v4 =  |x|               x + 1  ;
    
    let num1: u8 = 3;
    let num2: u64 = 1024;
    println!("v3: {}", add_one_v3(num1));
    println!("v4: {}", add_one_v4(num2));
    
    // Capturing references or moving ownership
    // Just as functions, closures can take onwership on the
    // parameters, or take mutable or inmutable refereces, but in this
    // case it's infered by it's behaviors
    
    // If the closure doesn't modify the value, it will be considered
    // as an imutable reference
    println!("Inmutable reference");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Closures captures their enviroment because they can use
    // variables that we don't need to pass through argumets, as
    // soon as they are in the same scope
    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    
    println!("Mutable reference");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
    
    // We can use the `move` keyword to take onwership. It's useful
    // when we work with threads, so the thread owns the data it uses
    println!("Takes onwership");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
