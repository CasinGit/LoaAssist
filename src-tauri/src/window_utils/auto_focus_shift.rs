use crate::state::store::get_state;
use enigo::{Enigo, MouseControllable};
use std::time::Instant;
use std::{sync::Mutex, thread, time::Duration};
use tauri::WebviewWindow;
use windows::core::HSTRING;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, IsIconic, IsWindowVisible, SetForegroundWindow,
};

/// ? 마우스 추적 활성화/비활성화 상태
static TRACKER_RUNNING: Mutex<bool> = Mutex::new(false);
/// ? 포커스 자동 이동 활성화/비활성화 상태
static AUTO_FOCUS_ENABLED: Mutex<bool> = Mutex::new(true);

#[tauri::command]
pub fn pause_auto_focus() {
    let mut enabled = AUTO_FOCUS_ENABLED.lock().unwrap();
    *enabled = false;
    println!("🚫 포커스 자동 이동 기능 비활성화됨");
}

#[tauri::command]
pub fn resume_auto_focus() {
    let mut enabled = AUTO_FOCUS_ENABLED.lock().unwrap();
    *enabled = true;
    println!("✅ 포커스 자동 이동 기능 활성화됨");
}

/// + 마우스 위치 가져오기
fn get_mouse_position() -> (i32, i32) {
    let enigo = Enigo::new();
    enigo.mouse_location()
}

/// + 마우스가 창 내부에 있는지 확인
fn is_mouse_inside_window(window: &WebviewWindow) -> bool {
    if let (Ok(win_pos), Ok(win_size)) = (window.outer_position(), window.outer_size()) {
        let (x, y) = get_mouse_position();
        return x >= win_pos.x
            && x <= win_pos.x + win_size.width as i32
            && y >= win_pos.y
            && y <= win_pos.y + win_size.height as i32;
    }
    false
}

/// + 윈도우가 최소화 또는 숨겨진 상태인지 확인
pub fn is_window_hidden_or_minimized(title: &str) -> bool {
    let title_wide = HSTRING::from(title);

    unsafe {
        let hwnd = FindWindowW(None, &title_wide);
        if hwnd.0 == 0 {
            println!("❌ 창 '{}' 을(를) 찾을 수 없습니다", title);
            return false;
        }

        let is_minimized = IsIconic(hwnd).as_bool();
        let is_visible = IsWindowVisible(hwnd).as_bool();

        if is_minimized || !is_visible {
            // * 창이 최소화 상태이거나 숨김 상태일 때
            return true;
        }

        false
    }
}

/// + 다른 창으로 포커스 변경
fn focus_other_window() {
    let enabled = AUTO_FOCUS_ENABLED.lock().unwrap();
    if !*enabled {
        println!("🔒 포커스 자동 이동이 비활성화되어 있어 실행하지 않음");
        return;
    }

    if is_window_hidden_or_minimized("Lost Ark Assist") {
        println!("⛔ 포커스 이동을 차단합니다");
        return;
    }

    let app_state = get_state().blocking_lock();
    let game_title: HSTRING = HSTRING::from(
        app_state
            .user_settings
            .auto_focus_settings
            .game_title
            .clone(),
    ); // ? 사용자 설정된 게임 제목 사용
    drop(app_state); // ? 상태 락 해제

    unsafe {
        let game_window = FindWindowW(None, &game_title);
        if game_window.0 != 0 {
            SetForegroundWindow(game_window);
            println!(
                "✅ '{}' 창으로 포커스 변경 완료!",
                game_title.to_string_lossy()
            );
        } else {
            println!("❌ '{}' 창을 찾을 수 없음!", game_title.to_string_lossy());
        }
    }
}

/// + 마우스 추적 시작 (전역 상태에서 `shift_idle_time` 사용)
pub fn start_mouse_tracking(window: WebviewWindow) {
    let mut running = TRACKER_RUNNING.lock().unwrap();
    if *running {
        return; // ? 이미 실행 중이면 중복 실행 방지
    }
    *running = true;

    thread::spawn(move || {
        let mut is_mouse_over = false;
        let mut last_out_time: Option<Instant> = None;

        while *TRACKER_RUNNING.lock().unwrap() {
            let inside = is_mouse_inside_window(&window);

            if inside {
                if !is_mouse_over {
                    is_mouse_over = true;
                    // let _ = window.set_ignore_cursor_events(false); // ? 클릭 스루 비활성화
                    let _ = window.set_focus();
                    println!("▶ 창 안에 있음: 입력 가능");
                }
                last_out_time = None; // ? 다시 창 안으로 들어오면 타이머 초기화
            } else {
                if is_mouse_over {
                    is_mouse_over = false;
                    // let _ = window.set_ignore_cursor_events(true); // ? 클릭 스루 활성화
                    last_out_time = Some(Instant::now()); // ? 창을 떠난 시간 기록
                    println!("◀ 창 밖으로 나감: 입력 차단 (대기 중)");
                } else if let Some(out_time) = last_out_time {
                    // ? 전역 상태에서 `shift_idle_time` 가져오기 (초 단위)
                    let shift_idle_time = {
                        let app_state = get_state().blocking_lock();
                        Duration::from_secs(
                            app_state.user_settings.auto_focus_settings.shift_idle_time,
                        ) // ? 초 단위 사용
                    };

                    // ? 설정된 대기시간이 지나면 포커스 변경
                    if out_time.elapsed() >= shift_idle_time {
                        focus_other_window();
                        last_out_time = None; // ? 포커스 변경 후 타이머 초기화
                    }
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    });
}

/// + 마우스 추적 중지
pub fn stop_mouse_tracking() {
    let mut running = TRACKER_RUNNING.lock().unwrap();
    *running = false;
}
