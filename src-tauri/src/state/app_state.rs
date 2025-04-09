use dirs::data_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    sync::{Arc, OnceLock},
};
use tauri::{AppHandle, Manager, PhysicalPosition};
use tokio::sync::Mutex;

use crate::window_utils::auto_focus_shift;

static STATE: OnceLock<Arc<Mutex<AppState>>> = OnceLock::new();

// > Type Setting

/// * AppState 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct AppState {
    pub gold: u32,                              // 골드 상태
    pub user_settings: UserSettings,            // 사용자 설정
    pub tasks: Vec<Task>,                       // 할 일(Task)
    pub window_position: PhysicalPosition<i32>, // 윈도우 위치 상태
}

/// * 사용자 설정 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub theme: String,                      // 프로그램 테마
    pub class_image: bool,                  // 클래스 이미지 사용 여부
    pub folded_opacity_enabled: bool,       // 창 접었을 때 투명해지는지 여부
    pub folded_settings: FoldedSettings,    // 창 접었을 때 투명도, 대기시간
    pub auto_focus_enabled: bool,           // Auto Focus 기능 사용 여부
    pub auto_focus_settings: FocusSettings, // Auto Focus 기능 Settings
    pub focus_border_enabled: bool,         // 포커싱 테두리 기능 사용 여부
}

/// * Window Folded 설정 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct FoldedSettings {
    pub opacity: u8,
    pub idle_time: u16,
}

/// * Window Focus 설정 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct FocusSettings {
    pub game_title: String,
    pub shift_idle_time: u64,
}

/// // 할 일(Task) 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

// > Type Setting
// ////////////////////////////////////////////////////////////////////
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
            .join("app_state.json")
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
                if let Ok(state) = serde_json::from_str(&contents) {
                    return state;
                }
            }
        }
    }

    // ? 파일이 없거나 오류 발생 시 기본 상태 반환
    AppState {
        gold: 0,
        user_settings: UserSettings {
            theme: "light".to_string(),
            class_image: true,
            folded_opacity_enabled: true,
            folded_settings: FoldedSettings {
                opacity: 60,   // opacity * 0.01
                idle_time: 10, // sec
            },
            auto_focus_enabled: true, // 활성화
            auto_focus_settings: FocusSettings {
                game_title: "LOST ARK (64-bit, DX11) v.3.5.7.1".to_string(), // 창 이름
                shift_idle_time: 1,                                          // sec
            },
            focus_border_enabled: true,
        },
        tasks: vec![],
        window_position: PhysicalPosition { x: 100, y: 100 },
    }
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
    app_state.gold -= value;
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
pub async fn add_task(task: Task) -> Result<(), String> {
    let mut app_state = get_state().lock().await;
    app_state.tasks.push(task);
    save_state(&app_state);
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

// > Tauri Invoke Functions
