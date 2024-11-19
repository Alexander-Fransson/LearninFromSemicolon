use server::server;

mod static_routes;
mod server_test;
mod server;
mod models;

#[tokio::main]
async fn main() {
    server().await;
    println!("Hello, world!");
}
