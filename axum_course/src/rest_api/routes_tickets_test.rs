#![allow(unused_must_use)]

#[cfg(test)]
mod tests {
    use crate::{rest_api::model::Ticket, server};
    use tokio::time::Duration;

    #[tokio::test]
    //#[ignore]
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
        //println!("present cookie: {:?}", cookies.clone().into_iter().next().unwrap().value());

        // does not work for some reason, how do I pass cookies to the

        let post_client = reqwest::Client::new();
        let post_res = post_client.post("http://127.0.0.1:3000/api/tickets")
        .header("set-cookie", format!("auth_token={}", cookies.into_iter().next().unwrap().value()))
        .json(&serde_json::json!({"title": "test"}))
        .send()
        .await
        .unwrap();

        assert_eq!(post_res.status(), 200);

        // let client = reqwest::Client::new();
        // let res = client.get("http://127.0.0.1:3000/api/tickets")
        // .send()
        // .await
        // .unwrap();

        // assert_eq!(res.status(), 200);

        // // geting the id of the ticket
        // let content = res.text().await.unwrap();

        // // etracting the id from vector string
        // let vector_string: Vec<Ticket> = serde_json::from_str(&content).unwrap();
        // let id = vector_string[0].id;

        // let delete_client = reqwest::Client::new();
        // let delete_res = delete_client.delete(format!("http://127.0.0.1:3000/api/tickets/{}", id))
        // .send()
        // .await
        // .unwrap();

        // assert_eq!(delete_res.status(), 200);

        server.abort();
    }
}