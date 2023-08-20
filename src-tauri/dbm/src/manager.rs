use abi::{CreatePlaylistRequest, LocalFolder, LoginRequest, QueryLocalFoldersRequest};
use chrono::Utc;
use entity::prelude::*;
use migration::{Migrator, MigratorTrait};

use sea_orm::{
  ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, Database, DatabaseConnection,
  EntityTrait, QueryFilter, QueryTrait, Set, TransactionTrait,
};
use tracing::{info, trace, warn};

use crate::{DbManager, MusyncError, PlaylistId, TrackId, UserId};

impl DbManager {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db }
  }

  pub async fn from_url(db_url: &str) -> crate::error::Result<Self> {
    let db = Database::connect(db_url).await?;
    Migrator::up(&db, None).await?;
    Ok(Self::new(db))
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

  pub async fn create_playlist(
    &self,
    owner_id: UserId,
    create: CreatePlaylistRequest,
  ) -> Result<abi::Playlist, MusyncError> {
    let now = Utc::now();
    // create playlist
    let inserted = entity::playlist::ActiveModel {
      id: NotSet,
      owner_id: Set(owner_id),
      name: Set(create.name.clone()),
      description: Set(create.description.clone()),
      created_at: Set(now),
      updated_at: Set(now),
      temp: Set(create.temp)
    }
    .insert(&self.db)
    .await?;

    trace!("playlist inserted, id: {}", inserted.id);

    let mut playlist = abi::Playlist::from_entity(inserted, vec![]);

    if create.track_ids.is_empty() {
      return Ok(playlist);
    }

    let track_list = create
      .track_ids
      .iter()
      .map(|track_id| entity::playlist_track::ActiveModel {
        playlist_id: Set(playlist.id),
        track_id: Set(*track_id),
      });
    PlaylistTrack::insert_many(track_list)
      .exec(&self.db)
      .await?;

    playlist.track_ids.extend(create.track_ids);

    Ok(playlist)
  }

  pub async fn update_playlist(
    &self,
    playlist: abi::UpdatePlaylistRequest,
  ) -> Result<abi::Playlist, MusyncError> {
    let now = Utc::now();

    let old = Playlist::find_by_id(playlist.id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::PlaylistNotFound(playlist.id))?;

    let mut updating: entity::playlist::ActiveModel = old.into();
    if let Some(name) = playlist.name {
      updating.name = Set(name);
    }
    if let Some(description) = playlist.description {
      updating.description = Set(description);
    }
    updating.updated_at = Set(now);
    updating.update(&self.db).await?;

    if !playlist.added_track_ids.is_empty() {
      let added_tracks =
        playlist
          .added_track_ids
          .iter()
          .map(|track_id| entity::playlist_track::ActiveModel {
            playlist_id: Set(playlist.id),
            track_id: Set(*track_id),
          });
      PlaylistTrack::insert_many(added_tracks)
        .exec(&self.db)
        .await?;
    }

    if !playlist.removed_track_ids.is_empty() {
      PlaylistTrack::delete_many()
        .filter(
          Condition::all()
            .add(entity::playlist_track::Column::PlaylistId.eq(playlist.id))
            .add(entity::playlist_track::Column::TrackId.is_in(playlist.removed_track_ids)),
        )
        .exec(&self.db)
        .await?;
    }

    self.playlist(playlist.id).await
  }

  pub async fn delete_playlists(&self, ids: &[PlaylistId]) -> Result<u64, MusyncError> {
    if ids.is_empty() {
      return Ok(0);
    }

    let deleted = Playlist::delete_many()
      .filter(entity::playlist::Column::Id.is_in(ids.to_owned()))
      .exec(&self.db)
      .await?;

    Ok(deleted.rows_affected)
  }

  pub async fn query_playlists(
    &self,
    query: abi::QueryPlaylistsRequest,
  ) -> Result<Vec<abi::Playlist>, MusyncError> {
    let playlists = Playlist::find()
      .apply_if(query.name, |builder, name| {
        builder.filter(entity::playlist::Column::Name.like(format!("%{}%", name)))
      })
      .apply_if(query.user_id, |b, user_id| {
        b.inner_join(UserPlaylist)
          .filter(entity::user_playlist::Column::UserId.eq(user_id))
      })
      .apply_if(query.track_id, |b, track_id| {
        b.inner_join(PlaylistTrack)
          .filter(entity::playlist_track::Column::TrackId.eq(track_id))
      })
      .all(&self.db)
      .await?;

    Ok(
      playlists
        .into_iter()
        .map(|row| abi::Playlist::from_entity(row, vec![]))
        .collect(),
    )
  }

  pub async fn playlist(&self, id: PlaylistId) -> Result<abi::Playlist, MusyncError> {
    let queried = Playlist::find_by_id(id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::PlaylistNotFound(id))?;

    let tracks: Vec<_> = self.get_tracks_in_playlist(id).await?;

    Ok(abi::Playlist::from_entity(queried, tracks))
  }

  pub async fn create_track(&self, track: abi::Track) -> Result<abi::Track, MusyncError> {
    let txn = self.db.begin().await?;
    let now = Utc::now();
    let inserted = entity::track::ActiveModel {
      id: NotSet,
      title: Set(track.title.clone()),
      artist: Set(track.artist.clone()),
      album: Set(track.album.clone()),
      duration: Set(track.duration),
      genre: Set(track.genre.clone()),
      year: Set(track.year),
      created_at: Set(now),
      updated_at: Set(now),
    }
    .insert(&txn)
    .await?;
    if let Some(src) = track.local_src {
      entity::local_src::ActiveModel {
        track_id: Set(inserted.id),
        path: Set(src.path),
        folder_id: NotSet,
      }
      .insert(&txn)
      .await?;
    }
    txn.commit().await?;
    Ok(abi::Track::from_entity(inserted))
  }

  pub async fn create_tracks(
    &self,
    tracks: Vec<abi::Track>,
    folder: &str,
  ) -> Result<TrackId, MusyncError> {
    let txn = self.db.begin().await?;
    let now = Utc::now();
    let folder = entity::local_src_folder::ActiveModel {
      id: NotSet,
      path: Set(folder.to_owned()),
      created_at: Set(now),
      updated_at: Set(now),
    }
    .insert(&txn)
    .await
    .map_err(|_| MusyncError::FolderExists(folder.to_owned()))?;
    let active_tracks = tracks.iter().map(|t| entity::track::ActiveModel {
      id: NotSet,
      title: Set(t.title.clone()),
      artist: Set(t.artist.clone()),
      album: Set(t.album.clone()),
      duration: Set(t.duration),
      genre: Set(t.genre.clone()),
      year: Set(t.year),
      created_at: Set(now),
      updated_at: Set(now),
    });
    let inserted = Track::insert_many(active_tracks).exec(&txn).await?;
    info!(
      "inserted tracks: {}",
      inserted.last_insert_id - tracks.len() as i32
    );
    let start_id = inserted.last_insert_id - tracks.len() as i32 + 1;
    let mut local_srcs = vec![];
    for (idx, track) in tracks.iter().enumerate() {
      if let Some(src) = track.local_src.as_ref() {
        local_srcs.push(entity::local_src::ActiveModel {
          track_id: Set(start_id + idx as i32),
          path: Set(src.path.clone()),
          folder_id: Set(Some(folder.id)),
        });
      }
      if let Some(_src) = track.netease_src.as_ref() {
        warn!("netease_src is not supported yet");
      }
    }
    LocalSrc::insert_many(local_srcs).exec(&txn).await?;
    txn.commit().await?;
    Ok(inserted.last_insert_id)
  }

  pub async fn remove_folder(&self, folder: &str) -> Result<u64, MusyncError> {
    let txn = self.db.begin().await?;
    // delete cascade
    LocalSrcFolder::delete_many()
      .filter(entity::local_src_folder::Column::Path.eq(folder))
      .exec(&txn)
      .await
      .map_err(|_| (MusyncError::FolderNotFound(folder.to_owned())))?;
    // delete tracks without local_src and netease_src
    use sea_orm::query::*;
    let deleted = Track::delete_many()
      .filter(
        entity::track::Column::Id.not_in_subquery(
          LocalSrc::find()
            .select_only()
            .column(entity::local_src::Column::TrackId)
            .into_query(),
        ),
      )
      .exec(&txn)
      .await?;
    txn.commit().await?;
    trace!("deleted tracks: {:?}", deleted);
    Ok(deleted.rows_affected)
  }

  pub async fn query_local_folders(
    &self,
    query: QueryLocalFoldersRequest,
  ) -> Result<Vec<LocalFolder>, MusyncError> {
    let QueryLocalFoldersRequest {} = query;
    let folders = LocalSrcFolder::find().all(&self.db).await?;
    Ok(folders.into_iter().map(LocalFolder::from_entity).collect())
  }

  pub async fn update_track(&self, update: abi::TrackUpdate) -> Result<abi::Track, MusyncError> {
    if update.netease_src.is_some() {
      warn!("netease_src is not supported yet");
    }
    let now = Utc::now();
    let old = Track::find_by_id(update.id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::TrackNotFound(update.id))?;

    let mut updating: entity::track::ActiveModel = old.into();
    if let Some(title) = update.title {
      updating.title = Set(title);
    }
    if let Some(artist) = update.artist {
      updating.artist = Set(Some(artist));
    }
    if let Some(album) = update.album {
      updating.album = Set(Some(album));
    }
    if let Some(duration) = update.duration {
      updating.duration = Set(Some(duration));
    }
    if let Some(genre) = update.genre {
      updating.genre = Set(Some(genre));
    }
    if let Some(year) = update.year {
      updating.year = Set(Some(year));
    }
    updating.updated_at = Set(now);
    updating.update(&self.db).await?;

    if let Some(src) = update.local_src {
      let old = LocalSrc::find_by_id(update.id).one(&self.db).await?;
      if let Some(old) = old {
        let mut updating: entity::local_src::ActiveModel = old.into();
        updating.path = Set(src.path);
        updating.update(&self.db).await?;
      } else {
        let inserted = entity::local_src::ActiveModel {
          track_id: Set(update.id),
          path: Set(src.path),
          folder_id: NotSet,
        };
        inserted.insert(&self.db).await?;
      }
    }

    self.track(update.id).await
  }

  pub async fn delete_tracks(&self, ids: &[TrackId]) -> Result<u64, MusyncError> {
    if ids.is_empty() {
      return Ok(0);
    }

    let deleted = Track::delete_many()
      .filter(entity::track::Column::Id.is_in(ids.to_owned()))
      .exec(&self.db)
      .await?;
    Ok(deleted.rows_affected)
  }

  pub async fn query_tracks(
    &self,
    query: abi::QueryTracksRequest,
  ) -> Result<Vec<abi::Track>, MusyncError> {
    let rows = Track::find()
      .apply_if(query.title, |b, title| {
        b.filter(entity::track::Column::Title.like(format!("%{}%", title)))
      })
      .apply_if(query.playlist_id, |b, playlist_id| {
        b.inner_join(PlaylistTrack)
          .filter(entity::playlist_track::Column::PlaylistId.eq(playlist_id))
      })
      .apply_if(query.artist, |b, artist| {
        b.filter(entity::track::Column::Artist.like(format!("%{}%", artist)))
      })
      .apply_if(query.album, |b, album| {
        b.filter(entity::track::Column::Album.like(format!("%{}%", album)))
      })
      .all(&self.db)
      .await?;

    rows
      .into_iter()
      .map(|row| Ok(abi::Track::from_entity(row)))
      .collect()
  }

  pub async fn track(&self, id: TrackId) -> Result<abi::Track, MusyncError> {
    let row = Track::find_by_id(id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::TrackNotFound(id))?;

    let mut abi_track = abi::Track::from_entity(row);
    let src = LocalSrc::find_by_id(id).one(&self.db).await?;
    if let Some(src) = src {
      abi_track.local_src = Some(abi::LocalSource { path: src.path })
    }

    Ok(abi_track)
  }

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

  pub async fn user(&self, id: UserId) -> Result<abi::User, MusyncError> {
    let row = User::find_by_id(id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::UserNotFound(id))?;
    Ok(abi::User::from_entity(row))
  }
}

