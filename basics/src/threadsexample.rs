#![allow(dead_code)]

use std::thread;
use std::time::Duration;

pub fn demonstrate_threads() {
    // main does not wait for threads to finish automatically
    let current_thread =thread::spawn(|| {
        for i in 1..5 {
            println!("number in theread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let concurrent_thread = thread::spawn(|| {
        for i in 1..5 {
            println!("another number in other theread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // this one will let it finish
    current_thread.join().unwrap();
    concurrent_thread.join().unwrap();
}