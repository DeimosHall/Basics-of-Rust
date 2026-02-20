use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc<T> is the atomic version of Rc<T>
    // It allow us to work in multi threaded environments
    // with a performance penalty
    let counter = Arc::new(Mutex::new(0)); // Mutex allows internal mutability
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}