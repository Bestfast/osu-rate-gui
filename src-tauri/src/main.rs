// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use spdlog::prelude::*;
use map::np::get_np;
use server::ws::Server;
use std::time::Duration;
use tauri::{Manager, Window};
use tokio::spawn;
use tokio::time::sleep;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub mod map;
pub mod server;

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
    let mut ws = Server::default();
    info!["Starting init..."];
    ws.init().await?;
    tauri::Builder::default()
        .setup(|app| {
            info!("Starting main window");
            let main_window = app.get_window("main").unwrap();
            spawn(async move {
                loop {
                    poll(&main_window, &ws).await;
                    sleep(Duration::from_millis(120)).await;
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![zzz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

#[tauri::command]
async fn zzz() {}
