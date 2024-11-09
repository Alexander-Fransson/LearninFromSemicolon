
#[cfg(test)]
mod tests {
    use crate::server;
    use axum::http::StatusCode;
    use std::time::Duration;

    #[tokio::test]
    async fn test_server() {
        let server_handler = tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:3000/")
        .send()
        .await
        .unwrap();

        println!("{:?}", res);
        assert_eq!(res.status(), StatusCode::ACCEPTED);

        server_handler.abort();
    }
}