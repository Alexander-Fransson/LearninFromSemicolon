#![allow(unused_must_use)]

use server::server;

mod static_routes;
mod server_test;
mod server;
mod models;
mod login_api;
mod rest_api;
mod auth_middleware;

pub use crate::login_api::errors::{Error, Result};

#[tokio::main]
async fn main() {
    server().await;
    println!("Hello, world!");
}
