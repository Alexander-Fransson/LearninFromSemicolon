#![allow(dead_code)]

use tokio::time::sleep;
use std::time::Duration;
// add tokio by cargo add tokio, it allows main to be async
// add #[tokio::main] to main.rs

pub async fn async_example() {

    async fn pretend_to_download() -> String {
        sleep(Duration::from_secs(1)).await;
        String::from("Downloading...")
    }

    let result = pretend_to_download().await;
    println!("{}", result);
}

pub async fn concurent_async_exampe() {
    // the tasks can be run simultaniusly

    async fn task1() -> String {
        sleep(Duration::from_secs(1)).await;
        String::from("task 1")
    }

    async fn task2() -> String {
        sleep(Duration::from_secs(1)).await;
        String::from("task 2")
    }

    let (result1, result2) = tokio::join!(task1(), task2()); // runs taks simulataniusly but waits for all taks to finish
    println!("{} {}", result1, result2);
}

