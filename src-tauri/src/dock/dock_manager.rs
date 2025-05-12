use std::sync::Arc;
use tauri::{PhysicalPosition, PhysicalSize, WebviewWindow};
use tokio::sync::Mutex;

pub struct DockManager {
    docked: Arc<Mutex<bool>>,
    threshold: i32,
}

impl DockManager {
    pub fn new() -> Self {
        Self {
            docked: Arc::new(Mutex::new(false)),
            threshold: 5, // * 도킹 감지 거리
        }
    }

    pub async fn handle_window_move(
        &self,
        position: PhysicalPosition<i32>,
        size: (u32, u32),
        screen_size: PhysicalSize<u32>,
        window: &WebviewWindow,
    ) {
        let mut docked = self.docked.lock().await;
        let mut new_position = None;

        let right_edge = screen_size.width as i32 - size.0 as i32;
        let bottom_edge = screen_size.height as i32 - size.1 as i32;

        let near_left = position.x <= self.threshold;
        let near_right = position.x >= right_edge - self.threshold;
        let near_top = position.y <= self.threshold;
        let near_bottom = position.y >= bottom_edge - self.threshold;

        if near_left && near_top && !*docked {
            // ? 왼쪽 상단
            new_position = Some(PhysicalPosition::new(0, 0));
        } else if near_right && near_top && !*docked {
            // ? 오른쪽 상단
            new_position = Some(PhysicalPosition::new(right_edge, 0));
        } else if near_left && near_bottom && !*docked {
            // ? 왼쪽 하단
            new_position = Some(PhysicalPosition::new(0, bottom_edge));
        } else if near_right && near_bottom && !*docked {
            // ? 오른쪽 하단
            new_position = Some(PhysicalPosition::new(right_edge, bottom_edge));
        } else if near_left && !*docked {
            // ? 왼쪽
            new_position = Some(PhysicalPosition::new(0, position.y));
        } else if near_right && !*docked {
            // ? 오른쪽
            new_position = Some(PhysicalPosition::new(right_edge, position.y));
        } else if near_top && !*docked {
            // ? 상단
            new_position = Some(PhysicalPosition::new(position.x, 0));
        } else if near_bottom && !*docked {
            // ? 하단
            new_position = Some(PhysicalPosition::new(position.x, bottom_edge));
        }

        if let Some(pos) = new_position {
            let current_position = window.outer_position().unwrap();

            // ? 현재 위치와 비교하여 변화가 있는 경우만 이동
            if (current_position.x - pos.x).abs() > 1 || (current_position.y - pos.y).abs() > 1 {
                *docked = true;
                let _ = window.set_position(pos);
            }
        } else {
            *docked = false;
        }
    }
}
