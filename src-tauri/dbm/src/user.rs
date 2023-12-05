use abi::LoginRequest;
use chrono::Utc;
use entity::prelude::*;

use sea_orm::{
  ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
};

use crate::{DbManager, MusyncError, UserId};

/// User
impl DbManager {
  pub async fn create_user(&self, user: abi::User) -> Result<abi::User, MusyncError> {
    let mut updating = user.clone();
    let now = Utc::now();
    let inserted = entity::user::ActiveModel {
      name: Set(user.name),
      created_at: Set(now),
      updated_at: Set(now),
      ..Default::default()
    }
    .insert(&self.db)
    .await?;

    updating.id = inserted.id;

    Ok(updating)
  }

  pub async fn update_user(&self, update: abi::UserUpdate) -> Result<abi::User, MusyncError> {
    let now = Utc::now();

    let old = User::find_by_id(update.id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::UserNotFound(update.id))?;
    let mut updating: entity::user::ActiveModel = old.into();
    updating.name = Set(update.name);
    updating.updated_at = Set(now);
    updating.update(&self.db).await?;

    self.user(update.id).await
  }

  pub async fn delete_users(&self, ids: &[UserId]) -> Result<u64, MusyncError> {
    if ids.is_empty() {
      return Ok(0);
    }

    let deleted = User::delete_many()
      .filter(entity::user::Column::Id.is_in(ids.to_owned()))
      .exec(&self.db)
      .await?;
    Ok(deleted.rows_affected)
  }

  pub async fn query_users(
    &self,
    query: abi::QueryUsersRequest,
  ) -> Result<Vec<abi::User>, MusyncError> {
    if let Some(name) = query.name {
      let rows = User::find()
        .filter(entity::user::Column::Name.like(format!("%{}%", name)))
        .all(&self.db)
        .await?;

      Ok(rows.into_iter().map(abi::User::from_entity).collect())
    } else {
      Ok(vec![])
    }
  }

  pub async fn login(&self, req: LoginRequest) -> Result<abi::User, MusyncError> {
    let user = User::find()
      .filter(entity::user::Column::Name.eq(&req.name))
      .one(&self.db)
      .await?;
    user
      .map(abi::User::from_entity)
      .ok_or(MusyncError::LoginFailed(req.name))
  }
  pub async fn user(&self, id: UserId) -> Result<abi::User, MusyncError> {
    let row = User::find_by_id(id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::UserNotFound(id))?;
    Ok(abi::User::from_entity(row))
  }
}
