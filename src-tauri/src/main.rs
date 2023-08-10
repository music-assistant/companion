// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod discord_rpc;
use gethostname::gethostname;
use std::thread;
use tauri::api::process::Command;
use tauri::{utils::config::AppUrl, window::WindowBuilder, WindowUrl};
use std::sync::Once;

static DISCORD_RPC_STARTER: Once = Once::new();
static SQUEEZELITE_STARTER: Once = Once::new();

// Set the IS_WINDOWS constant to true if the target OS is windows
#[cfg(target_os = "windows")]
const IS_WINDOWS: bool = true;
#[cfg(not(target_os = "windows"))]
const IS_WINDOWS: bool = false;

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
                    "MusicAssistantDesktop",
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
    // Port to use for the local webserver (Windows only)
    let port: u16 = 22863;

    // Set the window url to the local webserver if the target OS is windows
    let window_url = if IS_WINDOWS {
        WindowUrl::External(format!("http://localhost:{}", port).parse().unwrap())
    } else {
        WindowUrl::App("index.html".into())
    };

    // Create the tauri context and builder
    let mut context = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    // If the target OS is windows, set the dist dir to the local webserver and add the localhost plugin
    if IS_WINDOWS {
        context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
        builder = builder.plugin(tauri_plugin_localhost::Builder::new(port).build());
    }

    // Run the tauri application
    builder
        .invoke_handler(tauri::generate_handler![start_rpc, start_sqzlite])
        .setup(move |app| {
            WindowBuilder::new(
                app,
                "main".to_string(),
                if cfg!(dev) {
                    Default::default()
                } else {
                    window_url
                },
            )
            .title("Music Assistant")
            .build()?;
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
