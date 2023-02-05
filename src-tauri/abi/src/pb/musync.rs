/// Playlist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Playlist {
  /// unique id for the playlist
  #[prost(int64, tag = "1")]
  pub id: i64,
  /// id of the owner of the playlist
  #[prost(int64, tag = "2")]
  pub owner_id: i64,
  /// ids of tracks in the playlist
  #[prost(int64, repeated, tag = "3")]
  pub track_ids: ::prost::alloc::vec::Vec<i64>,
  /// name of the playlist
  #[prost(string, tag = "4")]
  pub name: ::prost::alloc::string::String,
  /// description of the playlist
  #[prost(string, tag = "5")]
  pub description: ::prost::alloc::string::String,
  /// time when the playlist is created
  #[prost(message, optional, tag = "6")]
  pub created_at: ::core::option::Option<::prost_types::Timestamp>,
  /// time of the last update
  #[prost(message, optional, tag = "7")]
  pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// CurrentPlaylist controller
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentPlaylist {
  /// unique id for the current playlist
  #[prost(int64, tag = "1")]
  pub id: i64,
  /// id of the playlist
  #[prost(int64, tag = "2")]
  pub playlist_id: i64,
  /// id of the current playing track
  #[prost(int64, tag = "3")]
  pub current_track_id: i64,
  /// is the playlist playing
  #[prost(bool, tag = "4")]
  pub playing: bool,
  /// time when the current track started playing
  #[prost(message, optional, tag = "5")]
  pub started_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Track {
  /// unique id for the track
  #[prost(int64, tag = "1")]
  pub id: i64,
  /// name of the track
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  /// artist of the track
  #[prost(string, tag = "3")]
  pub artist: ::prost::alloc::string::String,
  /// album of the track
  #[prost(string, tag = "4")]
  pub album: ::prost::alloc::string::String,
  /// duration of the track in milliseconds
  #[prost(int32, tag = "5")]
  pub duration: i32,
  /// local source of the track
  #[prost(message, optional, tag = "6")]
  pub local_src: ::core::option::Option<LocalSource>,
  /// netease source of the track
  #[prost(message, optional, tag = "7")]
  pub netease_src: ::core::option::Option<NeteaseSource>,
}
/// NeteaseSource, not implemented yet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeteaseSource {}
/// LocalSource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalSource {
  /// path of the track
  #[prost(string, tag = "1")]
  pub path: ::prost::alloc::string::String,
}
/// User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
  /// unique id for the user
  #[prost(int64, tag = "1")]
  pub id: i64,
  /// name of the user
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
}
/// Create playlist request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlaylistRequest {
  /// Playlist to be created
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
/// Create playlist response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlaylistResponse {
  /// Created playlist
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
/// Delete playlist request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePlaylistsRequest {
  /// Ids of playlists to be deleted
  #[prost(int64, repeated, tag = "1")]
  pub ids: ::prost::alloc::vec::Vec<i64>,
}
/// Query playlist by user id and track id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaylistQuery {
  /// which user has the playlist
  #[prost(int64, optional, tag = "1")]
  pub user_id: ::core::option::Option<i64>,
  /// Query by contained track id
  #[prost(int64, optional, tag = "2")]
  pub track_id: ::core::option::Option<i64>,
  /// Query by name
  #[prost(string, optional, tag = "3")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Query playlist request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPlaylistsRequest {
  /// Query
  #[prost(message, optional, tag = "1")]
  pub query: ::core::option::Option<PlaylistQuery>,
}
/// Update playlist: add/remove tracks, update name and description
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaylistUpdate {
  /// Id of the playlist to be updated
  #[prost(int64, tag = "1")]
  pub id: i64,
  /// Ids of tracks to be added
  #[prost(int64, repeated, tag = "2")]
  pub added_track_ids: ::prost::alloc::vec::Vec<i64>,
  /// Ids of tracks to be removed
  #[prost(int64, repeated, tag = "3")]
  pub removed_track_ids: ::prost::alloc::vec::Vec<i64>,
  /// New name of the playlist
  #[prost(string, optional, tag = "4")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
  /// New description of the playlist
  #[prost(string, optional, tag = "5")]
  pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlaylistRequest {
  #[prost(message, optional, tag = "1")]
  pub update: ::core::option::Option<PlaylistUpdate>,
}
/// Updated playlist will be returned in response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlaylistResponse {
  /// Updated playlist
  #[prost(message, optional, tag = "1")]
  pub playlist: ::core::option::Option<Playlist>,
}
/// Create track request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrackRequest {
  /// Track to be created
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
/// Create track response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrackResponse {
  /// Created track
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
/// Query track
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackQuery {
  /// Query by which playlist contains the track
  #[prost(int64, optional, tag = "1")]
  pub playlist_id: ::core::option::Option<i64>,
  /// Query by name
  #[prost(string, optional, tag = "2")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
  /// Query by artist
  #[prost(string, optional, tag = "3")]
  pub artist: ::core::option::Option<::prost::alloc::string::String>,
  /// Query by album
  #[prost(string, optional, tag = "4")]
  pub album: ::core::option::Option<::prost::alloc::string::String>,
}
/// Query track request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTracksRequest {
  /// Query
  #[prost(message, optional, tag = "1")]
  pub query: ::core::option::Option<TrackQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackUpdate {
  /// Id of the track to be updated
  #[prost(int64, tag = "1")]
  pub id: i64,
  /// New name of the track
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
  /// New artist of the track
  #[prost(string, tag = "3")]
  pub artist: ::prost::alloc::string::String,
  /// New album of the track
  #[prost(string, tag = "4")]
  pub album: ::prost::alloc::string::String,
  #[prost(int32, tag = "5")]
  pub duration: i32,
  /// New local source of the track
  #[prost(message, optional, tag = "6")]
  pub local_src: ::core::option::Option<LocalSource>,
  /// New netease source of the track
  #[prost(message, optional, tag = "7")]
  pub netease_src: ::core::option::Option<NeteaseSource>,
}
/// Update track request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTrackRequest {
  #[prost(message, optional, tag = "1")]
  pub update: ::core::option::Option<TrackUpdate>,
}
/// Update track response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTrackResponse {
  /// Updated track
  #[prost(message, optional, tag = "1")]
  pub track: ::core::option::Option<Track>,
}
/// Delete tracks request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTracksRequest {
  /// Ids of tracks to be deleted
  #[prost(int64, repeated, tag = "1")]
  pub track_ids: ::prost::alloc::vec::Vec<i64>,
}
/// Create user request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserRequest {
  /// User to be created
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
/// Create user response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUserResponse {
  /// Created user
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserQuery {
  /// Query by name
  #[prost(string, optional, tag = "1")]
  pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Query user
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUsersRequest {
  #[prost(message, optional, tag = "1")]
  pub query: ::core::option::Option<UserQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserUpdate {
  /// Id of the user to be updated
  #[prost(int64, tag = "1")]
  pub id: i64,
  /// New name of the user
  #[prost(string, tag = "2")]
  pub name: ::prost::alloc::string::String,
}
/// Update user request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserRequest {
  #[prost(message, optional, tag = "1")]
  pub update: ::core::option::Option<UserUpdate>,
}
/// Update user response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserResponse {
  /// Updated user
  #[prost(message, optional, tag = "1")]
  pub user: ::core::option::Option<User>,
}
/// Delete users request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUsersRequest {
  /// Ids of users to be deleted
  #[prost(int64, repeated, tag = "1")]
  pub ids: ::prost::alloc::vec::Vec<i64>,
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
      D: std::convert::TryInto<tonic::transport::Endpoint>,
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
    pub async fn create_playlist(
      &mut self,
      request: impl tonic::IntoRequest<super::CreatePlaylistRequest>,
    ) -> Result<tonic::Response<super::CreatePlaylistResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/createPlaylist");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn query_playlists(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryPlaylistsRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::Playlist>>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/queryPlaylists");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
    }
    pub async fn update_playlist(
      &mut self,
      request: impl tonic::IntoRequest<super::UpdatePlaylistRequest>,
    ) -> Result<tonic::Response<super::UpdatePlaylistResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/updatePlaylist");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn delete_playlists(
      &mut self,
      request: impl tonic::IntoRequest<super::DeletePlaylistsRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::Playlist>>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/deletePlaylists");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
    }
    pub async fn create_track(
      &mut self,
      request: impl tonic::IntoRequest<super::CreateTrackRequest>,
    ) -> Result<tonic::Response<super::CreateTrackResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/createTrack");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn query_tracks(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryTracksRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::Track>>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/queryTracks");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
    }
    pub async fn update_track(
      &mut self,
      request: impl tonic::IntoRequest<super::UpdateTrackRequest>,
    ) -> Result<tonic::Response<super::UpdateTrackResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/updateTrack");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn delete_tracks(
      &mut self,
      request: impl tonic::IntoRequest<super::DeleteTracksRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::Track>>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/deleteTracks");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
    }
    pub async fn create_user(
      &mut self,
      request: impl tonic::IntoRequest<super::CreateUserRequest>,
    ) -> Result<tonic::Response<super::CreateUserResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/createUser");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn query_users(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryUsersRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::User>>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/queryUsers");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
    }
    pub async fn update_user(
      &mut self,
      request: impl tonic::IntoRequest<super::UpdateUserRequest>,
    ) -> Result<tonic::Response<super::UpdateUserResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/updateUser");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn delete_users(
      &mut self,
      request: impl tonic::IntoRequest<super::DeleteUsersRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::User>>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/musync.MusyncService/deleteUsers");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
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
    async fn create_playlist(
      &self,
      request: tonic::Request<super::CreatePlaylistRequest>,
    ) -> Result<tonic::Response<super::CreatePlaylistResponse>, tonic::Status>;
    /// Server streaming response type for the queryPlaylists method.
    type queryPlaylistsStream: futures_core::Stream<Item = Result<super::Playlist, tonic::Status>>
      + Send
      + 'static;
    async fn query_playlists(
      &self,
      request: tonic::Request<super::QueryPlaylistsRequest>,
    ) -> Result<tonic::Response<Self::queryPlaylistsStream>, tonic::Status>;
    async fn update_playlist(
      &self,
      request: tonic::Request<super::UpdatePlaylistRequest>,
    ) -> Result<tonic::Response<super::UpdatePlaylistResponse>, tonic::Status>;
    /// Server streaming response type for the deletePlaylists method.
    type deletePlaylistsStream: futures_core::Stream<Item = Result<super::Playlist, tonic::Status>>
      + Send
      + 'static;
    async fn delete_playlists(
      &self,
      request: tonic::Request<super::DeletePlaylistsRequest>,
    ) -> Result<tonic::Response<Self::deletePlaylistsStream>, tonic::Status>;
    async fn create_track(
      &self,
      request: tonic::Request<super::CreateTrackRequest>,
    ) -> Result<tonic::Response<super::CreateTrackResponse>, tonic::Status>;
    /// Server streaming response type for the queryTracks method.
    type queryTracksStream: futures_core::Stream<Item = Result<super::Track, tonic::Status>>
      + Send
      + 'static;
    async fn query_tracks(
      &self,
      request: tonic::Request<super::QueryTracksRequest>,
    ) -> Result<tonic::Response<Self::queryTracksStream>, tonic::Status>;
    async fn update_track(
      &self,
      request: tonic::Request<super::UpdateTrackRequest>,
    ) -> Result<tonic::Response<super::UpdateTrackResponse>, tonic::Status>;
    /// Server streaming response type for the deleteTracks method.
    type deleteTracksStream: futures_core::Stream<Item = Result<super::Track, tonic::Status>>
      + Send
      + 'static;
    async fn delete_tracks(
      &self,
      request: tonic::Request<super::DeleteTracksRequest>,
    ) -> Result<tonic::Response<Self::deleteTracksStream>, tonic::Status>;
    async fn create_user(
      &self,
      request: tonic::Request<super::CreateUserRequest>,
    ) -> Result<tonic::Response<super::CreateUserResponse>, tonic::Status>;
    /// Server streaming response type for the queryUsers method.
    type queryUsersStream: futures_core::Stream<Item = Result<super::User, tonic::Status>>
      + Send
      + 'static;
    async fn query_users(
      &self,
      request: tonic::Request<super::QueryUsersRequest>,
    ) -> Result<tonic::Response<Self::queryUsersStream>, tonic::Status>;
    async fn update_user(
      &self,
      request: tonic::Request<super::UpdateUserRequest>,
    ) -> Result<tonic::Response<super::UpdateUserResponse>, tonic::Status>;
    /// Server streaming response type for the deleteUsers method.
    type deleteUsersStream: futures_core::Stream<Item = Result<super::User, tonic::Status>>
      + Send
      + 'static;
    async fn delete_users(
      &self,
      request: tonic::Request<super::DeleteUsersRequest>,
    ) -> Result<tonic::Response<Self::deleteUsersStream>, tonic::Status>;
  }
  /// Musync service
  #[derive(Debug)]
  pub struct MusyncServiceServer<T: MusyncService> {
    inner: _Inner<T>,
    accept_compression_encodings: EnabledCompressionEncodings,
    send_compression_encodings: EnabledCompressionEncodings,
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
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
      Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
      let inner = self.inner.clone();
      match req.uri().path() {
        "/musync.MusyncService/createPlaylist" => {
          #[allow(non_camel_case_types)]
          struct createPlaylistSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::CreatePlaylistRequest>
            for createPlaylistSvc<T>
          {
            type Response = super::CreatePlaylistResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::CreatePlaylistRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).create_playlist(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = createPlaylistSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/queryPlaylists" => {
          #[allow(non_camel_case_types)]
          struct queryPlaylistsSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::QueryPlaylistsRequest>
            for queryPlaylistsSvc<T>
          {
            type Response = super::Playlist;
            type ResponseStream = T::queryPlaylistsStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::QueryPlaylistsRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).query_playlists(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = queryPlaylistsSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/updatePlaylist" => {
          #[allow(non_camel_case_types)]
          struct updatePlaylistSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::UpdatePlaylistRequest>
            for updatePlaylistSvc<T>
          {
            type Response = super::UpdatePlaylistResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::UpdatePlaylistRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).update_playlist(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = updatePlaylistSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/deletePlaylists" => {
          #[allow(non_camel_case_types)]
          struct deletePlaylistsSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService>
            tonic::server::ServerStreamingService<super::DeletePlaylistsRequest>
            for deletePlaylistsSvc<T>
          {
            type Response = super::Playlist;
            type ResponseStream = T::deletePlaylistsStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::DeletePlaylistsRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).delete_playlists(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = deletePlaylistsSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/createTrack" => {
          #[allow(non_camel_case_types)]
          struct createTrackSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::CreateTrackRequest>
            for createTrackSvc<T>
          {
            type Response = super::CreateTrackResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::CreateTrackRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).create_track(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = createTrackSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/queryTracks" => {
          #[allow(non_camel_case_types)]
          struct queryTracksSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::QueryTracksRequest>
            for queryTracksSvc<T>
          {
            type Response = super::Track;
            type ResponseStream = T::queryTracksStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::QueryTracksRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).query_tracks(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = queryTracksSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/updateTrack" => {
          #[allow(non_camel_case_types)]
          struct updateTrackSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::UpdateTrackRequest>
            for updateTrackSvc<T>
          {
            type Response = super::UpdateTrackResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::UpdateTrackRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).update_track(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = updateTrackSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/deleteTracks" => {
          #[allow(non_camel_case_types)]
          struct deleteTracksSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::DeleteTracksRequest>
            for deleteTracksSvc<T>
          {
            type Response = super::Track;
            type ResponseStream = T::deleteTracksStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::DeleteTracksRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).delete_tracks(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = deleteTracksSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/createUser" => {
          #[allow(non_camel_case_types)]
          struct createUserSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::CreateUserRequest> for createUserSvc<T> {
            type Response = super::CreateUserResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::CreateUserRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).create_user(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = createUserSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/queryUsers" => {
          #[allow(non_camel_case_types)]
          struct queryUsersSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::QueryUsersRequest>
            for queryUsersSvc<T>
          {
            type Response = super::User;
            type ResponseStream = T::queryUsersStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::QueryUsersRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).query_users(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = queryUsersSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/updateUser" => {
          #[allow(non_camel_case_types)]
          struct updateUserSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::UnaryService<super::UpdateUserRequest> for updateUserSvc<T> {
            type Response = super::UpdateUserResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::UpdateUserRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).update_user(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = updateUserSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/musync.MusyncService/deleteUsers" => {
          #[allow(non_camel_case_types)]
          struct deleteUsersSvc<T: MusyncService>(pub Arc<T>);
          impl<T: MusyncService> tonic::server::ServerStreamingService<super::DeleteUsersRequest>
            for deleteUsersSvc<T>
          {
            type Response = super::User;
            type ResponseStream = T::deleteUsersStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::DeleteUsersRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).delete_users(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = deleteUsersSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
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
      }
    }
  }
  impl<T: MusyncService> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone())
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
