// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    api::process::{Command, CommandEvent},
    AppHandle, CustomMenuItem, Icon, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn chagen_system_tray_icon(app_handle: AppHandle) {
    let tray_handle = app_handle.tray_handle();
    // 设置新的系统托盘图标
    println!("dddd");

    if let Err(e) = tray_handle.set_icon(tauri::Icon::Raw(
        include_bytes!("../icons/creativity.png").to_vec(),
    )) {
        eprintln!("Failed to set new system tray icon: {:?}", e);
    }
}

#[tauri::command]
fn start_aria2c(app: AppHandle, window: tauri::Window, arg1: String) {
    let resource_path = app
        .path_resolver()
        .resolve_resource("resources/aria2.conf")
        .expect("failed to resolve resource");
    let resource_path = resource_path.to_str().unwrap();
    let (mut rx, mut child) = Command::new_sidecar("aria2c")
        .expect("failed to create `aria2c` binary command")
        .args([format!("--conf-path={resource_path}")])
        .spawn()
        .expect("Failed to spawn sidecar");

    println!("{}, {}", child.pid(), arg1);

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stderr(line) => {
                    window
                        .emit("message", Some(format!("'{}'", line)))
                        .expect("failed to emit event");
                    // write to stdin
                    child.write("message from Rust\n".as_bytes()).unwrap();
                }
                CommandEvent::Stdout(line) => {
                    window
                        .emit("message", Some(format!("'{}'", line)))
                        .expect("failed to emit event");
                    // write to stdin
                    child.write("message from Rust\n".as_bytes()).unwrap();
                }
                CommandEvent::Error(_) => todo!(),
                CommandEvent::Terminated(_) => todo!(),
                _ => todo!(),
            }
        }
    });
}

mod setup;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![
            start_aria2c,
            chagen_system_tray_icon
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
