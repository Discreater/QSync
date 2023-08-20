use std::time::Instant;

use axum::response::IntoResponse;
use axum_typed_websockets::{WebSocketUpgrade, WebSocket, Message};
use serde::{Serialize, Deserialize};

pub async fn handler(
  ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
) -> impl IntoResponse {
  ws.on_upgrade(ping_pong_socket)
}

// Send a ping and measure how long time it takes to get a pong back
async fn ping_pong_socket(mut socket: WebSocket<ServerMsg, ClientMsg>) {
  let start = Instant::now();
  socket.send(Message::Item(ServerMsg::Ping)).await.ok();

  if let Some(msg) = socket.recv().await {
      match msg {
          Ok(Message::Item(ClientMsg::Pong)) => {
              println!("ping: {:?}", start.elapsed());
          },
          Ok(_) => {},
          Err(err) => {
              eprintln!("got error: {}", err);
          },
      }
  }
}


#[derive(Debug, Serialize)]
pub enum ServerMsg {
    Ping,
}

#[derive(Debug, Deserialize)]
pub enum ClientMsg {
    Pong,
}