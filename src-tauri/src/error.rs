use std::num::ParseIntError;

use serde::{ser::Serializer, Serialize};

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Lofty(#[from] lofty::LoftyError),
  #[error(transparent)]
  Parse(#[from] ParseIntError),
  #[error(transparent)]
  Musyncer(#[from] musync::MusyncError),
}

// we must manually implement serde::Serialize
impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
