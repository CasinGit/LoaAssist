use tauri::{
    menu::{IconMenuItem, Menu, MenuItem, PredefinedMenuItem},
    App,
};

pub fn create_tray_menu<R: tauri::Runtime>(app: &App<R>) -> tauri::Result<Menu<R>> {
    let separator = PredefinedMenuItem::separator(app)?;

    let title_label = IconMenuItem::with_id(
        app,
        "open",
        "LoaAssist",
        true,
        Some(app.default_window_icon().cloned().unwrap()),
        None::<&str>,
    )?;

    let hide = MenuItem::with_id(app, "hide", "트레이 이동", true, None::<&str>)?;

    let restore_size =
        MenuItem::with_id(app, "restoreSize", "프로그램 위치 복원", true, None::<&str>)?;

    let quit = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;

    Menu::with_items(
        app,
        &[
            &title_label,
            &separator,
            &hide,
            &restore_size,
            &separator,
            &quit,
        ],
    )
}
