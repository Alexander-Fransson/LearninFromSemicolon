#![allow(unused_must_use)]

#[cfg(test)]
mod tests {
    use crate::server;
    use tokio::time::Duration;
    use crate::login_api::web::AUTH_TOKEN;


    #[tokio::test]
    #[ignore]
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

        let cookies = res.cookies();
        let cookie_exhists = cookies.into_iter().any(|cookie| cookie.name() == AUTH_TOKEN);

        assert!(cookie_exhists);


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