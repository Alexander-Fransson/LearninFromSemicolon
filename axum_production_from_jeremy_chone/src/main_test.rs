#[cfg(test)]
mod tests {
    use crate::test_tests;
    use crate::server;
    use tokio::time::{Duration, sleep};
    use reqwest::Client;
    use serde_json::json;

    #[tokio::test]
    async fn test_routes_login() {
        let server = tokio::spawn(async move {
            server().await;
        });

        sleep(Duration::from_millis(100)).await;

        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:3000/api/login")
        .json(&json!({"username": "test", "pwd": "test"}))
        .send()
        .await
        .unwrap();

        assert!(res.status().is_success());

        let bad_res = client.post("http://127.0.0.1:3000/api/login")
        .json(&json!({"username": "test", "pwd": "wrong_password"}))
        .send()
        .await
        .unwrap();

        assert!(!bad_res.status().is_success());

        server.abort();
    }
    
    #[tokio::test]
    async fn test_basic_routes() {
        let server = tokio::spawn(async move {
            server().await;
        });

        sleep(Duration::from_millis(100)).await;

        // get /

        let client = Client::new();
        let res = client.get("http://127.0.0.1:3000/")
        .send()
        .await
        .unwrap();

        let html = res.text().await.unwrap();
        assert!(html.contains("<h1>Hello, world!</h1>"));

        // get /hello with params

        let res_with_params = client.get("http://127.0.0.1:3000/hello?name=schubnigurath")
        .send()
        .await
        .unwrap();

        let html_with_params = res_with_params.text().await.unwrap();
        assert!(html_with_params.contains("<h1>Hello, schubnigurath!</h1>"));

        // get /hello with query params

        let res_with_query_params = client.get("http://127.0.0.1:3000/hello/schopenhauer")
        .send()
        .await
        .unwrap();

        let html_with_query_params = res_with_query_params.text().await.unwrap();
        assert!(html_with_query_params.contains("<h1>Hello, schopenhauer!</h1>"));

        server.abort();
    }

    #[test]
    fn test_test_test() {
        assert_eq!(test_tests(), 16);
    }

}