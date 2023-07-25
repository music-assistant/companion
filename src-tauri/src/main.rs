// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod discord_rpc;
use std::thread;
use subprocess::Exec;
use gethostname::gethostname;

#[tauri::command]
fn start_rpc(websocket: String) {
    // Start the discord rich presence manager in a new thread
    thread::spawn(|| discord_rpc::start_rpc(websocket)).join().expect("Couln't stop/start discord rpc");
}

#[tauri::command]
fn start_sqzlite(ip: String) {
    thread::spawn(move || {
        // Start squeezelite
        let hostname: std::ffi::OsString = gethostname();
        let command: String = format!("squeezelite -s {} -M MassDesktop -n {} -m aa:aa:aa:11:11:22 -U Master", ip.as_str(), hostname.to_str().expect("Couldnt convert hostname to &str -_-"));
        println!("Running: '{}' to start the squeezelite client. The command has to be in your path", command);
        Exec::shell(command).join().expect("Failed to start squeeselite");
    });
}

fn main() {
    // Build the tauri app
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_rpc,start_sqzlite])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
