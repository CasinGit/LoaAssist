#[macro_use]
mod macros;

mod audio; // * Audio Mod
mod dock; // * Window Docking Mod
mod state; // * Program App State Mod
mod tray; // * Window Tray Mod
mod ui; // * Window Ui Mod
mod update; // * Program Update Mod
mod window_utils; // * 각종 Window Util Mod

use audio::audio_manager;
use dock::dock_events::setup_window_events;
use dock::dock_manager::DockManager;
use dotenv::dotenv;
use state::store::{get_app_data_dir, init_state, load_state};
use std::{env, sync::Arc};
use tauri::{Emitter, Listener, Manager, State, Window};
use tokio::sync::Mutex;
use ui::animations::{
    animate_window_resize, ease_in_out_back, ease_in_out_expo, ease_in_out_quad, ease_in_out_quart,
};
use update::{get_update_check_result, run_update_with_info};
use window_utils::{auto_focus_shift::start_mouse_tracking, window_os_info::get_os_info};

pub const WINDOW_LABEL: &str = "main";

#[tauri::command(rename_all = "snake_case")]
fn get_env(name: &str) -> String {
    let env_data: String = env::var(name).unwrap_or_else(|_| "not found".to_string());
    // format!("DOM_SECTION_CHECK: {}", section_check);

    return env_data;
}

// > 애니메이션을 위한 비동기 함수
struct LockState {
    lock: Mutex<()>,
}

#[tauri::command]
async fn resize_with_custom(
    window: Window,
    width: f64,
    height: f64,
    easing: String,
    state: State<'_, Arc<LockState>>,
) -> Result<String, String> {
    println!("Executing async locked function...");

    // ? 비동기 락을 얻음
    let _guard = state.lock.lock().await;

    println!("1: resize_with_custom Start!");
    let window = Arc::new(window);

    // ? 선택한 이징 함수에 따라 매핑
    let easing_function: fn(f64) -> f64 = match easing.as_str() {
        "easeInOutQuad" => ease_in_out_quad,
        "easeInOutQuart" => ease_in_out_quart,
        "easeInOutExpo" => ease_in_out_expo,
        "easeInOutBack" => ease_in_out_back,
        _ => ease_in_out_quad, // 기본값
    };

    // ? 비동기 작업을 tokio::spawn을 사용해 백그라운드에서 실행
    let window_clone = window.clone();
    let _ = tokio::spawn(async move {
        animate_window_resize(window_clone, width, height, 800, easing_function).await;
        println!("4: resize_with_custom End!");
    })
    .await;

    Ok("Async function executed".to_string())
}
// > 애니메이션을 위한 비동기 함수

#[tauri::command]
async fn play_system_sound(sound: Option<&str>) -> Result<(), String> {
    // ? audio 모듈 호출 시 옵션 처리
    audio_manager::play_system_sound(sound)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(dev) {
        println!("This is a development build.");
    } else {
        println!("This is a production build.");
    }

    // > Env Code Section
    // * Load environment variables from .env file
    dotenv().ok();

    // * Access variables
    let section_check: String =
        env::var("DOM_SECTION_CHECK").unwrap_or_else(|_| "DOM_SECTION_CHECK no set".to_string());
    println!("Rust Env: {section_check}");
    // > Env Code Section

    // > 전역 상태 관리
    // * Animation lock state
    let state: Arc<LockState> = Arc::new(LockState {
        lock: Mutex::new(()),
    });

    // * AppData dir path
    println!("{:?}", get_app_data_dir());

    // * 전역 상태 초기화 (앱 실행 전에 한 번만 실행)
    init_state();
    // > 전역 상태 관리

    tauri::Builder::default()
        .setup(|app| {
            // ? 트레이 생성 mod 불러오기
            tray::create_tray(app)?;

            let window = app.get_webview_window("main").unwrap();
            let window_for_once = window.clone(); // once용 clone
            let window_for_emit = window.clone(); // emit용 clone
            let dock_manager: Arc<DockManager> = Arc::new(DockManager::new());

            window_for_once.once("frontend-ready", move |_| {
                println!("✅ 프로그램 실행시 1회성 함수 실행 : window.once");
                let _ = window_for_emit.emit("on:app_start_once", ());
            });

            setup_window_events(&window, dock_manager);
            // window.set_ignore_cursor_events(true).unwrap(); // 기본적으로 클릭 스루 활성화
            start_mouse_tracking(window);

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window(WINDOW_LABEL)
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .manage(state)
        .manage(Arc::new(Mutex::new(load_state())))
        // ? Frontend 에서 Rust 함수 호출을 위해서 invoke handler에 등록
        .invoke_handler(tauri::generate_handler![
            get_env,                                           // Get ENV Fn
            resize_with_custom,                                // * Resize Window Fn
            play_system_sound,                                 // * Play System Sound Fn
            state::store::get_gold,                            // * Get Gold Fn
            state::store::set_gold,                            // * Set Gold Fn
            state::store::increment_gold,                      // * Increment Gold Fn
            state::store::decrement_gold,                      // * Decrement Gold Fn
            state::store::get_user_settings,                   // * Get User Settings Fn
            state::store::set_user_settings,                   // * Set User Settings Fn
            state::store::add_task,                            // // Add Task Fn
            state::store::get_position,                        // * Get Window Position
            state::store::set_position,                        // * Set Window Position
            window_utils::window_search::find_window_by_title, // * Find Process Title
            window_utils::window_search::get_window_titles,    // Find All Process Title
            window_utils::auto_focus_shift::pause_auto_focus,  // * Auto Focus Pause
            window_utils::auto_focus_shift::resume_auto_focus, // * Auto focus Resume
            get_os_info,                                       // * OS information
            get_update_check_result,                           // * Update Check
            run_update_with_info,                              // * Run Update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
