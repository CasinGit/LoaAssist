<script lang="ts">
    import { getVersion } from "@tauri-apps/api/app";
    import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
    import { getAllWindows } from "@tauri-apps/api/window";
    import Database from "@tauri-apps/plugin-sql";
    import { Badge, Checkbox } from "flowbite-svelte";
    import { cloneDeep, isEmpty } from "lodash-es";
    import { onDestroy, onMount } from "svelte";

    import WorkResetModal from "./modal/WorkResetModal.svelte";
    import {
        appStore,
        loadLiveDB,
        getUserSettings,
        setUserSettings,
        openWorkResetModal,
        closeWorkResetModal
    } from "../../stores/appStore";

    import { DEFAULT_RAIDS_VERSION } from "$lib/db/schema";
    import { TABS, UserSettingsType } from "$lib/types";
    import { invoke } from "$lib/utils/invoke";
    import { updateCheckDialog } from "$lib/utils/utils";

    let appVersion: string | null = $state(null);

    let liveDbName: string = $state("");
    let defaultSettings: UserSettingsType = $state(new UserSettingsType());
    let currentSettings: UserSettingsType = $state(new UserSettingsType());
    let workResetModalOpen: boolean = $state(false);
    let isSaving: boolean = $state(false);
    let isTitleEffect: boolean = $state(false);
    let effectColor: string = $state("red");

    let updateElm: HTMLParagraphElement | null = $state(null);
    let themeElm: HTMLParagraphElement | null = $state(null);
    let classImgElm: HTMLParagraphElement | null = $state(null);
    let windowFoldElm: HTMLParagraphElement | null = $state(null);
    let windowOpacityElm: HTMLParagraphElement | null = $state(null);
    let windowIdleTimeElm: HTMLParagraphElement | null = $state(null);
    let autoFocusElm: HTMLParagraphElement | null = $state(null);
    let autoFocusTitleElm: HTMLParagraphElement | null = $state(null);
    let autoFocusIdleTimeElm: HTMLParagraphElement | null = $state(null);
    let focusBorderElm: HTMLParagraphElement | null = $state(null);
    let defaultTabElm: HTMLParagraphElement | null = $state(null);

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        liveDbName = value.liveDbName;
        defaultSettings = value.userSettings;
        currentSettings = value.userSettings;
        workResetModalOpen = value.workResetModalOpen;
    });

    onMount(async () => {
        await getUserSettings(); // ? 저장 후 데이터 동기화에 문제가 있어서 실행
        appVersion = await getVersion();
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on Setting.svelte component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        unsubscribe(); // ! Cleanup on unmount
    });

    $effect(() => {
        // console.log($inspect(defaultSettings));
        // console.log($inspect(currentSettings));

        const changedSettings = UserSettingsType.getChangedSettings(defaultSettings, currentSettings);

        if (!isEmpty(changedSettings)) {
            // ? 변경된 항목을 콘솔에 출력
            console.log("변경된 설정:", cloneDeep(changedSettings));

            // * 각 항목별로 변경 상태 확인 및 업데이트
            updateStyle(updateElm, changedSettings.update_check_enabled);
            updateStyle(themeElm, changedSettings.theme);
            updateStyle(classImgElm, changedSettings.class_image);
            updateStyle(windowFoldElm, changedSettings.folded_opacity_enabled);
            updateStyle(windowOpacityElm, changedSettings.folded_settings?.opacity);
            updateStyle(windowIdleTimeElm, changedSettings.folded_settings?.idle_time);
            updateStyle(autoFocusElm, changedSettings.auto_focus_enabled);
            updateStyle(autoFocusTitleElm, changedSettings.auto_focus_settings?.game_title);
            updateStyle(autoFocusIdleTimeElm, changedSettings.auto_focus_settings?.shift_idle_time);
            updateStyle(focusBorderElm, changedSettings.focus_border_enabled);
            updateStyle(defaultTabElm, changedSettings.default_tab);
        } else {
            // ? 변경된 설정이 없으면 모든 요소 초기화
            resetStyles([
                updateElm,
                themeElm,
                classImgElm,
                windowFoldElm,
                windowOpacityElm,
                windowIdleTimeElm,
                autoFocusElm,
                autoFocusTitleElm,
                autoFocusIdleTimeElm,
                focusBorderElm,
                defaultTabElm
            ]);
        }
    });

    // + Element Style Color 변경 재사용 함수
    function updateStyle(elm: HTMLElement | null, condition: any | undefined): void {
        if (elm) {
            elm.style.color = condition !== undefined ? "aqua" : "white";
        }
    }

    // + Element Style Color 초기화 함수
    function resetStyles(elements: (HTMLElement | null)[]): void {
        elements.forEach((elm) => {
            if (elm) elm.style.color = "white";
        });
    }

    // + 새로운 Setting WebviewWindow 생성
    async function handleWorkSetting() {
        // ? 현재 열려있는 모든 창 확인
        const allWindow = await getAllWindows();
        const existingWindow = allWindow.find((win) => win.label === "settings");
        // console.log(existingWindow);

        if (existingWindow) {
            // ? 창이 이미 열려 있을 때
            existingWindow.setFocus(); // 창에 포커스를 맞춤
            existingWindow.unminimize(); // 최소화된 상태를 해제
        } else {
            // ? 새로운 창 생성
            const newWindow = new WebviewWindow("settings", {
                url: "/settings", // 새로운 윈도우에 표시할 페이지
                title: "숙제표 설정",
                width: 400,
                height: 600,
                alwaysOnTop: true,
                visible: true,
                resizable: false,
                fullscreen: false,
                transparent: true,
                shadow: false,
                decorations: true,
                maximizable: false,
                dragDropEnabled: true
            });

            // ! 이벤트 리스너 추가 (필요한 경우)
            newWindow.once("tauri://created", () => {
                console.log("새 창이 성공적으로 생성되었습니다.");
            });

            newWindow.once("tauri://destroyed", () => {
                console.log("새 창이 종료되었습니다.");
            });

            newWindow.once("tauri://error", (e) => {
                console.error("창 생성 중 오류 발생:", e);
            });
        }
    }

    // + 숙제 초기화 Modal 컴포넌트에서 사용하는 함수
    async function workSheetReset(resetType: "twoWeek" | "week") {
        let sql_string;

        if (resetType === "twoWeek") {
            // ? 2주 쿨타임 숙제도 초기화
            console.log("2주쿨도 초기화");
            sql_string = `UPDATE live_raids SET complete = 0`;
        } else {
            // ? 2주 쿨타임 숙제 제외하고 초기화
            console.log("2주쿨 초기화 안함");
            sql_string = `UPDATE live_raids SET complete = CASE WHEN resetType = 1 THEN 0 ELSE complete END`;
        }

        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            await db.execute(sql_string);
            await loadLiveDB();
        } catch (error) {
            console.error("Transaction failed:", error);
        } finally {
            await db.close();
        }
    }

    // + 초 → 분:초 변환 함수
    function formatTime(seconds: number): string {
        const minutes = Math.floor(seconds / 60); // 분 계산
        const remainingSeconds = seconds % 60; // 나머지 초 계산

        if (minutes > 0) {
            return `${minutes}분 ${remainingSeconds}초`;
        } else {
            return `${remainingSeconds}초`;
        }
    }

    // + 프로그램 설정 저장
    async function handleSaveSettings() {
        if (isSaving) return; // ? 저장 중이거나 딜레이가 끝나지 않으면 아무 작업도 하지 않음.
        // console.log("세이브 실행");
        isSaving = true;

        invoke("play_system_sound", { sound: "Alarm" });
        await setUserSettings(currentSettings);

        setTimeout(() => (isSaving = false), 2000); // ? 2초 후 저장 딜레이 해제
    }

    // + LOST ARK 와 비슷한 프로세스 창 이름 찾기
    async function handleProcessFind() {
        const findTitle = await invoke("find_window_by_title", { target: "LOST ARK" });

        if (findTitle) {
            currentSettings.auto_focus_settings.game_title = findTitle;

            invoke("play_system_sound", { sound: "Alarm" });
            effectColor = "green";
            isTitleEffect = true;
            setTimeout(() => (isTitleEffect = false), 200); // ? 0.2초 후 테두리 제거
        } else {
            invoke("play_system_sound", { sound: "Foreground" });
            effectColor = "red";
            isTitleEffect = true;
            setTimeout(() => (isTitleEffect = false), 400); // ? 0.4초 후 테두리 제거
        }
    }

    // + 프로그램 수동 업데이트 함수
    async function handleUpdateCheck() {
        updateCheckDialog();
    }
