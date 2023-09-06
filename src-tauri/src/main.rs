// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use map::init::Server;
use tauri::{Manager, Window, App, async_runtime::{block_on, spawn_blocking}};
use tokio::spawn;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;
use crate::map::{np, init};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub mod map;

async fn np(window: &Window, server: &Server) {
    let data = server.get_data().await;
    window.emit("np", np::get_np(data).await).unwrap_or(());
}


#[tokio::main]
async fn main() {
    let mut server = init::Server::default();
    server.init().await.unwrap();
    tauri::Builder::default()
    .setup( | app | {
        let main_window = app.get_window("main").unwrap();
        tokio::spawn(async move {
            loop {
                np(&main_window, &server).await;
                sleep(Duration::from_millis(300)).await;
            }
        }
    );
        Ok(())
    }
        )
        .invoke_handler(tauri::generate_handler![zzz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn zzz() {

}