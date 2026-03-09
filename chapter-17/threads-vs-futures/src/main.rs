use std::time::Duration;

// This is similar to what I did in the previous chapter, but using
// the async syntax
fn counting_in_threads() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi number {i} from first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("Hi number {i} from second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // Waits for the first task to finish!
        handle.await.unwrap();
    });
}

fn counting_in_threads_v2() {
    trpl::block_on(async {
        let future1 = async {
            for i in 1..10 {
                println!("Hi number {i} from first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let future2 = async {
            for i in 1..5 {
                println!("Hi number {i} from second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // When we use async (here), the runtime will run one task
        // after the other one, just like this:
        // future1 -> future2 -> future1 -> future2
        //
        // When we use threads, the OS decides the other and the
        // priority, so we could have something like this:
        // future1 -> future2 -> future2 -> future1
        trpl::join(future1, future2).await;
    });
}

// Sending data between two tasks using message passing
fn sending_data() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        // We need to use move here in order to drop tx_fut,
        // so the program can exit!
        let tx_fut = async move {
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            // If rx.recv() is None, the loop ends
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(rx_fut, tx_fut).await;
    });
}

fn sending_multiple_messages() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx2 = tx.clone();
        let tx2_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received: '{}'", value);
            }
        };
        
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you")
            ];
            
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        
        trpl::join!(tx2_fut, rx_fut, tx_fut);
    });
}

fn main() {
    // counting_in_threads();
    counting_in_threads_v2();
    // sending_data();
    sending_multiple_messages();
}
