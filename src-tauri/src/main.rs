// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use spdlog::prelude::*;
use server::ws::Server;
use tauri::{Manager, Window, State};
use tokio::spawn;
use tokio::sync::watch;
use std::sync::Arc;
use tokio::sync::RwLock;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub mod map;
pub mod server;


#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}


async fn poll(window: &Window, server: &Server) {
    let data = server.get_struct().await;
    match window.emit("poll", data.menu.bm.metadata) {
        Ok(_) => trace!("Emitted poll successfully"),
        Err(e) => warn!("Something unexpected happened while emitting poll: {}", e),
    }
}

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    spdlog::default_logger().set_level_filter(LevelFilter::MoreSevereEqual(Level::Info));
    info!["Starting init..."];
    tauri::Builder::default()
        .manage(Arc::new(RwLock::new(Server::default())))
        .setup(|app| {
            info!("Starting main window");
            let main_window = app.get_window("main").unwrap();
            let state: State<'_, Arc<RwLock<Server>>> = app.state();
            let (tx, mut rx) = watch::channel(());
            let server = Arc::clone(&state);
            spawn(async move {
                server.write().await.init(tx).await.unwrap();
                loop {
                    while rx.changed().await.is_ok() {
                        poll(&main_window, &*server.read().await).await;
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![msd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

// placeholder function
#[tauri::command]
async fn msd(ws: State<'_, Arc<RwLock<Server>>>) -> Result<(), Error> {
    trace!("Requesting msd");
    let tmp = Arc::clone(&ws); 
    let data = tmp.read().await.get_struct().await;
    println!("data: {:?}", data);
    Ok(())
}
