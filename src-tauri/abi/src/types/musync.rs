use chrono::Utc;

use crate::{
  convert_from_timestamp, convert_to_timestamp, CreatePlaylistRequest, LocalFolder, PlayQueue,
  Playlist, Track,
};

impl CreatePlaylistRequest {
  pub fn new<SN, SD>(name: SN, description: SD, track_ids: Vec<i32>) -> CreatePlaylistRequest
  where
    SN: Into<String>,
    SD: Into<String>,
  {
    CreatePlaylistRequest {
      track_ids,
      name: name.into(),
      description: description.into(),
    }
  }
}

impl PlayQueue {
  pub fn from_entity(row: entity::play_queue::Model, track_ids: Vec<i32>) -> PlayQueue {
    PlayQueue {
      id: row.id,
      track_ids,
      position: row.position,
      playing: row.playing,
      started_at: Some(convert_to_timestamp(&row.started_at)),
      paused_at: row.paused_at,
    }
  }

  pub fn progress(&self) -> u32 {
    let now = Utc::now();
    if self.playing {
      let started_at = self
        .started_at
        .as_ref()
        .and_then(convert_from_timestamp)
        .unwrap_or(now);
      let duration = now - started_at;
      duration.num_milliseconds() as u32
    } else {
      self.paused_at
    }
  }
}

impl Playlist {
  pub fn from_entity(row: entity::playlist::Model, track_ids: Vec<i32>) -> Playlist {
    Playlist {
      id: row.id,
      owner_id: row.owner_id,
      track_ids,
      name: row.name,
      description: row.description,
      created_at: Some(convert_to_timestamp(&row.created_at)),
      updated_at: Some(convert_to_timestamp(&row.updated_at)),
    }
  }
}

impl Track {
  pub fn from_entity(row: entity::track::Model) -> Track {
    Track {
      id: row.id,
      title: row.title,
      artist: row.artist,
      album: row.album,
      duration: row.duration,
      genre: row.genre,
      year: row.year,
      created_at: Some(convert_to_timestamp(&row.created_at)),
      updated_at: Some(convert_to_timestamp(&row.updated_at)),
      local_src: None,
      netease_src: None,
    }
  }
}

impl LocalFolder {
  pub fn from_entity(row: entity::local_src_folder::Model) -> LocalFolder {
    LocalFolder { path: row.path }
  }
}
