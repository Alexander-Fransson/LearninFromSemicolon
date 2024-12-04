#[cfg(test)]
pub mod tests {

    use crate::{
        server_1,
        server_2, 
        server_3
    };
    use tokio::time::{Duration, sleep};
    use reqwest::Client;
    use crate::web::AUTH_TOKEN;

    #[tokio::test]
    async fn test_server_3() {
        let server =tokio::spawn(async move {
            server_3().await;
        });

        sleep(Duration::from_millis(100)).await;

        let client = Client::new();

        // login

        let login_res = client.post("http://127.0.0.1:3002/api/login/v2")
        .json(&serde_json::json!({"username": "test", "password": "test"})) 
        .send()
        .await
        .unwrap();

        assert!(login_res.status().is_success());

        let cookies = login_res.cookies(); // request features need to include cookie

        let cookie_exhists = cookies.into_iter().any(|cookie| cookie.name() == AUTH_TOKEN);
        assert!(cookie_exhists);

        server.abort();
    }

    #[tokio::test]
    async fn test_server_2() {

        let server =tokio::spawn(async move {
            server_2().await;
        });

        sleep(Duration::from_millis(100)).await;

        let client = Client::new();

        // fetch static files

        let res = client.get("http://127.0.0.1:3001/src/main.rs")
        .send()
        .await
        .unwrap();

        assert!(res.status().is_success());
        
        let text = res.text().await.unwrap();
        assert!(text.contains("fn main()"));

        // login is success

        let login_res = client.post("http://127.0.0.1:3001/api/login")
        .json(&serde_json::json!({"username": "test", "password": "test"})) // json needs to be among reqwest features
        .send()
        .await
        .unwrap();

        assert!(login_res.status().is_success());

        let login_text = login_res.text().await.unwrap();
        assert!(login_text.contains("success"));

        server.abort();
    }

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
