use super::types::{AppState, UserSettings};
use crate::window_utils::auto_focus_shift;
use dirs::data_dir;
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    sync::{Arc, OnceLock},
};
use tauri::{AppHandle, Manager, PhysicalPosition};
use tokio::sync::Mutex;

static STATE: OnceLock<Arc<Mutex<AppState>>> = OnceLock::new();

// > 상태 관리 함수

/// + 전역 상태 초기화
pub fn init_state() {
    STATE.set(Arc::new(Mutex::new(load_state()))).ok();
}

/// + 전역 상태 반환
pub fn get_state() -> &'static Arc<Mutex<AppState>> {
    STATE.get().expect("State is not initialized")
}

/// + 프로그램의 AppData 폴더 경로 가져오기
pub fn get_app_data_dir() -> Option<String> {
    let app_data_path = data_dir();
    app_data_path.map(|dir| {
        Path::new(&dir)
            .join("com.loaassist.app")
            .join("loaassist_state.json")
            .to_string_lossy()
            .to_string()
    })
}

/// + JSON 파일에서 상태 불러오기
pub fn load_state() -> AppState {
    if let Some(file_path) = get_app_data_dir() {
        if let Ok(mut file) = File::open(file_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(state) = serde_json::from_str::<AppState>(&contents) {
                    return state;
                }
            }
        }
    }

    // * 기본 상태 반환
    AppState::default()
}

// + 상태를 JSON 파일에 저장
pub fn save_state(state: &AppState) {
    if let Some(file_path) = get_app_data_dir() {
        if let Ok(json) = serde_json::to_string_pretty(state) {
            let _ = fs::write(file_path, json);
        }
    }
}

// > 상태 관리 함수
// ////////////////////////////////////////////////////////////////////
// > Tauri Invoke Functions

#[tauri::command]
pub async fn get_gold() -> Result<u32, String> {
    let app_state = get_state().lock().await;
    Ok(app_state.gold)
}

#[tauri::command]
pub async fn set_gold(value: u32) -> Result<u32, String> {
    let mut app_state = get_state().lock().await;
    app_state.gold = value;
    save_state(&app_state);
    Ok(app_state.gold)
}

#[tauri::command]
pub async fn increment_gold(value: u32) -> Result<u32, String> {
    let mut app_state = get_state().lock().await;
    app_state.gold += value;
    save_state(&app_state);
    Ok(app_state.gold)
}

#[tauri::command]
pub async fn decrement_gold(value: u32) -> Result<u32, String> {
    let mut app_state = get_state().lock().await;
    // ? underflow 에러 방지
    app_state.gold = app_state.gold.saturating_sub(value);
    save_state(&app_state);
    Ok(app_state.gold)
}

#[tauri::command]
pub async fn get_user_settings() -> Result<UserSettings, String> {
    let app_state = get_state().lock().await;
    Ok(app_state.user_settings.clone())
}

#[tauri::command]
pub async fn set_user_settings(app: AppHandle, settings: UserSettings) -> Result<(), String> {
    let mut app_state = get_state().lock().await;
    app_state.user_settings = settings.clone(); // ? `clone()`으로 복사
    save_state(&app_state);

    if settings.auto_focus_enabled {
        if let Some(window) = app.get_webview_window("main") {
            auto_focus_shift::start_mouse_tracking(window);
        }
    } else {
        auto_focus_shift::stop_mouse_tracking();
    }

    Ok(())
}

#[tauri::command]
pub async fn get_position() -> Result<PhysicalPosition<i32>, String> {
    let app_state = get_state().lock().await;
    Ok(app_state.window_position.clone())
}

#[tauri::command]
pub async fn set_position(new_position: PhysicalPosition<i32>) -> Result<(), String> {
    let mut app_state = get_state().lock().await;
    app_state.window_position = new_position;
    save_state(&app_state);
    Ok(())
}

#[tauri::command]
pub async fn get_default_tab() -> Result<String, String> {
    let app_state = get_state().lock().await;
    Ok(app_state.user_settings.default_tab.clone())
}

#[tauri::command]
pub async fn set_game_title(title: String) -> Result<(), String> {
    let mut app_state = get_state().lock().await;
    app_state.user_settings.auto_focus_settings.game_title = title;
    save_state(&app_state);
    Ok(())
}

// > Tauri Invoke Functions
