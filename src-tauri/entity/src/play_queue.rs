//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "play_queue")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub position: u32,
  pub playing: bool,
  pub started_at: DateTimeUtc,
  pub paused_at: u32,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::play_queue_track::Entity")]
  PlayQueueTrack,
  #[sea_orm(has_many = "super::user::Entity")]
  User,
}

impl Related<super::play_queue_track::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PlayQueueTrack.def()
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
