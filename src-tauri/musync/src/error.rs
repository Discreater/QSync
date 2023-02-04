use thiserror::Error;

#[derive(Debug, Error)]
pub enum MusyncError {
  #[error("Unknown error")]
  Unknown,

  #[error("Database error")]
  DbError(#[from] sqlx::Error),
}
