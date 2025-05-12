use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    App,
};

pub fn create_tray_menu<R: tauri::Runtime>(app: &App<R>) -> tauri::Result<Menu<R>> {
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let open = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let restore_size = MenuItem::with_id(app, "restoreSize", "Restore Size", true, None::<&str>)?;

    Menu::with_items(app, &[&open, &hide, &restore_size, &separator, &quit])
}
