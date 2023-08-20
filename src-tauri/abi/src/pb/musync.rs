/// Playlist
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Playlist {
  /// unique id for the playlist
  #[prost(int32, tag = "1")]
  pub id: i32,
  /// id of the owner of the playlist
  #[prost(int32, tag = "2")]
  pub owner_id: i32,
  /// ids of tracks in the playlist
  #[prost(int32, repeated, tag = "3")]
  pub track_ids: ::prost::alloc::vec::Vec<i32>,
  /// name of the playlist
  #[prost(string, tag = "4")]
  pub name: ::prost::alloc::string::String,
  /// description of the playlist
  #[prost(string, tag = "5")]
  pub description: ::prost::alloc::string::String,
  /// time when the playlist is created
  #[prost(message, optional, tag = "6")]
  pub created_at: ::core::option::Option<super::google::protobuf::Timestamp>,
  /// time of the last update
  #[prost(message, optional, tag = "7")]
  pub updated_at: ::core::option::Option<super::google::protobuf::Timestamp>,
}
/// CurrentPlaylist controller
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayQueue {
  /// unique id for the current playlist
  #[prost(int32, tag = "1")]
  pub id: i32,
  /// id of the playlist
  #[prost(int32, tag = "2")]
  pub playlist_id: i32,
  /// index of the current playing track in the playlist
  #[prost(uint32, tag = "3")]
  pub position: u32,
  /// is the playlist playing
  #[prost(bool, tag = "4")]
  pub playing: bool,
  /// time when the current track started playing.
  /// When not playing, it represents the progress directly.
  /// In milliseconds
  #[prost(message, optional, tag = "5")]
  pub started_at: ::core::option::Option<super::google::protobuf::Timestamp>,
  #[prost(uint32, tag = "6")]
  pub paused_at: u32,
}
/// Track
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Track {
  /// unique id for the track
  #[prost(int32, tag = "1")]
  pub id: i32,
  /// title of the track
  #[prost(string, tag = "2")]
  pub title: ::prost::alloc::string::String,
  /// artist of the track
  #[prost(string, optional, tag = "3")]
  pub artist: ::core::option::Option<::prost::alloc::string::String>,
  /// album of the track
  #[prost(string, optional, tag = "4")]
  pub album: ::core::option::Option<::prost::alloc::string::String>,
  /// duration of the track in milliseconds
  #[prost(uint32, optional, tag = "5")]
  pub duration: ::core::option::Option<u32>,
  /// genre of the track
  #[prost(string, optional, tag = "6")]
  pub genre: ::core::option::Option<::prost::alloc::string::String>,
  /// year of the track
  #[prost(uint32, optional, tag = "7")]
  pub year: ::core::option::Option<u32>,
  /// time when the track is created
  #[prost(message, optional, tag = "8")]
  pub created_at: ::core::option::Option<super::google::protobuf::Timestamp>,
  /// time of the last update
  #[prost(message, optional, tag = "9")]
  pub updated_at: ::core::option::Option<super::google::protobuf::Timestamp>,
  /// local source of the track
  #[prost(message, optional, tag = "10")]
  pub local_src: ::core::option::Option<LocalSource>,
  /// netease source of the track
  #[prost(message, optional, tag = "11")]
  pub netease_src: ::core::option::Option<NeteaseSource>,
}
/// NeteaseSource, not implemented yet
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeteaseSource {
  /// id of the track in netease
  #[prost(string, tag = "1")]
  pub id: ::prost::alloc::string::String,
}
/// LocalSource
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalSource {
  /// path of the track
  #[prost(string, tag = "1")]
  pub path: ::prost::alloc::string::String,
}
/// User
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
  /// unique id for the user
  #[prost(int32, tag = "1")]
  pub id: i32,
  /// name of the user
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  /// time when the user is created
  #[prost(message, optional, tag = "3")]
  pub created_at: ::core::option::Option<super::google::protobuf::Timestamp>,
  /// time of the last update
  #[prost(message, optional, tag = "4")]
  pub updated_at: ::core::option::Option<super::google::protobuf::Timestamp>,
}
/// Token
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
  #[prost(string, tag = "1")]
  pub data: ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub r#type: ::prost::alloc::string::String,
}
/// Cover
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Picture {
  /// picture type according to ID3v2 APIC
  #[prost(string, optional, tag = "1")]
  pub pic_type: ::core::option::Option<::prost::alloc::string::String>,
  #[prost(string, tag = "2")]
  pub mime_type: ::prost::alloc::string::String,
  #[prost(string, optional, tag = "3")]
  pub description: ::core::option::Option<::prost::alloc::string::String>,
  /// base64 picture data
  #[prost(string, tag = "4")]
  pub data: ::prost::alloc::string::String,
}
/// Create playlist request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlaylistRequest {
  /// ids of tracks in the playlist
  #[prost(int32, repeated, tag = "1")]
  pub track_ids: ::prost::alloc::vec::Vec<i32>,
  /// name of the playlist
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  /// description of the playlist
  #[prost(string, tag = "3")]
  pub description: ::prost::alloc::string::String,
  #[prost(bool, tag = "4")]
  pub temp: bool,
}
/// Create playlist response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlaylistResponse {
  /// Created playlist
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
/// Delete playlist request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePlaylistsRequest {
  /// Ids of playlists to be deleted
  #[prost(int32, repeated, tag = "1")]
  pub ids: ::prost::alloc::vec::Vec<i32>,
}
/// Query playlist by user id and track id
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlaylistsRequest {
  /// which user has the playlist
  #[prost(int32, optional, tag = "1")]
  pub user_id: ::core::option::Option<i32>,
  /// Query by contained track id
  #[prost(int32, optional, tag = "2")]
  pub track_id: ::core::option::Option<i32>,
  /// Query by name
  #[prost(string, optional, tag = "3")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Update playlist: add/remove tracks, update name and description
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlaylistRequest {
  /// Id of the playlist to be updated
  #[prost(int32, tag = "1")]
  pub id: i32,
  /// Ids of tracks to be added
  #[prost(int32, repeated, tag = "2")]
  pub added_track_ids: ::prost::alloc::vec::Vec<i32>,
  /// Ids of tracks to be removed
  #[prost(int32, repeated, tag = "3")]
  pub removed_track_ids: ::prost::alloc::vec::Vec<i32>,
  /// New name of the playlist
  #[prost(string, optional, tag = "4")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
  /// New description of the playlist
  #[prost(string, optional, tag = "5")]
  pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Updated playlist will be returned in response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlaylistResponse {
  /// Updated playlist
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
/// Create track request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrackRequest {
  /// Track to be created
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
/// Create track response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrackResponse {
  /// Created track
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
/// Query track
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTracksRequest {
  /// Query by which playlist contains the track
  #[prost(int32, optional, tag = "1")]
  pub playlist_id: ::core::option::Option<i32>,
  /// Query by title
  #[prost(string, optional, tag = "2")]
  pub title: ::core::option::Option<::prost::alloc::string::String>,
  /// Query by artist
  #[prost(string, optional, tag = "3")]
  pub artist: ::core::option::Option<::prost::alloc::string::String>,
  /// Query by album
  #[prost(string, optional, tag = "4")]
  pub album: ::core::option::Option<::prost::alloc::string::String>,
  /// Query by genre
  #[prost(string, optional, tag = "5")]
  pub genre: ::core::option::Option<::prost::alloc::string::String>,
  /// Query by year
  #[prost(uint32, optional, tag = "6")]
  pub year: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackUpdate {
  /// Id of the track to be updated
  #[prost(int32, tag = "1")]
  pub id: i32,
  /// New title of the track
  #[prost(string, optional, tag = "2")]
  pub title: ::core::option::Option<::prost::alloc::string::String>,
  /// New artist of the track
  #[prost(string, optional, tag = "3")]
  pub artist: ::core::option::Option<::prost::alloc::string::String>,
  /// New album of the track
  #[prost(string, optional, tag = "4")]
  pub album: ::core::option::Option<::prost::alloc::string::String>,
  #[prost(uint32, optional, tag = "5")]
  pub duration: ::core::option::Option<u32>,
  /// New genre of the track
  #[prost(string, optional, tag = "6")]
  pub genre: ::core::option::Option<::prost::alloc::string::String>,
  /// New year of the track
  #[prost(uint32, optional, tag = "7")]
  pub year: ::core::option::Option<u32>,
  /// New local source of the track
  #[prost(message, optional, tag = "8")]
  pub local_src: ::core::option::Option<LocalSource>,
  /// New netease source of the track
  #[prost(message, optional, tag = "9")]
  pub netease_src: ::core::option::Option<NeteaseSource>,
}
/// Update track request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTrackRequest {
  #[prost(message, optional, tag = "1")]
  pub update: ::core::option::Option<TrackUpdate>,
}
/// Update track response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTrackResponse {
  /// Updated track
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
/// Delete tracks request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTracksRequest {
  /// Ids of tracks to be deleted
  #[prost(int32, repeated, tag = "1")]
  pub track_ids: ::prost::alloc::vec::Vec<i32>,
}
/// Create user request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
  /// User to be created
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
/// Create user response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResponse {
  /// Created user
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
/// Query user
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUsersRequest {
  /// Query by name
  #[prost(string, optional, tag = "1")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserUpdate {
  /// Id of the user to be updated
  #[prost(int32, tag = "1")]
  pub id: i32,
  /// New name of the user
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
}
/// Update user request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
  #[prost(message, optional, tag = "1")]
  pub update: ::core::option::Option<UserUpdate>,
}
/// Update user response
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResponse {
  /// Updated user
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
/// Delete users request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUsersRequest {
  /// Ids of users to be deleted
  #[prost(int32, repeated, tag = "1")]
  pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
  #[prost(string, tag = "1")]
  pub name: ::prost::alloc::string::String,
  #[prost(string, tag = "2")]
  pub r#type: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLocalFolderRequest {
  #[prost(string, tag = "1")]
  pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLocalFolderRequest {
  #[prost(string, tag = "1")]
  pub path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLocalFolderResponse {
  #[prost(uint32, tag = "1")]
  pub added_tracks: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLocalFolderResponse {
  #[prost(uint32, tag = "1")]
  pub removed_tracks: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrackRequest {
  #[prost(int32, tag = "1")]
  pub id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrackCoverRequest {
  #[prost(int32, tag = "1")]
  pub track_id: i32,
  #[prost(uint32, optional, tag = "2")]
  pub cover_idx: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLocalFoldersRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalFolder {
  #[prost(string, tag = "1")]
  pub path: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod musync_service_client {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::http::Uri;
  use tonic::codegen::*;
  /// Musync service
  #[derive(Debug, Clone)]
  pub struct MusyncServiceClient<T> {
    inner: tonic::client::Grpc<T>,
  }
  impl MusyncServiceClient<tonic::transport::Channel> {
    /// Attempt to create a new client by connecting to a given endpoint.
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
      D: TryInto<tonic::transport::Endpoint>,
      D::Error: Into<StdError>,
    {
      let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
      Ok(Self::new(conn))
    }
  }
  impl<T> MusyncServiceClient<T>
  where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
  {
    pub fn new(inner: T) -> Self {
      let inner = tonic::client::Grpc::new(inner);
      Self { inner }
    }
    pub fn with_origin(inner: T, origin: Uri) -> Self {
      let inner = tonic::client::Grpc::with_origin(inner, origin);
      Self { inner }
    }
    pub fn with_interceptor<F>(
      inner: T,
      interceptor: F,
    ) -> MusyncServiceClient<InterceptedService<T, F>>
    where
      F: tonic::service::Interceptor,
      T::ResponseBody: Default,
      T: tonic::codegen::Service<
        http::Request<tonic::body::BoxBody>,
        Response = http::Response<
          <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
        >,
      >,
      <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
        Into<StdError> + Send + Sync,
    {
      MusyncServiceClient::new(InterceptedService::new(inner, interceptor))
    }
    /// Compress requests with the given encoding.
    ///
    /// This requires the server to support it otherwise it might respond with an
    /// error.
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
      self.inner = self.inner.send_compressed(encoding);
      self
    }
    /// Enable decompressing responses.
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
      self.inner = self.inner.accept_compressed(encoding);
      self
    }
    /// Limits the maximum size of a decoded message.
    ///
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
      self.inner = self.inner.max_decoding_message_size(limit);
      self
    }
    /// Limits the maximum size of an encoded message.
    ///
    /// Default: `usize::MAX`
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
      self.inner = self.inner.max_encoding_message_size(limit);
      self
    }
    pub async fn login(
      &mut self,
      request: impl tonic::IntoRequest<super::LoginRequest>,
    ) -> std::result::Result<tonic::Response<super::Token>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/Login");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "Login"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn add_local_folder(
      &mut self,
      request: impl tonic::IntoRequest<super::AddLocalFolderRequest>,
    ) -> std::result::Result<tonic::Response<super::AddLocalFolderResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/AddLocalFolder");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "AddLocalFolder"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn remove_local_folder(
      &mut self,
      request: impl tonic::IntoRequest<super::RemoveLocalFolderRequest>,
    ) -> std::result::Result<tonic::Response<super::RemoveLocalFolderResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/RemoveLocalFolder");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "RemoveLocalFolder"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn query_local_folders(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryLocalFoldersRequest>,
    ) -> std::result::Result<
      tonic::Response<tonic::codec::Streaming<super::LocalFolder>>,
      tonic::Status,
    > {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/QueryLocalFolders");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "QueryLocalFolders"));
      self.inner.server_streaming(req, path, codec).await
    }
    pub async fn create_playlist(
      &mut self,
      request: impl tonic::IntoRequest<super::CreatePlaylistRequest>,
    ) -> std::result::Result<tonic::Response<super::CreatePlaylistResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/CreatePlaylist");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "CreatePlaylist"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn query_playlists(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryPlaylistsRequest>,
    ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::Playlist>>, tonic::Status>
    {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/QueryPlaylists");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "QueryPlaylists"));
      self.inner.server_streaming(req, path, codec).await
    }
    pub async fn update_playlist(
      &mut self,
      request: impl tonic::IntoRequest<super::UpdatePlaylistRequest>,
    ) -> std::result::Result<tonic::Response<super::UpdatePlaylistResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/UpdatePlaylist");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "UpdatePlaylist"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn delete_playlists(
      &mut self,
      request: impl tonic::IntoRequest<super::DeletePlaylistsRequest>,
    ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::Playlist>>, tonic::Status>
    {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/DeletePlaylists");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "DeletePlaylists"));
      self.inner.server_streaming(req, path, codec).await
    }
    pub async fn get_track(
      &mut self,
      request: impl tonic::IntoRequest<super::GetTrackRequest>,
    ) -> std::result::Result<tonic::Response<super::Track>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/GetTrack");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "GetTrack"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn get_track_cover(
      &mut self,
      request: impl tonic::IntoRequest<super::GetTrackCoverRequest>,
    ) -> std::result::Result<tonic::Response<super::Picture>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/GetTrackCover");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "GetTrackCover"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn create_track(
      &mut self,
      request: impl tonic::IntoRequest<super::CreateTrackRequest>,
    ) -> std::result::Result<tonic::Response<super::CreateTrackResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/CreateTrack");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "CreateTrack"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn query_tracks(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryTracksRequest>,
    ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::Track>>, tonic::Status>
    {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/QueryTracks");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "QueryTracks"));
      self.inner.server_streaming(req, path, codec).await
    }
    pub async fn update_track(
      &mut self,
      request: impl tonic::IntoRequest<super::UpdateTrackRequest>,
    ) -> std::result::Result<tonic::Response<super::UpdateTrackResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/UpdateTrack");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "UpdateTrack"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn delete_tracks(
      &mut self,
      request: impl tonic::IntoRequest<super::DeleteTracksRequest>,
    ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::Track>>, tonic::Status>
    {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/DeleteTracks");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "DeleteTracks"));
      self.inner.server_streaming(req, path, codec).await
    }
    pub async fn create_user(
      &mut self,
      request: impl tonic::IntoRequest<super::CreateUserRequest>,
    ) -> std::result::Result<tonic::Response<super::CreateUserResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/CreateUser");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "CreateUser"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn query_users(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryUsersRequest>,
    ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::User>>, tonic::Status>
    {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/QueryUsers");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "QueryUsers"));
      self.inner.server_streaming(req, path, codec).await
    }
    pub async fn update_user(
      &mut self,
      request: impl tonic::IntoRequest<super::UpdateUserRequest>,
    ) -> std::result::Result<tonic::Response<super::UpdateUserResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/UpdateUser");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "UpdateUser"));
      self.inner.unary(req, path, codec).await
    }
    pub async fn delete_users(
      &mut self,
      request: impl tonic::IntoRequest<super::DeleteUsersRequest>,
    ) -> std::result::Result<tonic::Response<tonic::codec::Streaming<super::User>>, tonic::Status>
    {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/DeleteUsers");
      let mut req = request.into_request();
      req
        .extensions_mut()
        .insert(GrpcMethod::new("musync.MusyncService", "DeleteUsers"));
      self.inner.server_streaming(req, path, codec).await
    }
  }
}
/// Generated server implementations.
pub mod musync_service_server {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  /// Generated trait containing gRPC methods that should be implemented for use with MusyncServiceServer.
  #[async_trait]
  pub trait MusyncService: Send + Sync + 'static {
    async fn login(
      &self,
      request: tonic::Request<super::LoginRequest>,
    ) -> std::result::Result<tonic::Response<super::Token>, tonic::Status>;
    async fn add_local_folder(
      &self,
      request: tonic::Request<super::AddLocalFolderRequest>,
    ) -> std::result::Result<tonic::Response<super::AddLocalFolderResponse>, tonic::Status>;
    async fn remove_local_folder(
      &self,
      request: tonic::Request<super::RemoveLocalFolderRequest>,
    ) -> std::result::Result<tonic::Response<super::RemoveLocalFolderResponse>, tonic::Status>;
    /// Server streaming response type for the QueryLocalFolders method.
    type QueryLocalFoldersStream: futures_core::Stream<Item = std::result::Result<super::LocalFolder, tonic::Status>>
      + Send
      + 'static;
    async fn query_local_folders(
      &self,
      request: tonic::Request<super::QueryLocalFoldersRequest>,
    ) -> std::result::Result<tonic::Response<Self::QueryLocalFoldersStream>, tonic::Status>;
    async fn create_playlist(
      &self,
      request: tonic::Request<super::CreatePlaylistRequest>,
    ) -> std::result::Result<tonic::Response<super::CreatePlaylistResponse>, tonic::Status>;
    /// Server streaming response type for the QueryPlaylists method.
    type QueryPlaylistsStream: futures_core::Stream<Item = std::result::Result<super::Playlist, tonic::Status>>
      + Send
      + 'static;
    async fn query_playlists(
      &self,
      request: tonic::Request<super::QueryPlaylistsRequest>,
    ) -> std::result::Result<tonic::Response<Self::QueryPlaylistsStream>, tonic::Status>;
    async fn update_playlist(
      &self,
      request: tonic::Request<super::UpdatePlaylistRequest>,
    ) -> std::result::Result<tonic::Response<super::UpdatePlaylistResponse>, tonic::Status>;
    /// Server streaming response type for the DeletePlaylists method.
    type DeletePlaylistsStream: futures_core::Stream<Item = std::result::Result<super::Playlist, tonic::Status>>
      + Send
      + 'static;
    async fn delete_playlists(
      &self,
      request: tonic::Request<super::DeletePlaylistsRequest>,
    ) -> std::result::Result<tonic::Response<Self::DeletePlaylistsStream>, tonic::Status>;
    async fn get_track(
      &self,
      request: tonic::Request<super::GetTrackRequest>,
    ) -> std::result::Result<tonic::Response<super::Track>, tonic::Status>;
    async fn get_track_cover(
      &self,
      request: tonic::Request<super::GetTrackCoverRequest>,
    ) -> std::result::Result<tonic::Response<super::Picture>, tonic::Status>;
    async fn create_track(
      &self,
      request: tonic::Request<super::CreateTrackRequest>,
    ) -> std::result::Result<tonic::Response<super::CreateTrackResponse>, tonic::Status>;
    /// Server streaming response type for the QueryTracks method.
    type QueryTracksStream: futures_core::Stream<Item = std::result::Result<super::Track, tonic::Status>>
      + Send
      + 'static;
    async fn query_tracks(
      &self,
      request: tonic::Request<super::QueryTracksRequest>,
    ) -> std::result::Result<tonic::Response<Self::QueryTracksStream>, tonic::Status>;
    async fn update_track(
      &self,
      request: tonic::Request<super::UpdateTrackRequest>,
    ) -> std::result::Result<tonic::Response<super::UpdateTrackResponse>, tonic::Status>;
    /// Server streaming response type for the DeleteTracks method.
    type DeleteTracksStream: futures_core::Stream<Item = std::result::Result<super::Track, tonic::Status>>
      + Send
      + 'static;
    async fn delete_tracks(
      &self,
      request: tonic::Request<super::DeleteTracksRequest>,
    ) -> std::result::Result<tonic::Response<Self::DeleteTracksStream>, tonic::Status>;
    async fn create_user(
      &self,
      request: tonic::Request<super::CreateUserRequest>,
    ) -> std::result::Result<tonic::Response<super::CreateUserResponse>, tonic::Status>;
    /// Server streaming response type for the QueryUsers method.
    type QueryUsersStream: futures_core::Stream<Item = std::result::Result<super::User, tonic::Status>>
      + Send
      + 'static;
    async fn query_users(
      &self,
      request: tonic::Request<super::QueryUsersRequest>,
    ) -> std::result::Result<tonic::Response<Self::QueryUsersStream>, tonic::Status>;
    async fn update_user(
      &self,
      request: tonic::Request<super::UpdateUserRequest>,
    ) -> std::result::Result<tonic::Response<super::UpdateUserResponse>, tonic::Status>;
    /// Server streaming response type for the DeleteUsers method.
    type DeleteUsersStream: futures_core::Stream<Item = std::result::Result<super::User, tonic::Status>>
      + Send
      + 'static;
    async fn delete_users(
      &self,
      request: tonic::Request<super::DeleteUsersRequest>,
    ) -> std::result::Result<tonic::Response<Self::DeleteUsersStream>, tonic::Status>;
  }
  /// Musync service
  #[derive(Debug)]
  pub struct MusyncServiceServer<T: MusyncService> {
    inner: _Inner<T>,
    accept_compression_encodings: EnabledCompressionEncodings,
    send_compression_encodings: EnabledCompressionEncodings,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
  }
  struct _Inner<T>(Arc<T>);
  impl<T: MusyncService> MusyncServiceServer<T> {
    pub fn new(inner: T) -> Self {
      Self::from_arc(Arc::new(inner))
    }
    pub fn from_arc(inner: Arc<T>) -> Self {
      let inner = _Inner(inner);
      Self {
        inner,
        accept_compression_encodings: Default::default(),
        send_compression_encodings: Default::default(),
        max_decoding_message_size: None,
        max_encoding_message_size: None,
      }
    }
    pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
    where
      F: tonic::service::Interceptor,
    {
      InterceptedService::new(Self::new(inner), interceptor)
    }
    /// Enable decompressing requests with the given encoding.
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
      self.accept_compression_encodings.enable(encoding);
      self
    }
    /// Compress responses with the given encoding, if the client supports it.
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
      self.send_compression_encodings.enable(encoding);
      self
    }
    /// Limits the maximum size of a decoded message.
    ///
    /// Default: `4MB`
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
      self.max_decoding_message_size = Some(limit);
      self
    }
    /// Limits the maximum size of an encoded message.
    ///
    /// Default: `usize::MAX`
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
      self.max_encoding_message_size = Some(limit);
      self
    }
  }
  impl<T, B> tonic::codegen::Service<http::Request<B>> for MusyncServiceServer<T>
  where
    T: MusyncService,
    B: Body + Send + 'static,
    B::Error: Into<StdError> + Send + 'static,
  {
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
      Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
      let inner = self.inner.clone();
      match req.uri().path() {
        "/musync.MusyncService/Login" => {
          #[allow(non_camel_case_types)]
          struct LoginSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::LoginRequest> for LoginSvc<T> {
            type Response = super::Token;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::LoginRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).login(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = LoginSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/AddLocalFolder" => {
          #[allow(non_camel_case_types)]
          struct AddLocalFolderSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::AddLocalFolderRequest>
            for AddLocalFolderSvc<T>
          {
            type Response = super::AddLocalFolderResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::AddLocalFolderRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).add_local_folder(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = AddLocalFolderSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/RemoveLocalFolder" => {
          #[allow(non_camel_case_types)]
          struct RemoveLocalFolderSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::RemoveLocalFolderRequest>
            for RemoveLocalFolderSvc<T>
          {
            type Response = super::RemoveLocalFolderResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::RemoveLocalFolderRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).remove_local_folder(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = RemoveLocalFolderSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/QueryLocalFolders" => {
          #[allow(non_camel_case_types)]
          struct QueryLocalFoldersSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService>
            tonic::server::ServerStreamingService<super::QueryLocalFoldersRequest>
            for QueryLocalFoldersSvc<T>
          {
            type Response = super::LocalFolder;
            type ResponseStream = T::QueryLocalFoldersStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::QueryLocalFoldersRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).query_local_folders(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = QueryLocalFoldersSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/CreatePlaylist" => {
          #[allow(non_camel_case_types)]
          struct CreatePlaylistSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::CreatePlaylistRequest>
            for CreatePlaylistSvc<T>
          {
            type Response = super::CreatePlaylistResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::CreatePlaylistRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).create_playlist(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = CreatePlaylistSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/QueryPlaylists" => {
          #[allow(non_camel_case_types)]
          struct QueryPlaylistsSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::QueryPlaylistsRequest>
            for QueryPlaylistsSvc<T>
          {
            type Response = super::Playlist;
            type ResponseStream = T::QueryPlaylistsStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::QueryPlaylistsRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).query_playlists(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = QueryPlaylistsSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/UpdatePlaylist" => {
          #[allow(non_camel_case_types)]
          struct UpdatePlaylistSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::UpdatePlaylistRequest>
            for UpdatePlaylistSvc<T>
          {
            type Response = super::UpdatePlaylistResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::UpdatePlaylistRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).update_playlist(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = UpdatePlaylistSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/DeletePlaylists" => {
          #[allow(non_camel_case_types)]
          struct DeletePlaylistsSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService>
            tonic::server::ServerStreamingService<super::DeletePlaylistsRequest>
            for DeletePlaylistsSvc<T>
          {
            type Response = super::Playlist;
            type ResponseStream = T::DeletePlaylistsStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::DeletePlaylistsRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).delete_playlists(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = DeletePlaylistsSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/GetTrack" => {
          #[allow(non_camel_case_types)]
          struct GetTrackSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::GetTrackRequest> for GetTrackSvc<T> {
            type Response = super::Track;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::GetTrackRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).get_track(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = GetTrackSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/GetTrackCover" => {
          #[allow(non_camel_case_types)]
          struct GetTrackCoverSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::GetTrackCoverRequest>
            for GetTrackCoverSvc<T>
          {
            type Response = super::Picture;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::GetTrackCoverRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).get_track_cover(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = GetTrackCoverSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/CreateTrack" => {
          #[allow(non_camel_case_types)]
          struct CreateTrackSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::CreateTrackRequest>
            for CreateTrackSvc<T>
          {
            type Response = super::CreateTrackResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::CreateTrackRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).create_track(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = CreateTrackSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/QueryTracks" => {
          #[allow(non_camel_case_types)]
          struct QueryTracksSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::QueryTracksRequest>
            for QueryTracksSvc<T>
          {
            type Response = super::Track;
            type ResponseStream = T::QueryTracksStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::QueryTracksRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).query_tracks(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = QueryTracksSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/UpdateTrack" => {
          #[allow(non_camel_case_types)]
          struct UpdateTrackSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::UpdateTrackRequest>
            for UpdateTrackSvc<T>
          {
            type Response = super::UpdateTrackResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::UpdateTrackRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).update_track(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = UpdateTrackSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/DeleteTracks" => {
          #[allow(non_camel_case_types)]
          struct DeleteTracksSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::DeleteTracksRequest>
            for DeleteTracksSvc<T>
          {
            type Response = super::Track;
            type ResponseStream = T::DeleteTracksStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::DeleteTracksRequest>,
            ) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).delete_tracks(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = DeleteTracksSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/CreateUser" => {
          #[allow(non_camel_case_types)]
          struct CreateUserSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::CreateUserRequest> for CreateUserSvc<T> {
            type Response = super::CreateUserResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::CreateUserRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).create_user(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = CreateUserSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/QueryUsers" => {
          #[allow(non_camel_case_types)]
          struct QueryUsersSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::QueryUsersRequest>
            for QueryUsersSvc<T>
          {
            type Response = super::User;
            type ResponseStream = T::QueryUsersStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::QueryUsersRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).query_users(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = QueryUsersSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/UpdateUser" => {
          #[allow(non_camel_case_types)]
          struct UpdateUserSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::UpdateUserRequest> for UpdateUserSvc<T> {
            type Response = super::UpdateUserResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::UpdateUserRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).update_user(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = UpdateUserSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/DeleteUsers" => {
          #[allow(non_camel_case_types)]
          struct DeleteUsersSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::DeleteUsersRequest>
            for DeleteUsersSvc<T>
          {
            type Response = super::User;
            type ResponseStream = T::DeleteUsersStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::DeleteUsersRequest>) -> Self::Future {
              let inner = Arc::clone(&self.0);
              let fut = async move { (*inner).delete_users(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let max_decoding_message_size = self.max_decoding_message_size;
          let max_encoding_message_size = self.max_encoding_message_size;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = DeleteUsersSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings)
              .apply_max_message_size_config(max_decoding_message_size, max_encoding_message_size);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        _ => Box::pin(async move {
          Ok(
            http::Response::builder()
              .status(200)
              .header("grpc-status", "12")
              .header("content-type", "application/grpc")
              .body(empty_body())
              .unwrap(),
          )
        }),
      }
    }
  }
  impl<T: MusyncService> Clone for MusyncServiceServer<T> {
    fn clone(&self) -> Self {
      let inner = self.inner.clone();
      Self {
        inner,
        accept_compression_encodings: self.accept_compression_encodings,
        send_compression_encodings: self.send_compression_encodings,
        max_decoding_message_size: self.max_decoding_message_size,
        max_encoding_message_size: self.max_encoding_message_size,
      }
    }
  }
  impl<T: MusyncService> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(Arc::clone(&self.0))
    }
  }
  impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.0)
    }
  }
  impl<T: MusyncService> tonic::server::NamedService for MusyncServiceServer<T> {
    const NAME: &'static str = "musync.MusyncService";
  }
}
