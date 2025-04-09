/**
 * ! animations.rs 모듈은
 * ! CSS 에서 구현할 수 있는 animation 이징 함수를 rust로 이식하고
 * ! 재사용 하기 위해 만들어진 모듈입니다.
 */
use std::sync::Arc;
use tauri::{PhysicalSize, Window};
use tokio::time::{interval, Duration};

#[allow(unused)]
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

#[allow(unused)]
pub fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// 이 함수는 애니메이션의 시작과 끝 부분은 천천히, 중간은 빠르게 진행됩니다.
///
/// 부드러운 가속과 감속이 필요한 애니메이션에 적합합니다.
///
/// ### 매개변수
/// - `t`: binary64
///
/// ### 반환 값
/// 이징값을 반환합니다.
///
/// ### 예시
///
/// ```
/// let result = ease_in_out_quad(t);
/// ```
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// 이 함수는 4차 함수로, `easeInOutQuad`보다 더 급격하게 변화하며,
///
/// 시작과 끝 부분은 매우 천천히, 중간은 매우 빠르게 변화합니다.
///
/// 매우 빠른 중간 가속이 필요할 때 사용합니다.
///
/// ### 매개변수
/// - `t`: binary64
///
/// ### 반환 값
/// 이징값을 반환합니다.
///
/// ### 예시
///
/// ```
/// let result = ease_in_out_quart(t);
/// ```
pub fn ease_in_out_quart(t: f64) -> f64 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

/// 이 함수는 지수 함수로, 극적인 변화를 원할 때 사용합니다.
///
/// 초반과 후반에 극단적으로 느리며, 중간에 급격히 가속됩니다.
///
/// 초반에 거의 변화가 없어 서서히 시작되길 원할 때 유용합니다.
///
/// ### 매개변수
/// - `t`: binary64
///
/// ### 반환 값
/// 이징값을 반환합니다.
///
/// ### 예시
///
/// ```
/// let result = ease_in_out_expo(t);
/// ```
pub fn ease_in_out_expo(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        (2.0_f64).powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0_f64 - (2.0_f64).powf(-20.0 * t + 10.0)) / 2.0
    }
}

/// 이 함수는 시작과 끝 부분에서 약간의 "반동(back)" 효과를 줍니다.
///
/// 목표 위치에 도달하기 전에 잠시 되돌아가는 반동 효과를 추가하고 싶을 때 사용합니다.
///
/// ### 매개변수
/// - `t`: binary64
///
/// ### 반환 값
/// 이징값을 반환합니다.
///
/// ### 예시
///
/// ```
/// let result = ease_in_out_back(t);
/// ```
pub fn ease_in_out_back(t: f64) -> f64 {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;

    if t < 0.5 {
        ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
    }
}

/// + 애니메이션 함수
pub async fn animate_window_resize(
    window: Arc<Window>,
    target_width: f64,
    target_height: f64,
    duration_ms: u64,
    easing_function: fn(f64) -> f64,
) {
    println!("2: animate_window_resize Start!");
    let current_size = window.inner_size().unwrap();
    let current_width = current_size.width as f64;
    let current_height = current_size.height as f64;

    let steps = 60;
    let step_duration = Duration::from_millis(duration_ms / steps);

    let width_change = target_width - current_width;
    let height_change = target_height - current_height;

    let mut interval = interval(step_duration);

    for i in 0..=steps {
        interval.tick().await;
        let t = i as f64 / steps as f64;
        let easing_value = easing_function(t);

        let new_width = current_width + width_change * easing_value;
        let new_height = current_height + height_change * easing_value;

        if window
            .set_size(PhysicalSize::new(new_width, new_height))
            .is_err()
        {
            break;
        }
    }
    println!("3: animate_window_resize End!");
}
