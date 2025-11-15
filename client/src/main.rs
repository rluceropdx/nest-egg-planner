/// ChatGPT provided example of websocket client
/// Prompt: basic html javascript websocket client
///
use warp::Filter;

#[tokio::main]
async fn main() {
    // Define route: GET /
    let html = warp::path::end().map(|| warp::reply::html(include_str!("./main.html")));

    // Start server on localhost:8080
    println!("Server running at http://127.0.0.1:8080/");
    warp::serve(html).run(([127, 0, 0, 1], 8080)).await;
}
