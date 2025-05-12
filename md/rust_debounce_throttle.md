Rust에서 **debounce**와 **throttle** 기능을 구현하는 방법을 설명하겠습니다. 이 기능들은 비동기 환경에서 특히 유용하며, Rust에서는 `tokio`를 사용해 비동기 작업을 쉽게 처리할 수 있습니다.

### 1. Debounce 기능

**Debounce**는 특정 이벤트가 연속적으로 발생할 때 마지막 이벤트만 처리하는 기능입니다. 일반적으로 사용자 입력과 같은 이벤트에 사용됩니다. 예를 들어, 사용자가 연속적으로 키를 입력할 때 마지막 입력만 처리하고 싶을 때 사용됩니다.

#### Debounce 구현

아래는 `tokio`와 `async`를 사용하여 debounce 기능을 구현한 예제입니다:

```rust
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::task::JoinHandle;

// Debounce 함수: 특정 시간 동안 새로운 호출이 발생하지 않을 때만 마지막 함수를 호출
fn debounce<F>(delay_ms: u64, mut callback: F) -> Arc<Mutex<Option<JoinHandle<()>>>>
where
    F: FnMut() + Send + 'static,
{
    let handle = Arc::new(Mutex::new(None));

    let handle_clone = Arc::clone(&handle);
    Arc::new(Mutex::new(Some(tokio::spawn(async move {
        let mut last_call = Instant::now();

        loop {
            let now = Instant::now();
            if now.duration_since(last_call) >= Duration::from_millis(delay_ms) {
                if let Some(callback_fn) = handle_clone.lock().unwrap().take() {
                    callback_fn.await.unwrap();
                }
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }
    }))))
}
```

#### 사용 예제

위의 `debounce` 함수를 호출할 때마다 동일한 작업이 예약되지만, 지정된 지연 시간(`delay_ms`) 동안 새로운 호출이 없을 때만 마지막 콜백이 실행됩니다.

```rust
#[tokio::main]
async fn main() {
    let callback = || {
        println!("Debounced function executed!");
    };

    let debounced = debounce(300, callback);

    // 여러 번 호출되지만 300ms 동안 기다린 후에 마지막 것만 실행
    debounced();
    debounced();
    debounced();
    tokio::time::sleep(Duration::from_millis(500)).await;
}
```

### 2. Throttle 기능

**Throttle**는 특정 시간 간격 내에서 이벤트가 여러 번 발생하더라도 첫 번째 이벤트만 처리하는 기능입니다. 예를 들어, 사용자가 빠르게 스크롤할 때 일정 시간 간격마다 스크롤 위치를 업데이트할 때 사용할 수 있습니다.

#### Throttle 구현

`throttle`은 마지막 실행 시간을 추적하여 지정된 시간 동안에는 추가 실행을 방지합니다.

```rust
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use std::time::{Instant};

struct Throttler {
    last_call: Arc<Mutex<Instant>>,
    interval: Duration,
}

impl Throttler {
    // 새로운 Throttler 생성
    fn new(interval_ms: u64) -> Self {
        Throttler {
            last_call: Arc::new(Mutex::new(Instant::now() - Duration::from_millis(interval_ms))),
            interval: Duration::from_millis(interval_ms),
        }
    }

    // 특정 시간 간격 내에서만 함수를 실행
    fn throttle<F>(&self, mut callback: F)
    where
        F: FnMut() + Send + 'static,
    {
        let last_call = Arc::clone(&self.last_call);
        let interval = self.interval;

        tokio::spawn(async move {
            let mut last = last_call.lock().unwrap();
            if last.elapsed() >= interval {
                *last = Instant::now();
                callback();
            }
        });
    }
}
```

#### 사용 예제

위의 `throttle` 메소드는 지정된 시간 간격 동안에만 콜백을 호출합니다.

```rust
#[tokio::main]
async fn main() {
    let throttler = Throttler::new(500); // 500ms 간격

    let callback = || {
        println!("Throttled function executed!");
    };

    // 여러 번 호출되지만 500ms마다 한 번만 실행
    throttler.throttle(callback);
    tokio::time::sleep(Duration::from_millis(200)).await;
    throttler.throttle(callback);
    tokio::time::sleep(Duration::from_millis(600)).await;
    throttler.throttle(callback);
}
```

### 3. Debounce와 Throttle의 차이점

- **Debounce**: 마지막 이벤트가 발생한 후 일정 시간 동안 새로운 이벤트가 발생하지 않을 때만 실행합니다. 마지막 이벤트만 중요할 때 유용합니다.
- **Throttle**: 주어진 시간 간격 내에서 이벤트를 제한합니다. 첫 번째 이벤트 이후 지정된 시간 동안 추가 이벤트가 발생해도 무시합니다. 지속적으로 업데이트를 원할 때 유용합니다.

### 개선 가능성

- `debounce`와 `throttle` 구현에서 비동기 컨텍스트를 고려하여 이벤트 처리에 `async`를 사용할 수 있습니다.
- `Arc<Mutex<T>>`를 사용하여 상태를 공유하고 보호하지만, 필요에 따라 `RwLock` 또는 `Atomic` 타입을 사용할 수도 있습니다.
- 더 많은 유연성을 위해, 지연 시간 및 인터벌을 런타임에서 변경할 수 있는 기능을 추가할 수 있습니다.

이 코드들을 사용하여 Rust에서 고급 이벤트 관리와 비동기 작업 제어를 쉽게 할 수 있습니다.
