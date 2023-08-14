use crate::error::Result;
use lofty::{Accessor, AudioFile, ItemKey, Picture, TaggedFileExt};
use musync::{Musync, Musyncer};
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, fs};
use tauri::State;

pub fn get_track_info(path: &str) -> Result<abi::Track> {
  let probe = lofty::Probe::open(path)?;
  let mut song = abi::Track {
    local_src: Some(abi::LocalSource {
      path: path.to_string(),
    }),
    ..Default::default()
  };
  if let Ok(mut tagged_file) = probe.read() {
    let properties = tagged_file.properties();
    song.duration = Some(properties.duration().as_millis() as u32);
    if let Some(tag) = tagged_file.primary_tag_mut() {
      // Check for a length tag (Ex. TLEN in ID3v2)
      if let Some(len_tag) = tag.get_string(&ItemKey::Length) {
        song.duration = Some(len_tag.parse::<u32>()?);
      }
      song.artist = tag.artist().map(Cow::into_owned);
      song.album = tag.album().map(Cow::into_owned);
      song.title = tag.title().map(Cow::into_owned).unwrap_or_else(|| {
        let filename = std::path::Path::new(path).file_name().unwrap();
        filename.to_str().unwrap().to_string()
      });
      song.genre = tag.genre().map(Cow::into_owned);
      song.year = tag.year();

      // if full {
      //   song.pictures = tag.pictures().to_vec();
      // }
    }
  }
  Ok(song)
}

fn add_track(dir: &str, tracks: &mut Vec<abi::Track>) -> Result<()> {
  let entries = fs::read_dir(dir).unwrap();
  for entry in entries {
    let entry = entry.unwrap();
    let path = entry.path();
    let path_str = path.to_str().unwrap();
    if path.is_file() {
      tracks.push(get_track_info(path_str)?);
    } else if path.is_dir() {
      add_track(path_str, tracks)?;
    }
  }
  Ok(())
}

#[tauri::command]
pub async fn update_folder(dir: String, manager: State<'_, Musyncer>) -> Result<()> {
  let mut tracks = vec![];
  add_track(&dir, &mut tracks)?;
  manager.create_tracks(tracks, &dir).await?;
  Ok(())
}

#[tauri::command]
pub async fn get_track_pictures(
  id: i32,
  manager: State<'_, Musyncer>,
) -> Result<Vec<PictureSerializer>> {
  let track = manager.track(id).await?;
  let path = track.local_src.as_ref().unwrap().path.clone();
  let probe = lofty::Probe::open(path)?;
  let mut pictures = vec![];
  if let Ok(mut tagged_file) = probe.read() {
    if let Some(tag) = tagged_file.primary_tag_mut() {
      pictures = tag.pictures().to_vec();
    }
  }
  Ok(pictures.into_iter().map(PictureSerializer).collect())
}

pub struct PictureSerializer(Picture);

impl Serialize for PictureSerializer {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut map = serializer.serialize_struct("Picture", 4)?;
    map.serialize_field("mime_type", &self.0.mime_type().to_string())?;
    map.serialize_field("picture_type", &self.0.pic_type().as_u8())?;
    map.serialize_field("description", &self.0.description())?;
    use base64::{engine::general_purpose, Engine as _};
    let data_base64: String = general_purpose::STANDARD_NO_PAD.encode(self.0.data());
    map.serialize_field("data", &data_base64)?;
    map.end()
  }
}

// struct SerializeLyrics<'a>(&'a Lyrics);

// impl<'a> Serialize for SerializeLyrics<'a> {
//   fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//   where
//     S: serde::Serializer,
//   {
//     let mut map = serializer.serialize_struct("Lyrics", 3)?;
//     map.serialize_field("lang", &self.0.lang)?;
//     map.serialize_field("description", &self.0.description)?;
//     map.serialize_field("text", &self.0.text)?;
//     map.end()
//   }
// }

// fn serialize_lyrics<S>(lyrics: &Vec<Lyrics>, serializer: S) -> Result<S::Ok, S::Error>
// where
//   S: serde::Serializer,
// {
//   let mut seq = serializer.serialize_seq(Some(lyrics.len()))?;
//   for lyric in lyrics {
//     seq.serialize_element(&SerializeLyrics(lyric))?;
//   }
//   seq.end()
// }
