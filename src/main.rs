/// ChatGPT provided example of websocket server
/// Prompt: Rust websocket server example
use futures::{SinkExt, StreamExt};
use rand::rng;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::Filter;
use warp::ws::Message;

#[derive(Debug, Deserialize, Serialize)]
struct ClientData {
    age: u16,
    current_savings: isize,
    current_salary: u32,
    retirement_expenses: u32,
    action: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct YearlyData {
    year: u16,
    age: u16,
    savings: isize,
    ss_payment: isize,
    rmd_withdrawal: isize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Simulation {
    processed: Vec<YearlyData>,
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

const MAX_AGE: u8 = 99;
const CURRENT_YEAR: u16 = 2025;

/// shuffle the performance returns for the last 99 years of the S&P 500
async fn random_sp_500_historical() -> f32 {
    // S&P 500 Performance from years 2024 - 1926 descending
    let sp_500_historical: Vec<f32> = vec![
        25.02, 26.29, -18.11, 28.71, 18.40, 31.49, -4.38, 21.83, 11.96, 1.38, 13.69, 32.39, 16.00,
        2.11, 15.06, 26.46, -37.00, 5.49, 15.79, 4.91, 10.88, 28.68, -22.10, -11.89, -9.10, 21.04,
        28.58, 33.36, 22.96, 37.58, 1.32, 10.08, 7.62, 30.47, -3.10, 31.69, 16.61, 5.25, 18.67,
        31.73, 6.27, 22.56, 21.55, -4.91, 32.42, 18.44, 6.56, -7.18, 23.84, 37.20, -26.47, -14.66,
        18.98, 14.31, 4.01, -8.50, 11.06, 23.98, -10.06, 12.45, 16.48, 22.80, -8.73, 26.89, 0.47,
        11.96, 43.36, -10.78, 6.56, 31.56, 52.62, -0.99, 18.37, 24.02, 31.71, 18.79, 5.50, 5.71,
        -8.07, 36.44, 19.75, 25.90, 20.34, -11.59, -9.78, -0.41, 31.12, -35.03, 33.92, 47.67,
        -1.44, 53.99, -8.19, -43.34, -24.90, -8.42, 43.61, 37.49, 11.62,
    ];

    let mut random_num = rng();
    let item = sp_500_historical.choose(&mut random_num);

    *item.unwrap()
}

/// incorporate social security payment estimate (very rough)
async fn add_ss_payment(age: u16, salary: u32) -> isize {
    let mut result = 0;

    if age > 67 {
        let annual_payment = match salary {
            50000 => 1300 * 12,
            60000 => 1540 * 12,
            70000 => 1780 * 12,
            80000 => 2020 * 12,
            90000 => 2260 * 12,
            100000 => 2500 * 12,
            _ => 0,
        };
        result = annual_payment
    };

    result as isize
}

/// subtract expenses during retirement years
async fn subtract_retirement_expenses(age: u16, retirement_expenses: u32) -> isize {
    let mut result = 0;

    if age > 67 {
        result = retirement_expenses
    };

    result as isize
}

async fn calc_required_minimum_distribution(age: u16, current_savings: isize) -> isize {
    if current_savings < 0 {
        return 0;
    }

    let rmd = match age {
        73 => (current_savings * 3 / 4) as f32 / 26.5,
        74 => (current_savings * 3 / 4) as f32 / 25.5,
        75 => (current_savings * 3 / 4) as f32 / 24.6,
        76 => (current_savings * 3 / 4) as f32 / 23.7,
        77 => (current_savings * 3 / 4) as f32 / 22.9,
        78 => (current_savings * 3 / 4) as f32 / 22.0,
        79 => (current_savings * 3 / 4) as f32 / 21.1,
        80 => (current_savings * 3 / 4) as f32 / 20.2,
        81 => (current_savings * 3 / 4) as f32 / 19.4,
        82 => (current_savings * 3 / 4) as f32 / 18.5,
        83 => (current_savings * 3 / 4) as f32 / 17.7,
        84 => (current_savings * 3 / 4) as f32 / 16.8,
        85 => (current_savings * 3 / 4) as f32 / 16.0,
        86 => (current_savings * 3 / 4) as f32 / 15.2,
        87 => (current_savings * 3 / 4) as f32 / 14.4,
        88 => (current_savings * 3 / 4) as f32 / 13.7,
        89 => (current_savings * 3 / 4) as f32 / 12.9,
        90 => (current_savings * 3 / 4) as f32 / 12.2,
        91 => (current_savings * 3 / 4) as f32 / 11.5,
        92 => (current_savings * 3 / 4) as f32 / 10.8,
        93 => (current_savings * 3 / 4) as f32 / 10.1,
        94 => (current_savings * 3 / 4) as f32 / 9.5,
        95 => (current_savings * 3 / 4) as f32 / 8.9,
        96 => (current_savings * 3 / 4) as f32 / 8.4,
        97 => (current_savings * 3 / 4) as f32 / 12.0,
        98 => (current_savings * 3 / 4) as f32 / 11.4,
        99 => (current_savings * 3 / 4) as f32 / 10.8,
        _ => 0 as f32,
    };

    rmd as isize
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
                    match serde_json::from_str::<ClientData>(text) {
                        Ok(parsed) => {
                            // client data received for simulation
                            println!("Parsed JSON: {:?}", parsed);

                            // simulate data from current client age until 99
                            if parsed.action == "simulate" {
                                let diff: u16 = MAX_AGE as u16 - parsed.age;
                                let end_year: u16 = CURRENT_YEAR + diff;

                                let mut return_data = Simulation { processed: vec![] };

                                let mut calc_age = parsed.age;
                                let mut calc_savings: isize = 0;
                                for i in CURRENT_YEAR..=end_year {
                                    let ss_payment =
                                        add_ss_payment(calc_age, parsed.current_salary).await;

                                    let retirement_exp = subtract_retirement_expenses(
                                        calc_age,
                                        parsed.retirement_expenses,
                                    )
                                    .await;

                                    let rmd =
                                        calc_required_minimum_distribution(calc_age, calc_savings)
                                            .await;

                                    let sp500 = random_sp_500_historical().await * 0.01;
                                    calc_savings += parsed.current_savings
                                        + (calc_savings as f32 * sp500) as isize
                                        + ss_payment
                                        - retirement_exp
                                        - rmd;

                                    let data = YearlyData {
                                        year: i,
                                        age: calc_age,
                                        savings: calc_savings,
                                        ss_payment: ss_payment,
                                        rmd_withdrawal: rmd,
                                    };
                                    return_data.processed.push(data);

                                    calc_age += 1;
                                }

                                // Example response
                                let response = json!({
                                    "status": "simulation completed",
                                    "results": return_data,
                                });
                                let response_text = serde_json::to_string(&response).unwrap();
                                tx.send(Message::text(response_text)).await.ok();
                            }
                        }
                        Err(err) => {
                            eprintln!("JSON parse error: {}", err);
                            let err_msg = json!({ "error": "invalid JSON" });
                            tx.send(Message::text(err_msg.to_string())).await.ok();
                        }
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
