// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use map::np::get_np;
use server::ws::Server;
use tauri::{Manager, Window};
use tokio::spawn;
use tokio::time::sleep;
use std::time::Duration;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub mod map;
pub mod server;

async fn np(window: &Window, server: &Server) {
    let data = server.get_struct().await;
    window.emit("np", get_np(data).await).unwrap_or(());
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut ws = Server::default();
    ws.init().await?;
    tauri::Builder::default()
    .setup(|app| {
        let main_window = app.get_window("main").unwrap();
        spawn(async move {
            loop {
                np(&main_window, &ws).await;
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
async fn zzz() {

}