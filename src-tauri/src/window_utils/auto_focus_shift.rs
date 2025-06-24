use crate::state::store::get_state;
use enigo::{Enigo, MouseControllable};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use std::{thread, time::Duration};
use tauri::WebviewWindow;
use windows::core::HSTRING;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetForegroundWindow, GetWindowTextW, IsIconic, IsWindowVisible,
    SetForegroundWindow,
};

/// ? 마우스 추적 활성화/비활성화 상태
static TRACKER_RUNNING: AtomicBool = AtomicBool::new(false);
/// ? 포커스 자동 이동 활성화/비활성화 상태
static AUTO_FOCUS_ENABLED: AtomicBool = AtomicBool::new(true);

#[tauri::command]
pub fn pause_auto_focus() {
    AUTO_FOCUS_ENABLED.store(false, Ordering::SeqCst);
    println!("🚫 포커스 자동 이동 기능 비활성화됨");
}

#[tauri::command]
pub fn resume_auto_focus() {
    AUTO_FOCUS_ENABLED.store(true, Ordering::SeqCst);
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
        match FindWindowW(None, &title_wide) {
            Ok(hwnd) => {
                let is_minimized = IsIconic(hwnd).as_bool();
                let is_visible = IsWindowVisible(hwnd).as_bool();

                if is_minimized || !is_visible {
                    return true;
                }
                false
            }
            Err(_) => {
                println!("❌ 창 '{}' 을(를) 찾을 수 없습니다", title);
                false
            }
        }
    }
}

/// + 포커스된 창 제목에 숙제표 설정이 포함되어 있는지 확인
fn is_settings_window_focused() -> bool {
    unsafe {
        // 현재 포커스된 창 핸들 가져오기
        let fg_hwnd = GetForegroundWindow();
        if fg_hwnd.0 == null_mut() {
            // 포커스된 창이 없으면 false 반환
            return false;
        }

        // 창 제목을 저장할 버퍼 (UTF-16)
        let mut buf = [0u16; 256];
        // 제목 길이 가져오기
        let length = GetWindowTextW(fg_hwnd, &mut buf);
        if length == 0 {
            // 제목을 읽을 수 없으면 false
            return false;
        }

        // UTF-16 -> Rust String 변환
        let window_title = String::from_utf16_lossy(&buf[..length as usize]);

        // 설정 창 제목과 비교
        window_title.contains("숙제표 설정")
    }
}

/// + 다른 창으로 포커스 변경
fn focus_other_window() {
    if !AUTO_FOCUS_ENABLED.load(Ordering::SeqCst) {
        println!("🔒 포커스 자동 이동이 비활성화되어 있어 실행하지 않음");
        return;
    }

    if is_window_hidden_or_minimized("Lost Ark Assist") {
        println!("⛔ 포커스 이동을 차단합니다 (최소화, 트레이 상태)");
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
        match FindWindowW(None, &game_title) {
            Ok(hwnd) => {
                if SetForegroundWindow(hwnd).as_bool() {
                    println!(
                        "✅ \x1b[32m'{}' 창으로 포커스 이동 완료\x1b[0m",
                        game_title.to_string_lossy()
                    );
                } else {
                    println!("⚠️ '{}' 창 포커스 이동 실패", game_title.to_string_lossy());
                }
            }
            Err(_err) => {
                println!(
                    "❌ '{}' 창을 찾을 수 없습니다",
                    game_title.to_string_lossy()
                );
            }
        }
    }
}

/// + 마우스 추적 시작 (전역 상태에서 `shift_idle_time` 사용)
pub fn start_mouse_tracking(window: WebviewWindow) {
    if TRACKER_RUNNING.load(Ordering::SeqCst) {
        return; // ? 이미 실행 중이면 중복 실행 방지
    }
    TRACKER_RUNNING.store(true, Ordering::SeqCst);

    thread::spawn(move || {
        let mut is_mouse_over = false;
        let mut last_out_time: Option<Instant> = None;

        while TRACKER_RUNNING.load(Ordering::SeqCst) {
            let inside = is_mouse_inside_window(&window);
            let settings_active = is_settings_window_focused(); // settings 창 확인

            if inside {
                if !is_mouse_over {
                    is_mouse_over = true;
                    // let _ = window.set_ignore_cursor_events(false); // ? 클릭 스루 비활성화

                    if !settings_active {
                        // ? 설정 창이 포커스 되어있지 않을 때만 포커스 이동
                        let _ = window.set_focus();
                        println!("▶ 메인 창에 포커스 이동");
                    } else {
                        println!(
                            "🔒 설정 창 활성화 중, 메인 창 \x1b[31m포커스 이동 즉시 차단\x1b[0m"
                        );
                    }
                }
                last_out_time = None; // ? 다시 창 안으로 들어오면 타이머 초기화
            } else {
                if is_mouse_over {
                    is_mouse_over = false;
                    // let _ = window.set_ignore_cursor_events(true); // ? 클릭 스루 활성화

                    if !settings_active {
                        // 설정 창 비활성화 상태면 대기 시작
                        last_out_time = Some(Instant::now()); // ? 창을 떠난 시간 기록
                        println!("◀ 창 밖으로 나감: 입력 차단 (대기 중)");
                    } else {
                        // 설정 창 활성화 중이면 즉시 포커스 이동 차단
                        println!("🔒 설정 창 활성화 중, \x1b[31m포커스 이동 즉시 차단\x1b[0m");
                        last_out_time = None; // 타이머 초기화
                    }
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
                        if !settings_active {
                            focus_other_window();
                        } else {
                            println!("🔒 설정 창 활성화 중이라 포커스 이동 차단");
                        }
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
    TRACKER_RUNNING.store(false, Ordering::SeqCst);
}
