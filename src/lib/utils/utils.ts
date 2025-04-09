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
