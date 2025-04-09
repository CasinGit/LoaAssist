use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
};

/// + 창 제목을 가져오는 콜백 함수
unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let titles = &mut *(lparam.0 as *mut Vec<String>);

    // ? 창이 보이는 상태인지 확인
    if IsWindowVisible(hwnd).as_bool() {
        let length = GetWindowTextLengthW(hwnd) + 1;
        let mut buffer = vec![0u16; length as usize];

        // ? 창 제목 가져오기
        if GetWindowTextW(hwnd, &mut buffer) > 0 {
            let title = String::from_utf16_lossy(&buffer[..buffer.len() - 1]);
            if !title.is_empty() {
                titles.push(title);
            }
        }
    }

    BOOL(1) // ? 계속 탐색
}

/// + 모든 창 제목을 가져오는 함수
pub fn get_all_window_titles() -> Vec<String> {
    let mut titles: Vec<String> = Vec::new();

    unsafe {
        match EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut titles as *mut _ as isize),
        ) {
            Ok(_) => { /* 성공적으로 실행됨 */ }
            Err(e) => eprintln!("EnumWindows 호출 실패: {:?}", e),
        }
    }

    titles
}

/// + 특정 제목을 가진 창 찾기 (Tauri Command)
#[tauri::command]
pub fn find_window_by_title(target: String) -> Option<String> {
    let titles = get_all_window_titles();

    for title in titles {
        if title.contains(&target) {
            return Some(title);
        }
    }

    None
}

/// + 모든 창 제목을 가져오는 함수 (Tauri Command)
#[tauri::command]
pub fn get_window_titles() -> Vec<String> {
    get_all_window_titles()
}
