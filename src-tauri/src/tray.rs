use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::App<R>) -> tauri::Result<()> {
    let play_pause_i = MenuItem::with_id(app, "play_pause", "Play/Pause", true, None::<&str>)?;
    let prev_i = MenuItem::with_id(app, "prev", "Previous", true, None::<&str>)?;
    let next_i = MenuItem::with_id(app, "next", "Next", true, None::<&str>)?;
    let mute_unmute_i = MenuItem::with_id(app, "mute_unmute", "Mute/Unmute", true, None::<&str>)?;
    let vol_up_i = MenuItem::with_id(app, "vol_up", "Volume Up", true, None::<&str>)?;
    let vol_down_i = MenuItem::with_id(app, "vol_down", "Volume Down", true, None::<&str>)?;
    let track_info_i = MenuItem::with_id(app, "track_info", "Monochrome", false, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show Monochrome", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &play_pause_i,
            &prev_i,
            &next_i,
            &PredefinedMenuItem::separator(app)?,
            &mute_unmute_i,
            &vol_up_i,
            &vol_down_i,
            &PredefinedMenuItem::separator(app)?,
            &track_info_i,
            &PredefinedMenuItem::separator(app)?,
            &show_i,
            &quit_i,
        ],
    )?;

    let _tray = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .tooltip("Monochrome")
        .icon(app.default_window_icon().unwrap().clone())
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "play_pause" => { app.emit("tray-play-pause", ()).unwrap_or(()); }
                "prev" => { app.emit("tray-prev", ()).unwrap_or(()); }
                "next" => { app.emit("tray-next", ()).unwrap_or(()); }
                "mute_unmute" => { app.emit("tray-mute", ()).unwrap_or(()); }
                "vol_up" => { app.emit("tray-volume-up", ()).unwrap_or(()); }
                "vol_down" => { app.emit("tray-volume-down", ()).unwrap_or(()); }
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
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
}
