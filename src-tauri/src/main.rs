#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::error::Result;
use std::fs;
use track::{get_track_info, Track};

mod error;
mod player;
mod track;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {name}!")
}

fn main() {
  tauri::Builder::default()
    .manage(AppState::default())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      update_folder,
      greet,
      player::read_track,
      track::get_track_info,
    ])
    .setup(|app| {
      let _data_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("failed to resolve app data dir");
      todo!()
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Default)]
pub struct AppState {
  pub playback: Vec<Track>,
}

fn add_track(dir: String, tracks: &mut Vec<Track>) -> Result<()> {
  let entries = fs::read_dir(dir).unwrap();
  for entry in entries {
    let entry = entry.unwrap();
    let path = entry.path();
    let path_str = path.to_str().unwrap();
    if path.is_file() {
      tracks.push(get_track_info(path_str, false)?);
    } else if path.is_dir() {
      add_track(path_str.to_string(), tracks)?;
    }
  }
  Ok(())
}

#[tauri::command]
fn update_folder(dir: String) -> Result<Vec<Track>> {
  let mut tracks = vec![];
  add_track(dir, &mut tracks)?;
  Ok(tracks)
}
