use axum::response::IntoResponse;
use hyper::StatusCode;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Hyper(#[from] hyper::Error),
  #[error(transparent)]
  Dbm(#[from] dbm::MusyncError),
}

pub enum HttpError {
  Auth,
  Internal,
  NotFound,
  AlreadyExists,
}

impl From<dbm::MusyncError> for HttpError {
  fn from(e: dbm::MusyncError) -> Self {
    error!("MusyncError: {e}");
    match e {
      dbm::MusyncError::Unknown | dbm::MusyncError::SeaOrm(_) => HttpError::Internal,
      dbm::MusyncError::PlaylistNotFound(_)
      | dbm::MusyncError::FolderNotFound(_)
      | dbm::MusyncError::TrackNotFound(_)
      | dbm::MusyncError::UserNotFound(_) => HttpError::NotFound,
      dbm::MusyncError::LoginFailed(_) => HttpError::Auth,
      dbm::MusyncError::FolderExists(_) => HttpError::AlreadyExists,
    }
  }
}

impl IntoResponse for HttpError {
  fn into_response(self) -> axum::response::Response {
    match self {
      HttpError::Auth => StatusCode::UNAUTHORIZED,
      HttpError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
      HttpError::NotFound => StatusCode::NOT_FOUND,
      HttpError::AlreadyExists => StatusCode::CONFLICT,
    }
    .into_response()
  }
}

pub type Result<T> = std::result::Result<T, Error>;
