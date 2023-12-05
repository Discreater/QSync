use std::path::PathBuf;

use entity::prelude::*;
use migration::{Migrator, MigratorTrait};

use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{DbManager, MusyncError, PlaylistId, TrackId};

impl DbManager {
  pub fn new(db: DatabaseConnection, _data_folder: PathBuf) -> Self {
    Self { db }
  }

  pub async fn from_url(db_url: &str, data_folder: PathBuf) -> crate::error::Result<Self> {
    let db = Database::connect(db_url).await?;
    Migrator::up(&db, None).await?;
    Ok(Self::new(db, data_folder))
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
    DbManager::new(db, PathBuf::from("./target/test_data"))
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
