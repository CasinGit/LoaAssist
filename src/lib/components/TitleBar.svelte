<script module lang="ts">
    const __DEV__: boolean = import.meta.env.DEV; // DEV = true : false
</script>

<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { message } from "@tauri-apps/plugin-dialog";
    import { Tooltip } from "flowbite-svelte";
    import {
        AngleDownOutline,
        AngleUpOutline,
        BugOutline,
        CloseOutline,
        MinusOutline,
        EyeSlashSolid,
        EyeSolid
    } from "flowbite-svelte-icons";
    import { onDestroy, onMount } from "svelte";

    import { appStore, setGold } from "../../stores/appStore";

    import { UserSettingsType } from "$lib/types";
    import { invoke } from "$lib/utils/invoke";
    import { checkWindowsVersion } from "$lib/utils/utils";

    // * Current Window 변수
    const appWindow = getCurrentWindow();

    // * 숫자 타입 변수 (반응형)
    let gold: number = $state(0);
    let remainingReward: number = $state(0);
    let remainingRaids: number = $state(0);
    let totalReward: number = $state(0);

    // * UI 상태를 나타내는 불리언 타입 변수 (반응형)
    let isEditing: boolean = $state(false); // ? 편집 모드 활성화 여부
    let isTaskbarHide: boolean = $state(false); // ? TaskBar Hide 상태 여부
    let isOpacity: boolean = $state(true); // ? 창이 투명한 상태인지 여부
    let isUpDown: boolean = $state(false); // ? Up = true, Down = false

    // * 사용자 설정 객체 (반응형)
    let userSettings: UserSettingsType = $state(new UserSettingsType());

    // * 일반적인 변수 (반응형 X)
    let isProcessing: boolean = false; // ? 창 접힘 중복 실행 방지 변수
    let timeout: number; // ? 창 투명 상태 적용 타이머 변수
    let editTimeout: number; // ? 자동 편집 종료 타이머 변수

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        gold = value.gold;
        remainingReward = value.remainingReward;
        remainingRaids = value.remainingRaids;
        totalReward = value.totalReward;
        userSettings = value.userSettings;
    });

    $effect.pre(() => {
        console.log(
            "%cTitleBar $effect.pre",
            "color:white; font-style:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
        );

        // Up & Dawn 아이콘 초기 설정
        appWindow.innerSize().then((windowSize) => {
            isUpDown = windowSize.height === 500 ? false : true;
        });

        // App State에 저장된 Gold 불러오기
        invoke("get_gold").then((value) => {
            setGold(value);
            // console.log(gold);
        });

        checkWindowsVersion().then((result) => {
            if (result === "unsupported") {
                message(
                    "이 프로그램은 Windows 11 환경에서 개발 및 테스트되었습니다.\nWindows 10에서는 일부 기능이 제한될 수 있으며,\nWindows 10 미만의 환경에서는 정상적인 실행이 보장되지 않습니다.\n안정적인 사용을 위해 Windows 11 사용을 권장 드립니다.",
                    { title: "경고", kind: "warning" }
                );
            }
        });
    });

    onMount(() => {
        console.log(
            "%cTitleBar onMount",
            "color:white; font-style:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
        );

        setupListener(); // ! Listener 추가
        listenerResetTransparency(); // ? 투명도 조작 초기 타이머 시작
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on TitleBar.svelte component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        removeListener(); // ! Listener 제거
        unsubscribe(); // ! Cleanup on unmount
    });

    // DEV 환경에서만 존재하는 핸들러를 변수로 저장
    let testHandler: (() => void) | null = null;

    // + Event Listener Group (Add)
    function setupListener() {
        document.getElementById("titlebar-minimize")?.addEventListener("click", () => {
            isTaskbarHide = false;
            appWindow.setSkipTaskbar(false);
            appWindow.minimize();
        });
        document.getElementById("titlebar-close")?.addEventListener("click", () => {
            isTaskbarHide = false;
            appWindow.setSkipTaskbar(false);
            appWindow.hide();
        });
        document.getElementById("titlebar-resize")?.addEventListener("click", listenerResize);
        document.getElementById("titlebar-hide")?.addEventListener("dblclick", listenerHideFromTaskbar);
        window.addEventListener("keydown", listenerTabKeyDown);
        window.addEventListener("keydown", listenerResetTransparency);
        window.addEventListener("mousemove", listenerResetTransparency);
        window.addEventListener("mousedown", listenerResetTransparency);

        if (__DEV__) {
            // DEV 환경에서만 빌드
            testHandler = async () => {
                console.log(
                    "%cTEST FUNCTION",
                    "color:white; font-style:bold; background-color:coral; padding:3px; border-radius:4px; font-size:12px;"
                );
                const findTitle = await invoke("find_window_by_title", { target: "Fo" });
                console.log(findTitle);

                // Shows message
                await message(findTitle, { title: "Tauri", kind: "info" });
            };
            document.getElementById("titlebar-test")?.addEventListener("dblclick", testHandler);
        }
    }

    // + Event Listener Group (Remove)
    function removeListener() {
        document.getElementById("titlebar-minimize")?.removeEventListener("click", () => {
            isTaskbarHide = false;
            appWindow.setSkipTaskbar(false);
            appWindow.minimize();
        });
        document.getElementById("titlebar-close")?.removeEventListener("click", () => {
            isTaskbarHide = false;
            appWindow.setSkipTaskbar(false);
            appWindow.hide();
        });
        document.getElementById("titlebar-resize")?.removeEventListener("click", listenerResize);
        document.getElementById("titlebar-hide")?.removeEventListener("dblclick", listenerHideFromTaskbar);
        window.removeEventListener("keydown", listenerTabKeyDown);
        window.removeEventListener("keydown", listenerResetTransparency);
        window.removeEventListener("mousemove", listenerResetTransparency);
        window.removeEventListener("mousedown", listenerResetTransparency);

        if (__DEV__ && testHandler) {
            // DEV 환경 + testHandler가 있을때만 빌드
            document.getElementById("titlebar-test")?.removeEventListener("dblclick", testHandler);
            testHandler = null;
        }
    }

    // + 창 투명화 기능 타이머 리셋
    function listenerResetTransparency() {
        isOpacity = true; // ? 창을 완전히 보이도록 설정

        clearTimeout(timeout); // ? 기존 타이머를 제거하고 새로 설정
        timeout = setTimeout(() => {
            // ? 창이 접혀있거나 folded_opacity_enabled 옵션이 false 라면 무시
            if (!isUpDown || !userSettings.folded_opacity_enabled) return;

            isOpacity = false; // ? 일정 시간이 지나면 창을 반투명하게 설정
        }, userSettings.folded_settings.idle_time * 1000); // ? 10초 후 창을 반투명 상태로
    }

    // + Window 접는 기능 On/Off
    async function listenerResize(): Promise<void> {
        console.log("1: Processing started...", isProcessing);

        if (isProcessing) return; // ? 이미 실행 중이면 무시
        isProcessing = true; // ? 함수 동작 잠금 활성화

        await (async () => {
            const windowSize = await appWindow.innerSize();
            const newWindowSize = windowSize.height === 500 ? 26 : 500;

            const resolve = await invoke("resize_with_custom", {
                width: 320,
                height: newWindowSize,
                easing: "easeInOutQuart"
            });
            isUpDown = newWindowSize === 500 ? false : true;
            console.log("2: invoke resolve:", resolve);
        })();

        console.log("3: Processing finished!");
        isProcessing = false; // ? 함수 동작 잠금 해제
    }

    // + Tab Key Event 비활성화
    const listenerTabKeyDown = (event: KeyboardEvent) => {
        if (event.key === "Tab") {
            event.preventDefault();
            console.log("Tab key press disabled");
        }
    };

    // + Taskbar 감추는 기능 On/Off
    async function listenerHideFromTaskbar() {
        if (isTaskbarHide) {
            isTaskbarHide = false;
            await appWindow.setSkipTaskbar(false);
        } else {
            isTaskbarHide = true;
            await appWindow.setSkipTaskbar(true);
        }
    }

    // + 더블클릭 시 편집 모드 활성화하는 함수
    function handleEnableEditing() {
        isEditing = true;
        invoke("pause_auto_focus");
        resetEditTimeout();
    }

    // + 일정 시간이 지나면 자동으로 편집 종료
    function resetEditTimeout() {
        clearTimeout(editTimeout); // ? 기존 타이머 초기화

        editTimeout = setTimeout(() => {
            if (!isEditing) return;
            isEditing = false;
            invoke("resume_auto_focus"); // ? 자동 포커스 다시 활성화
        }, 5000); // ? 5초 후 자동 종료
    }

    // + 포커스를 잃거나 엔터키 입력 시 편집 모드 종료
    function handleDisableEditing(event: KeyboardEvent | FocusEvent) {
        if (!isEditing) return;

        const element = event.target as HTMLElement;

        isEditing = false;
        invoke("resume_auto_focus"); // ? 포커스 자동 이동 다시 활성화

        invoke("set_gold", { value: Number(element.textContent?.trim()) });
        setGold(Number(element.textContent?.trim()));
    }

    // + 골드 입력 KeyDown 이벤트
    function handleKeyDown(event: KeyboardEvent) {
        const target = event.target as HTMLElement;
        const allowedKeys = ["Backspace", "ArrowLeft", "ArrowRight"];
        const isNumberKey = /^[0-9]$/.test(event.key); // 숫자인지 확인하는 정규식
        const MAX_LENGTH = 8; // 최대 입력 자릿수를 8로 제한

        // 자릿수 초과 방지 (contenteditable 대응)
        const text = target.textContent ?? "";
        const selection = window.getSelection();
        const isCollapsed = selection?.isCollapsed ?? true;

        if (
            isNumberKey &&
            isCollapsed && // 드래그 선택이 아닌 경우만 제한
            text.length >= MAX_LENGTH
        ) {
            event.preventDefault();
            return;
        }

        if (!isNumberKey && !allowedKeys.includes(event.key)) {
            event.preventDefault(); // ? 숫자가 아니면 입력 차단
        }

        if (event.key === "Enter") {
            event.preventDefault(); // ? Enter 이벤트에서 줄바꿈 방지
            handleDisableEditing(event); // ? 편집 모드 종료
        } else {
            resetEditTimeout(); // ? 입력이 있으면 타이머 초기화
        }
    }
