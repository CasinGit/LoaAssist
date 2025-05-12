use crate::tray::{create_tray_menu, handle_menu_event, handle_tray_event};
use tauri::{tray::TrayIconBuilder, App};

pub fn create_tray(app: &mut App) -> tauri::Result<()> {
    let tray_menu = create_tray_menu(app)?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .title("LostArk Assist")
        .tooltip("LostArk Assist")
        .menu(&tray_menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| handle_menu_event(app, event))
        .on_tray_icon_event(handle_tray_event)
        .build(app)?;

    Ok(())
}