impl DbManager {
  pub async fn get_tracks_in_playlist(
    &self,
    playlist_id: PlaylistId,
  ) -> Result<Vec<TrackId>, MusyncError> {
    Ok(
      PlaylistTrack::find()
        .filter(entity::playlist_track::Column::PlaylistId.eq(playlist_id))
        .all(&self.db)
        .await?
        .into_iter()
        .map(|row| row.track_id)
        .collect(),
    )
  }
}

#[cfg(test)]
mod tests {
  use migration::{Migrator, MigratorTrait};

  use sea_orm::Database;
  use tracing::Level;

  use super::*;

  fn init_tracing() {
    tracing_subscriber::fmt()
      .with_max_level(Level::DEBUG)
      .init();
  }

  pub async fn init_manager() -> DbManager {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    DbManager::new(db)
  }

  pub async fn create_test_user(manager: &DbManager) -> abi::User {
    manager.create_user(abi::User::new("test")).await.unwrap()
  }

  #[tokio::test]
  pub async fn create_playlist_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::CreatePlaylistRequest::new("test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(user.id, playlist).await.unwrap();
    assert_ne!(playlist.id, 0);
    assert!(playlist.created_at.is_some());
    assert!(playlist.updated_at.is_some());
  }

  #[tokio::test]
  pub async fn update_playlist_info_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::CreatePlaylistRequest::new("test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(user.id, playlist).await.unwrap();

    let playlist = manager
      .update_playlist(abi::UpdatePlaylistRequest {
        id: playlist.id,
        name: Some("test2".to_string()),
        description: Some("test2".to_string()),
        added_track_ids: vec![],
        removed_track_ids: vec![],
      })
      .await
      .unwrap();

    assert_ne!(playlist.id, 0);
    assert_eq!(playlist.name, "test2");
    assert_eq!(playlist.description, "test2");
    assert!(playlist.created_at.is_some());
    assert!(playlist.updated_at.is_some());
  }

