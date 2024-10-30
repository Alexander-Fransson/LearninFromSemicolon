mod topics;

#[tokio::main]
async fn main() {
    topics::jsonexamples::serialize_example();
}
