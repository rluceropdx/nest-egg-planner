/// ChatGPT provided example of websocket server
/// Prompt: Rust websocket server example

use futures::{SinkExt, StreamExt};
use warp::Filter;

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
