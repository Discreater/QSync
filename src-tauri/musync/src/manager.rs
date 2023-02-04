use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite};

use crate::{Musync, MusyncError, Musyncer, PlaylistId, TrackId, UserId};

#[async_trait]
impl Musync for Musyncer {
  async fn create_playlist(
    &self,
    mut playlist: abi::Playlist,
    tracks: &[TrackId],
  ) -> Result<abi::Playlist, MusyncError> {
    // execute the sql
    let id = sqlx::query!(
      "INSERT INTO playlists (owner_id, name, description) VALUES ($1, $2, $3) RETURNING id",
      playlist.owner_id,
      playlist.name,
      playlist.description
    )
    .fetch_one(&self.pool)
    .await?
    .id;

    playlist.id = id;

    // insert the tracks

    let mut query_builder =
      QueryBuilder::<Sqlite>::new("INSERT INTO playlist_tracks (playlist_id, track_id) ");

    const BIND_LIMIT: usize = 256;

    let track_list = tracks.iter().map(|track_id| (id, track_id));

    query_builder.push_values(
      track_list.take(BIND_LIMIT / 4),
      |mut b, (playlist_id, track_id)| {
        b.push_bind(playlist_id);
        b.push_bind(track_id);
      },
    );

    query_builder.build().execute(&self.pool).await?;

    Ok(playlist)
  }

  async fn update_playlist(
    &self,
    _playlist: abi::PlaylistUpdate,
  ) -> Result<abi::Playlist, MusyncError> {
    todo!()
  }

  async fn delete_playlists(&self, _ids: &[PlaylistId]) -> Result<Vec<abi::Playlist>, MusyncError> {
    todo!()
  }

  async fn query_playlists(
    &self,
    _query: abi::PlaylistQuery,
  ) -> Result<Vec<abi::Playlist>, MusyncError> {
    todo!()
  }

  async fn create_track(&self, _track: abi::Track) -> Result<abi::Track, MusyncError> {
    todo!()
  }

  async fn update_track(&self, _track: abi::TrackUpdate) -> Result<abi::Track, MusyncError> {
    todo!()
  }

  async fn delete_tracks(&self, _ids: &[TrackId]) -> Result<Vec<abi::Track>, MusyncError> {
    todo!()
  }

  async fn query_tracks(&self, _query: abi::TrackQuery) -> Result<Vec<abi::Track>, MusyncError> {
    todo!()
  }

  async fn create_user(&self, _user: abi::User) -> Result<abi::User, MusyncError> {
    todo!()
  }

  async fn update_user(&self, _user: abi::UserUpdate) -> Result<abi::User, MusyncError> {
    todo!()
  }

  async fn delete_users(&self, _ids: &[UserId]) -> Result<Vec<abi::User>, MusyncError> {
    todo!()
  }

  async fn query_users(&self, _query: abi::UserQuery) -> Result<Vec<abi::User>, MusyncError> {
    todo!()
  }
}
