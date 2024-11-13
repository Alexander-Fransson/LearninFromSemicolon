
#[cfg(test)]
mod tests {
    use crate::server;
    use axum::http::StatusCode;
    use std::time::Duration;

    use futures_util::{SinkExt, StreamExt}; // required for tungstenite to have send
    use tokio_tungstenite::{connect_async, tungstenite::Message};

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

    #[tokio::test]
    async fn test_jwt_authentication() {
        let server_handler = tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        // making post request to login endpoint
        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:3000/login")
        .json(&serde_json::json!({"username": "user", "password": "password"}))
        .send()
        .await
        .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        // checking if the response contains the token
        let token = res.text().await.unwrap();
        assert!(token.contains("token"));

        // get the token from the response string
        let token_object: serde_json::Value = serde_json::from_str(&token).unwrap();
        let token_value = token_object["token"].as_str().unwrap();

        // making get request to info endpoint
        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:3000/info")
        .header("Authorization", format!("Bearer {}", token_value))
        .send()
        .await
        .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        server_handler.abort();
    }

    #[tokio::test]
    async fn test_websocket() {
        let server_handler = tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let (mut socket, _) = connect_async("ws://127.0.0.1:3000/send").await.unwrap();
        socket.send(Message::Text("Hello".to_string())).await.unwrap();

        // wait for a message from the server
        let msg = socket.next().await.unwrap().unwrap();
        assert_eq!(msg, Message::Text("You said: Hello".to_string()));

        server_handler.abort();
    }
}

// next step is proper integration with db or to test web sockets with axum