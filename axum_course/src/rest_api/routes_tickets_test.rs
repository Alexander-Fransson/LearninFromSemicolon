#![allow(unused_must_use)]

#[cfg(test)]
mod tests {
    use crate::{rest_api::model::Ticket, server};
    use tokio::time::Duration;
    use reqwest;

    #[tokio::test]
    #[ignore]
    async fn test_fetch_tickets() {
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

        assert_eq!(res.status(), 200);

        let cookies = res.cookies();
        let auth_token = cookies.into_iter().next().unwrap();

        let post_res = client.post("http://127.0.0.1:3000/api/tickets")
        .header("cookie", format!("{}={}","auth-token", &auth_token.value()))
        .json(&serde_json::json!({"title": "test"}))
        .send()
        .await
        .unwrap();

        assert_eq!(post_res.status(), 200);

        let ticket = post_res.text().await.unwrap();
        let ticket_id = serde_json::from_str::<Ticket>(&ticket).unwrap().id;

        let delete_res = client.delete(format!("http://127.0.0.1:3000/api/tickets/{}", ticket_id))
        .header("cookie", format!("{}={}","auth-token", &auth_token.value()))
        .send()
        .await
        .unwrap();

        assert_eq!(delete_res.status(), 200);

        server.abort();
    }
}