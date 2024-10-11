
```rust

use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // single transmitter/producer, single receiver/consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = [
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // let received = rx.recv().unwrap(); // used to receive single message
    for received in rx {
        println!("Got: {received}");
    }
}
```