</script>

<div
    data-tauri-drag-region
    class={`titlebar ${isOpacity ? "fade-in" : "fade-out"}`}
    style="--custom-opacity: {userSettings.folded_settings.opacity * 0.01}"
>
    <div data-tauri-drag-region class="z-50 flex h-full w-full flex-1 bg-yellow-400 opacity-0"></div>

    <div class="absolute left-0 ml-1 flex gap-x-1.5">
        <div class="flex items-center gap-x-0.5">
            <img class="size-4" src="/images/gold.jpg" alt="gold" />
            {#if isEditing}
                <!-- 수정 중 -->
                <p
                    class="z-50 text-sm"
                    contenteditable={isEditing}
                    onblur={handleDisableEditing}
                    onkeydown={handleKeyDown}
                >
                    {gold}
                </p>
            {:else}
                <!-- 수정 중이 아닐 때 -->
                <p data-tauri-drag-region class="z-50 cursor-pointer text-sm" ondblclick={handleEnableEditing}>
                    {Intl.NumberFormat("ko-KR").format(gold)}
                </p>
            {/if}

            <p id="type-1" class="z-50 text-xs text-lime-400">
                ({Intl.NumberFormat("ko-KR").format(gold + remainingReward)})
            </p>
            {#if !isUpDown}
                <Tooltip color="yellow" triggeredBy="#type-1" class="w-max p-1.5 px-2 text-xs font-bold">
                    <p class="text-[#723B13]">
                        남은 골드: {Intl.NumberFormat("ko-KR").format(remainingReward)}
                    </p>
                    <p class="text-[#723B13]">총 골드: {Intl.NumberFormat("ko-KR").format(totalReward)}</p>
                </Tooltip>
            {/if}
        </div>
        <div class="flex items-center gap-x-0.5">
            <img class="size-4" src="/images/boss.png" alt="boss" />
            <p class="text-sm">{remainingRaids}</p>
        </div>
    </div>
    {#if __DEV__}
        <div class="titlebar-button" id="titlebar-test">
            <BugOutline />
        </div>
    {/if}
    <div class="titlebar-button" id="titlebar-hide">
        {#if isTaskbarHide}
            <EyeSolid id="type-2" />
            {#if !isUpDown}
                <Tooltip color="indigo" triggeredBy="#type-2" class="w-max p-1.5 px-2 text-xs font-bold">
                    <p class="text-red-500">Hidden</p>
                </Tooltip>
            {/if}
        {:else}
            <EyeSlashSolid id="type-2" />
            {#if !isUpDown}
                <Tooltip color="indigo" triggeredBy="#type-2" class="w-max p-1.5 px-2 text-xs font-bold">
                    <p class="text-sky-500">Visible</p>
                </Tooltip>
            {/if}
        {/if}
    </div>
    <div class="titlebar-button" id="titlebar-resize">
        {#if isUpDown}
            <AngleDownOutline />
        {:else}
            <AngleUpOutline />
        {/if}
    </div>
    <div class="titlebar-button" id="titlebar-minimize">
        <MinusOutline />
    </div>
    <div class="titlebar-button" id="titlebar-close">
        <CloseOutline />
    </div>
</div>

<style>
    .titlebar {
        height: 26px;
        background: rgba(38, 38, 38, 1);
        user-select: none;
        display: flex;
        justify-content: flex-end;
        position: relative; /* default : fixed */
        top: 0;
        left: 0;
        right: 0;
        align-items: center;
        opacity: 0.95;
    }
    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 26px;
        height: 26px;
        user-select: none;
        -webkit-user-select: none;
    }
    .titlebar-button:hover {
        background: rgba(82, 82, 82, 1);
    }

    /* 편집 중일 때 스타일 추가 */
    p[contenteditable="true"] {
        cursor: text;
        color: aqua;
    }

    .fade-in {
        opacity: 0.9;
        transition: opacity 0.3s ease-in-out;
    }

    .fade-out {
        /* opacity: 0.6; */
        opacity: var(--custom-opacity, 0.6);
        transition: opacity 0.5s ease-in-out;
    }
</style>
