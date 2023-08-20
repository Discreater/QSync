use std::pin::Pin;

use abi::{
  AddLocalFolderRequest, AddLocalFolderResponse, CreatePlayQueueRequest, CreatePlayQueueResponse,
  CreatePlaylistRequest, CreatePlaylistResponse, CreateTrackRequest, CreateTrackResponse,
  CreateUserRequest, CreateUserResponse, DeletePlaylistsRequest, DeleteTracksRequest,
  DeleteUsersRequest, GetPlayQueueRequest, GetTrackCoverRequest, GetTrackRequest, LocalFolder,
  LoginRequest, Picture, PlayQueue, Playlist, QueryLocalFoldersRequest, QueryPlaylistsRequest,
  QueryTracksRequest, QueryUsersRequest, RemoveLocalFolderRequest, RemoveLocalFolderResponse,
  Token, Track, UpdatePlaylistRequest, UpdatePlaylistResponse, UpdateTrackRequest,
  UpdateTrackResponse, UpdateUserRequest, UpdateUserResponse, User,
};
use chrono::{Days, Utc};
use dbm::UserId;
use futures::Stream;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tonic::{Extensions, Request, Response, Status};
use tracing::trace;

use crate::musync;

pub struct GrpcServer {
  db: dbm::DbManager,
}

impl GrpcServer {
  pub fn new(db: dbm::DbManager) -> Self {
    Self { db }
  }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
  let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  Keys::new(secret.as_bytes())
});

