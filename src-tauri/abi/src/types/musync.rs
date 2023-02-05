use sqlx::{sqlite::SqliteRow, Row};

use crate::{convert_to_timestamp, LocalSource, Playlist, Track};

impl Playlist {
  pub fn new<SN, SD>(owner_id: i64, name: SN, description: SD, track_ids: Vec<i64>) -> Playlist
  where
    SN: Into<String>,
    SD: Into<String>,
  {
    Playlist {
      id: 0,
      owner_id,
      track_ids,
      name: name.into(),
      description: description.into(),
      created_at: None,
      updated_at: None,
    }
  }

  pub fn from_row(row: SqliteRow, track_ids: Vec<i64>) -> Result<Playlist, sqlx::Error> {
    Ok(Playlist {
      id: row.try_get("id")?,
      owner_id: row.try_get("owner_id")?,
      track_ids,
      name: row.try_get("name")?,
      description: row.try_get("description")?,
      created_at: Some(convert_to_timestamp(&row.try_get("created_at")?)),
      updated_at: Some(convert_to_timestamp(&row.try_get("updated_at")?)),
    })
  }
}

impl Track {
  pub fn new(
    name: String,
    artist: String,
    album: String,
    duration: i32,
    local_src: Option<LocalSource>,
  ) -> Track {
    Track {
      id: 0,
      name,
      artist,
      album,
      duration,
      local_src,
      netease_src: None,
    }
  }

  pub fn from_row(row: SqliteRow) -> Result<Track, sqlx::Error> {
    Ok(Track {
      id: row.try_get("id")?,
      name: row.try_get("name")?,
      artist: row.try_get("artist")?,
      album: row.try_get("album")?,
      duration: row.try_get("duration")?,
      local_src: row
        .try_get::<'_, Option<String>, _>("local_src_path")?
        .map(|path: String| crate::LocalSource { path }),
      netease_src: None,
    })
  }
}
