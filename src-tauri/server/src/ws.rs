use std::sync::Arc;

use abi::{UpdatePlayQueueEvent, UpdatePlayerEvent, UpdatePlayerRequest};
use axum::{extract::State, response::IntoResponse, Extension};
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use dbm::{DbManager, UserId};
use futures::{SinkExt, StreamExt};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};
use tracing::{error, info, warn};

use crate::{error::HttpError, user::Claims};

const CAPACITY: usize = 64;

#[derive(Debug, Clone)]
pub struct WsState {
  pub tx: broadcast::Sender<Arc<BroadcastMsg>>,
}

impl Default for WsState {
  fn default() -> Self {
    let (tx, _) = broadcast::channel(CAPACITY);
    Self { tx }
  }
}

#[derive(Debug, Clone)]
pub enum BroadcastMsg {
  UpdatePlayer {
    user_id: UserId,
    update: UpdatePlayerEvent,
  },
  UpdatePlayQueue {
    user_id: UserId,
    update: UpdatePlayQueueEvent,
  },
}

pub async fn handler(
  ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
  State(manager): State<DbManager>,
  Extension(state): Extension<Arc<WsState>>,
) -> Result<impl IntoResponse, StatusCode> {
  Ok(ws.on_upgrade(move |socket| handle_socket(socket, manager, state)))
}

// Send a ping and measure how long time it takes to get a pong back
async fn handle_socket(
  mut socket: WebSocket<ServerMsg, ClientMsg>,
  db: DbManager,
  state: Arc<WsState>,
) {
  let mut rx = state.tx.subscribe();

  let user_id = if let Some(Ok(Message::Item(ClientMsg::Auth(token)))) = socket.recv().await {
    if let Some(user_id) = get_user_id(&token) {
      info!("user {} connected", user_id);
      user_id
    } else {
      info!("auto failed");
      socket.close().await.ok();
      return;
    }
  } else {
    error!("not auth");
    socket.close().await.ok();
    return;
  };

  // send play queue and player
  if let Ok(play_queue) = db.get_user_play_queue(user_id).await {
    if let Some(play_queue) = play_queue {
      // play queue
      let msg = ServerMsg::UpdatePlayQueue(UpdatePlayQueueEvent {
        track_ids: play_queue.track_ids.clone(),
      });
      socket.send(Message::Item(msg)).await.ok();

      // send player
      let msg = ServerMsg::UpdatePlayer(UpdatePlayerEvent {
        position: play_queue.position,
        playing: play_queue.playing,
        progress: play_queue.progress(),
      });
      socket.send(Message::Item(msg)).await.ok();
    }
  } else {
    error!("get user play queue failed");
    socket.close().await.ok();
    return;
  }

  let (sender, mut receiver) = socket.split();

  let sender = Arc::new(Mutex::new(sender));
  let sender1 = Arc::clone(&sender);
  let mut recv_task = tokio::spawn(async move {
    while let Some(data) = receiver.next().await {
      match data {
        Ok(Message::Item(cm)) => {
          if let Err(e) = handle_message(cm, Arc::clone(&state), db.clone(), user_id).await {
            error!("error handling message: {:?}", e);
          }
        }
        Ok(Message::Ping(i)) => {
          sender1.lock().await.send(Message::Pong(i)).await.ok();
        }
        Ok(Message::Pong(i)) => {
          let msg = String::from_utf8_lossy(&i);
          info!("got raw pong: {msg}");
        }
        Ok(Message::Close(c)) => {
          info!("connection close: {c:?}");
        }
        Err(err) => {
          error!("got error: {}", err);
          break;
        }
      }
    }
  });
  let mut send_task = tokio::spawn(async move {
    while let Ok(msg) = rx.recv().await {
      let send_msg = match msg.as_ref() {
        BroadcastMsg::UpdatePlayer {
          user_id: id,
          update,
        } => {
          if *id == user_id {
            Some(Message::Item(ServerMsg::UpdatePlayer(update.clone())))
          } else {
            None
          }
        }
        BroadcastMsg::UpdatePlayQueue {
          user_id: id,
          update,
        } => {
          if *id == user_id {
            Some(Message::Item(ServerMsg::UpdatePlayQueue(update.clone())))
          } else {
            None
          }
        }
      };
      if let Some(msg) = send_msg {
        sender.lock().await.send(msg).await.ok();
      }
    }
  });

  // If any of the tasks fail, we need to shut down the other one
  tokio::select! {
    _v1 = &mut recv_task => send_task.abort(),
    _v2 = &mut send_task => recv_task.abort(),
  }
}

async fn handle_message(
  msg: ClientMsg,
  state: Arc<WsState>,
  db: DbManager,
  id: UserId,
) -> Result<(), HttpError> {
  match msg {
    ClientMsg::UpdatePlayer(update) => {
      let updated = db.update_player(&update, id).await?;
      if update.manul {
        if let Some(updated) = updated {
          if let Err(e) = state.tx.send(Arc::new(BroadcastMsg::UpdatePlayer {
            user_id: id,
            update: updated,
          })) {
            warn!("error broadcast message: {e}");
          }
        }
      }
    }
    ClientMsg::Auth(_) => error!("reauth not supported"),
  }
  Ok(())
}

#[derive(Debug, Serialize)]
pub enum ServerMsg {
  UpdatePlayer(UpdatePlayerEvent),
  UpdatePlayQueue(UpdatePlayQueueEvent),
}

#[derive(Debug, Deserialize)]
pub enum ClientMsg {
  Auth(String),
  UpdatePlayer(UpdatePlayerRequest),
}

fn get_user_id(auth: &str) -> Option<UserId> {
  Claims::decode(auth).ok().map(|c| c.id)
}
