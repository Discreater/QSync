use axum::response::IntoResponse;
use hyper::StatusCode;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Hyper(#[from] hyper::Error),
  #[error(transparent)]
  Dbm(#[from] dbm::MusyncError),
  #[error(transparent)]
  NcmApi(#[from] ncmapi::ApiErr),
}

#[derive(Debug)]
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
      dbm::MusyncError::Unknown | dbm::MusyncError::SeaOrm(_) | dbm::MusyncError::SearchActor => {
        HttpError::Internal
      }
      dbm::MusyncError::PlaylistNotFound(_)
      | dbm::MusyncError::PlayQueueNotFound(_)
      | dbm::MusyncError::FolderNotFound(_)
      | dbm::MusyncError::TrackNotFound(_)
      | dbm::MusyncError::UserNotFound(_) => HttpError::NotFound,
      dbm::MusyncError::LoginFailed(_) => HttpError::Auth,
      dbm::MusyncError::FolderExists(_) => HttpError::AlreadyExists,
    }
  }
}

impl From<Error> for HttpError {
  fn from(value: Error) -> Self {
    error!("error: {value}");
    match value {
      Error::Hyper(_) => HttpError::Internal,
      Error::Dbm(e) => e.into(),
      Error::NcmApi(_) => HttpError::NotFound,
    }
  }
}

impl From<Error> for tonic::Status {
  fn from(e: Error) -> Self {
    error!("error: {e}");
    match e {
      Error::Hyper(h) => tonic::Status::internal(h.message().to_string()),
      Error::Dbm(e) => e.into(),
      Error::NcmApi(e) => tonic::Status::not_found(e.to_string()),
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
