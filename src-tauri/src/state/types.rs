use serde::{Deserialize, Serialize};
use tauri::PhysicalPosition;

/// * AppState 메인 구조체
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppState {
    pub gold: u32,                              // 골드 상태
    pub user_settings: UserSettings,            // 사용자 설정
    pub window_position: PhysicalPosition<i32>, // 윈도우 위치 상태
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            gold: 0,
            user_settings: UserSettings::default(),
            window_position: PhysicalPosition { x: 100, y: 100 },
        }
    }
}

/// * 사용자 설정 구조체
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct UserSettings {
    pub update_check_enabled: bool,         // 업데이트 확인 여부
    pub theme: String,                      // 프로그램 테마
    pub class_image: bool,                  // 클래스 이미지 사용 여부
    pub folded_opacity_enabled: bool,       // 창 접었을 때 투명해지는지 여부
    pub folded_settings: FoldedSettings,    // 창 접었을 때 투명도, 대기시간
    pub auto_focus_enabled: bool,           // Auto Focus 기능 사용 여부
    pub auto_focus_settings: FocusSettings, // Auto Focus 기능 Settings
    pub focus_border_enabled: bool,         // 포커싱 테두리 기능 사용 여부
    pub default_tab: String,                // 실행 시 기본적으로 보일 탭
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            update_check_enabled: true,
            theme: "light".to_string(),
            class_image: true,
            folded_opacity_enabled: true,
            folded_settings: FoldedSettings::default(),
            auto_focus_enabled: true,
            auto_focus_settings: FocusSettings::default(),
            focus_border_enabled: true,
            default_tab: "Tab1".to_string(),
        }
    }
}

/// * Window Folded 설정 구조체
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct FoldedSettings {
    pub opacity: u8,    // opacity * 0.01
    pub idle_time: u16, // sec
}

impl Default for FoldedSettings {
    fn default() -> Self {
        Self {
            opacity: 60,
            idle_time: 10,
        }
    }
}

/// * Window Focus 설정 구조체
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct FocusSettings {
    pub game_title: String,   // 창 이름
    pub shift_idle_time: u64, // sec
}

impl Default for FocusSettings {
    fn default() -> Self {
        Self {
            game_title: "LOST ARK (64-bit, DX11) v.3.5.7.1".to_string(),
            shift_idle_time: 1,
        }
    }
}
