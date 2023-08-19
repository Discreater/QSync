use axum::{
  body::StreamBody,
  extract::{Path, State},
  response::{AppendHeaders, IntoResponse},
};
use dbm::{DbManager, TrackId};
use http::header::{CACHE_CONTROL, CONTENT_TYPE};
use tokio_util::io::ReaderStream;

use crate::error::HttpError;

use self::track::get_track_pictures_internal;

pub mod track;

pub async fn get_track(
  Path(id): Path<TrackId>,
  State(manager): State<DbManager>,
) -> Result<impl IntoResponse, HttpError> {
  let track = manager.track(id).await?;

  if let Some(src) = track.local_src {
    let file = tokio::fs::File::open(&src.path)
      .await
      .map_err(|_| HttpError::Internal)?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let headers = AppendHeaders([
      (
        CONTENT_TYPE,
        mime_guess::from_path(src.path)
          .first()
          .map(|e| e.to_string())
          .ok_or(HttpError::Internal)?,
      ),
      (CACHE_CONTROL, "max-age=3600".to_owned()),
    ]);
    Ok((headers, body))
  } else {
    Err(HttpError::NotFound)
  }
}

pub async fn get_cover(
  Path(track_id): Path<TrackId>,
  State(manager): State<DbManager>,
) -> Result<impl IntoResponse, HttpError> {
  let track = manager.track(track_id).await?;
  let covers = get_track_pictures_internal(&track);
  if let Some(cover) = covers.into_iter().next() {
    let headers = AppendHeaders([
      (CONTENT_TYPE, cover.mime_type().to_string()),
      (CACHE_CONTROL, "max-age=3600".to_owned()),
    ]);
    return Ok((headers, cover.data().to_owned()));
  }
  Err(HttpError::NotFound)
}
