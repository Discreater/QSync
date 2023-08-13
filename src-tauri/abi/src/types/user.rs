use crate::{convert_to_timestamp, User};
impl User {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      ..Default::default()
    }
  }

  pub fn from_entity(value: entity::user::Model) -> User {
    Self {
      id: value.id,
      name: value.name,
      created_at: Some(convert_to_timestamp(&value.created_at)),
      updated_at: Some(convert_to_timestamp(&value.updated_at)),
    }
  }
}
