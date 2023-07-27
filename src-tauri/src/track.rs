use crate::error::Result;
use id3::frame::{Lyrics, Picture};
use lofty::{Accessor, AudioFile, ItemKey, TaggedFileExt};
use serde::{
  ser::{SerializeSeq, SerializeStruct},
  Serialize,
};
use std::{borrow::Cow, path::PathBuf};

#[derive(Serialize, Default)]
pub struct Track {
  path: String,
  /// milli seconds
  duration: u64,
  artist: Option<String>,
  album: Option<String>,
  title: Option<String>,
  genre: Option<String>,
  year: Option<u32>,
}

#[tauri::command]
pub fn get_track_info(path: &PathBuf) -> Result<Track> {
  let probe = lofty::Probe::open(path)?;
  let mut song = Track::new(path);
  if let Ok(mut tagged_file) = probe.read() {
    let properties = tagged_file.properties();
    song.duration = properties.duration().as_millis() as u64;
    if let Some(tag) = tagged_file.primary_tag_mut() {
      // Check for a length tag (Ex. TLEN in ID3v2)
      if let Some(len_tag) = tag.get_string(&ItemKey::Length) {
        song.duration = len_tag.parse::<u64>()?;
      }
      song.artist = tag.artist().map(Cow::into_owned);
      song.album = tag.album().map(Cow::into_owned);
      song.title = tag.title().map(Cow::into_owned);
      song.genre = tag.genre().map(Cow::into_owned);
      song.year = tag.year();
    }
  }
  Ok(song)
}

impl Track {
  fn new(path: &PathBuf) -> Self {
    Track {
      path: path.to_str().unwrap().to_string(),
      ..Default::default()
    }
  }
}

struct SerializeLyrics<'a>(&'a Lyrics);

impl<'a> Serialize for SerializeLyrics<'a> {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut map = serializer.serialize_struct("Lyrics", 3)?;
    map.serialize_field("lang", &self.0.lang)?;
    map.serialize_field("description", &self.0.description)?;
    map.serialize_field("text", &self.0.text)?;
    map.end()
  }
}

fn serialize_lyrics<S>(lyrics: &Vec<Lyrics>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  let mut seq = serializer.serialize_seq(Some(lyrics.len()))?;
  for lyric in lyrics {
    seq.serialize_element(&SerializeLyrics(lyric))?;
  }
  seq.end()
}
