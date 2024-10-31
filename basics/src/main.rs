use topics::restexample;

mod topics;

#[tokio::main]
async fn main() {
    restexample::test_rest_example().await;
}
