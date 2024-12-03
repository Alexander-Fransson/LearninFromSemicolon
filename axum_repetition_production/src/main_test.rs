#[cfg(test)]
pub mod tests {

    use crate::server_1;
    use tokio::time::{Duration, sleep};
    use reqwest::Client;

    #[tokio::test]
    async fn test_server_1() {
        let server =tokio::spawn(async move {
            server_1().await;
        });

        sleep(Duration::from_millis(100)).await;

        let client = Client::new();
        let res = client.get("http://127.0.0.1:3000/hello")
        .send()
        .await
        .unwrap();

        assert!(res.status().is_success());

        let html = res.text().await.unwrap();
        assert!(html.contains("<h1>Hello, world!</h1>"));

        let res_with_param = client.get("http://127.0.0.1:3000/hello_with_param?name=schubnigurath")
        .send()
        .await
        .unwrap();

        assert!(res_with_param.status().is_success());

        let html2 = res_with_param.text().await.unwrap();
        assert!(html2.contains("<h1>Hello, schubnigurath</h1>"));

        let res_with_path_param = client.get("http://127.0.0.1:3000/hello_with_path_param/schubnigurath")
        .send()
        .await
        .unwrap();

        assert!(res_with_path_param.status().is_success());

        let html3 = res_with_path_param.text().await.unwrap();
        assert!(html3.contains("<h1>Hello, schubnigurath</h1>"));

        server.abort();
    }
}
