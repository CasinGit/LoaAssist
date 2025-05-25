import { ask, message } from "@tauri-apps/plugin-dialog";

import { invoke } from "./invoke";

/**
 * 연이어 호출되는 함수들 중 마지막 함수(또는 제일 처음)만 호출하도록 하는 것
 * @param {(arg0: any) => void} callback
 * @param {number | undefined} delay
 */
export function debounce(callback: (arg0: any) => void, delay: number | undefined) {
    let timeout: number | undefined;
    return (...args: any) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => {
            callback(args);
        }, delay);
    };
}

/**
 * 일정 시간 동안 이벤트 핸들러를 한 번만 실행하도록 제어하는 것
 * @param {(arg0: any) => void} callback
 * @param {number | undefined} delay
 */
export function throttle(callback: (arg0: any) => void, delay: number | undefined) {
    let shouldWait = false;
    let waitingArgs: any[] | null = null;

    const timeoutFunc = () => {
        if (waitingArgs === null) {
            shouldWait = false;
        } else {
            callback(waitingArgs);
            waitingArgs = null;
            setTimeout(timeoutFunc, delay);
        }
    };

    return (...args: any) => {
        if (shouldWait) {
            waitingArgs = args;
            return;
        }

        callback(args);
        shouldWait = true;

        setTimeout(timeoutFunc, delay);
    };
}

export function generateUniqueId(): number {
    const now = new Date();

    // * Date 객체를 기반으로 yymmddHHMMSSfff 형식의 문자열 생성
    const year = now.getFullYear().toString().slice(-2); // ? 두 자리 연도
    const month = (now.getMonth() + 1).toString().padStart(2, "0"); // ? 월 (1~12)
    const day = now.getDate().toString().padStart(2, "0"); // ? 일
    const hours = now.getHours().toString().padStart(2, "0"); // ? 시
    const minutes = now.getMinutes().toString().padStart(2, "0"); // ? 분
    const seconds = now.getSeconds().toString().padStart(2, "0"); // ? 초
    const milliseconds = now.getMilliseconds().toString().padStart(3, "0"); // ? 밀리초

    return Number(`${year}${month}${day}${hours}${minutes}${seconds}${milliseconds}`);
}

export async function checkWindowsVersion(): Promise<"ok" | "unsupported"> {
    const [osType, version] = await invoke("get_os_info");
    console.log(osType, version);

    if (osType !== "Windows") {
        console.log("Unsupported OS");
        return "unsupported";
    }

    const parts = version.split(".");
    const build = parseInt(parts[2]);

    if (isNaN(build)) return "unsupported";

    // 예시: Windows 10 이상 허용
    return build >= 19041 ? "ok" : "unsupported";

    // 예시: Windows 11 이상만 허용
    // return build >= 22000 ? "ok" : "unsupported";
}

// + 프로그램 수동 업데이트 함수
export async function updateCheckDialog(noDialog?: boolean) {
    try {
        invoke("pause_auto_focus"); // ? 오토 포커스 기능 정지

        const result = await invoke("get_update_check_result");
        console.log(result);

        if (result.should_update) {
            const accepted = await ask(
                `새로운 버전(${result.latest_version})이 출시되었습니다.\n현재 버전: ${result.current_version}\n\n업데이트를 진행하지 않을 경우,\n- 새로 추가된 기능을 사용할 수 없으며\n- 변경된 기본 레이드 정보가 반영되지 않을 수 있습니다.\n\n지금 업데이트하시겠습니까?
                    `,
                { title: "업데이트 확인", kind: "info" }
            );

            if (accepted) {
                await invoke("run_update_with_info", { info: result.info });
                console.log("사용자가 업데이트 확정함");
            }
        } else {
            console.log("최신 상태입니다.");
            if (!noDialog) {
                await message(
                    `✔️ 최신 상태입니다.\n현재 버전: v${result.current_version}\n최신 버전: v${result.latest_version}`,
                    {
                        title: "업데이트 확인",
                        kind: "info"
                    }
                );
            }
        }
    } catch (err) {
        console.error("업데이트 확인 실패:", err);
    } finally {
        invoke("resume_auto_focus"); // ? 자동 포커스 다시 활성화
    }
}
