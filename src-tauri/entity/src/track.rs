//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "track")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub title: String,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub duration: Option<u32>,
  pub genre: Option<String>,
  pub year: Option<u32>,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::local_src::Entity")]
  LocalSrc,
  #[sea_orm(has_many = "super::playlist_track::Entity")]
  PlaylistTrack,
}

impl Related<super::local_src::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::LocalSrc.def()
  }
}

impl Related<super::playlist_track::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::PlaylistTrack.def()
  }
}

impl Related<super::playlist::Entity> for Entity {
  fn to() -> RelationDef {
    super::playlist_track::Relation::Playlist.def()
  }
  fn via() -> Option<RelationDef> {
    Some(super::playlist_track::Relation::Track.def().rev())
  }
}

impl ActiveModelBehavior for ActiveModel {}