pub type GrpcResult<T> = Result<Response<T>, Status>;
pub type GrpcStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[tonic::async_trait]
impl abi::musync_service_server::MusyncService for GrpcServer {
  async fn login(&self, req: Request<LoginRequest>) -> GrpcResult<Token> {
    let req = req.into_inner();
    let res = self.db.login(req).await.map_err(|e| match e {
      dbm::MusyncError::LoginFailed(_) => Status::not_found("User not found"),
      _ => Status::internal("Unknown error"),
    })?;

    let claims = Claims {
      id: res.id,
      exp: Utc::now()
        .checked_add_days(Days::new(365))
        .unwrap()
        .timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
      .map_err(|_| Status::internal("token creation"))?;
    let res = Token {
      data: token,
      r#type: "Bearer".to_string(),
    };
    Ok(Response::new(res))
  }
  async fn add_local_folder(
    &self,
    request: Request<AddLocalFolderRequest>,
  ) -> GrpcResult<AddLocalFolderResponse> {
    let req = request.into_inner();
    trace!("add local folder: {}", req.path);

    let tracks = musync::track::get_tracks_in_folder(&req.path);
    let len = tracks.len();
    self.db.create_tracks(tracks, &req.path).await?;

    Ok(Response::new(AddLocalFolderResponse {
      added_tracks: len as u32,
    }))
  }

  async fn remove_local_folder(
    &self,
    request: Request<RemoveLocalFolderRequest>,
  ) -> GrpcResult<RemoveLocalFolderResponse> {
    let req = request.into_inner();
    trace!("remove local folder: {}", req.path);

    let deleted_tracks = self.db.remove_folder(&req.path).await?;

    Ok(Response::new(RemoveLocalFolderResponse {
      removed_tracks: deleted_tracks as u32,
    }))
  }
  type QueryLocalFoldersStream = GrpcStream<LocalFolder>;
  async fn query_local_folders(
    &self,
    req: Request<QueryLocalFoldersRequest>,
  ) -> GrpcResult<Self::QueryLocalFoldersStream> {
    let req = req.into_inner();
    let folders = self.db.query_local_folders(req).await?;
    let stream = futures::stream::iter(folders.into_iter().map(Ok));
    Ok(Response::new(Box::pin(stream)))
  }

  async fn create_play_queue(
    &self,
    request: Request<CreatePlayQueueRequest>,
  ) -> GrpcResult<CreatePlayQueueResponse> {
    todo!()
  }

  async fn get_play_queue(&self, request: Request<GetPlayQueueRequest>) -> GrpcResult<PlayQueue> {
    todo!()
  }

  async fn create_playlist(
    &self,
    request: Request<CreatePlaylistRequest>,
  ) -> GrpcResult<CreatePlaylistResponse> {
    let user_id = get_userid(request.extensions())?;
    let req = request.into_inner();
    let playlist = self.db.create_playlist(user_id, req).await?;
    Ok(Response::new(CreatePlaylistResponse {
      playlist: Some(playlist),
    }))
  }
  /// Server streaming response type for the QueryPlaylists method.
  type QueryPlaylistsStream = GrpcStream<Playlist>;
  async fn query_playlists(
    &self,
    request: Request<QueryPlaylistsRequest>,
  ) -> GrpcResult<Self::QueryPlaylistsStream> {
    todo!()
  }
  async fn update_playlist(
    &self,
    request: Request<UpdatePlaylistRequest>,
  ) -> GrpcResult<UpdatePlaylistResponse> {
    todo!()
  }
  /// Server streaming response type for the DeletePlaylists method.
  type DeletePlaylistsStream = GrpcStream<Playlist>;
  async fn delete_playlists(
    &self,
    request: Request<DeletePlaylistsRequest>,
  ) -> GrpcResult<Self::DeletePlaylistsStream> {
    todo!()
  }
  async fn get_track(&self, request: Request<GetTrackRequest>) -> GrpcResult<Track> {
    let req = request.into_inner();
    let track = self.db.track(req.id).await?;
    Ok(Response::new(track))
  }

  async fn get_track_cover(&self, req: Request<GetTrackCoverRequest>) -> GrpcResult<Picture> {
    let req = req.into_inner();
    let track = self.db.track(req.track_id).await?;
    let covers = musync::track::get_track_pictures(&track);
    let idx = req.cover_idx.unwrap_or_default() as usize;
    match covers.into_iter().nth(idx) {
      Some(c) => Ok(Response::new(c)),
      None => Err(Status::not_found("")),
    }
  }

  async fn create_track(
    &self,
    request: Request<CreateTrackRequest>,
  ) -> GrpcResult<CreateTrackResponse> {
    todo!()
  }
  /// Server streaming response type for the QueryTracks method.
  type QueryTracksStream = GrpcStream<Track>;
  async fn query_tracks(
    &self,
    request: Request<QueryTracksRequest>,
  ) -> GrpcResult<Self::QueryTracksStream> {
    let query = request.into_inner();
    let tracks = self.db.query_tracks(query).await?;
    let stream = futures::stream::iter(tracks.into_iter().map(Ok));
    Ok(Response::new(Box::pin(stream)))
  }
  async fn update_track(
    &self,
    request: Request<UpdateTrackRequest>,
  ) -> GrpcResult<UpdateTrackResponse> {
    todo!()
  }
  /// Server streaming response type for the DeleteTracks method.
  type DeleteTracksStream = GrpcStream<Track>;
  async fn delete_tracks(
    &self,
    request: Request<DeleteTracksRequest>,
  ) -> GrpcResult<Self::DeleteTracksStream> {
    todo!()
  }
  async fn create_user(
    &self,
    request: Request<CreateUserRequest>,
  ) -> GrpcResult<CreateUserResponse> {
    todo!()
  }
  /// Server streaming response type for the QueryUsers method.
  type QueryUsersStream = GrpcStream<User>;
  async fn query_users(
    &self,
    request: Request<QueryUsersRequest>,
  ) -> GrpcResult<Self::QueryUsersStream> {
    todo!()
  }
  async fn update_user(
    &self,
    request: Request<UpdateUserRequest>,
  ) -> GrpcResult<UpdateUserResponse> {
    todo!()
  }
  /// Server streaming response type for the DeleteUsers method.
  type DeleteUsersStream = GrpcStream<User>;
  async fn delete_users(
    &self,
    request: Request<DeleteUsersRequest>,
  ) -> GrpcResult<Self::DeleteUsersStream> {
    todo!()
  }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Claims {
  id: UserId,
  exp: usize,
}

struct Keys {
  encoding: EncodingKey,
  decoding: DecodingKey,
}

impl Keys {
  fn new(secret: &[u8]) -> Self {
    Self {
      encoding: EncodingKey::from_secret(secret),
      decoding: DecodingKey::from_secret(secret),
    }
  }
}

pub fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
  if let Some(v) = req.metadata().get("authorization") {
    let bearer = v
      .to_str()
      .map_err(|_| Status::new(tonic::Code::Unauthenticated, "Invalid token format"))?;
    let token_data = decode::<Claims>(bearer, &KEYS.decoding, &Validation::default())
      .map_err(|_| Status::unauthenticated("Invalid token format"))?;
    trace!("auth {} success", token_data.claims.id);
    req.extensions_mut().insert(token_data.claims);
  }
  Ok(req)
}

fn get_userid(ext: &Extensions) -> Result<UserId, Status> {
  let claims = ext
    .get::<Option<Claims>>()
    .and_then(|t| t.as_ref())
    .ok_or(Status::unauthenticated("No token"))?;
  Ok(claims.id)
}
