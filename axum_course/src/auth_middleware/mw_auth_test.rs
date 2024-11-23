#![allow(unused_must_use)]

#[cfg(test)]
mod tests {
    use crate::server;
    use tokio::time::Duration;

    #[tokio::test]
    #[ignore]
    async fn test_tickets_unable_to_fetch_if_not_authenticated() {
        let server = tokio::spawn(async move {
            server().await;
        });
        
        tokio::time::sleep(Duration::from_millis(100)).await;

        let post_client = reqwest::Client::new();
        let post_res = post_client.post("http://127.0.0.1:3000/api/tickets")
        .json(&serde_json::json!({"title": "test"}))
        .send()
        .await
        .unwrap();

        assert_eq!(post_res.status(), 401);

        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:3000/api/tickets")
        .send()
        .await
        .unwrap();

        assert_eq!(res.status(), 401);

        let delete_client = reqwest::Client::new();
        let delete_res = delete_client.delete(format!("http://127.0.0.1:3000/api/tickets/{}", "0"))
        .send()
        .await
        .unwrap();

        assert_eq!(delete_res.status(), 401);

        server.abort();

        //assert!(false)
    }
}