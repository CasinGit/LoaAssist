<script lang="ts">
    import { getCurrentWindow, LogicalPosition } from "@tauri-apps/api/window";
    import { onDestroy, onMount } from "svelte";

    import Dashboard from "$lib/components/Dashboard.svelte";
    import Setting from "$lib/components/Setting.svelte";
    import Tab from "$lib/components/Tab.svelte";
    import TitleBar from "$lib/components/TitleBar.svelte";
    import WorkSheet from "$lib/components/WorkSheet.svelte";
    import { type TabType } from "$lib/types";
    import setWindowPosition from "$lib/utils/docking";
    import { invoke } from "$lib/utils/invoke";

    const appWindow = getCurrentWindow();

    let unlisten: () => void;

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
