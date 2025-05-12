#[tauri::command]
pub fn get_os_info() -> (String, String) {
    let info = os_info::get();
    (info.os_type().to_string(), info.version().to_string())
}
