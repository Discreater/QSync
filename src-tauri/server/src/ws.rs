use std::{
  ops::{AddAssign, SubAssign},
  sync::Arc,
};

use abi::{UpdatePlayQueueEvent, UpdatePlayerEvent, UpdatePlayerRequest};
use axum::{extract::State, response::IntoResponse, Extension};
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use dashmap::DashMap;
use dbm::{DbManager, UserId};
use futures::{SinkExt, StreamExt};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};
use tracing::{error, info, trace, warn};
use utils::WithError;

use crate::{error::HttpError, user::Claims};

const CAPACITY: usize = 64;

#[derive(Debug, Clone)]
pub struct WsState {
  clients: DashMap<UserId, usize>,
  pub tx: broadcast::Sender<Arc<BroadcastMsg>>,
}

impl Default for WsState {
  fn default() -> Self {
    let (tx, _) = broadcast::channel(CAPACITY);
    Self {
      clients: DashMap::new(),
      tx,
    }
  }
}

type ClientId = usize;

#[derive(Debug, Clone)]
pub enum BroadcastMsg {
  UpdatePlayer {
    source: ClientId,
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
      socket
        .close()
        .await
        .with_err(|e| error!("close socket failed: {}", e));
      return;
    }
  } else {
    error!("no auth");
    socket
      .close()
      .await
      .with_err(|e| error!("close socket failed: {}", e));
    return;
  };

  let client_id = {
    let mut client_num = state.clients.entry(user_id).or_insert(0);
    let client_id = *client_num;

    socket
      .send(Message::Item(ServerMsg::AuthSuccess(client_id)))
      .await
      .with_err(|e| error!("send auth response failed: {e:?}"));

    client_num.add_assign(1);
    client_id
  };

  // send player status
  if let Ok(play_queue) = db.get_user_play_queue(user_id).await {
    if let Some(play_queue) = play_queue {
      // send player
      let msg = ServerMsg::UpdatePlayer(UpdatePlayerEvent {
        position: play_queue.position,
        playing: play_queue.playing,
        progress: play_queue.progress(),
      });
      socket
        .send(Message::Item(msg))
        .await
        .with_err(|e| error!("send message failed: {}", e));
    }
  } else {
    error!("get user play queue failed");
    socket
      .close()
      .await
      .with_err(|e| error!("close socket failed: {}", e));
    return;
  }

  let (sender, mut receiver) = socket.split();

  let sender = Arc::new(Mutex::new(sender));
  let sender_in_recv = Arc::clone(&sender);
  let state_in_recv = Arc::clone(&state);
  let db_in_recv = db.clone();
  let mut recv_task = tokio::spawn(async move {
    while let Some(data) = receiver.next().await {
      match data {
        Ok(Message::Item(cm)) => {
          handle_message(
            cm,
            Arc::clone(&state_in_recv),
            db_in_recv.clone(),
            user_id,
            client_id,
          )
          .await
          .with_err(|e| {
            error!("error handling message: {:?}", e);
          });
        }
        Ok(Message::Ping(i)) => {
          sender_in_recv
            .lock()
            .await
            .send(Message::Pong(i))
            .await
            .with_err(|e| {
              error!("error sending pong: {:?}", e);
            });
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
          source,
          user_id: uid,
          update,
        } => {
          if *uid == user_id && *source != client_id {
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
        sender.lock().await.send(msg).await.with_err(|e| {
          error!("error broadcast message: {:?}", e);
        });
      }
    }
  });

  // If any of the tasks fail, we need to shut down the other one
  tokio::select! {
    _v1 = &mut recv_task => send_task.abort(),
    _v2 = &mut send_task => recv_task.abort(),
  }

  trace!("A client of user {user_id} disconnected");
  // Remove client from state
  {
    if let Some(mut client_num) = state.clients.get_mut(&user_id) {
      client_num.sub_assign(1);
      let current_num = *client_num;
      drop(client_num); // avoid deadlock
      trace!("Client number of user {user_id} is {}", current_num);
      if current_num == 0 {
        state.clients.remove(&user_id);
        trace!("All client of user {user_id} disconnected");
        // All clients disconnected, pause player after 5 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        match state.clients.get(&user_id) {
          None => {
            trace!("Pause player for user {user_id}");
            db.update_player(
              &UpdatePlayerRequest {
                manul: true,
                playing: Some(false),
                position: None,
                progress: None,
              },
              user_id,
            )
            .await
            .with_err(|e| error!("error pause player: {:?}", e));
          }
          _ => {
            trace!("Other client of user {user_id} connected, skip pause player");
          }
        }
      }
    }
  }
}

async fn handle_message(
  msg: ClientMsg,
  state: Arc<WsState>,
  db: DbManager,
  uid: UserId,
  cid: ClientId,
) -> Result<(), HttpError> {
  match msg {
    ClientMsg::UpdatePlayer(update) => {
      let updated = db.update_player(&update, uid).await?;
      if update.manul {
        if let Some(updated) = updated {
          state
            .tx
            .send(Arc::new(BroadcastMsg::UpdatePlayer {
              source: cid,
              user_id: uid,
              update: updated,
            }))
            .with_err(|e| {
              warn!("error broadcast message: {e}");
            });
        }
      }
    }
    ClientMsg::Auth(_) => error!("reauth not supported"),
  }
  Ok(())
}

#[derive(Debug, Serialize)]
pub enum ServerMsg {
  AuthSuccess(ClientId),
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
