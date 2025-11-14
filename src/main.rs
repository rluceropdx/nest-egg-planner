/// ChatGPT provided example of websocket server
/// Prompt: Rust websocket server example
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::Filter;
use warp::ws::Message;

#[derive(Debug, Deserialize, Serialize)]
struct ClientData {
    age: u16,
    current_savings: usize,

    action: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct YearlyData {
    year: u16,
    age: u16,
    savings: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Simulation {
    processed: Vec<YearlyData>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
struct SpPerformance {
    year: u16,
    total_return: f32,
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

async fn handle_connection(websocket: warp::ws::WebSocket) {
    let sp_500_historical: Vec<SpPerformance> = vec![
        SpPerformance {
            year: 2024,
            total_return: 25.02,
        },
        SpPerformance {
            year: 2023,
            total_return: 26.29,
        },
        SpPerformance {
            year: 2022,
            total_return: -18.11,
        },
        SpPerformance {
            year: 2021,
            total_return: 28.71,
        },
        SpPerformance {
            year: 2020,
            total_return: 18.40,
        },
        SpPerformance {
            year: 2019,
            total_return: 31.49,
        },
        SpPerformance {
            year: 2018,
            total_return: -4.38,
        },
        SpPerformance {
            year: 2017,
            total_return: 21.83,
        },
        SpPerformance {
            year: 2016,
            total_return: 11.96,
        },
        SpPerformance {
            year: 2015,
            total_return: 1.38,
        },
        SpPerformance {
            year: 2014,
            total_return: 13.69,
        },
        SpPerformance {
            year: 2013,
            total_return: 32.39,
        },
        SpPerformance {
            year: 2012,
            total_return: 16.00,
        },
        SpPerformance {
            year: 2011,
            total_return: 2.11,
        },
        SpPerformance {
            year: 2010,
            total_return: 15.06,
        },
        SpPerformance {
            year: 2009,
            total_return: 26.46,
        },
        SpPerformance {
            year: 2008,
            total_return: -37.00,
        },
        SpPerformance {
            year: 2007,
            total_return: 5.49,
        },
        SpPerformance {
            year: 2006,
            total_return: 15.79,
        },
        SpPerformance {
            year: 2005,
            total_return: 4.91,
        },
        SpPerformance {
            year: 2004,
            total_return: 10.88,
        },
        SpPerformance {
            year: 2003,
            total_return: 28.68,
        },
        SpPerformance {
            year: 2002,
            total_return: -22.10,
        },
        SpPerformance {
            year: 2001,
            total_return: -11.89,
        },
        SpPerformance {
            year: 2000,
            total_return: -9.10,
        },
        SpPerformance {
            year: 1999,
            total_return: 21.04,
        },
        SpPerformance {
            year: 1998,
            total_return: 28.58,
        },
        SpPerformance {
            year: 1997,
            total_return: 33.36,
        },
        SpPerformance {
            year: 1996,
            total_return: 22.96,
        },
        SpPerformance {
            year: 1995,
            total_return: 37.58,
        },
        SpPerformance {
            year: 1994,
            total_return: 1.32,
        },
        SpPerformance {
            year: 1993,
            total_return: 10.08,
        },
        SpPerformance {
            year: 1992,
            total_return: 7.62,
        },
        SpPerformance {
            year: 1991,
            total_return: 30.47,
        },
        SpPerformance {
            year: 1990,
            total_return: -3.10,
        },
        SpPerformance {
            year: 1989,
            total_return: 31.69,
        },
        SpPerformance {
            year: 1988,
            total_return: 16.61,
        },
        SpPerformance {
            year: 1987,
            total_return: 5.25,
        },
        SpPerformance {
            year: 1986,
            total_return: 18.67,
        },
        SpPerformance {
            year: 1985,
            total_return: 31.73,
        },
        SpPerformance {
            year: 1984,
            total_return: 6.27,
        },
        SpPerformance {
            year: 1983,
            total_return: 22.56,
        },
        SpPerformance {
            year: 1982,
            total_return: 21.55,
        },
        SpPerformance {
            year: 1981,
            total_return: -4.91,
        },
        SpPerformance {
            year: 1980,
            total_return: 32.42,
        },
        SpPerformance {
            year: 1979,
            total_return: 18.44,
        },
        SpPerformance {
            year: 1978,
            total_return: 6.56,
        },
        SpPerformance {
            year: 1977,
            total_return: -7.18,
        },
        SpPerformance {
            year: 1976,
            total_return: 23.84,
        },
        SpPerformance {
            year: 1975,
            total_return: 37.20,
        },
        SpPerformance {
            year: 1974,
            total_return: -26.47,
        },
        SpPerformance {
            year: 1973,
            total_return: -14.66,
        },
        SpPerformance {
            year: 1972,
            total_return: 18.98,
        },
        SpPerformance {
            year: 1971,
            total_return: 14.31,
        },
        SpPerformance {
            year: 1970,
            total_return: 4.01,
        },
        SpPerformance {
            year: 1969,
            total_return: -8.50,
        },
        SpPerformance {
            year: 1968,
            total_return: 11.06,
        },
        SpPerformance {
            year: 1967,
            total_return: 23.98,
        },
        SpPerformance {
            year: 1966,
            total_return: -10.06,
        },
        SpPerformance {
            year: 1965,
            total_return: 12.45,
        },
        SpPerformance {
            year: 1964,
            total_return: 16.48,
        },
        SpPerformance {
            year: 1963,
            total_return: 22.80,
        },
        SpPerformance {
            year: 1962,
            total_return: -8.73,
        },
        SpPerformance {
            year: 1961,
            total_return: 26.89,
        },
        SpPerformance {
            year: 1960,
            total_return: 0.47,
        },
        SpPerformance {
            year: 1959,
            total_return: 11.96,
        },
        SpPerformance {
            year: 1958,
            total_return: 43.36,
        },
        SpPerformance {
            year: 1957,
            total_return: -10.78,
        },
        SpPerformance {
            year: 1956,
            total_return: 6.56,
        },
        SpPerformance {
            year: 1955,
            total_return: 31.56,
        },
        SpPerformance {
            year: 1954,
            total_return: 52.62,
        },
        SpPerformance {
            year: 1953,
            total_return: -0.99,
        },
        SpPerformance {
            year: 1952,
            total_return: 18.37,
        },
        SpPerformance {
            year: 1951,
            total_return: 24.02,
        },
        SpPerformance {
            year: 1950,
            total_return: 31.71,
        },
        SpPerformance {
            year: 1949,
            total_return: 18.79,
        },
        SpPerformance {
            year: 1948,
            total_return: 5.50,
        },
        SpPerformance {
            year: 1947,
            total_return: 5.71,
        },
        SpPerformance {
            year: 1946,
            total_return: -8.07,
        },
        SpPerformance {
            year: 1945,
            total_return: 36.44,
        },
        SpPerformance {
            year: 1944,
            total_return: 19.75,
        },
        SpPerformance {
            year: 1943,
            total_return: 25.90,
        },
        SpPerformance {
            year: 1942,
            total_return: 20.34,
        },
        SpPerformance {
            year: 1941,
            total_return: -11.59,
        },
        SpPerformance {
            year: 1940,
            total_return: -9.78,
        },
        SpPerformance {
            year: 1939,
            total_return: -0.41,
        },
        SpPerformance {
            year: 1938,
            total_return: 31.12,
        },
        SpPerformance {
            year: 1937,
            total_return: -35.03,
        },
        SpPerformance {
            year: 1936,
            total_return: 33.92,
        },
        SpPerformance {
            year: 1935,
            total_return: 47.67,
        },
        SpPerformance {
            year: 1934,
            total_return: -1.44,
        },
        SpPerformance {
            year: 1933,
            total_return: 53.99,
        },
        SpPerformance {
            year: 1932,
            total_return: -8.19,
        },
        SpPerformance {
            year: 1931,
            total_return: -43.34,
        },
        SpPerformance {
            year: 1930,
            total_return: -24.90,
        },
        SpPerformance {
            year: 1929,
            total_return: -8.42,
        },
        SpPerformance {
            year: 1928,
            total_return: 43.61,
        },
        SpPerformance {
            year: 1927,
            total_return: 37.49,
        },
        SpPerformance {
            year: 1926,
            total_return: 11.62,
        },
    ];

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
                                let mut calc_savings: usize = 0;
                                let mut sp_index = 0;
                                for i in CURRENT_YEAR..=end_year {
                                    calc_savings += parsed.current_savings
                                        + (calc_savings as f32
                                            * sp_500_historical.get(sp_index).unwrap().total_return
                                            * 0.01)
                                            as usize;

                                    let data = YearlyData {
                                        year: i,
                                        age: calc_age,
                                        savings: calc_savings,
                                    };
                                    return_data.processed.push(data);

                                    calc_age += 1;
                                    sp_index += 1;
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
