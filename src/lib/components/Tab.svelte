<script lang="ts">
    import { message } from "@tauri-apps/plugin-dialog";
    import { open } from "@tauri-apps/plugin-shell";
    import { Tooltip } from "flowbite-svelte";
    import { onDestroy } from "svelte";

    import { appStore } from "../../stores/appStore";

    import { DB_NOT_EXIST, SUPABASE_OFFLINE } from "$lib/db";
    import { TABS } from "$lib/types";
    import { checkUpdateUnified } from "$lib/utils/utils";

    const {
        currentTab, // * 현재 활성화된 탭
        onTabSelect // * 탭 선택 시 호출되는 콜백 함수
    }: {
        currentTab: string;
        onTabSelect: (id: string) => void;
    } = $props();

    let updateExists = $state(false);

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        updateExists = value.updateExists;
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on Tab.svelte component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        unsubscribe(); // ! Cleanup on unmount
    });

    // + 프로그램 수동 업데이트 함수
    async function handleUpdateCheck() {
        checkUpdateUnified(true, true); // * Dialog 포함 + forceRefresh
    }

    // + DB 복구 안내 함수
    async function handleSafeDatabase() {
        await message(
            "서버 데이터베이스에 연결하지 못했고 로컬 데이터베이스도 찾을 수 없습니다.\n\n프로그램을 정상적으로 사용할 수 없는 상태이며,\n데이터베이스 복구가 필요합니다.\n\n복구 방법을 안내하는 페이지를 브라우저에서 엽니다.",
            {
                title: "데이터베이스 오류",
                kind: "error",
                okLabel: "브라우저 열기"
            }
        );
        await open(
            "https://github.com/CasinGit/LoaAssist/wiki/%EC%86%90%EC%83%81%EB%90%9C-DB-%EB%B3%B5%EA%B5%AC-%EB%B0%A9%EB%B2%95"
        );
    }
</script>

<!-- 탭 UI -->
<div class="tabs">
    <div>
        {#each TABS as tab}
            <button
                class:selected={tab.id === currentTab}
                class="me-2 inline-block cursor-pointer rounded-t-lg border-b-2 border-transparent p-1 hover:border-gray-300 hover:text-gray-300"
                onclick={() => onTabSelect(tab.id)}
            >
                {tab.name}
            </button>
        {/each}
    </div>

    <div class="flex gap-x-1">
        <!-- * 프로그램 DB 에러 -->
        {#if DB_NOT_EXIST}
            <Tooltip color="red" triggeredBy="#type-5" class="z-50 p-1.5 px-2 text-xs font-bold">
                <p class="text-[#ff0000]">DB 복구 필요</p>
            </Tooltip>

            <button id="type-5" class="z-50 mb-1 text-amber-300" onclick={handleSafeDatabase}>🚨</button>
        {/if}

        <!-- * Supabase 문제 발생 알림 -->
        {#if SUPABASE_OFFLINE}
            <Tooltip color="orange" triggeredBy="#type-4" class="p-1.5 px-2 text-xs font-bold">
                <p class="text-[#723B13]">서버 연결 문제</p>
            </Tooltip>

            <button id="type-4" class="mb-1 text-amber-300">⚠️</button>
        {/if}

        <!-- * 업데이트 알림 -->
        {#if updateExists}
            <Tooltip color="green" triggeredBy="#type-3" class="p-1.5 px-2 text-xs">
                <p class="text-[#65bd00]">새 업데이트가 있습니다.</p>
            </Tooltip>
            <button id="type-3" class="mb-1 text-amber-300" ondblclick={handleUpdateCheck}>🌟</button>
        {/if}
    </div>
</div>

<style>
    .tabs {
        margin-bottom: -1px;
        display: flex;
        flex-wrap: wrap;
        margin-inline-end: 0.3rem /* 8px */;
        justify-content: space-between;
    }
    .selected {
        display: inline-block;
        cursor: default;
        border-top-left-radius: 0.5rem /* 8px */;
        border-top-right-radius: 0.5rem /* 8px */;
        border-bottom-width: 2px;
        --tw-border-opacity: 1;
        border-color: rgb(14 159 110 / var(--tw-border-opacity)) /* #0e9f6e */;
        --tw-text-opacity: 1;
        color: rgb(49 196 141 / var(--tw-text-opacity)) /* #31c48d */;
        padding: 0.25rem /* 4px */;
    }
</style>
