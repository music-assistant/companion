// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod discord_rpc;
use gethostname::gethostname;
use std::thread;
use tauri::api::process::Command;
use tauri::{utils::config::AppUrl, window::WindowBuilder, WindowUrl};

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
                "MusicAssistantDesktop",
                "-n",
                hostname
                    .to_str()
                    .expect("Couldnt convert hostname to &str -_-"),
            ])
            .spawn()
            .expect("Failed to start squeeselite");
    });
}

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");

    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
  
    tauri::Builder::default()
      .plugin(tauri_plugin_localhost::Builder::new(port).build())
      .invoke_handler(tauri::generate_handler![start_rpc, start_sqzlite])
      .setup(move |app| {
        WindowBuilder::new(
          app,
          "main".to_string(),
          if cfg!(dev) {
            Default::default()
          } else {
            window_url
          }
        )
        .title("Music Assistant")
        .build()?;
        Ok(())
      })
      .run(context)
      .expect("error while running tauri application");
}
