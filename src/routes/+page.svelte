<script lang="ts">
    import { emit, listen } from "@tauri-apps/api/event";
    import { getCurrentWindow, LogicalPosition } from "@tauri-apps/api/window";
    import { onDestroy, onMount } from "svelte";

    import { appStore } from "../stores/appStore";

    import Dashboard from "$lib/components/Dashboard.svelte";
    import Setting from "$lib/components/Setting.svelte";
    import Tab from "$lib/components/Tab.svelte";
    import TitleBar from "$lib/components/TitleBar.svelte";
    import WorkSheet from "$lib/components/WorkSheet.svelte";
    import { UserSettingsType, type TabType } from "$lib/types";
    import setWindowPosition from "$lib/utils/docking";
    import { invoke } from "$lib/utils/invoke";
    import { updateCheckDialog } from "$lib/utils/utils";

    const appWindow = getCurrentWindow();

    let unlisten: () => void;

    let defaultSettings: UserSettingsType = $state(new UserSettingsType());

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        defaultSettings = value.userSettings;
    });

    $effect.pre(() => {
        console.log(
            "%c+layout.svelte $effect.pre",
            "color:white; font-style:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
        );

        // ? 프로그램 닫기 전 마지막 프로그램 위치로 설정
        invoke("get_position").then((position) => {
            if (position) {
                // console.log(position);
                appWindow.setPosition(new LogicalPosition(position.x, position.y));
            }
        });
    });

    onMount(() => {
        console.log(
            "%c‡index +page.svelte onMount",
            "color:white; font-style:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
        );

        (async () => {
            // ? 창 이동 시마다 위치를 검사
            unlisten = await appWindow.listen("tauri://move", setWindowPosition);
        })();

        emit("frontend-ready"); // ! 프론트 로드 완료시 Emit to Backend
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

    // * 탭 목록 정의
    const tabs: TabType[] = [
        { id: "Tab1", name: "대시보드", component: Dashboard },
        { id: "Tab2", name: "숙제표", component: WorkSheet },
        { id: "Tab3", name: "설정", component: Setting }
    ];
    let activeTab: string = $state("Tab2"); // ? 현재 활성화된 탭
    let activeComponent = $state(tabs.find((tab) => tab.id === activeTab)?.component); // ? 활성화된 탭의 컴포넌트 찾기

    // + 탭 선택 시 호출되는 함수
    function handleTabSelect(id: string) {
        activeTab = id;
        activeComponent = tabs.find((tab) => tab.id === activeTab)?.component;
    }

    // + 프로그램을 시작했을때 한번만 실행 (새로고침 X)
    listen("on:app_start_once", () => {
        console.log(
            "%cApp Start Once Event!",
            "color:white; font-style:bold; background-color:limeGreen; padding:3px; border-radius:4px; font-size:12px;"
        );
        // ? 사용자가 업데이트 확인 설정을 했을때만 실행
        if (defaultSettings.update_check_enabled) updateCheckDialog(true);
    });
</script>

<TitleBar />

<div class="flex flex-auto flex-col bg-neutral-800 bg-opacity-70">
    <main class="h-px flex-1 basis-auto overflow-y-auto p-1">
        {#if activeComponent}
            <!-- svelte-ignore svelte_component_deprecated -->
            <svelte:component this={activeComponent} />
        {/if}
    </main>
    <footer class="bottom-0 bg-opacity-70 p-[0.05rem]">
        <div class="border-b border-t border-gray-200 text-center text-sm font-medium text-gray-500">
            <Tab {tabs} {activeTab} onTabSelect={handleTabSelect} />
        </div>
    </footer>
</div>
