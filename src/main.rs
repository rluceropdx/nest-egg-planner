/// ChatGPT provided example of websocket server
/// Prompt: Rust websocket server example
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::Filter;
use warp::ws::Message;

#[derive(Debug, Deserialize, Serialize)]
struct ClientMessage {
    action: String,
    value: Option<String>,
}

#[tokio::main]
async fn main() {
    // Create a warp filter that upgrades HTTP to WebSocket
    let ws_route = warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws| {
        // When a client connects, call this function
        ws.on_upgrade(handle_connection)
    });

    println!("WebSocket server running on ws://127.0.0.1:3030/ws");

    // Start the server
    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_connection(websocket: warp::ws::WebSocket) {
    println!("New WebSocket connection established!");

    let (mut tx, mut rx) = websocket.split();

    // Listen for messages from the client
    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_str().unwrap_or("");
                    println!("Received: {}", text);

                    // Try to parse JSON
                    match serde_json::from_str::<ClientMessage>(text) {
                        Ok(parsed) => {
                            println!("Parsed JSON: {:?}", parsed);

                            // Example response
                            let response = json!({
                                "status": "ok",
                                "echo": parsed,
                            });
                            let response_text = serde_json::to_string(&response).unwrap();
                            tx.send(Message::text(response_text)).await.ok();
                        }
                        Err(err) => {
                            eprintln!("JSON parse error: {}", err);
                            let err_msg = json!({ "error": "invalid JSON" });
                            tx.send(Message::text(err_msg.to_string())).await.ok();
                        }
                    }

                    // Echo back the same message
                    if let Err(e) = tx
                        .send(warp::ws::Message::text(format!("Echo: {}", text)))
                        .await
                    {
                        eprintln!("Error sending message: {}", e);
                        break;
                    }
                } else if msg.is_close() {
                    println!("Client disconnected");
                    break;
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}