</script>

<section class="flex flex-col gap-1">
    <div class="flex flex-col gap-0.5 text-sm">
        <Badge class="w-fit cursor-default px-1.5" border color="dark">Program Version : {appVersion}</Badge>
        <Badge class="w-fit cursor-default px-1.5" border color="dark"
            >Activated DB : {liveDbName} (Default Raids Table V.{DEFAULT_RAIDS_VERSION})</Badge
        >
    </div>

    <div class="flex gap-1 text-sm">
        <p bind:this={updateElm}>🔸프로그램 시작할 때 업데이트 확인</p>
        <Checkbox color="red" class="ml-[0.05rem] mr-1" bind:checked={currentSettings.update_check_enabled} />
    </div>

    <div class="flex gap-1 text-sm">
        <p bind:this={defaultTabElm}>🔸프로그램 시작할 때 기본 탭</p>
        <select
            class={`w-auto rounded-sm bg-slate-200 p-0 !pr-8 pl-0.5 text-xs font-bold text-black`}
            name="tab"
            id="tab"
            bind:value={currentSettings.default_tab}
        >
            {#each TABS as tab}
                <option class="font-bold text-black" value={tab.id}>{tab.name}</option>
            {/each}
        </select>
    </div>

    <div class="flex gap-1 text-sm">
        <p bind:this={themeElm}>🔸테마</p>
        <select
            class={`w-auto rounded-sm bg-slate-200 p-0 !pr-8 pl-0.5 text-xs font-bold text-black`}
            name="theme"
            id="theme"
            bind:value={currentSettings.theme}
            disabled
        >
            <option class="font-bold text-black" value={"light"}>light</option>
            <option class="font-bold text-black" value={"dark"}>dark</option>
        </select>
    </div>

    <div class="flex gap-1 text-sm">
        <p bind:this={classImgElm}>🔸클래스 이미지 사용</p>
        <Checkbox color="red" class="ml-[0.05rem] mr-1" bind:checked={currentSettings.class_image} />
    </div>

    <div class="flex gap-1 text-sm">
        <p bind:this={windowFoldElm}>🔸창 접힘 상태에서 투명도 사용</p>
        <Checkbox color="red" class="ml-[0.05rem] mr-1" bind:checked={currentSettings.folded_opacity_enabled} />
    </div>
    {#if currentSettings.folded_opacity_enabled}
        <div class="content ml-2 w-auto">
            <div class="flex gap-1 text-sm">
                <p bind:this={windowOpacityElm}>▫️투명도</p>
                <input
                    class="w-auto"
                    type="range"
                    min="0"
                    max="100"
                    step="5"
                    bind:value={currentSettings.folded_settings.opacity}
                />
                <p>{currentSettings.folded_settings.opacity}%</p>
            </div>
            <div class="flex gap-1 text-sm">
                <p bind:this={windowIdleTimeElm}>▫️대기 시간</p>
                <input
                    class="w-auto"
                    type="range"
                    min="10"
                    max="600"
                    step="10"
                    bind:value={currentSettings.folded_settings.idle_time}
                />
                <p>{formatTime(currentSettings.folded_settings.idle_time)}</p>
            </div>
        </div>
    {/if}

    <div class="flex gap-1 text-sm">
        <p bind:this={autoFocusElm}>🔸포커스 자동 전환 모드</p>
        <Checkbox color="red" class="ml-[0.05rem] mr-1" bind:checked={currentSettings.auto_focus_enabled} />
    </div>
    {#if currentSettings.auto_focus_enabled}
        <div class="content ml-2 w-auto">
            <div class="flex gap-1 text-sm">
                <p bind:this={autoFocusTitleElm}>▫️창 이름</p>
                <input
                    class={`size-5 w-auto flex-1 p-1 text-xs font-medium text-black ${isTitleEffect && (effectColor === "red" ? "border-effect-red" : "border-effect-green")}`}
                    type="text"
                    bind:value={currentSettings.auto_focus_settings.game_title}
                />
                <button onclick={handleProcessFind}>🖥️</button>
            </div>
            <div class="flex gap-1 text-sm">
                <p bind:this={autoFocusIdleTimeElm}>▫️전환 대기 시간</p>
                <input
                    class="w-auto"
                    type="range"
                    min="0"
                    max="60"
                    step="1"
                    bind:value={currentSettings.auto_focus_settings.shift_idle_time}
                />
                <p>{formatTime(currentSettings.auto_focus_settings.shift_idle_time)}</p>
            </div>
        </div>
    {/if}

    <div class="flex gap-1 text-sm">
        <p bind:this={focusBorderElm}>🔸포커스 테두리 효과</p>
        <Checkbox color="red" class="ml-[0.05rem] mr-1" bind:checked={currentSettings.focus_border_enabled} />
    </div>

    <div class="flex w-full text-sm">
        <button
            class="group relative inline-flex w-full items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-teal-300 to-lime-300 p-0.5 disabled:cursor-not-allowed disabled:opacity-80 group-hover:from-teal-300 group-hover:to-lime-300"
            onclick={handleSaveSettings}
            disabled={isSaving}
        >
            <span
                class="relative w-full rounded-md bg-white px-2.5 py-2 text-sm font-semibold text-black transition-all duration-75 ease-in group-hover:bg-transparent group-disabled:bg-transparent dark:bg-gray-900 group-hover:dark:bg-transparent"
            >
                {#if isSaving}
                    저장 완료!
                {:else}
                    설정 저장
                {/if}
            </span>
        </button>
    </div>
    <div class="flex gap-1 text-sm">
        <button
            class="group relative inline-flex items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-teal-300 to-lime-300 p-0.5 group-hover:from-teal-300 group-hover:to-lime-300"
            onclick={handleWorkSetting}
        >
            <span
                class="relative w-full rounded-md bg-white px-2.5 py-2 text-sm font-semibold text-black transition-all duration-75 ease-in group-hover:bg-transparent dark:bg-gray-900 group-hover:dark:bg-transparent"
            >
                숙제표 세팅
            </span>
        </button>
        <button
            class="group relative inline-flex items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-teal-300 to-lime-300 p-0.5 group-hover:from-teal-300 group-hover:to-lime-300"
            onclick={openWorkResetModal}
        >
            <span
                class="relative w-full rounded-md bg-white px-2.5 py-2 text-sm font-semibold text-black transition-all duration-75 ease-in group-hover:bg-transparent dark:bg-gray-900 group-hover:dark:bg-transparent"
            >
                숙제표 초기화
            </span>
        </button>
    </div>
    <div class="flex text-sm">
        <button
            class="group relative inline-flex items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-teal-300 to-lime-300 p-0.5 group-hover:from-teal-300 group-hover:to-lime-300"
            ondblclick={handleUpdateCheck}
        >
            <span
                class="relative w-full rounded-md bg-white px-2.5 py-2 text-sm font-semibold text-black transition-all duration-75 ease-in group-hover:bg-transparent dark:bg-gray-900 group-hover:dark:bg-transparent"
            >
                새로운 버전 확인
            </span>
        </button>
    </div>
</section>

{#if workResetModalOpen}
    <WorkResetModal isOpen={workResetModalOpen} {closeWorkResetModal} {workSheetReset} />
{/if}

<style>
    .border-effect-red {
        border: 1px solid transparent;
        pointer-events: none;
        animation: glow-red 0.4s ease-in-out;
    }

    .border-effect-green {
        border: 1px solid transparent;
        pointer-events: none;
        animation: glow-green 0.2s ease-in-out;
    }

    @keyframes glow-red {
        0% {
            border-color: rgba(0, 0, 0, 0);
            box-shadow: 0 0 0px rgba(0, 0, 0, 0);
        }
        20% {
            border-color: rgba(255, 0, 0, 1);
            box-shadow: 0 0 5px rgba(255, 0, 0, 1);
        }
        50% {
            border-color: rgba(0, 0, 0, 0);
            box-shadow: 0 0 10px rgba(0, 0, 0, 0);
        }
        80% {
            border-color: rgba(255, 0, 0, 1);
            box-shadow: 0 0 5px rgba(255, 0, 0, 1);
        }
        100% {
            border-color: rgba(0, 0, 0, 0);
            box-shadow: 0 0 0px rgba(0, 0, 0, 0);
        }
    }

    @keyframes glow-green {
        0% {
            border-color: rgba(0, 0, 0, 0);
            box-shadow: 0 0 0px rgba(0, 0, 0, 0);
        }
        50% {
            border-color: rgb(0, 255, 20);
            box-shadow: 0 0 5px rgb(0, 255, 20);
        }
        100% {
            border-color: rgba(0, 0, 0, 0);
            box-shadow: 0 0 0px rgba(0, 0, 0, 0);
        }
    }
</style>
