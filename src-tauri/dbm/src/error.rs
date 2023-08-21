use thiserror::Error;
use tonic::Status;
use tracing::error;

#[derive(Debug, Error)]
pub enum MusyncError {
  #[error("Unknown error")]
  Unknown,

  #[error("SeaOrm error: {0}")]
  SeaOrm(#[from] sea_orm::DbErr),

  #[error("Playlist {0} not found")]
  PlaylistNotFound(i32),

  #[error("Track {0} not found")]
  TrackNotFound(i32),

  #[error("Folder {0} not found")]
  FolderNotFound(String),

  #[error("Login failed: {0}")]
  LoginFailed(String),

  #[error("User {0} not found")]
  UserNotFound(i32),

  #[error("PlayQueue {0} not found")]
  PlayQueueNotFound(i32),

  #[error("Folder {0} exists")]
  FolderExists(String),
}
impl From<MusyncError> for tonic::Status {
  fn from(value: MusyncError) -> Self {
    error!("{}", value.to_string());
    match value {
      MusyncError::Unknown | MusyncError::SeaOrm(_) => Status::internal("unknown"),
      MusyncError::PlaylistNotFound(_)
      | MusyncError::PlayQueueNotFound(_)
      | MusyncError::FolderNotFound(_)
      | MusyncError::TrackNotFound(_)
      | MusyncError::UserNotFound(_) => Status::not_found("not found"),
      MusyncError::LoginFailed(_) => Status::unauthenticated("login failed"),
      MusyncError::FolderExists(_) => Status::already_exists("exists"),
    }
  }
}

pub type Result<T> = std::result::Result<T, MusyncError>;
