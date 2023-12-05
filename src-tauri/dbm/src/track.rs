use std::collections::HashMap;

use abi::{LocalFolder, QueryLocalFoldersRequest};
use chrono::Utc;
use entity::prelude::*;

use sea_orm::{
  ActiveModelTrait,
  ActiveValue::{self, NotSet},
  ColumnTrait, Condition, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use tracing::{info, trace, warn};

use crate::{DbManager, MusyncError, TrackId};

impl DbManager {
  pub async fn create_track(&self, track: abi::Track) -> Result<abi::Track, MusyncError> {
    let txn = self.db.begin().await?;
    let now = Utc::now();
    let inserted = entity::track::ActiveModel {
      id: NotSet,
      title: Set(track.title.clone()),
      artist: Set(track.artist.clone()),
      album: Set(track.album.clone()),
      duration: Set(track.duration),
      genre: Set(track.genre.clone()),
      year: Set(track.year),
      created_at: Set(now),
      updated_at: Set(now),
    }
    .insert(&txn)
    .await?;
    if let Some(src) = track.local_src {
      entity::local_src::ActiveModel {
        track_id: Set(inserted.id),
        path: Set(src.path),
        folder_id: NotSet,
      }
      .insert(&txn)
      .await?;
    }
    txn.commit().await?;
    Ok(abi::Track::from_entity(inserted))
  }

  pub async fn create_local_tracks(
    &self,
    tracks: Vec<abi::Track>,
    folder: &str,
  ) -> Result<TrackId, MusyncError> {
    let txn = self.db.begin().await?;
    let now = Utc::now();
    let folder = entity::local_src_folder::ActiveModel {
      id: NotSet,
      path: Set(folder.to_owned()),
      created_at: Set(now),
      updated_at: Set(now),
    }
    .insert(&txn)
    .await
    .map_err(|_| MusyncError::FolderExists(folder.to_owned()))?;
    let active_tracks = tracks.iter().map(|t| entity::track::ActiveModel {
      id: NotSet,
      title: Set(t.title.clone()),
      artist: Set(t.artist.clone()),
      album: Set(t.album.clone()),
      duration: Set(t.duration),
      genre: Set(t.genre.clone()),
      year: Set(t.year),
      created_at: Set(now),
      updated_at: Set(now),
    });
    let inserted = Track::insert_many(active_tracks).exec(&txn).await?;
    info!(
      "inserted tracks: {}",
      inserted.last_insert_id - tracks.len() as i32
    );
    let start_id = inserted.last_insert_id - tracks.len() as i32 + 1;
    let mut local_srcs = vec![];
    for (idx, track) in tracks.iter().enumerate() {
      if let Some(src) = track.local_src.as_ref() {
        local_srcs.push(entity::local_src::ActiveModel {
          track_id: Set(start_id + idx as i32),
          path: Set(src.path.clone()),
          folder_id: Set(Some(folder.id)),
        });
      }
      if let Some(_src) = track.netease_src.as_ref() {
        warn!("netease_src is not supported yet");
      }
    }
    LocalSrc::insert_many(local_srcs).exec(&txn).await?;
    txn.commit().await?;
    Ok(inserted.last_insert_id)
  }

  pub async fn create_ncm_track(
    &self,
    ncm_track: &ncmapi::types::Song,
    raw_json: Option<&str>,
  ) -> Result<abi::Track, MusyncError> {
    let now = Utc::now();
    let artists = ncm_track
      .artists
      .iter()
      .flat_map(|a| a.name.clone())
      .collect::<Vec<_>>();
    let artist = if artists.is_empty() {
      NotSet
    } else {
      Set(Some(artists.join(",")))
    };
    let mut active_track = entity::track::ActiveModel {
      id: NotSet,
      title: Set(ncm_track.name.clone()),
      artist,
      album: Set(ncm_track.album.name.clone()),
      duration: Set(Some(ncm_track.duration as u32)),
      genre: NotSet,
      year: NotSet,
      created_at: NotSet,
      updated_at: Set(now),
    };

    let mut active_netease_src = entity::netease_src::ActiveModel {
      track_id: NotSet,
      netease_id: Set(ncm_track.id.to_string()),
      pop: Set(Some(ncm_track.pop as f64)),
      raw_json: Set(raw_json.map(|s| s.to_owned())),
    };

    let ncm: Option<entity::netease_src::Model> = NeteaseSrc::find()
      .filter(entity::netease_src::Column::NeteaseId.eq(ncm_track.id as u32))
      .one(&self.db)
      .await?;
    let (track_entity, netease_src) = if let Some(ncm) = ncm {
      // update exists track info
      let txn = self.db.begin().await?;

      active_track.id = Set(ncm.track_id);
      let track_entity = active_track.update(&txn).await?;

      active_netease_src.track_id = Set(track_entity.id);
      let netease_src = active_netease_src.update(&txn).await?;
      txn.commit().await?;

      (track_entity, netease_src)
    } else {
      // create track with netease src
      let txn = self.db.begin().await?;

      active_track.created_at = Set(now);
      let track_entity = active_track.insert(&txn).await?;

      active_netease_src.track_id = Set(track_entity.id);
      let netease_src = active_netease_src.insert(&txn).await?;

      txn.commit().await?;

      (track_entity, netease_src)
    };
    let mut track = abi::Track::from_entity(track_entity);
    track.netease_src = Some(abi::NeteaseSource {
      id: netease_src.netease_id,
      pop: netease_src.pop.map(|p| p as f32),
    });
    Ok(track)
  }

  pub async fn create_ncm_tracks(
    &self,
    tracks: &[ncmapi::types::Song],
  ) -> Result<Vec<abi::Track>, MusyncError> {
    fn compress_artists(ncm_track: &ncmapi::types::Song) -> ActiveValue<Option<String>> {
      let artists = ncm_track
        .artists
        .iter()
        .flat_map(|a| a.name.clone())
        .collect::<Vec<_>>();
      if artists.is_empty() {
        NotSet
      } else {
        Set(Some(artists.join(",")))
      }
    }
    trace!("creating ncm tracks, size: {}", tracks.len());
    let now = Utc::now();

    let exist_tracks = NeteaseSrc::find()
      .filter(
        entity::netease_src::Column::NeteaseId.is_in(
          tracks
            .iter()
            .map(|t| t.id.to_string())
            .collect::<Vec<_>>()
            .to_owned(),
        ),
      )
      .all(&self.db)
      .await?;
    let mut exists = HashMap::with_capacity(exist_tracks.len());
    for track in exist_tracks {
      exists.insert(track.netease_id.clone(), track);
    }
    let mut abi_tracks = vec![];
    let txn = self.db.begin().await?;

    let mut updated_count = 0;
    let mut inserted_count = 0;
    for track in tracks {
      let mut active_netease_src = entity::netease_src::ActiveModel {
        track_id: NotSet,
        netease_id: Set(track.id.to_string()),
        pop: Set(Some(track.pop as f64)),
        raw_json: NotSet,
      };
      let mut active_track = entity::track::ActiveModel {
        id: NotSet,
        title: Set(track.name.clone()),
        artist: compress_artists(track),
        album: Set(track.album.name.clone()),
        duration: Set(Some(track.duration as u32)),
        genre: NotSet,
        year: NotSet,
        created_at: NotSet,
        updated_at: Set(now),
      };

      let (track_entity, netease_src_entity) =
        if let Some(exist) = exists.get(&track.id.to_string()) {
          // update exists track info
          active_track.id = Set(exist.track_id);
          let track_entity = active_track.update(&txn).await?;
          active_netease_src.track_id = Set(track_entity.id);
          let netease_src_entity = active_netease_src.update(&txn).await?;

          updated_count += 1;
          (track_entity, netease_src_entity)
        } else {
          // create track with netease src
          active_track.created_at = Set(now);
          let track_entity = active_track.insert(&txn).await?;
          active_netease_src.track_id = Set(track_entity.id);
          let netease_src_entity = active_netease_src.insert(&txn).await?;

          inserted_count += 1;
          (track_entity, netease_src_entity)
        };
      let mut track = abi::Track::from_entity(track_entity);
      track.netease_src = Some(abi::NeteaseSource {
        id: netease_src_entity.netease_id,
        pop: netease_src_entity.pop.map(|p| p as f32),
      });
      abi_tracks.push(track);
    }

    txn.commit().await?;

    trace!("created ncm tracks, updated: {updated_count}, inserted: {inserted_count}");
    Ok(abi_tracks)
  }

  pub async fn remove_folder(&self, folder: &str) -> Result<u64, MusyncError> {
    let txn = self.db.begin().await?;
    // delete cascade
    LocalSrcFolder::delete_many()
      .filter(entity::local_src_folder::Column::Path.eq(folder))
      .exec(&txn)
      .await
      .map_err(|_| (MusyncError::FolderNotFound(folder.to_owned())))?;
    // delete tracks without local_src and netease_src
    use sea_orm::query::*;
    let deleted = Track::delete_many()
      .filter(
        entity::track::Column::Id.not_in_subquery(
          LocalSrc::find()
            .select_only()
            .column(entity::local_src::Column::TrackId)
            .into_query(),
        ),
      )
      .exec(&txn)
      .await?;
    txn.commit().await?;
    trace!("deleted tracks: {:?}", deleted);
    Ok(deleted.rows_affected)
  }

  pub async fn query_local_folders(
    &self,
    query: QueryLocalFoldersRequest,
  ) -> Result<Vec<LocalFolder>, MusyncError> {
    let QueryLocalFoldersRequest {} = query;
    let folders = LocalSrcFolder::find().all(&self.db).await?;
    Ok(folders.into_iter().map(LocalFolder::from_entity).collect())
  }

  pub async fn update_track(&self, update: abi::TrackUpdate) -> Result<abi::Track, MusyncError> {
    if update.netease_src.is_some() {
      warn!("netease_src is not supported yet");
    }
    let now = Utc::now();
    let old = Track::find_by_id(update.id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::TrackNotFound(update.id))?;

    let mut updating: entity::track::ActiveModel = old.into();
    if let Some(title) = update.title {
      updating.title = Set(title);
    }
    if let Some(artist) = update.artist {
      updating.artist = Set(Some(artist));
    }
    if let Some(album) = update.album {
      updating.album = Set(Some(album));
    }
    if let Some(duration) = update.duration {
      updating.duration = Set(Some(duration));
    }
    if let Some(genre) = update.genre {
      updating.genre = Set(Some(genre));
    }
    if let Some(year) = update.year {
      updating.year = Set(Some(year));
    }
    updating.updated_at = Set(now);
    updating.update(&self.db).await?;

    if let Some(src) = update.local_src {
      let old = LocalSrc::find_by_id(update.id).one(&self.db).await?;
      if let Some(old) = old {
        let mut updating: entity::local_src::ActiveModel = old.into();
        updating.path = Set(src.path);
        updating.update(&self.db).await?;
      } else {
        let inserted = entity::local_src::ActiveModel {
          track_id: Set(update.id),
          path: Set(src.path),
          folder_id: NotSet,
        };
        inserted.insert(&self.db).await?;
      }
    }

    self.track(update.id).await
  }

  pub async fn delete_tracks(&self, ids: &[TrackId]) -> Result<u64, MusyncError> {
    if ids.is_empty() {
      return Ok(0);
    }

    let deleted = Track::delete_many()
      .filter(entity::track::Column::Id.is_in(ids.to_owned()))
      .exec(&self.db)
      .await?;
    Ok(deleted.rows_affected)
  }

  pub async fn query_tracks(
    &self,
    query: abi::QueryTracksRequest,
  ) -> Result<Vec<abi::Track>, MusyncError> {
    trace!("query tracks: {:?}", query);

    let rows = if query.title.is_none() && query.artist.is_none() && query.album.is_none() {
      Track::find().all(&self.db).await?
    } else {
      Track::find()
        .filter(
          Condition::any()
            .add_option(
              query
                .title
                .map(|title| entity::track::Column::Title.like(format!("%{}%", title))),
            )
            .add_option(
              query
                .artist
                .map(|artist| entity::track::Column::Artist.like(format!("%{}%", artist))),
            )
            .add_option(
              query
                .album
                .map(|album| entity::track::Column::Album.like(format!("%{}%", album))),
            ),
        )
        .all(&self.db)
        .await?
    };

    rows
      .into_iter()
      .map(|row| Ok(abi::Track::from_entity(row)))
      .collect()
  }

  pub async fn track(&self, id: TrackId) -> Result<abi::Track, MusyncError> {
    let row = Track::find_by_id(id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::TrackNotFound(id))?;

    let mut abi_track = abi::Track::from_entity(row);
    let src = LocalSrc::find_by_id(id).one(&self.db).await?;
    if let Some(src) = src {
      abi_track.local_src = Some(abi::LocalSource { path: src.path })
    }

    Ok(abi_track)
  }
}
