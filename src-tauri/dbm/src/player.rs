use abi::CreatePlayQueueRequest;
use chrono::Utc;
use entity::prelude::*;

use sea_orm::{
  ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set,
  TransactionTrait,
};
use tracing::trace;

use crate::{DbManager, MusyncError, UserId};

/// Player
impl DbManager {
  pub async fn stop_all(&self) -> Result<(), MusyncError> {
    trace!("stoping all");
    let updater = entity::play_queue::ActiveModel {
      playing: Set(false),
      paused_at: Set(0),
      ..Default::default()
    };
    PlayQueue::update_many().set(updater).exec(&self.db).await?;
    Ok(())
  }

  pub async fn create_play_queue(
    &self,
    create: CreatePlayQueueRequest,
    owner_id: UserId,
  ) -> Result<abi::PlayQueue, MusyncError> {
    let CreatePlayQueueRequest { track_ids } = create;
    let txn = self.db.begin().await?;

    let now = Utc::now();
    // insert play queue
    let inserted = entity::play_queue::ActiveModel {
      id: NotSet,
      playing: Set(false),
      position: Set(0),
      started_at: Set(now),
      paused_at: Set(0),
      created_at: Set(now),
      updated_at: Set(now),
    }
    .insert(&txn)
    .await?;

    // insert tracks
    let play_queue_tracks = track_ids.iter().enumerate().map(|(position, track_id)| {
      entity::play_queue_track::ActiveModel {
        play_queue_id: Set(inserted.id),
        track_id: Set(*track_id),
        position: Set(position as u32),
      }
    });
    PlayQueueTrack::insert_many(play_queue_tracks)
      .exec(&txn)
      .await?;

    // update user play queue id
    let user = User::find_by_id(owner_id)
      .one(&txn)
      .await?
      .ok_or(MusyncError::UserNotFound(owner_id))?;
    let old_queue_id = user.play_queue_id;
    let mut user: entity::user::ActiveModel = user.into();
    user.play_queue_id = Set(Some(inserted.id));
    user.update(&txn).await?;

    if let Some(old_queue_id) = old_queue_id {
      // delete old play queue
      PlayQueue::delete_by_id(old_queue_id).exec(&txn).await?;
    }

    txn.commit().await?;
    Ok(abi::PlayQueue::from_entity(inserted, track_ids))
  }

  pub async fn get_user_play_queue(
    &self,
    owner_id: UserId,
  ) -> Result<Option<abi::PlayQueue>, MusyncError> {
    let user = User::find_by_id(owner_id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::UserNotFound(owner_id))?;
    if user.play_queue_id.is_none() {
      return Ok(None);
    }
    let play_queue_id = user.play_queue_id.unwrap();
    let play_queue = PlayQueue::find_by_id(play_queue_id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::PlayQueueNotFound(play_queue_id))?;
    let play_queue_tracks = PlayQueueTrack::find()
      .filter(entity::play_queue_track::Column::PlayQueueId.eq(play_queue.id))
      .order_by_asc(entity::play_queue_track::Column::Position)
      .all(&self.db)
      .await?;
    let track_ids = play_queue_tracks
      .into_iter()
      .map(|t| t.track_id)
      .collect::<Vec<_>>();
    Ok(Some(abi::PlayQueue::from_entity(play_queue, track_ids)))
  }

  pub async fn update_player(
    &self,
    update: &abi::UpdatePlayerRequest,
    owner_id: UserId,
  ) -> Result<Option<abi::UpdatePlayerEvent>, MusyncError> {
    let user = User::find_by_id(owner_id)
      .one(&self.db)
      .await?
      .ok_or(MusyncError::UserNotFound(owner_id))?;
    if let Some(play_queue_id) = user.play_queue_id {
      let abi::UpdatePlayerRequest {
        manul: _manul,
        playing,
        position,
        progress,
      } = update;

      let play_queue = PlayQueue::find_by_id(play_queue_id)
        .one(&self.db)
        .await?
        .ok_or(MusyncError::PlayQueueNotFound(play_queue_id))?;
      let progress = progress.unwrap_or_else(|| {
        if play_queue.playing {
          (Utc::now() - play_queue.started_at).num_milliseconds() as u32
        } else {
          play_queue.paused_at
        }
      });

      let mut play_queue: entity::play_queue::ActiveModel = play_queue.into();
      if let Some(playing) = playing {
        play_queue.playing = Set(*playing);
      }
      if let Some(position) = position {
        play_queue.position = Set(*position);
      }
      if *play_queue.playing.as_ref() {
        play_queue.started_at = Set(Utc::now() - chrono::Duration::milliseconds(progress as i64));
      } else {
        play_queue.paused_at = Set(progress);
      }
      let updated = play_queue.update(&self.db).await?;
      Ok(Some(abi::UpdatePlayerEvent {
        playing: updated.playing,
        position: updated.position,
        progress,
      }))
    } else {
      Ok(None)
    }
  }
}
