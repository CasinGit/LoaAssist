<script lang="ts">
    export let tabs: any[] = []; // * 탭 리스트
    export let activeTab = ""; // * 현재 활성화된 탭
    export let onTabSelect; // * 탭 선택 시 호출되는 콜백 함수
</script>

<!-- 탭 UI -->
<div class="tabs">
    {#each tabs as tab}
        <button
            class:selected={tab.id === activeTab}
            class="me-2 inline-block cursor-pointer rounded-t-lg border-b-2 border-transparent p-1 hover:border-gray-300 hover:text-gray-300"
            on:click={() => onTabSelect(tab.id)}
        >
            {tab.name}
        </button>
    {/each}
</div>

<style>
    .tabs {
        margin-bottom: -1px;
        display: flex;
        flex-wrap: wrap;
        margin-inline-end: 0.5rem /* 8px */;
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
