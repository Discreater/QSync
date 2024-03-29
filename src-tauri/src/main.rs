#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::net::SocketAddr;

use log::{info, LevelFilter};
use server::Server;
use tauri::{path::BaseDirectory, Manager, State};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind};

mod error;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {name}!")
}

fn main() {
  // init_tracing();
  // load envrionment
  // dotenvy::dotenv().expect(".env file not found");
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(
      tauri_plugin_log::Builder::default()
        .with_colors(ColoredLevelConfig::default())
        .level(LevelFilter::Warn)
        .level_for("qsync", LevelFilter::Trace)
        .level_for("server", LevelFilter::Trace)
        .level_for("dbm", LevelFilter::Trace)
        .level_for("abi", LevelFilter::Trace)
        .level_for("entity", LevelFilter::Trace)
        .level_for("webview", LevelFilter::Trace)
        .targets([
          Target::new(TargetKind::LogDir { file_name: None }),
          Target::new(TargetKind::Stdout),
          Target::new(TargetKind::Webview),
        ])
        .build(),
    )
    .invoke_handler(tauri::generate_handler![greet, get_server,])
    .setup(|app| {
      #[cfg(any(windows, target_os = "macos"))]
      {
        let window = app.get_window("main").unwrap();
        window_shadows::set_shadow(&window, true).expect("failed to set shadow");
      }
      let path_resolver = app.path();
      let resource_path = path_resolver
        .resolve("../.env", BaseDirectory::Resource)
        .expect("failed to resolve `.env`");
      dotenvy::from_filename(resource_path).expect(".env file resolve failed");
      let data_dir = path_resolver
        .app_data_dir()
        .expect("failed to resolve app data dir");
      let db_file: std::path::PathBuf = data_dir.join("db.sqlite");
      if !db_file.exists() {
        std::fs::File::create(&db_file)?;
      }
      let db_url = format!("sqlite://{}", db_file.to_string_lossy());
      info!("db_url: {}", db_url);
      let addr = SocketAddr::from(([0, 0, 0, 0], 8396));
      let server = tauri::async_runtime::block_on(Server::serve(&addr, &db_url, data_dir)).unwrap();
      app.manage(ServerState { server });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

struct ServerState {
  server: Server,
}

#[tauri::command]
async fn get_server(manager: State<'_, ServerState>) -> Result<String, String> {
  let addr = manager.server.handle.listening().await;
  if let Some(addr) = addr {
    Ok(addr.to_string())
  } else {
    Err(String::from("Server not binded"))
  }
}
