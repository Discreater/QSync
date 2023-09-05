use std::{pin::Pin, sync::Arc};

use abi::{
  AddLocalFolderRequest, AddLocalFolderResponse, CreatePlayQueueRequest, CreatePlayQueueResponse,
  CreatePlaylistRequest, CreatePlaylistResponse, CreateTrackRequest, CreateTrackResponse,
  CreateUserRequest, CreateUserResponse, DeletePlaylistsRequest, DeleteTracksRequest,
  DeleteUsersRequest, GetPlayQueueRequest, GetTrackCoverRequest, GetTrackRequest, LocalFolder,
  LoginRequest, Picture, PlayQueue, Playlist, QueryLocalFoldersRequest, QueryPlaylistsRequest,
  QueryTracksRequest, QueryUsersRequest, RemoveLocalFolderRequest, RemoveLocalFolderResponse,
  SearchAllRequest, SearchAllResponse, Token, Track, UpdatePlayQueueEvent, UpdatePlaylistRequest,
  UpdatePlaylistResponse, UpdateTrackRequest, UpdateTrackResponse, UpdateUserRequest,
  UpdateUserResponse, User,
};
use chrono::{Days, Utc};
use dbm::UserId;
use futures::Stream;

use ncmapi::NcmApi;
use tonic::{Extensions, Request, Response, Status};
use tracing::{error, trace};
use utils::WithError;

use crate::{musync, user::Claims, ws::WsState};

pub struct GrpcServer {
  db: dbm::DbManager,
  ws_state: Arc<WsState>,
  ncm_api: Arc<NcmApi>,
}

impl GrpcServer {
  pub fn new(db: dbm::DbManager, ws_state: Arc<WsState>) -> Self {
    Self {
      db,
      ws_state,
      ncm_api: Arc::new(NcmApi::default()),
    }
  }
}

pub type GrpcResult<T> = Result<Response<T>, Status>;
pub type GrpcStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;

#[tonic::async_trait]
impl abi::musync_service_server::MusyncService for GrpcServer {
  async fn login(&self, req: Request<LoginRequest>) -> GrpcResult<Token> {
    let req = req.into_inner();
    let res = self.db.login(req).await.map_err(|e| match e {
      dbm::MusyncError::LoginFailed(_) => Status::not_found("User not found"),
      e => {
        error!("login failed: {}", e);
        Status::internal("unknown error")
      }
    })?;

    let claims = Claims {
      id: res.id,
      exp: Utc::now()
        .checked_add_days(Days::new(365))
        .unwrap()
        .timestamp() as usize,
    };
    let token = claims
      .encode()
      .map_err(|_| Status::internal("token creation failed"))?;
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
    let owner_id = get_userid(request.extensions())?;
    let req = request.into_inner();
    let playqueue = self.db.create_play_queue(req, owner_id).await?;
    self
      .ws_state
      .tx
      .send(Arc::new(crate::ws::BroadcastMsg::UpdatePlayQueue {
        user_id: owner_id,
        update: UpdatePlayQueueEvent {
          track_ids: playqueue.track_ids,
        },
      }))
      .with_err(|e| {
        error!("broadcast update play queue event failed: {}", e);
      });
    Ok(Response::new(CreatePlayQueueResponse {
      play_queue_id: playqueue.id,
    }))
  }

  async fn get_play_queue(&self, request: Request<GetPlayQueueRequest>) -> GrpcResult<PlayQueue> {
    let owner_id = get_userid(request.extensions())?;
    let playqueue = self
      .db
      .get_user_play_queue(owner_id)
      .await?
      .ok_or(Status::not_found("not found play queue"))?;
    Ok(Response::new(playqueue))
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
    _request: Request<QueryPlaylistsRequest>,
  ) -> GrpcResult<Self::QueryPlaylistsStream> {
    unimplemented!()
  }
  async fn update_playlist(
    &self,
    _request: Request<UpdatePlaylistRequest>,
  ) -> GrpcResult<UpdatePlaylistResponse> {
    unimplemented!()
  }
  /// Server streaming response type for the DeletePlaylists method.
  type DeletePlaylistsStream = GrpcStream<Playlist>;
  async fn delete_playlists(
    &self,
    _request: Request<DeletePlaylistsRequest>,
  ) -> GrpcResult<Self::DeletePlaylistsStream> {
    unimplemented!()
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
    _request: Request<CreateTrackRequest>,
  ) -> GrpcResult<CreateTrackResponse> {
    unimplemented!()
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
    _request: Request<UpdateTrackRequest>,
  ) -> GrpcResult<UpdateTrackResponse> {
    unimplemented!()
  }
  /// Server streaming response type for the DeleteTracks method.
  type DeleteTracksStream = GrpcStream<Track>;
  async fn delete_tracks(
    &self,
    _request: Request<DeleteTracksRequest>,
  ) -> GrpcResult<Self::DeleteTracksStream> {
    unimplemented!()
  }
  async fn create_user(
    &self,
    _request: Request<CreateUserRequest>,
  ) -> GrpcResult<CreateUserResponse> {
    unimplemented!()
  }
  /// Server streaming response type for the QueryUsers method.
  type QueryUsersStream = GrpcStream<User>;
  async fn query_users(
    &self,
    _request: Request<QueryUsersRequest>,
  ) -> GrpcResult<Self::QueryUsersStream> {
    unimplemented!()
  }
  async fn update_user(
    &self,
    _request: Request<UpdateUserRequest>,
  ) -> GrpcResult<UpdateUserResponse> {
    unimplemented!()
  }
  /// Server streaming response type for the DeleteUsers method.
  type DeleteUsersStream = GrpcStream<User>;
  async fn delete_users(
    &self,
    _request: Request<DeleteUsersRequest>,
  ) -> GrpcResult<Self::DeleteUsersStream> {
    unimplemented!()
  }

  async fn search_all(&self, request: Request<SearchAllRequest>) -> GrpcResult<SearchAllResponse> {
    let req = request.into_inner();
    let db_search = self.db.search_tracks(&req.query);
    let ncm_search = self.ncm_api.search(&req.query, None);
    let (db_tracks, ncm_res) = tokio::join!(db_search, ncm_search);
    let db_tracks = db_tracks?;
    let ncm_res = match ncm_res {
      Ok(res) => String::from_utf8_lossy(res.data()).into_owned(),
      Err(e) => e.to_string(),
    };
    Ok(Response::new(SearchAllResponse { db_tracks, ncm_res }))
  }
}

pub fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
  if let Some(v) = req.metadata().get("authorization") {
    let bearer = v
      .to_str()
      .map_err(|_| Status::new(tonic::Code::Unauthenticated, "Invalid token format"))?;
    let claims =
      Claims::decode(bearer).map_err(|_| Status::unauthenticated("Invalid token format"))?;
    trace!("auth {} success", claims.id);
    req.extensions_mut().insert(claims);
  }
  Ok(req)
}

fn get_userid(ext: &Extensions) -> Result<UserId, Status> {
  let claims = ext
    .get::<Claims>()
    .ok_or(Status::unauthenticated("No token"))?;
  Ok(claims.id)
}