  #[tokio::test]
  pub async fn delete_playlists_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::CreatePlaylistRequest::new("test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(user.id, playlist).await.unwrap();

    let deleted = manager.delete_playlists(&[playlist.id]).await.unwrap();
    assert_eq!(deleted, 1);
    // manager.playlist(deleted[0].id).await.unwrap_err();
    // assert_eq!(deleted[0].id, playlist.id);
  }

  #[tokio::test]
  pub async fn query_playlists_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::CreatePlaylistRequest::new("test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(user.id, playlist).await.unwrap();

    let playlists = manager
      .query_playlists(abi::QueryPlaylistsRequest {
        name: Some("test".to_string()),
        ..Default::default()
      })
      .await
      .unwrap();
    assert_eq!(playlists.len(), 1);
    assert_eq!(playlists[0].id, playlist.id);
  }

  #[tokio::test]
  pub async fn create_track_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track {
        title: "track_name".to_owned(),
        album: Some("discc".to_owned()),
        artist: Some("cras".to_owned()),
        duration: Some(210),
        local_src: Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
        ..Default::default()
      })
      .await
      .unwrap();
    assert_ne!(track.id, 0);
  }

  #[tokio::test]
  pub async fn update_track_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track {
        title: "track_name".to_owned(),
        album: Some("discc".to_owned()),
        artist: Some("cras".to_owned()),
        duration: Some(210),
        local_src: Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
        ..Default::default()
      })
      .await
      .unwrap();

    let track = manager
      .update_track(abi::TrackUpdate {
        id: track.id,
        title: Some("track_name2".to_owned()),
        artist: Some("discc2".to_owned()),
        album: Some("cras2".to_owned()),
        duration: Some(214),
        local_src: Some(abi::LocalSource {
          path: "/path/to/file2".to_owned(),
        }),
        ..Default::default()
      })
      .await
      .unwrap();

    assert_eq!(track.title, "track_name2");
    assert_eq!(track.artist, Some("discc2".to_owned()));
    assert_eq!(track.album, Some("cras2".to_owned()));
    assert_eq!(track.duration, Some(214));
    assert_eq!(track.local_src.unwrap().path, "/path/to/file2");
  }

  #[tokio::test]
  pub async fn delete_tracks_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track {
        title: "track_title".to_owned(),
        album: Some("discc".to_owned()),
        artist: Some("cras".to_owned()),
        duration: Some(210),
        local_src: Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
        ..Default::default()
      })
      .await
      .unwrap();

    let deleted = manager.delete_tracks(&[track.id]).await.unwrap();
    assert_eq!(deleted, 1);
    // manager.track(deleted[0].id).await.unwrap_err();
    // assert_eq!(deleted[0].id, track.id);
  }

  #[tokio::test]
  pub async fn query_tracks_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track {
        title: "track_title".to_owned(),
        album: Some("discc".to_owned()),
        artist: Some("cras".to_owned()),
        duration: Some(210),
        local_src: Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
        ..Default::default()
      })
      .await
      .unwrap();

    let tracks = manager
      .query_tracks(abi::QueryTracksRequest {
        title: Some("track_title".to_owned()),
        album: Some("discc".to_owned()),
        artist: Some("cras".to_owned()),
        ..Default::default()
      })
      .await
      .unwrap();
    assert_eq!(tracks.len(), 1);
    assert_eq!(tracks[0].id, track.id);
  }

  #[tokio::test]
  pub async fn create_user_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = manager.create_user(abi::User::new("user1")).await.unwrap();
    assert_ne!(user.id, 0);
  }

  #[tokio::test]
  pub async fn update_user_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = manager.create_user(abi::User::new("user1")).await.unwrap();

    let user = manager
      .update_user(abi::UserUpdate {
        id: user.id,
        name: "user2".to_owned(),
      })
      .await
      .unwrap();

    assert_eq!(user.name, "user2");
  }

  #[tokio::test]
  pub async fn delete_users_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = manager.create_user(abi::User::new("user1")).await.unwrap();

    let deleted = manager.delete_users(&[user.id]).await.unwrap();
    assert_eq!(deleted, 1);
    // manager.user(deleted[0].id).await.unwrap_err();
    // assert_eq!(deleted[0].id, user.id);
  }

  #[tokio::test]
  pub async fn query_users_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = manager.create_user(abi::User::new("user1")).await.unwrap();

    let users = manager
      .query_users(abi::QueryUsersRequest {
        name: Some("user1".to_owned()),
      })
      .await
      .unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, user.id);
  }
}
