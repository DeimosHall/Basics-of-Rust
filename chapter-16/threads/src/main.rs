use std::{thread, time::Duration, vec};

/// This functions doesn't warranty the spawned thread will
/// be able to print all the values.
fn no_warranties() {
    // This thread should print 10 numbers, but it only prints
    // as many as it can before the main thread finishes, which
    // means around 5.
    thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // This section will finish sooner than the previous one,
    // so the spanwed thread won't finish.
    for i in 1..5 {
        println!("Number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

fn warranties() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        } 
    });
    
    for i in 1..5 {
        println!("Number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
    
    // Although the main thread finishes printing its numbers, the join
    // warranties the main thread will wait for the spanwed thread to finish.
    handle.join().unwrap();
}

fn data_to_threads() {
    // We can pass data to threads by transferring ownership with the `move`
    // keyword.
    let vector = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", vector);
    });
    
    handle.join().unwrap();
}

fn main() {
    // Comment one or another to see the differences.
    // no_warranties();
    warranties();
    
    data_to_threads();
}
