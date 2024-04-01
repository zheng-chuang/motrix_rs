// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    api::process::{Command, CommandEvent},
    AppHandle, Manager,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
    tauri::Builder::default()
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![start_aria2c])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
