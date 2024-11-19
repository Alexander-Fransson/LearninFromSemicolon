use server::server;

mod server_test;
mod server;

#[tokio::main]
async fn main() {
    server().await;
    println!("Hello, world!");
}
