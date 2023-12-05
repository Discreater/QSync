use abi::CreatePlaylistRequest;
use chrono::Utc;
use entity::prelude::*;

use sea_orm::{
  ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, Condition, EntityTrait, QueryFilter,
  QueryTrait, Set,
};
use tracing::trace;

use crate::{DbManager, MusyncError, PlaylistId, UserId};

/// Playlist
impl DbManager {
  pub async fn create_playlist(
    &self,
    owner_id: UserId,
    create: CreatePlaylistRequest,
  ) -> Result<abi::Playlist, MusyncError> {
    let now = Utc::now();
    // create playlist
    let inserted = entity::playlist::ActiveModel {
      id: NotSet,
      owner_id: Set(owner_id),
      name: Set(create.name.clone()),
      description: Set(create.description.clone()),
      created_at: Set(now),
      updated_at: Set(now),
    }
    .insert(&self.db)
    .await?;

    trace!("playlist inserted, id: {}", inserted.id);

    let mut playlist = abi::Playlist::from_entity(inserted, vec![]);

    if create.track_ids.is_empty() {
      return Ok(playlist);
    }

    let track_list = create
      .track_ids
      .iter()
      .map(|track_id| entity::playlist_track::ActiveModel {
        playlist_id: Set(playlist.id),
        track_id: Set(*track_id),
      });
    PlaylistTrack::insert_many(track_list)
      .exec(&self.db)
      .await?;

    playlist.track_ids.extend(create.track_ids);

    Ok(playlist)
  }

  pub async fn update_playlist(
    &self,
    playlist: abi::UpdatePlaylistRequest,
  ) -> Result<abi::Playlist, MusyncError> {
    let now = Utc::now();

    let old = Playlist::find_by_id(playlist.id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::PlaylistNotFound(playlist.id))?;

    let mut updating: entity::playlist::ActiveModel = old.into();
    if let Some(name) = playlist.name {
      updating.name = Set(name);
    }
    if let Some(description) = playlist.description {
      updating.description = Set(description);
    }
    updating.updated_at = Set(now);
    updating.update(&self.db).await?;

    if !playlist.added_track_ids.is_empty() {
      let added_tracks =
        playlist
          .added_track_ids
          .iter()
          .map(|track_id| entity::playlist_track::ActiveModel {
            playlist_id: Set(playlist.id),
            track_id: Set(*track_id),
          });
      PlaylistTrack::insert_many(added_tracks)
        .exec(&self.db)
        .await?;
    }

    if !playlist.removed_track_ids.is_empty() {
      PlaylistTrack::delete_many()
        .filter(
          Condition::all()
            .add(entity::playlist_track::Column::PlaylistId.eq(playlist.id))
            .add(entity::playlist_track::Column::TrackId.is_in(playlist.removed_track_ids)),
        )
        .exec(&self.db)
        .await?;
    }

    self.playlist(playlist.id).await
  }

  pub async fn delete_playlists(&self, ids: &[PlaylistId]) -> Result<u64, MusyncError> {
    if ids.is_empty() {
      return Ok(0);
    }

    let deleted = Playlist::delete_many()
      .filter(entity::playlist::Column::Id.is_in(ids.to_owned()))
      .exec(&self.db)
      .await?;

    Ok(deleted.rows_affected)
  }

  pub async fn query_playlists(
    &self,
    query: abi::QueryPlaylistsRequest,
  ) -> Result<Vec<abi::Playlist>, MusyncError> {
    let playlists = Playlist::find()
      .apply_if(query.name, |builder, name| {
        builder.filter(entity::playlist::Column::Name.like(format!("%{}%", name)))
      })
      .apply_if(query.user_id, |b, user_id| {
        b.inner_join(UserPlaylist)
          .filter(entity::user_playlist::Column::UserId.eq(user_id))
      })
      .apply_if(query.track_id, |b, track_id| {
        b.inner_join(PlaylistTrack)
          .filter(entity::playlist_track::Column::TrackId.eq(track_id))
      })
      .all(&self.db)
      .await?;

    Ok(
      playlists
        .into_iter()
        .map(|row| abi::Playlist::from_entity(row, vec![]))
        .collect(),
    )
  }

  pub async fn playlist(&self, id: PlaylistId) -> Result<abi::Playlist, MusyncError> {
    let queried = Playlist::find_by_id(id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::PlaylistNotFound(id))?;

    let tracks: Vec<_> = self.get_tracks_in_playlist(id).await?;

    Ok(abi::Playlist::from_entity(queried, tracks))
  }
}
