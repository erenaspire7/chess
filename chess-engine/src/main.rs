use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use serde_json;
use state::State;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::Filter;

mod board;
mod piece;
mod player;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    let games: HashMap<String, Option<State>> = HashMap::new();

    let games = warp::any().map(move || games.clone());

    let route = warp::path("game")
        .and(warp::ws())
        .and(games)
        .map(|ws: warp::ws::Ws, games| {
            ws.on_upgrade(move |socket| handle_connection(socket, games))
        });

    let routes = route.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_connection(ws: WebSocket, mut games: HashMap<String, Option<State>>) {
    // Generate New ID
    let id = Uuid::new_v4();

    // Add Game
    games.insert(id.to_string(), None);

    let (mut sender, mut receiver) = ws.split();

    let (tx, rx): (UnboundedSender<i32>, UnboundedReceiver<i32>) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    let message = Message::text(format!("id: {}", id.to_string()));

    sender
        .send(message)
        .unwrap_or_else(|e| {
            println!("websocket send error: {}", e);
        })
        .await;

    // Receive Messages
    while let Some(result) = receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("websocket error(uid={}): {}", id, e);
                break;
            }
        };

        let message = msg.to_str().unwrap();
        let data: serde_json::Value = serde_json::from_str(message).unwrap();

        let id = &data["id"];
        let action = &data["action"];

        match action.as_str().unwrap() {
            "setup" => {
                let id_val = id.as_str().unwrap();

                let game = games.get_mut(id_val).unwrap();

                // game = Some()
            }
            _ => {}
        }

        // handle_message(id.to_string(), msg, &mut games, ws);
    }

    // Disconnect Safely
    handle_disconnect(id.to_string(), games)
}

fn handle_disconnect(id: String, mut games: HashMap<String, Option<State>>) {
    games.remove(&id);
}
