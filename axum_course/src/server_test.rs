#[cfg(test)]
mod tests {
    use crate::server::server;
    use reqwest::Client;
    use tokio::time::Duration;

    #[tokio::test]
    async fn initial_test() {
        tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let client = Client::new();
        let res = client.get("http://127.0.0.1:3000/")
        .send()
        .await
        .unwrap();

        assert_eq!(res.status(), 200);
        
        let html = res.text().await.unwrap();
        assert!(html.contains("<h1>Hello, world!</h1>"));
    }
}