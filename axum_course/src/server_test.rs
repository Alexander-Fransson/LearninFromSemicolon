#[cfg(test)]
mod tests {
    use crate::server::server;
    use reqwest::Client;
    use tokio::time::Duration;

    #[tokio::test]
    async fn initial_test() {
        let thread = tokio::spawn(async move {
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

        thread.abort();
    }

    #[tokio::test]
    async fn test_hello_with_param() {
        let server_thread = tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let client = Client::new();
        let res = client.get("http://127.0.0.1:3000/hello?name=schubnigurath")
        .send()
        .await
        .unwrap();

        assert_eq!(res.status(), 200);
        
        let html = res.text().await.unwrap();
        assert!(html.contains("<h1>Hello, schubnigurath!</h1>"));

        let client = Client::new();
        let res = client.get("http://127.0.0.1:3000/hello")
        .send()
        .await
        .unwrap();

        assert_eq!(res.status(), 200);
        
        let html = res.text().await.unwrap();
        assert!(html.contains("<h1>Hello, World!</h1>"));

        server_thread.abort();
    }

    #[tokio::test]
    async fn test_hello_with_path_param() {
        let server_thread = tokio::spawn(async move {
            server().await;
        });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let client = Client::new();
        let res = client.get("http://127.0.0.1:3000/hello/schopenhauer")
        .send()
        .await
        .unwrap(); 

        assert_eq!(res.status(), 200);
        
        let html = res.text().await.unwrap();
        assert!(html.contains("<h1>Hello, schopenhauer!</h1>"));

        server_thread.abort();}
}