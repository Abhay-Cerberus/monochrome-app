pub mod tray;

use serde::{Deserialize, Serialize};
use tauri::{Manager, WindowEvent};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri_plugin_store::StoreExt;
use serde_json::json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
    pub is_playing: bool,
    pub title: String,
    pub artist: String,
    pub volume: f32,
    pub is_muted: bool,
}

#[tauri::command]
fn set_playback_state(app: tauri::AppHandle, state: PlaybackState) {
    if let Some(tray) = app.tray_by_id("main-tray") {
        let play_text = if state.is_playing { "Pause" } else { "Play" };
        let mute_text = if state.is_muted { "Unmute" } else { "Mute" };
        
        let info_text = if state.title.is_empty() {
            "Monochrome".to_string()
        } else if state.artist.is_empty() {
            state.title.clone()
        } else {
            format!("{} - {}", state.artist, state.title)
        };

        if let Ok(play_pause_i) = MenuItem::with_id(&app, "play_pause", play_text, true, None::<&str>) {
        if let Ok(prev_i) = MenuItem::with_id(&app, "prev", "Previous", true, None::<&str>) {
        if let Ok(next_i) = MenuItem::with_id(&app, "next", "Next", true, None::<&str>) {
        if let Ok(mute_unmute_i) = MenuItem::with_id(&app, "mute_unmute", mute_text, true, None::<&str>) {
        if let Ok(vol_up_i) = MenuItem::with_id(&app, "vol_up", "Volume Up", true, None::<&str>) {
        if let Ok(vol_down_i) = MenuItem::with_id(&app, "vol_down", "Volume Down", true, None::<&str>) {
        if let Ok(track_info_i) = MenuItem::with_id(&app, "track_info", &info_text, false, None::<&str>) {
        if let Ok(show_i) = MenuItem::with_id(&app, "show", "Show Monochrome", true, None::<&str>) {
        if let Ok(quit_i) = MenuItem::with_id(&app, "quit", "Quit", true, None::<&str>) {
        if let Ok(sep1) = PredefinedMenuItem::separator(&app) {
        if let Ok(sep2) = PredefinedMenuItem::separator(&app) {
        if let Ok(sep3) = PredefinedMenuItem::separator(&app) {
            
            if let Ok(menu) = Menu::with_items(
                &app,
                &[
                    &play_pause_i,
                    &prev_i,
                    &next_i,
                    &sep1,
                    &mute_unmute_i,
                    &vol_up_i,
                    &vol_down_i,
                    &sep2,
                    &track_info_i,
                    &sep3,
                    &show_i,
                    &quit_i,
                ],
            ) {
                let _ = tray.set_menu(Some(menu));
                let _ = tray.set_tooltip(Some(info_text));
            }
        }}}}}}}}}}}}
    }
}

#[tauri::command]
fn get_close_to_tray_setting(app: tauri::AppHandle) -> bool {
    let store = app.store("settings.json").unwrap();
    store.get("closeToTray").and_then(|v| v.as_bool()).unwrap_or(true)
}

#[tauri::command]
fn set_close_to_tray_setting(app: tauri::AppHandle, enabled: bool) {
    let store = app.store("settings.json").unwrap();
    store.set("closeToTray", json!(enabled));
    store.save().unwrap_or(());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            tray::create_tray(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();
                let store = app.store("settings.json").unwrap();
                let close_to_tray = store.get("closeToTray").and_then(|v| v.as_bool()).unwrap_or(true);
                
                if close_to_tray {
                    window.hide().unwrap();
                    api.prevent_close();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            set_playback_state,
            get_close_to_tray_setting,
            set_close_to_tray_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
