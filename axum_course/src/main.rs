use server::server;

mod server_test;
mod server;
mod models;

#[tokio::main]
async fn main() {
    server().await;
    println!("Hello, world!");
}
