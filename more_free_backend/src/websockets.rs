use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                println!("Got message from client: {}", text);
                let formatted_response = format!("You said: {}", text);
                // do something using the server
                socket.send(Message::Text(formatted_response)).await.unwrap();         
            },
            Message::Close(_) => {
                println!("Client disconnected");
                break;
            },
            _ => {}
        }
    }
}