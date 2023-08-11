use std::vec;

use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tracing::{trace, warn};

use crate::{Musync, MusyncError, Musyncer, PlaylistId, TrackId, UserId};
const BIND_LIMIT: usize = 256;

impl Musyncer {
  pub fn new(pool: SqlitePool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl Musync for Musyncer {
  async fn create_playlist(
    &self,
    mut playlist: abi::Playlist,
    tracks: &[TrackId],
  ) -> Result<abi::Playlist, MusyncError> {
    // create playlist
    let inserted = sqlx::query!(
      "INSERT INTO playlists (owner_id, name, description) VALUES ($1, $2, $3) RETURNING id, created_at, updated_at",
      playlist.owner_id,
      playlist.name,
      playlist.description
    )
    .fetch_one(&self.pool)
    .await?;

    trace!("playlist inserted, id: {}", inserted.id);

    playlist.id = inserted.id;
    playlist.created_at = Some(abi::convert_to_timestamp(&inserted.created_at));
    playlist.updated_at = Some(abi::convert_to_timestamp(&inserted.updated_at));

    if tracks.is_empty() {
      return Ok(playlist);
    }

    let track_list = tracks.iter().map(|track_id| (playlist.id, track_id));
    QueryBuilder::<Sqlite>::new("INSERT INTO playlist_tracks (playlist_id, track_id) ")
      .push_values(
        track_list.take(BIND_LIMIT / 4),
        |mut b, (playlist_id, track_id)| {
          b.push_bind(playlist_id);
          b.push_bind(track_id);
        },
      )
      .build()
      .execute(&self.pool)
      .await?;
    playlist.track_ids.extend(tracks);

    Ok(playlist)
  }

  async fn update_playlist(
    &self,
    playlist: abi::PlaylistUpdate,
  ) -> Result<abi::Playlist, MusyncError> {
    sqlx::query!(
      "UPDATE playlists SET name = $1, description = $2 WHERE id = $3",
      playlist.name,
      playlist.description,
      playlist.id
    )
    .execute(&self.pool)
    .await?;

    if !playlist.added_track_ids.is_empty() {
      QueryBuilder::<Sqlite>::new("INSERT INTO playlist_tracks (playlist_id, track_id) ")
        .push_values(
          playlist
            .added_track_ids
            .iter()
            .map(|track_id| (playlist.id, track_id))
            .take(BIND_LIMIT / 4),
          |mut b, (playlist_id, track_id)| {
            b.push_bind(playlist_id);
            b.push_bind(track_id);
          },
        )
        .build()
        .execute(&self.pool)
        .await?;
    }

    if !playlist.removed_track_ids.is_empty() {
      QueryBuilder::<Sqlite>::new(
        "DELETE FROM playlist_tracks WHERE playlist_id = $1 AND track_id IN ",
      )
      .push_tuples(
        playlist.removed_track_ids.iter().take(BIND_LIMIT / 4),
        |mut b, track_id| {
          b.push_bind(track_id);
        },
      )
      .build()
      .execute(&self.pool)
      .await?;
    }

    self.playlist(playlist.id).await
  }

  async fn delete_playlists(&self, ids: &[PlaylistId]) -> Result<Vec<abi::Playlist>, MusyncError> {
    if ids.is_empty() {
      return Ok(vec![]);
    }

    let deleted = QueryBuilder::<Sqlite>::new("DELETE FROM playlists WHERE id IN ")
      .push_tuples(ids.iter().take(BIND_LIMIT / 4), |mut b, id| {
        b.push_bind(id);
      })
      .push("RETURNING *")
      .build()
      .fetch_all(&self.pool)
      .await?;

    deleted
      .into_iter()
      .map(|row| Ok(abi::Playlist::from_row(row, vec![])?))
      .collect()
  }

  async fn query_playlists(
    &self,
    query: abi::PlaylistQuery,
  ) -> Result<Vec<abi::Playlist>, MusyncError> {
    let mut querier = QueryBuilder::<Sqlite>::new("SELECT * from playlists");
    let mut whered = false;
    if let Some(name) = query.name {
      if !whered {
        querier.push(" WHERE")
      } else {
        querier.push(" OR")
      }
      .push(" name LIKE '%' || ")
      .push_bind(name)
      .push(" || '%'");
      whered = true;
    }

    if let Some(user_id) = query.user_id {
      if !whered {
        querier.push(" WHERE")
      } else {
        querier.push(" OR")
      }
      .push(" id IN (SELECT playlist_id FROM users_playlists WHERE user_id = ")
      .push_bind(user_id)
      .push(")");
      whered = true;
    }

    if let Some(track_id) = query.track_id {
      if !whered {
        querier.push(" WHERE")
      } else {
        querier.push(" OR")
      }
      .push(" id IN (SELECT playlist_id FROM playlists_tracks WHERE track_id = ")
      .push_bind(track_id)
      .push(")");
    }

    let rows = querier.build().fetch_all(&self.pool).await?;

    rows
      .into_iter()
      .map(|row| Ok(abi::Playlist::from_row(row, vec![])?))
      .collect()
  }

  async fn playlist(&self, id: PlaylistId) -> Result<abi::Playlist, MusyncError> {
    let queried = sqlx::query!(
      "SELECT id, owner_id, name, description, created_at, updated_at FROM playlists WHERE id = $1",
      id
    )
    .fetch_one(&self.pool)
    .await?;

    let tracks: Vec<_> = self.get_tracks_in_playlist(id).await?;

    Ok(abi::Playlist {
      id: queried.id,
      owner_id: queried.owner_id,
      name: queried.name,
      description: queried.description,
      created_at: Some(abi::convert_to_timestamp(&queried.created_at)),
      updated_at: Some(abi::convert_to_timestamp(&queried.updated_at)),
      track_ids: tracks,
    })
  }

  async fn create_track(&self, mut track: abi::Track) -> Result<abi::Track, MusyncError> {
    if track.netease_src.is_some() {
      warn!("netease_src is not supported yet");
    }
    let local_src_path = track.local_src.as_ref().map(|s| s.path.clone());

    let id = sqlx::query!(
      "INSERT INTO tracks (name, artist, album, duration, local_src_path) VALUES ($1, $2, $3, $4, $5) RETURNING id",
      track.name,
      track.artist,
      track.album,
      track.duration,
      local_src_path
    ).fetch_one(&self.pool).await?.id;

    track.id = id;

    Ok(track)
  }

  async fn update_track(&self, update: abi::TrackUpdate) -> Result<abi::Track, MusyncError> {
    if update.netease_src.is_some() {
      warn!("netease_src is not supported yet");
    }
    let local_src_path = update.local_src.as_ref().map(|s| s.path.clone());
    sqlx::query!(
      "UPDATE tracks SET name = $2, artist = $3, album = $4, duration = $5, local_src_path = $6 WHERE id = $1",
      update.id,
      update.name,
      update.artist,
      update.album,
      update.duration,
      local_src_path,
    )
    .execute(&self.pool)
    .await?;

    self.track(update.id).await
  }

  async fn delete_tracks(&self, ids: &[TrackId]) -> Result<Vec<abi::Track>, MusyncError> {
    if ids.is_empty() {
      return Ok(vec![]);
    }

    let deleted = QueryBuilder::<Sqlite>::new("DELETE FROM tracks WHERE id IN ")
      .push_tuples(ids.iter().take(BIND_LIMIT / 4), |mut b, id| {
        b.push_bind(id);
      })
      .push("RETURNING *")
      .build()
      .fetch_all(&self.pool)
      .await?;

    deleted
      .into_iter()
      .map(|row| Ok(abi::Track::from_row(row)?))
      .collect()
  }

  async fn query_tracks(&self, query: abi::TrackQuery) -> Result<Vec<abi::Track>, MusyncError> {
    let mut querier = QueryBuilder::<Sqlite>::new("SELECT * from tracks");
    let mut whered = false;
    if let Some(name) = query.name {
      if !whered {
        querier.push(" WHERE")
      } else {
        querier.push(" OR")
      }
      .push(" name LIKE '%' || ")
      .push_bind(name)
      .push(" || '%'");
      whered = true;
    }

    if let Some(playlist_id) = query.playlist_id {
      if !whered {
        querier.push(" WHERE")
      } else {
        querier.push(" OR")
      }
      .push(" id IN (SELECT track_id FROM playlists_tracks WHERE playlist_id = ")
      .push_bind(playlist_id)
      .push(")");
      whered = true;
    }

    if let Some(artist) = query.artist {
      if !whered {
        querier.push(" WHERE")
      } else {
        querier.push(" OR")
      }
      .push(" artist LIKE '%' || ")
      .push_bind(artist)
      .push(" || '%'");
      whered = true;
    }

    if let Some(album) = query.album {
      if !whered {
        querier.push(" WHERE")
      } else {
        querier.push(" OR")
      }
      .push(" album LIKE '%' || ")
      .push_bind(album)
      .push(" || '%'");
    }

    let rows = querier.build().fetch_all(&self.pool).await?;

    rows
      .into_iter()
      .map(|row| Ok(abi::Track::from_row(row)?))
      .collect()
  }

  async fn track(&self, id: TrackId) -> Result<abi::Track, MusyncError> {
    let row = sqlx::query(
      "SELECT * from tracks
        WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await?;

    Ok(abi::Track::from_row(row)?)
  }

  async fn create_user(&self, mut user: abi::User) -> Result<abi::User, MusyncError> {
    let id = sqlx::query!(
      "INSERT INTO users (name) VALUES ($1) RETURNING id",
      user.name,
    )
    .fetch_one(&self.pool)
    .await?
    .id;
    user.id = id;

    Ok(user)
  }

  async fn update_user(&self, update: abi::UserUpdate) -> Result<abi::User, MusyncError> {
    sqlx::query!(
      "UPDATE users SET name = $2 WHERE id = $1",
      update.id,
      update.name,
    )
    .execute(&self.pool)
    .await?;

    self.user(update.id).await
  }

  async fn delete_users(&self, ids: &[UserId]) -> Result<Vec<abi::User>, MusyncError> {
    if ids.is_empty() {
      return Ok(vec![]);
    }

    let deleted = QueryBuilder::<Sqlite>::new("DELETE FROM users WHERE id IN ")
      .push_tuples(ids.iter().take(BIND_LIMIT / 4), |mut b, id| {
        b.push_bind(id);
      })
      .push("RETURNING *")
      .build()
      .fetch_all(&self.pool)
      .await?;

    deleted
      .into_iter()
      .map(|row| Ok(abi::User::from_row(row)?))
      .collect()
  }

  async fn query_users(&self, query: abi::UserQuery) -> Result<Vec<abi::User>, MusyncError> {
    let rows = sqlx::query(
      "SELECT * from users
        WHERE name LIKE '%' || $1 || '%'",
    )
    .bind(query.name)
    .fetch_all(&self.pool)
    .await?;
    rows
      .into_iter()
      .map(|row| Ok(abi::User::from_row(row)?))
      .collect()
  }

  async fn user(&self, id: UserId) -> Result<abi::User, MusyncError> {
    let row = sqlx::query(
      "SELECT * from users
        WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await?;
    Ok(abi::User::from_row(row)?)
  }
}

impl Musyncer {
  async fn get_tracks_in_playlist(
    &self,
    playlist_id: PlaylistId,
  ) -> Result<Vec<TrackId>, MusyncError> {
    Ok(
      sqlx::query!(
        "SELECT track_id FROM playlists_tracks WHERE playlist_id = $1",
        playlist_id
      )
      .fetch_all(&self.pool)
      .await?
      .into_iter()
      .map(|row| row.track_id)
      .collect(),
    )
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use sqlx_db_tester::TestSqlite;
  use tracing::Level;

  use super::*;

  fn init_tracing() {
    tracing_subscriber::fmt()
      .with_max_level(Level::DEBUG)
      .init();
  }

  async fn init_manager() -> Musyncer {
    let tdb = TestSqlite::new(Path::new("../migrations")).await;
    let pool = tdb.get_pool();
    Musyncer::new(pool)
  }

  async fn create_test_user(manager: &Musyncer) -> abi::User {
    manager.create_user(abi::User::new("test")).await.unwrap()
  }

  #[tokio::test]
  async fn create_playlist_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::Playlist::new(user.id, "test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(playlist, &[]).await.unwrap();
    assert_ne!(playlist.id, 0);
    assert!(playlist.created_at.is_some());
    assert!(playlist.updated_at.is_some());
  }

  #[tokio::test]
  async fn update_playlist_info_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::Playlist::new(user.id, "test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(playlist, &[]).await.unwrap();

    let playlist = manager
      .update_playlist(abi::PlaylistUpdate {
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
  async fn delete_playlists_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::Playlist::new(user.id, "test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(playlist, &[]).await.unwrap();

    let deleted = manager.delete_playlists(&[playlist.id]).await.unwrap();
    assert_eq!(deleted.len(), 1);
    manager.playlist(deleted[0].id).await.unwrap_err();
    assert_eq!(deleted[0].id, playlist.id);
  }

  #[tokio::test]
  async fn query_playlists_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = create_test_user(&manager).await;

    let playlist = abi::Playlist::new(user.id, "test".to_string(), "test".to_string(), vec![]);
    let playlist = manager.create_playlist(playlist, &[]).await.unwrap();

    let playlists = manager
      .query_playlists(abi::PlaylistQuery {
        name: Some("test".to_string()),
        user_id: Some(user.id),
        track_id: None,
      })
      .await
      .unwrap();
    assert_eq!(playlists.len(), 1);
    assert_eq!(playlists[0].id, playlist.id);
  }

  #[tokio::test]
  async fn create_track_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track::new(
        "track_name".to_owned(),
        "discc".to_owned(),
        "cras".to_owned(),
        210,
        Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
      ))
      .await
      .unwrap();
    assert_ne!(track.id, 0);
  }

  #[tokio::test]
  async fn update_track_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track::new(
        "track_name".to_owned(),
        "discc".to_owned(),
        "cras".to_owned(),
        210,
        Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
      ))
      .await
      .unwrap();

    let track = manager
      .update_track(abi::TrackUpdate {
        id: track.id,
        name: "track_name2".to_owned(),
        artist: "discc2".to_owned(),
        album: "cras2".to_owned(),
        duration: 214,
        local_src: Some(abi::LocalSource {
          path: "/path/to/file2".to_owned(),
        }),
        netease_src: None,
      })
      .await
      .unwrap();

    assert_eq!(track.name, "track_name2");
    assert_eq!(track.artist, "discc2");
    assert_eq!(track.album, "cras2");
    assert_eq!(track.duration, 214);
    assert_eq!(track.local_src.unwrap().path, "/path/to/file2");
  }

  #[tokio::test]
  async fn delete_tracks_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track::new(
        "track_name".to_owned(),
        "discc".to_owned(),
        "cras".to_owned(),
        210,
        Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
      ))
      .await
      .unwrap();

    let deleted = manager.delete_tracks(&[track.id]).await.unwrap();
    assert_eq!(deleted.len(), 1);
    manager.track(deleted[0].id).await.unwrap_err();
    assert_eq!(deleted[0].id, track.id);
  }

  #[tokio::test]
  async fn query_tracks_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let track = manager
      .create_track(abi::Track::new(
        "track_name".to_owned(),
        "discc".to_owned(),
        "cras".to_owned(),
        210,
        Some(abi::LocalSource {
          path: "/path/to/file".to_owned(),
        }),
      ))
      .await
      .unwrap();

    let tracks = manager
      .query_tracks(abi::TrackQuery {
        name: Some("track_name".to_owned()),
        artist: Some("discc".to_owned()),
        album: Some("cras".to_owned()),
        playlist_id: None,
      })
      .await
      .unwrap();
    assert_eq!(tracks.len(), 1);
    assert_eq!(tracks[0].id, track.id);
  }

  #[tokio::test]
  async fn create_user_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = manager.create_user(abi::User::new("user1")).await.unwrap();
    assert_ne!(user.id, 0);
  }

  #[tokio::test]
  async fn update_user_should_work() {
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
  async fn delete_users_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = manager.create_user(abi::User::new("user1")).await.unwrap();

    let deleted = manager.delete_users(&[user.id]).await.unwrap();
    assert_eq!(deleted.len(), 1);
    manager.user(deleted[0].id).await.unwrap_err();
    assert_eq!(deleted[0].id, user.id);
  }

  #[tokio::test]
  async fn query_users_should_work() {
    init_tracing();
    let manager = init_manager().await;

    let user = manager.create_user(abi::User::new("user1")).await.unwrap();

    let users = manager
      .query_users(abi::UserQuery {
        name: Some("user1".to_owned()),
      })
      .await
      .unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, user.id);
  }
}
