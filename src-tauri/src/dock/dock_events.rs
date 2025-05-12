// ? dock_manager는 같은 폴더에 있기 때문에 super(상대경로)로 처리
use super::dock_manager::DockManager;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{PhysicalPosition, WebviewWindow, WindowEvent};
use tokio::sync::Mutex;

static MOVING: AtomicBool = AtomicBool::new(false);

pub fn setup_window_events(window: &WebviewWindow, dock_manager: Arc<DockManager>) {
    let window_clone = window.clone();

    let screen_size = match window_clone.primary_monitor() {
        Ok(Some(monitor)) => Arc::new(Mutex::new(monitor.size().clone())),
        _ => return,
    };

    let screen_size_clone = Arc::clone(&screen_size);
    window.on_window_event(move |event| {
        if let WindowEvent::Moved(position) = *event {
            // ? 연속 호출 방지: true면 return
            if MOVING.swap(true, Ordering::Relaxed) {
                println!("연속 호출 방지");
                return;
            }

            let start = std::time::Instant::now();

            let dock_manager = Arc::clone(&dock_manager);
            let window_clone = window_clone.clone();
            let screen_size_clone = Arc::clone(&screen_size_clone);

            let position = PhysicalPosition::new(position.x, position.y);

            tauri::async_runtime::spawn(async move {
                let screen_size = *screen_size_clone.lock().await;
                let size = (
                    window_clone.outer_size().unwrap().width,
                    window_clone.outer_size().unwrap().height,
                );

                dock_manager
                    .handle_window_move(position, size, screen_size, &window_clone)
                    .await;

                // ? 창 이동이 완료된 후 다시 false로 변경 (지연 시간 없이)
                MOVING.store(false, Ordering::Relaxed);
            });

            let duration = start.elapsed();
            println!("Window was moved! Function execution took: {:?}", duration);
        }
    });
}
