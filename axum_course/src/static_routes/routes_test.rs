#[cfg(test)]
mod tests {
    use tokio::time::Duration;

    use crate::server::server;

    #[tokio::test]
    #[ignore]
    async fn fetch_static_files() {
        let server = tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(200)).await;

        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:3000/src/static_routes/file_to_fetch.txt")
        .send()
        .await
        .unwrap();

        assert_eq!(res.status(), 200);

        // getting the content of the file
        let content = res.text().await.unwrap();
        assert!(content.contains("I am some text"));

        server.abort();
    }
}