#[cfg(test)]
mod tests {
    use crate::server;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_api_login() {
        let server = tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:3000/api/login")
        .json(&serde_json::json!({"username": "test", "pwd": "test"}))
        .send()
        .await
        .unwrap();

        println!("res: {:#?}", res);

        assert_eq!(res.status(), 200);

        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:3000/api/login")
        .json(&serde_json::json!({"username": "test", "pwd": "wrong_password"}))
        .send()
        .await
        .unwrap();

        assert_ne!(res.status(), 200);

        server.abort();
    }
}