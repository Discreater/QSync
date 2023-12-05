use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Track::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Track::Id)
              .integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(Track::Title).string().not_null())
          .col(ColumnDef::new(Track::Artist).string())
          .col(ColumnDef::new(Track::Album).string())
          .col(ColumnDef::new(Track::Duration).unsigned())
          .col(ColumnDef::new(Track::Genre).string())
          .col(ColumnDef::new(Track::Year).unsigned())
          .col(ColumnDef::new(Track::CreatedAt).timestamp().not_null())
          .col(ColumnDef::new(Track::UpdatedAt).timestamp().not_null())
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        sea_query::Index::create()
          .name("idx-track-title")
          .table(Track::Table)
          .col(Track::Title)
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        sea_query::Index::create()
          .name("idx-track-artist")
          .table(Track::Table)
          .col(Track::Artist)
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        sea_query::Index::create()
          .name("idx-track-album")
          .table(Track::Table)
          .col(Track::Album)
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        sea_query::Index::create()
          .name("idx-track-genre")
          .table(Track::Table)
          .col(Track::Genre)
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        sea_query::Index::create()
          .name("idx-track-year")
          .table(Track::Table)
          .col(Track::Year)
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(LocalSrc::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(LocalSrc::TrackId)
              .integer()
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(LocalSrc::Path).string().not_null())
          .col(ColumnDef::new(LocalSrc::FolderId).integer())
          .foreign_key(
            ForeignKey::create()
              .name("fk-local_src-track_id")
              .from(LocalSrc::Table, LocalSrc::TrackId)
              .to(Track::Table, Track::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-local_src-folder_id")
              .from(LocalSrc::Table, LocalSrc::FolderId)
              .to(LocalSrcFolder::Table, LocalSrcFolder::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(Playlist::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Playlist::Id)
              .integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(Playlist::OwnerId).integer().not_null())
          .col(ColumnDef::new(Playlist::Name).string().not_null())
          .col(
            ColumnDef::new(Playlist::Description)
              .string()
              .not_null()
              .default("".to_string()),
          )
          .col(ColumnDef::new(Playlist::CreatedAt).timestamp().not_null())
          .col(ColumnDef::new(Playlist::UpdatedAt).timestamp().not_null())
          .foreign_key(
            ForeignKey::create()
              .name("fk-playlist-owner_id")
              .from(Playlist::Table, Playlist::OwnerId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        sea_query::Index::create()
          .name("idx-playlist-owner_id")
          .table(Playlist::Table)
          .col(Playlist::OwnerId)
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        Index::create()
          .name("idx-playlist-name")
          .table(Playlist::Table)
          .col(Playlist::Name)
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(User::Id)
              .integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(User::Name).string().not_null())
          .col(ColumnDef::new(User::PlayQueueId).integer())
          .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
          .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
          .foreign_key(
            ForeignKey::create()
              .name("fk-user-play_queue_id")
              .from(User::Table, User::PlayQueueId)
              .to(PlayQueue::Table, PlayQueue::Id),
          )
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        sea_query::Index::create()
          .name("idx-user-name")
          .table(User::Table)
          .col(User::Name)
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(PlaylistTrack::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(PlaylistTrack::PlaylistId)
              .integer()
              .not_null(),
          )
          .col(ColumnDef::new(PlaylistTrack::TrackId).integer().not_null())
          .primary_key(
            Index::create()
              .col(PlaylistTrack::PlaylistId)
              .col(PlaylistTrack::TrackId),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-playlist_track-playlist_id")
              .from(PlaylistTrack::Table, PlaylistTrack::PlaylistId)
              .to(Playlist::Table, Playlist::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-playlist_track-track_id")
              .from(PlaylistTrack::Table, PlaylistTrack::TrackId)
              .to(Track::Table, Track::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(UserPlaylist::Table)
          .if_not_exists()
          .col(ColumnDef::new(UserPlaylist::UserId).integer().not_null())
          .col(
            ColumnDef::new(UserPlaylist::PlaylistId)
              .integer()
              .not_null(),
          )
          .primary_key(
            Index::create()
              .col(UserPlaylist::UserId)
              .col(UserPlaylist::PlaylistId),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-user_playlist-user_id")
              .from(UserPlaylist::Table, UserPlaylist::UserId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-user_playlist-playlist_id")
              .from(UserPlaylist::Table, UserPlaylist::PlaylistId)
              .to(Playlist::Table, Playlist::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        Index::create()
          .name("idx-user_playlist-user_id")
          .table(UserPlaylist::Table)
          .col(UserPlaylist::UserId)
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        Index::create()
          .name("idx-user_playlist-playlist_id")
          .table(UserPlaylist::Table)
          .col(UserPlaylist::PlaylistId)
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(PlayQueue::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(PlayQueue::Id)
              .integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(PlayQueue::Position).integer().not_null())
          .col(ColumnDef::new(PlayQueue::Playing).boolean().not_null())
          .col(ColumnDef::new(PlayQueue::StartedAt).timestamp().not_null())
          .col(ColumnDef::new(PlayQueue::PausedAt).unsigned().not_null())
          .col(ColumnDef::new(PlayQueue::CreatedAt).timestamp().not_null())
          .col(ColumnDef::new(PlayQueue::UpdatedAt).timestamp().not_null())
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(PlayQueueTrack::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(PlayQueueTrack::PlayQueueId)
              .integer()
              .not_null(),
          )
          .col(ColumnDef::new(PlayQueueTrack::TrackId).integer().not_null())
          .col(
            ColumnDef::new(PlayQueueTrack::Position)
              .integer()
              .not_null(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-play_queue_track-play_queue_id")
              .from(PlayQueueTrack::Table, PlayQueueTrack::PlayQueueId)
              .to(PlayQueue::Table, PlayQueue::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-play_queue_track-track_id")
              .from(PlayQueueTrack::Table, PlayQueueTrack::TrackId)
              .to(Track::Table, Track::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .primary_key(
            Index::create()
              .col(PlayQueueTrack::PlayQueueId)
              .col(PlayQueueTrack::TrackId)
              .col(PlayQueueTrack::Position),
          )
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        Index::create()
          .name("idx-play_queue_track-play_queue_id")
          .table(PlayQueueTrack::Table)
          .col(PlayQueueTrack::PlayQueueId)
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        Index::create()
          .name("idx-play_queue_track-track_id")
          .table(PlayQueueTrack::Table)
          .col(PlayQueueTrack::TrackId)
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(LocalSrcFolder::Table)
          .col(
            ColumnDef::new(LocalSrcFolder::Id)
              .integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(
            ColumnDef::new(LocalSrcFolder::Path)
              .string()
              .not_null()
              .unique_key(),
          )
          .col(
            ColumnDef::new(LocalSrcFolder::CreatedAt)
              .timestamp()
              .not_null(),
          )
          .col(
            ColumnDef::new(LocalSrcFolder::UpdatedAt)
              .timestamp()
              .not_null(),
          )
          .to_owned(),
      )
      .await?;
    manager
      .create_index(
        Index::create()
          .name("idx-local_src_folder-path")
          .table(LocalSrcFolder::Table)
          .col(LocalSrcFolder::Path)
          .to_owned(),
      )
      .await?;

    let now = chrono::Utc::now();

    let insert = Query::insert()
      .into_table(User::Table)
      .columns([User::Name, User::CreatedAt, User::UpdatedAt])
      .values_panic([
        "Kerse".into(),
        now.to_rfc3339().into(),
        now.to_rfc3339().into(),
      ])
      .to_owned();
    manager.exec_stmt(insert).await?;
    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(
        Table::drop()
          .if_exists()
          .table(LocalSrcFolder::Table)
          .to_owned(),
      )
      .await?;
    manager
      .drop_table(
        Table::drop()
          .if_exists()
          .table(PlayQueueTrack::Table)
          .to_owned(),
      )
      .await?;
    manager
      .drop_table(Table::drop().if_exists().table(PlayQueue::Table).to_owned())
      .await?;
    manager
      .drop_table(
        Table::drop()
          .if_exists()
          .table(PlaylistTrack::Table)
          .to_owned(),
      )
      .await?;
    manager
      .drop_table(
        Table::drop()
          .if_exists()
          .table(UserPlaylist::Table)
          .to_owned(),
      )
      .await?;
    manager
      .drop_table(Table::drop().if_exists().table(LocalSrc::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().if_exists().table(Track::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().if_exists().table(Playlist::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
      .await?;

    Ok(())
  }
}

#[derive(DeriveIden)]
pub(crate) enum Track {
  Table,
  Id,
  Title,
  Artist,
  Album,
  Duration,
  Genre,
  Year,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum LocalSrc {
  Table,
  TrackId,
  Path,
  FolderId,
}

#[derive(DeriveIden)]
enum Playlist {
  Table,
  Id,
  OwnerId,
  Name,
  Description,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
  Table,
  Id,
  Name,
  PlayQueueId,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum PlaylistTrack {
  Table,
  PlaylistId,
  TrackId,
}

#[derive(DeriveIden)]
enum UserPlaylist {
  Table,
  UserId,
  PlaylistId,
}

#[derive(DeriveIden)]
enum PlayQueue {
  Table,
  Id,
  Position,
  Playing,
  StartedAt,
  PausedAt,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveIden)]
enum PlayQueueTrack {
  Table,
  PlayQueueId,
  TrackId,
  Position,
}

#[derive(DeriveIden)]
enum LocalSrcFolder {
  Table,
  Id,
  Path,
  CreatedAt,
  UpdatedAt,
}
