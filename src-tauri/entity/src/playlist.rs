//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "playlist")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub owner_id: i32,
  pub name: String,
  pub description: String,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::playlist_track::Entity")]
  PlaylistTrack,
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::OwnerId",
    to = "super::user::Column::Id",
    on_update = "NoAction",
    on_delete = "Cascade"
  )]
  User,
  #[sea_orm(has_many = "super::user_playlist::Entity")]
  UserPlaylist,
}

impl Related<super::playlist_track::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PlaylistTrack.def()
  }
}

impl Related<super::user_playlist::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::UserPlaylist.def()
  }
}

impl Related<super::track::Entity> for Entity {
  fn to() -> RelationDef {
    super::playlist_track::Relation::Track.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::playlist_track::Relation::Playlist.def().rev())
  }
}

impl Related<super::user::Entity> for Entity {
  fn to() -> RelationDef {
    super::user_playlist::Relation::User.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::user_playlist::Relation::Playlist.def().rev())
  }
}

impl ActiveModelBehavior for ActiveModel {}
