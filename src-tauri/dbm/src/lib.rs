pub mod error;
mod manager;

pub use error::MusyncError;
use sea_orm::DatabaseConnection;

pub type PlaylistId = i32;
pub type TrackId = i32;
pub type UserId = i32;

#[derive(Debug, Clone)]
pub struct DbManager {
  db: DatabaseConnection,
}
