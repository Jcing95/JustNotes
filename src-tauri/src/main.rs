// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{Duration, SystemTime};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn format(name: &str) -> String {

    let current_time = SystemTime::now();
    let since_the_epoch = current_time.duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");

    let hours = (since_the_epoch.as_secs() / 3600) % 24;
    let minutes = (since_the_epoch.as_secs() / 60) % 60;
    let seconds = since_the_epoch.as_secs() % 60;

    let formatted_time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    format!("{}: [{}]", formatted_time, name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![format])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
