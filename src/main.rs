use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2: mpsc::Sender<String> = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for message in messages {
            tx.send(message).unwrap()
        }

        thread::sleep(std::time::Duration::from_millis(1000))
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for message in messages {
            tx2.send(message).unwrap()
        }

        thread::sleep(std::time::Duration::from_millis(1000))
    });

    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
