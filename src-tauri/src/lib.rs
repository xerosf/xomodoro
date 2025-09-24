use window_vibrancy::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            apply_acrylic(&window, None)
                .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
