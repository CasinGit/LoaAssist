import { getCurrentWindow } from "@tauri-apps/api/window";

import { invoke } from "./invoke";

const appWindow = getCurrentWindow();

// + Window Position 저장 함수
async function setWindowPosition() {
    const minimized = await appWindow.isMinimized(); // ? 창이 최소화 상태인지 확인
    if (minimized) return; // ? 창이 최소화 상태라면 실행하지 않음

    // ? 창의 현재 위치를 가져옵니다.
    const { x, y } = await appWindow.outerPosition();
    debounceInvoke(x, y);
}

let timeout: number | undefined;

// + Debounce Invoke 실행
const debounceInvoke = (x: number, y: number) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => {
        invoke("set_position", { newPosition: { x, y } }); // * Rust 단에서 position 위치 저장
        console.log("Window 포지션 설정됨");
    }, 200);
};

export default setWindowPosition;
