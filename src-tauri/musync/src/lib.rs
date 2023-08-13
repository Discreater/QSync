pub mod error;
mod manager;

use async_trait::async_trait;
pub use error::MusyncError;
use sea_orm::DatabaseConnection;

pub type PlaylistId = i32;
pub type TrackId = i32;
pub type UserId = i32;

#[derive(Debug)]
pub struct Musyncer {
  db: DatabaseConnection,
}

#[async_trait]
pub trait Musync {
  // create a playlist
  async fn create_playlist(
    &self,
    playlist: abi::Playlist,
    tracks: &[TrackId],
  ) -> Result<abi::Playlist, MusyncError>;
  // update a playlist
  async fn update_playlist(
    &self,
    playlist: abi::PlaylistUpdate,
  ) -> Result<abi::Playlist, MusyncError>;
  // delete playlists
  async fn delete_playlists(&self, ids: &[PlaylistId]) -> Result<u64, MusyncError>;
  // query playlists
  async fn query_playlists(
    &self,
    query: abi::PlaylistQuery,
  ) -> Result<Vec<abi::Playlist>, MusyncError>;
  async fn playlist(&self, id: PlaylistId) -> Result<abi::Playlist, MusyncError>;

  // create a track
  async fn create_track(&self, track: abi::Track) -> Result<abi::Track, MusyncError>;
  // update a track
  async fn update_track(&self, track: abi::TrackUpdate) -> Result<abi::Track, MusyncError>;
  // delete tracks
  async fn delete_tracks(&self, ids: &[TrackId]) -> Result<u64, MusyncError>;
  // query tracks
  async fn query_tracks(&self, query: abi::TrackQuery) -> Result<Vec<abi::Track>, MusyncError>;

  async fn track(&self, id: TrackId) -> Result<abi::Track, MusyncError>;

  // create a user
  async fn create_user(&self, user: abi::User) -> Result<abi::User, MusyncError>;
  // update a user
  async fn update_user(&self, user: abi::UserUpdate) -> Result<abi::User, MusyncError>;
  // delete users
  async fn delete_users(&self, ids: &[UserId]) -> Result<u64, MusyncError>;
  // query users
  async fn query_users(&self, query: abi::UserQuery) -> Result<Vec<abi::User>, MusyncError>;
  async fn user(&self, id: UserId) -> Result<abi::User, MusyncError>;
}
