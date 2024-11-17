
#[cfg(test)]
mod tests {
    use crate::server;
    use axum::Extension;
    use axum::http::StatusCode;
    use std::time::Duration;
    use crate::db_interactions::controllers::{
        create_bird,
        delete_bird,
        update_bird,
        get_birds,
        get_bird,
        seed_birds
    };
    use crate::db_interactions::service::Services;
    use crate::db_interactions::models::BirdInfo;

    use futures_util::{SinkExt, StreamExt}; // required for tungstenite to have send
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    // complains about connections refused

    #[tokio::test]
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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

    #[tokio::test]
    async fn test_bird_seeding() {
        let bird_service = Services::new().await.unwrap();
        let service_extension = Extension(bird_service);
        seed_birds(&service_extension).await;
        let birds = get_birds(&service_extension).await;

        match birds {
            Ok(birds) => assert_ne!(birds.len(), 0),
            Err(err) => {
                println!("Error: {}", err);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_bird_crud() {
        let bird_service = Services::new().await.unwrap();
        let service_extension = Extension(bird_service);
        seed_birds(&service_extension).await;
        
        let created_bird = create_bird(&service_extension, BirdInfo {
            name: "test".to_string(),
            password: "test".to_string()
        }).await;

        let new_bird_id = match created_bird {
            Ok(bird) => {
                assert_eq!(bird.name, "test");
                bird.id
            },
            Err(err) => {
                println!("Failed to create bird, Error: {}", err);
                assert!(false);
                0
            }
        };

        let updated_bird = update_bird(&service_extension, new_bird_id, BirdInfo {
            name: "test2".to_string(),
            password: "test2".to_string()
        }).await;

        match updated_bird {
            Ok(bird) => assert_eq!(bird.name, "test2"),
            Err(err) => {
                println!("Failed to update bird, Error: {}", err);
                assert!(false);
            }
        }

        let the_bird = get_bird(&service_extension, new_bird_id).await;

        match the_bird {
            Ok(bird) => assert_eq!(bird.name, "test2"),
            Err(err) => {
                println!("Failed to get bird, Error: {}", err);
                assert!(false);
            }
        }

        let bird_deleted = delete_bird(&service_extension, new_bird_id).await;

        match bird_deleted {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("Failed to delete bird, Error: {}", err);
                assert!(false);
            }
        }
        
    }
}

// next step is proper integration with db or to test web sockets with axum