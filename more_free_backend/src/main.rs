mod server;
mod server_test;
mod model;
mod controller;

use crate::server::server;

#[tokio::main]
async fn main() {
    server().await;
}
