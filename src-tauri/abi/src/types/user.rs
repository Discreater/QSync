use sqlx::{sqlite::SqliteRow, Row};

use crate::User;

impl User {
  pub fn new<S>(name: S) -> User
  where
    S: Into<String>,
  {
    User {
      id: 0,
      name: name.into(),
    }
  }

  pub fn from_row(row: SqliteRow) -> Result<User, sqlx::Error> {
    Ok(User {
      id: row.try_get("id")?,
      name: row.try_get("name")?,
    })
  }
}
