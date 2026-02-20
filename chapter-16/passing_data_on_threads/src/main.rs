use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // We use channels to send data through threads
    // mpsc stands for multiple producer, single consumer.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("message from thread");
        tx.send(val).unwrap();
        // println!("Val is not accesible here: {}", val);
    });

    // recv method blocks the thread until there's a message,
    // try_recv doesn't
    let received = rx.recv().unwrap();
    println!("Main thread: {}", received);

    // Sending multiple messages
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        // Sending the vals in intervals
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    rx.iter().for_each(|recv| println!("Got: {}", recv));
}
