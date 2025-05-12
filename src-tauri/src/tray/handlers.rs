use crate::WINDOW_LABEL;
use tauri::{
    menu::MenuEvent,
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent},
    AppHandle, LogicalPosition, LogicalSize, Manager, Wry,
};

pub fn handle_menu_event(app: &AppHandle<Wry>, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => app.exit(0),
        "open" => show_main_window(app),
        "hide" => hide_main_window(app),
        "restoreSize" => restore_main_window_size(app),
        _ => println!("Unhandled menu event: {:?}", event.id),
    }
}

pub fn handle_tray_event(tray: &TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        println!("Tray icon left-clicked");
        show_main_window(&tray.app_handle());
    }
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
        let _ = window.set_ignore_cursor_events(false);
    }
}

fn hide_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        let _ = window.hide();
    }
}

fn restore_main_window_size(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
        let _ = window.set_ignore_cursor_events(false);
        let _ = window.set_size(LogicalSize::new(320.0, 500.0));
        let _ = window.set_position(LogicalPosition::new(0.0, 0.0));
    }
}
