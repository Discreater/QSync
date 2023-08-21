use std::{borrow::Cow, fs};

use lofty::{Accessor, AudioFile, ItemKey, TaggedFileExt};

pub(crate) fn get_track_info(path: &str) -> abi::Track {
  let mut song = abi::Track {
    local_src: Some(abi::LocalSource {
      path: path.to_string(),
    }),
    ..Default::default()
  };
  if let Ok(probe) = lofty::Probe::open(path) {
    if let Ok(tagged_file) = probe.read() {
      let properties = tagged_file.properties();
      song.duration = Some(properties.duration().as_millis() as u32);
      if let Some(tag) = tagged_file.primary_tag() {
        // Check for a length tag (Ex. TLEN in ID3v2)
        if let Some(len_tag) = tag.get_string(&ItemKey::Length) {
          song.duration = len_tag.parse::<u32>().ok();
        }
        song.artist = tag.artist().map(Cow::into_owned);
        song.album = tag.album().map(Cow::into_owned);
        song.title = tag
          .title()
          .map(Cow::into_owned)
          .filter(|s| s.is_empty())
          .unwrap_or_else(|| {
            let filename = std::path::Path::new(path).file_name().unwrap();
            filename.to_str().unwrap().to_string()
          });
        song.genre = tag.genre().map(Cow::into_owned);
        song.year = tag.year();
      }
    }
  }
  song
}

pub(crate) fn get_track_pictures_internal(track: &abi::Track) -> Vec<lofty::Picture> {
  let mut pictures = vec![];
  if let Some(src) = track.local_src.as_ref() {
    let path = &src.path;
    if let Ok(probe) = lofty::Probe::open(path) {
      if let Ok(tagged_file) = probe.read() {
        if let Some(tag) = tagged_file.primary_tag() {
          pictures = tag.pictures().to_owned();
        }
      }
    }
  }
  pictures
}

pub(crate) fn get_track_pictures(track: &abi::Track) -> Vec<abi::Picture> {
  use base64::{engine::general_purpose, Engine as _};
  let pictures = get_track_pictures_internal(track);
  pictures
    .into_iter()
    .map(|p| {
      let data_base64: String = general_purpose::STANDARD_NO_PAD.encode(p.data());
      abi::Picture {
        pic_type: p.pic_type().as_ape_key().map(str::to_owned),
        mime_type: p.mime_type().to_string(),
        description: p.description().map(str::to_owned),
        data: data_base64,
      }
    })
    .collect()
}

fn add_track_from_folder(dir: &str, tracks: &mut Vec<abi::Track>) -> Option<()> {
  let entries = fs::read_dir(dir).ok()?;
  for entry in entries {
    let entry = entry.unwrap();
    let path = entry.path();
    let path_str = path.to_str().unwrap();
    if path.is_file() {
      tracks.push(get_track_info(path_str));
    } else if path.is_dir() {
      add_track_from_folder(path_str, tracks);
    }
  }
  Some(())
}

pub(crate) fn get_tracks_in_folder(dir: &str) -> Vec<abi::Track> {
  let mut tracks = vec![];
  add_track_from_folder(dir, &mut tracks);
  tracks
}
