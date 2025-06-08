<script lang="ts">
    import Database from "@tauri-apps/plugin-sql";
    import { ChevronDoubleDownOutline, ChevronDoubleUpOutline } from "flowbite-svelte-icons";
    import { onDestroy, onMount } from "svelte";
    import { slide } from "svelte/transition";

    import { appStore, liveDbName, loadLiveDB } from "../../stores/appStore";

    import type { ExtendsRaidType } from "$lib/types";

    let gold: number = $state(0);
    let totalReward: number = $state(0);
    let remainingReward: number = $state(0);
    let totalRaids: number = $state(0);
    let remainingRaids: number = $state(0);
    let expansion: boolean = $state(false);
    let remainingRaidsTable: ExtendsRaidType[] = $state([]);

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        gold = value.gold;
        totalReward = value.totalReward;
        remainingReward = value.remainingReward;
        totalRaids = value.totalRaids;
        remainingRaids = value.remainingRaids;
    });

    onMount(async () => {
        await loadRemainingRaids();
        await loadLiveDB();

        const expansionValue = sessionStorage.getItem("expanded");
        if (expansionValue === "true") {
            expansion = Boolean(expansionValue);
        }
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on Dashboard.svelte component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        unsubscribe(); // ! Cleanup on unmount
    });

    async function loadRemainingRaids() {
        const db = await Database.load(`sqlite:${liveDbName}.db`);

        try {
            remainingRaidsTable = await db.select(`
        SELECT
            id, complete,
            COALESCE(default_raids.raidId, group_raids.raidId) AS raidId,
            COALESCE(default_raids.raidName, group_raids.raidName) AS raidName,
            COALESCE(default_raids.difficulty, group_raids.difficulty) AS difficulty,
            COALESCE(default_raids.gate, group_raids.gate) AS gate,
            COUNT(*) as count
        FROM live_raids
            LEFT OUTER JOIN default_raids ON live_raids.raidId = default_raids.raidId
            LEFT OUTER JOIN group_raids ON live_raids.raidId = group_raids.raidId
        WHERE complete = 0
        GROUP BY default_raids.raidId, group_raids.raidId
        ORDER BY count DESC;
    `);
        } catch (error) {
            console.error("Transaction failed:", error);
        } finally {
            await db.close();
        }

        // console.log(remainingRaidsTable);
    }

    function handleExpansion() {
        expansion = !expansion;
        sessionStorage.setItem("expanded", String(expansion));
    }
</script>

<div class="mx-1 flex flex-col">
    <div class="flex flex-col gap-1">
        <div class="flex justify-between">
            <div class="text-sm">
                <p class="font-medium">총 골드 획득량</p>
                <p class="text-right">{Intl.NumberFormat("ko-KR").format(totalReward)}</p>
            </div>
            <div class="mx-1 h-auto border-l border-gray-400"></div>
            <div class="text-sm">
                <p class="font-medium">남은 골드 획득량</p>
                <p class="text-right">{Intl.NumberFormat("ko-KR").format(remainingReward)}</p>
            </div>
            <div class="mx-1 h-auto border-l border-gray-400"></div>
            <div class="text-sm">
                <p class="font-medium">최종 골드 보유량</p>
                <p class="text-right">{Intl.NumberFormat("ko-KR").format(remainingReward + gold)}</p>
            </div>
        </div>
        <div class="w-auto border-t border-gray-400"></div>
        <div>
            <div class="flex justify-between">
                <p class="text-sm font-medium">숙제 완료율</p>
                <p class="mr-2 text-xs">{remainingRaids} / {totalRaids}</p>
            </div>
            <div class="relative mb-1 flex h-3 w-full rounded-full bg-gray-200 text-center leading-none">
                <div
                    class="h-3 rounded-full bg-red-600"
                    style="width: {((totalRaids - remainingRaids) / totalRaids) * 100}%"
                ></div>
                <div class="absolute text-[3.5vw] font-normal mix-blend-difference" style="position-area: top;">
                    {!isNaN((totalRaids - remainingRaids) / totalRaids)
                        ? Number(((totalRaids - remainingRaids) / totalRaids) * 100).toFixed(2)
                        : 0}%
                </div>
            </div>
        </div>
    </div>

    <button class="flex w-fit self-center hover:scale-125" onclick={handleExpansion}>
        {#if expansion}
            <ChevronDoubleUpOutline />
        {:else}
            <ChevronDoubleDownOutline />
        {/if}
    </button>
    {#if expansion}
        <div transition:slide class="flex flex-col gap-1">
            <p class="text-sm font-medium">남은 레이드 개수</p>
            {#each remainingRaidsTable as item (item.id)}
                <div
                    class="group flex items-center rounded-md bg-zinc-200 p-0.5 text-[0.7rem] font-black leading-[1-rem] text-gray-900 hover:bg-gray-400 hover:shadow"
                >
                    <p class="mx-1 text-black">{item.raidName} {item.difficulty}</p>
                    <p class="mr-1 text-black">{item.gate}관문</p>
                    <p class="text-shadow mr-1 flex flex-auto justify-end text-right text-amber-300">
                        {item.count} 개
                    </p>
                </div>
            {/each}
        </div>
    {/if}
</div>
