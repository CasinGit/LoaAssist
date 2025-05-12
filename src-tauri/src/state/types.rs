use serde::{Deserialize, Serialize};
use tauri::PhysicalPosition;

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

/// * 사용자 설정 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub update_check_enabled: bool,         // 업데이트 확인 여부
    pub theme: String,                      // 프로그램 테마
    pub class_image: bool,                  // 클래스 이미지 사용 여부
    pub folded_opacity_enabled: bool,       // 창 접었을 때 투명해지는지 여부
    pub folded_settings: FoldedSettings,    // 창 접었을 때 투명도, 대기시간
    pub auto_focus_enabled: bool,           // Auto Focus 기능 사용 여부
    pub auto_focus_settings: FocusSettings, // Auto Focus 기능 Settings
    pub focus_border_enabled: bool,         // 포커싱 테두리 기능 사용 여부
}

/// // 할 일(Task) 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

/// * AppState 구조체
#[derive(Serialize, Deserialize, Clone)]
pub struct AppState {
    pub gold: u32,                              // 골드 상태
    pub user_settings: UserSettings,            // 사용자 설정
    pub tasks: Vec<Task>,                       // 할 일(Task)
    pub window_position: PhysicalPosition<i32>, // 윈도우 위치 상태
}
