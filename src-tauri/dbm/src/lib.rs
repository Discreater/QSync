pub mod error;
mod manager;
mod search;

pub use error::MusyncError;
use sea_orm::DatabaseConnection;
use search::SearchActorHandle;

pub type PlaylistId = i32;
pub type TrackId = i32;
pub type UserId = i32;

#[derive(Debug, Clone)]
pub struct DbManager {
  db: DatabaseConnection,
  search_actor_handle: SearchActorHandle,
}
