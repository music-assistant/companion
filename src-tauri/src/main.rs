// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod discord_rpc;
use gethostname::gethostname;
use std::thread;
use tauri::api::process::Command;

#[tauri::command]
fn start_rpc(websocket: String) {
    // Start the discord rich presence manager in a new thread
    thread::spawn(move || {
        let hostname: std::ffi::OsString = gethostname();
        discord_rpc::start_rpc(websocket, hostname);
    });
}

#[tauri::command]
fn start_sqzlite(ip: String) {
    thread::spawn(move || {
        // Start squeezelite
        let hostname: std::ffi::OsString = gethostname();
        Command::new_sidecar("squeezelite")
            .expect("Failed to create  command")
            .args([
                "-s",
                ip.as_str(),
                "-M",
                "MassDesktop",
                "-n",
                hostname
                    .to_str()
                    .expect("Couldnt convert hostname to &str -_-"),
                "-U",
                "Master",
            ])
            .spawn()
            .expect("Failed to start squeeselite");
    });
}

fn main() {
    // Build the tauri app
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_rpc, start_sqzlite])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
