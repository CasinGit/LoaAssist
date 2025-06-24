<script lang="ts">
    import { onDestroy } from "svelte";

    import { appStore } from "../../stores/appStore";

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

    {#if updateExists}
        <button class="mb-1 text-amber-300" ondblclick={handleUpdateCheck}>🌟</button>
    {/if}
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
