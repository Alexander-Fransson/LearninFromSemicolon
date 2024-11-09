mod server;
mod server_test;

use crate::server::server;

#[tokio::main]
async fn main() {
    server().await;
}
