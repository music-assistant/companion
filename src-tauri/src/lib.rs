// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod discord_rpc;
use gethostname::gethostname;
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::Manager;
use tauri_plugin_shell::ShellExt;
use std::sync::Once;
use std::thread;
use tauri_plugin_updater::UpdaterExt;

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
async fn get_output_devices(app: tauri::AppHandle) -> Vec<String> {
    // Get the output devices from squeezelite
    let squeezelite_response: tauri_plugin_shell::process::Output = app.shell().sidecar("squeezelite")
        .expect("Failed to create command")
        .args(["-l"])
        .output()
        .await
        .expect("Failed to get output devices");
    let stdout = String::from_utf8_lossy(&squeezelite_response.stdout);
    // Send the output devices to the frontend
    return stdout
        .lines()
        .filter(|line: &&str| !line.trim().is_empty() && !line.starts_with("Output devices:"))
        .map(|line: &str| line.split_whitespace().next().unwrap().to_owned())
        .collect();
}

#[tauri::command]
fn start_sqzlite(app: tauri::AppHandle, ip: String, output_device: String, port: String) {
    // To prevent it from starting multiple times even if frontend gets reloaded
    SQUEEZELITE_STARTER.call_once(|| {
        // Start squeezelite in a new thread
        thread::spawn(move || {
            let hostname: std::ffi::OsString = gethostname();
            let combined_ip: String = ip.as_str().to_owned() + ":" + port.as_str();
            println!(
                "Starting squeezelite with ip: {}, output device: {}, port: {}",
                ip, output_device, port
            );
            app.shell().sidecar("squeezelite")
                .expect("Failed to create command")
                .args([
                    "-s",
                    combined_ip.as_str(),
                    "-M",
                    "Companion",
                    "-n",
                    hostname
                        .to_str()
                        .expect("Couldnt convert hostname to &str -_-"),
                    "-o",
                    output_device.as_str(),
                ])
                .spawn()
                .expect("Failed to start squeeselite");
        });
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create the tauri context, builder and handler
    let context = tauri::generate_context!();
    let _builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            start_rpc,
            start_sqzlite,
            get_output_devices
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();
            app.emit("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let _response = handle.updater().unwrap().check().await;
            });

            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let update = MenuItemBuilder::with_id("update", "Check for updates").build(app)?;
            let hide = MenuItemBuilder::with_id("hide", "Hide").build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let relaunch = MenuItemBuilder::with_id("relaunch", "Relaunch").build(app)?;
            let seperator = PredefinedMenuItem::separator(app)?;
            let menu = MenuBuilder::new(app).items(&[&hide, &show, &seperator, &update, &relaunch, &seperator, &quit]).build()?;
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Tauri")
                .icon(app.default_window_icon().unwrap().clone())
                .menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(1);
                    },
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.hide().unwrap();
                    },
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                    },
                    "relaunch" => {
                        tauri::process::restart(&app.env());
                    },
                    "update" => {
                        let handle = app.app_handle().clone();
                        tauri::async_runtime::spawn(async move {
                            let _response = handle.updater().unwrap().check().await;
                        });
                    },
                    _ => (),
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                      button: MouseButton::Left,
                      button_state: MouseButtonState::Up,
                      ..
                    } = event
                    {
                      let app = tray.app_handle();
                      if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                      }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
