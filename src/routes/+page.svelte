<script lang="ts">
    import { emit, listen } from "@tauri-apps/api/event";
    import { getCurrentWindow, LogicalPosition } from "@tauri-apps/api/window";
    import { onDestroy, onMount } from "svelte";

    import { appStore, setDetectTitle } from "../stores/appStore";

    import Tab from "$lib/components/Tab.svelte";
    import TitleBar from "$lib/components/TitleBar.svelte";
    import { TABS, UserSettingsType } from "$lib/types";
    import setWindowPosition from "$lib/utils/docking";
    import { invoke } from "$lib/utils/invoke";
    import { checkUpdateUnified } from "$lib/utils/utils";

    const appWindow = getCurrentWindow();

    let unlisten: () => void;

    let currentTab = $state("Tab1"); // * 현재 활성화된 탭
    let ActiveComponent = $state(TABS.find((tab) => tab.id === currentTab)?.component);

    let defaultSettings: UserSettingsType = $state(new UserSettingsType());

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        defaultSettings = value.userSettings;
    });

    $effect.pre(() => {
        ["+layout.svelte $effect.pre", "[Position Restore] +layout.svelte $effect.pre"].forEach((msg) =>
            console.log(
                "%c" + msg,
                "color:white; font-weight:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
            )
        );

        // ? 프로그램 닫기 전 마지막 프로그램 위치로 설정
        invoke("get_position").then((position) => {
            if (position) {
                // console.log(position);
                appWindow.setPosition(new LogicalPosition(position.x, position.y));
            }
        });
    });

    // + currentTab 값 변경시 ActiveComponent 변경
    $effect.pre(() => {
        ActiveComponent = TABS.find((tab) => tab.id === currentTab)?.component;
    });

    onMount(async () => {
        console.log(
            "%c‡index +page.svelte onMount",
            "color:white; font-style:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
        );
        emit("frontend-ready"); // ! 프론트 로드 완료시 Emit to Backend

        // * Session Storage에서 current_tab 옵션 확인
        const lastTab = sessionStorage.getItem("current_tab");
        if (lastTab) {
            // ? SessionStorage가 있을 때
            currentTab = lastTab;
        } else {
            // ? SessionStorage가 없을 때
            currentTab = await invoke("get_default_tab");
            sessionStorage.setItem("current_tab", currentTab);
        }

        // ? 창 이동 시마다 위치를 검사
        unlisten = await appWindow.listen("tauri://move", setWindowPosition);

        checkUpdateUnified(false); // * 새로고침시 업데이트 확인 (캐시 데이터 사용)
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );

        if (unlisten) {
            unlisten(); // ! 리스너 해제
            console.log(
                "%cSetWindowPosition Listener removed on destroy!",
                "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
            );
        }
        unsubscribe(); // ! Cleanup on unmount
    });

    // + 탭 선택 시 호출되는 함수
    function handleTabSelect(id: string) {
        currentTab = id;
        sessionStorage.setItem("current_tab", id);
    }

    // + 프로그램을 시작했을때 한번만 실행 (새로고침 X)
    listen("on:app_start_once", async () => {
        console.log(
            "%cApp Start Once Event!",
            "color:white; font-style:bold; background-color:limeGreen; padding:3px; border-radius:4px; font-size:12px;"
        );

        // * 프로그램 시작 시 업데이트 확인 (캐시 데이터 사용)
        if (defaultSettings.update_check_enabled) checkUpdateUnified(true, false, true);

        // ? 사용자가 자동 감지 기능을 허용했을 때만 실행
        if (defaultSettings.auto_detect_title) {
            const findTitle = await invoke("find_window_by_title", { target: "LOST ARK" });

            if (findTitle) {
                // * 클라이언트 이름을 찾았을 때
                await setDetectTitle(findTitle);
                invoke("play_system_sound", { sound: "Alarm" });
            }
        }
    });
</script>

<TitleBar />

<div class="flex flex-auto flex-col bg-neutral-800 bg-opacity-70">
    <main class="h-px flex-1 basis-auto overflow-y-auto p-1">
        {#if ActiveComponent}
            <ActiveComponent />
        {/if}
    </main>
    <footer class="bottom-0 bg-opacity-70 p-[0.05rem]">
        <div class="border-b border-t border-gray-200 text-center text-sm font-medium text-gray-500">
            <Tab {currentTab} onTabSelect={handleTabSelect} />
        </div>
    </footer>
</div>
