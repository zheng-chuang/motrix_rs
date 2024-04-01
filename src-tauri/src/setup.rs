use tauri::{
    api::process::{Command, CommandEvent},
    App, Manager,
};
use window_vibrancy::NSVisualEffectMaterial;
/// setup
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    set_blur_material(app);
    // start_aria2c(app);
    Ok(())
}

fn set_blur_material(app: &mut App) {
    let window = app.get_window("main").unwrap();
    // 仅在 macOS 下执行
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // 仅在 windows 下执行
    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
}

pub fn start_aria2c(app: &mut App) {
    let window = app.get_window("main").unwrap();
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

    println!("{}", child.pid());

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
