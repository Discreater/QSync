#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Playlist {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(int64, tag = "2")]
  pub owner_id: i64,
  #[prost(int64, repeated, tag = "3")]
  pub track_ids: ::prost::alloc::vec::Vec<i64>,
  #[prost(string, tag = "4")]
  pub name: ::prost::alloc::string::String,
  #[prost(string, tag = "5")]
  pub description: ::prost::alloc::string::String,
  #[prost(message, optional, tag = "6")]
  pub created_at: ::core::option::Option<::prost_types::Timestamp>,
  #[prost(message, optional, tag = "7")]
  pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentPlaylist {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(int64, tag = "2")]
  pub playlist_id: i64,
  #[prost(int64, tag = "3")]
  pub user_id: i64,
  #[prost(int64, tag = "4")]
  pub current_track_id: i64,
  #[prost(bool, tag = "5")]
  pub playing: bool,
  #[prost(message, optional, tag = "6")]
  pub started_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Track {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  #[prost(string, tag = "3")]
  pub artist: ::prost::alloc::string::String,
  #[prost(string, tag = "4")]
  pub album: ::prost::alloc::string::String,
  #[prost(message, optional, tag = "5")]
  pub local_src: ::core::option::Option<LocalSource>,
  #[prost(message, optional, tag = "6")]
  pub netease_src: ::core::option::Option<NeteaseSource>,
}
/// TODO: add fields
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeteaseSource {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalSource {
  #[prost(string, tag = "1")]
  pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  #[prost(string, tag = "3")]
  pub current_playlist_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlaylistRequest {
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlaylistResponse {
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePlaylistsRequest {
  #[prost(int64, repeated, tag = "1")]
  pub ids: ::prost::alloc::vec::Vec<i64>,
}
/// Query playlist by user id and track id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaylistQuery {
  /// If empty, query all playlists
  #[prost(int64, tag = "1")]
  pub user_id: i64,
  /// If empty, query all playlists
  #[prost(int64, tag = "2")]
  pub track_id: i64,
  #[prost(string, tag = "3")]
  pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlaylistRequest {
  #[prost(message, optional, tag = "1")]
  pub query: ::core::option::Option<PlaylistQuery>,
}
/// Update playlist: add/remove tracks, update name and description
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlaylistRequest {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(int64, repeated, tag = "2")]
  pub added_track_ids: ::prost::alloc::vec::Vec<i64>,
  #[prost(int64, repeated, tag = "3")]
  pub removed_track_ids: ::prost::alloc::vec::Vec<i64>,
  #[prost(string, optional, tag = "4")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
  #[prost(string, optional, tag = "5")]
  pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Updated playlist will be returned in response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlaylistResponse {
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrackRequest {
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrackResponse {
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackQuery {
  #[prost(int64, tag = "1")]
  pub playlist_id: i64,
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  #[prost(string, tag = "3")]
  pub artist: ::prost::alloc::string::String,
  #[prost(string, tag = "4")]
  pub ablum: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTracksRequest {
  #[prost(message, optional, tag = "1")]
  pub query: ::core::option::Option<TrackQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTrackRequest {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  #[prost(string, tag = "3")]
  pub artist: ::prost::alloc::string::String,
  #[prost(string, tag = "4")]
  pub album: ::prost::alloc::string::String,
  #[prost(message, optional, tag = "5")]
  pub local_src: ::core::option::Option<LocalSource>,
  #[prost(message, optional, tag = "6")]
  pub netease_src: ::core::option::Option<NeteaseSource>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTrackResponse {
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTracksRequest {
  #[prost(int64, repeated, tag = "1")]
  pub track_ids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResponse {
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUserRequest {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
  #[prost(int64, tag = "1")]
  pub id: i64,
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  #[prost(int64, tag = "3")]
  pub current_playlist_id: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResponse {
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUsersRequest {
  #[prost(int64, repeated, tag = "1")]
  pub ids: ::prost::alloc::vec::Vec<i64>,
}
