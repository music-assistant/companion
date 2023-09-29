// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod discord_rpc;
use gethostname::gethostname;
use std::thread;
use tauri::api::process::Command;
use std::sync::Once;

static DISCORD_RPC_STARTER: Once = Once::new();
static SQUEEZELITE_STARTER: Once = Once::new();

#[tauri::command]
fn start_rpc(websocket: String) {
    // To prevent it from starting multiple times even if frontend gets reloaded
    DISCORD_RPC_STARTER.call_once(|| {
        // Start the discord rich presence manager in a new thread
        thread::spawn(move || {
            let hostname: std::ffi::OsString = gethostname();
            discord_rpc::start_rpc(websocket, hostname);
        });
    });
}

#[tauri::command]
fn start_sqzlite(ip: String) {
    // To prevent it from starting multiple times even if frontend gets reloaded
    SQUEEZELITE_STARTER.call_once(|| {
        // Start squeezelite in a new thread
        thread::spawn(move || {
            let hostname: std::ffi::OsString = gethostname();
            Command::new_sidecar("squeezelite")
                .expect("Failed to create  command")
                .args([
                    "-s",
                    ip.as_str(),
                    "-M",
                    "Companion",
                    "-n",
                    hostname
                        .to_str()
                        .expect("Couldnt convert hostname to &str -_-"),
                ])
                .spawn()
                .expect("Failed to start squeeselite");
        });
    });
}

fn main() {
    // Create the tauri context, builder and handler
    let context = tauri::generate_context!();
    let builder = tauri::Builder::default();

    // Run the tauri application
    builder
        .invoke_handler(tauri::generate_handler![start_rpc, start_sqzlite])
        .run(context)
        .expect("error while running tauri application");
}
