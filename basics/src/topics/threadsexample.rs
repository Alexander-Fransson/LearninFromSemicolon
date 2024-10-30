#![allow(dead_code)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc; // multiple producer, single consumer

pub fn sync_channels() {
    let (sender, receiver) = mpsc::sync_channel(1); // buffer size(); waits for things to be sent?
    thread::spawn(move || {
        for i in 1..3 {
            sender.send("Message").unwrap();
            println!("Message: '{}' sent", i);
        }
    });

    thread::sleep(Duration::from_millis(400));

    for msg in receiver {
        println!("Message received: {}", msg);
    }
}

pub fn produce_using_multiple_threads() {
    let (sender, receiver) = mpsc::channel();
    let cloned_sender = sender.clone(); // clone sender to send to other thread

    let messages = vec![
        "message",
        "I am messaging",
    ];

    thread::spawn(move || { // needs to take ownership so that function does not outlive the thread
        for message in messages {
            let formatted_msg = format!("message: {}", message);
            cloned_sender.send(formatted_msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let dummy_messages = vec![
        "message 2",
        "I am messaging more",
    ];

    thread::spawn(move || { // needs to take ownership so that function does not outlive the thread
        for message in dummy_messages {
            let formatted_msg = format!("message from thread 2: {}", message);
            sender.send(formatted_msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for msg in receiver {   // use data from thread in main thread   
        println!("{}", msg);
    }
}

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

    for msg in receiver {   // use data from thread in main thread   
        println!("{}", msg);
    }

    // let other = thread::spawn(|| {  // this will also work if you remove the code above
    //     for msg in receiver {
    //         println!("message: '{}' can be accessed here to", msg);
    //     }
    // });

    // other.join().unwrap();

}