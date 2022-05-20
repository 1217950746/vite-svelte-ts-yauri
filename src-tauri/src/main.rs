#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::App;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| Ok(setup(app)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &App) {
    let window = app.get_window("main").unwrap();
    #[cfg(target_os = "windows")]
    {
        use window_shadows::set_shadow;
        let _ = set_shadow(&window, true);
    }

    #[cfg(target_os = "macos")]
    {
       let _ = window.set_decorations(true);
    }
}
