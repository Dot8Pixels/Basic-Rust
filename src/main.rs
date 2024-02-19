use std::thread;
use std::time::Duration;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 0..10 {
            println!("Hello, thread1: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 0..15 {
            println!("Hello, thread2: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Finished !!!");
}
