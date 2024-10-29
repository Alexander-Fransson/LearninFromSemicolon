#![allow(dead_code)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc; // multiple producer, single consumer

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

pub fn passing_data_between_threads(){
    let (sender, receiver) = mpsc::channel();

    let messages = vec![
        "hello",
        "I am a thread",
    ];

    thread::spawn(move || { // needs to take ownership so that function does not outlive the thread
        for message in messages {
            sender.send(message).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // for msg in receiver {    // this will also work if you remove the code bellow
    //     println!("{}", msg);
    // }

    let other = thread::spawn(|| {
        for msg in receiver {
            println!("message: '{}' can be accessed here to", msg);
        }
    });

    other.join().unwrap();

}