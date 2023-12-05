use sea_orm_migration::prelude::*;

use crate::m20230813_000001_create_table::Track;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(NeteaseSrc::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(NeteaseSrc::TrackId)
              .integer()
              .not_null()
              .primary_key(),
          )
          .col(
            ColumnDef::new(NeteaseSrc::NeteaseId)
              .string()
              .not_null()
              .unique_key(),
          )
          .col(ColumnDef::new(NeteaseSrc::Pop).float())
          .col(ColumnDef::new(NeteaseSrc::RawJson).string())
          .foreign_key(
            ForeignKey::create()
              .name("fk-netease_src-track_id")
              .from(NeteaseSrc::Table, NeteaseSrc::TrackId)
              .to(Track::Table, Track::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager.create_index(
      sea_query::Index::create()
        .name("idx-netease_src-netease_id")
        .table(NeteaseSrc::Table)
        .col(NeteaseSrc::NeteaseId)
        .to_owned(),
    ).await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(
        Table::drop()
          .if_exists()
          .table(NeteaseSrc::Table)
          .to_owned(),
      )
      .await
  }
}

#[derive(DeriveIden)]
enum NeteaseSrc {
  Table,
  TrackId,
  NeteaseId,
  Pop,
  RawJson,
}
