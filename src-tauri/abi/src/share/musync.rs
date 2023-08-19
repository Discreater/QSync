use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use typeshare::typeshare;

#[typeshare]
pub type TrackId = i32;
#[typeshare]
pub type PlaylistId = i32;

/// Wrapper for DateTime<Utc> to serialize/deserialize as ISO8601
#[typeshare(serialized_as = "String")]
#[derive(Debug, Clone)]
pub struct DateTimeISO8601(pub DateTime<Utc>);

impl Serialize for DateTimeISO8601 {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.0.to_rfc3339())
  }
}
impl<'a> Deserialize<'a> for DateTimeISO8601 {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    let s = String::deserialize(deserializer)?;
    Ok(DateTimeISO8601(DateTime::<Utc>::from(
      DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?,
    )))
  }
}

#[typeshare]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Picture {
  pub pic_type: u8,
  pub mime_type: String,
  pub description: Option<String>,
  pub data: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalSource {
  pub path: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeteaseSource {
  pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Track {
  pub id: TrackId,
  pub title: String,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub duration: Option<u32>,
  pub genre: Option<String>,
  pub year: Option<u32>,
  pub pictures: Vec<Picture>,
  pub local_src: Option<LocalSource>,
  pub netease_src: Option<NeteaseSource>,
  pub created_at: Option<DateTimeISO8601>,
  pub updated_at: Option<DateTimeISO8601>,
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
      pictures: vec![],
      created_at: Some(DateTimeISO8601(row.created_at)),
      updated_at: Some(DateTimeISO8601(row.updated_at)),
      local_src: None,
      netease_src: None,
    }
  }
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateFolder {
  pub path: String,
}

#[typeshare::typeshare]
#[derive(Deserialize, Default, Debug)]
pub struct TrackQuery {
  pub playlist_id: Option<PlaylistId>,
  pub title: Option<String>,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub genre: Option<String>,
  pub year: Option<u32>,
}

#[cfg(test)]
mod tests {
  use chrono::Utc;

  #[test]
  fn serde_chrono() {
    let t = Utc::now();
    let t = serde_json::to_string(&t).unwrap();
    println!("{t}");
  }
}
