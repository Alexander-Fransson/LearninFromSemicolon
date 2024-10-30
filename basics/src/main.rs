mod topics;

#[tokio::main]
async fn main() {
    topics::asyncexamples::tokio_select_example().await;
}
