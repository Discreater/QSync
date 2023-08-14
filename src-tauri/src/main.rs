#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use musync::Musyncer;
use tauri::Manager;
use tracing::{info, Level};

mod error;
mod track;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {name}!")
}

fn init_tracing() {
  tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .init();
}

fn main() {
  init_tracing();
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      greet,
      track::update_folder,
      track::get_track_pictures,
    ])
    .setup(|app| {
      let _data_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("failed to resolve app data dir");
      let db_file = _data_dir.join("db.sqlite");
      if !db_file.exists() {
        std::fs::File::create(&db_file)?;
      }
      let db_url = format!("sqlite://{}", db_file.to_string_lossy());
      info!("db_url: {}", db_url);
      let server = tauri::async_runtime::block_on(Musyncer::from_url(&db_url))?;
      app.manage(server);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Default)]
pub struct MusyncServer {
  pub manager: Option<Musyncer>,
}
