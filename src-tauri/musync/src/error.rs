use thiserror::Error;

#[derive(Debug, Error)]
pub enum MusyncError {
  #[error("Unknown error")]
  Unknown,

  #[error("SeaOrm error")]
  SeaOrm(#[from] sea_orm::DbErr),

  #[error("Playlist {0} not found")]
  PlaylistNotFound(i32),

  #[error("Track {0} not found")]
  TrackNotFound(i32),

  #[error("User {0} not found")]
  UserNotFound(i32),
}

pub type Result<T> = std::result::Result<T, MusyncError>;
