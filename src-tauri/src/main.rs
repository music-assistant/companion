// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod discord_rpc;
use gethostname::gethostname;
use std::sync::Once;
use std::thread;
use tauri::api::process::Command;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

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
fn get_output_devices() -> Vec<String> {
    // Get the output devices from squeezelite
    let squeezelite_response: String = Command::new_sidecar("squeezelite")
        .expect("Failed to create command")
        .args(["-l"])
        .output()
        .expect("Failed to get output devices").stdout;
    // Send the output devices to the frontend
    return squeezelite_response.lines()
        .filter(|line: &&str| !line.trim().is_empty() && !line.starts_with("Output devices:"))
        .map(|line: &str| line.split_whitespace().next().unwrap().to_owned())
        .collect();
}

#[tauri::command]
fn start_sqzlite(ip: String, output_device: String, port: String) {
    // To prevent it from starting multiple times even if frontend gets reloaded
    SQUEEZELITE_STARTER.call_once(|| {
        // Start squeezelite in a new thread
        thread::spawn(move || {
            let hostname: std::ffi::OsString = gethostname();
            let combined_hostname: String = hostname
                .to_str()
                .expect("Couldnt convert hostname to &str -_-")
                .to_owned()
                + ":"
                + port.as_str();
            Command::new_sidecar("squeezelite")
                .expect("Failed to create command")
                .args([
                    "-s",
                    ip.as_str(),
                    "-M",
                    "Companion",
                    "-n",
                    combined_hostname
                        .as_str(),
                    "-o",
                    output_device.as_str(),
                ])
                .spawn()
                .expect("Failed to start squeeselite");
        });
    });
}

fn main() {
    // Create the tray menu
    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
    let update: CustomMenuItem = CustomMenuItem::new("update".to_string(), "Check for updates");
    let hide: CustomMenuItem = CustomMenuItem::new("hide".to_string(), "Hide");
    let show: CustomMenuItem = CustomMenuItem::new("show".to_string(), "Show");
    let relaunch: CustomMenuItem = CustomMenuItem::new("relaunch".to_string(), "Relaunch");
    let tray_menu: SystemTrayMenu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(update)
        .add_item(relaunch)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    // Create the tray
    let tray: SystemTray = SystemTray::new().with_menu(tray_menu);

    // Create the tauri context, builder and handler
    let context = tauri::generate_context!();
    let builder = tauri::Builder::default();

    // Run the tauri application
    builder
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window: tauri::Window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window: tauri::Window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    app.exit(1);
                }
                "hide" => {
                    let window: tauri::Window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    let window: tauri::Window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "relaunch" => {
                    tauri::api::process::restart(&app.env());
                }
                "update" => {
                    app.updater();
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![start_rpc, start_sqzlite, get_output_devices])
        .system_tray(tray)
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            let window: tauri::Window = app.get_window("main").unwrap();
            window.show().unwrap();
            app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .run(context)
        .expect("error while running tauri application");
}
