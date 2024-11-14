use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("CONNECTION_STRING").expect("FAILED TO GET CONNECTION_STRING");

    println!("db_url: {}", db_url);
}
