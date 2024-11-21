// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;
mod stream;

fn main() {
  let show = CustomMenuItem::new("show".to_string(), "打开窗口");
  let quit = CustomMenuItem::new("quit".to_string(), "退出");
  let tray_menu = SystemTrayMenu::new()
    .add_item(show)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let system_tray = SystemTray::new().with_menu(tray_menu);

  let mut app = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![stream::stream_fetch])
    .system_tray(system_tray)
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          "show" => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        event.window().hide().unwrap();
        api.prevent_close();
      }
      _ => {}
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

    app.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../icons/icon.png").to_vec())).unwrap();
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    app.run(|_app_handle, _event| {});
}
