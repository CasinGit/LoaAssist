<script lang="ts">
    import Database from "@tauri-apps/plugin-sql";
    import { GridPlusSolid, UploadSolid, UserRemoveOutline } from "flowbite-svelte-icons";
    import { flip } from "svelte/animate";
    import { dndzone, type DndEvent } from "svelte-dnd-action";

    import type { RaidType } from "$lib/types";
    import { generateUniqueId } from "$lib/utils/utils";

    const flipDurationMs = 200; // * dnd duration 딜레이

    // ? Component props
    let {
        liveDbName = $bindable(),
        customRaidTable = $bindable(),
        reloadAllTable
    }: { liveDbName: string; customRaidTable: RaidType[]; reloadAllTable: () => Promise<void> } = $props();

    $effect(() => {
        // console.log($inspect(customRaidTable));
    });

    // + 커스텀 레이드 행 순서 Moved
    function handleDndConsider(e: CustomEvent<DndEvent<RaidType>>) {
        console.log("CustomRaid Dnd Moved");
        customRaidTable = e.detail.items;
    }

    // + 커스텀 레이드 행 순서 Move End
    async function handleDndFinalize(e: CustomEvent<DndEvent<RaidType>>) {
        console.log("CustomRaid Dnd Move End");
        customRaidTable = e.detail.items;
    }

    // + 커스텀 레이드 행 추가 핸들러
    async function handleAddRaid() {
        const raidIdx = customRaidTable.length + 1;
        const uniqueId = generateUniqueId();
        console.log(uniqueId);

        customRaidTable.push({
            raidId: uniqueId,
            priority: raidIdx,
            level: null,
            raidName: null,
            difficulty: null,
            gate: null,
            reward: null,
            resetType: 1,
            id: uniqueId
        });
    }

    // + 커스텀 레이드 행 삭제 핸들러
    function handleDelRaid(item: RaidType) {
        // console.log(item);

        const targetIndex = customRaidTable.findIndex((e) => e.raidId === item.raidId);
        console.log(targetIndex);

        customRaidTable.splice(targetIndex, 1);
        console.log(customRaidTable);
    }

    // + 커스텀 레이드 리스트 저장 핸들러
    async function handleSaveRaid() {
        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            await db.execute("BEGIN TRANSACTION");

            await db.execute(`DELETE FROM custom_raids`);

            let index: number = 1;
            for (const item of customRaidTable) {
                if (!item.level || !item.raidName || !item.difficulty || !item.gate || !item.reward) continue;

                item.priority = index;

                await db.execute(
                    "INSERT INTO custom_raids (raidId, priority, level, raidName, difficulty, gate, reward, resetType) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                    [
                        item.raidId,
                        item.priority,
                        item.level,
                        item.raidName,
                        item.difficulty,
                        item.gate,
                        item.reward,
                        item.resetType
                    ]
                );
                index++;
            }

            await db.execute("COMMIT");
            console.log("Transaction committed successfully.");
        } catch (error) {
            // ! 오류가 발생하면 롤백
            await db.execute("ROLLBACK");
            console.error("Transaction failed and rolled back:", error);
        } finally {
            await db.close();
            await reloadAllTable();
        }
    }
</script>

<div class="p-1">
    <div class="flex gap-1 text-center">
        <p class="mb-1 text-sm">※ 커스텀 레이드 추가</p>
        <button onclick={handleAddRaid}>
            <GridPlusSolid />
        </button>
        <button onclick={handleSaveRaid}>
            <UploadSolid />
        </button>
    </div>
    <div class="h-auto max-h-80 w-auto overflow-y-auto">
        <section
            class="space-y-0.5"
            use:dndzone={{ items: customRaidTable, flipDurationMs, dropFromOthersDisabled: true }}
            onconsider={handleDndConsider}
            onfinalize={handleDndFinalize}
        >
            {#each customRaidTable as item (item.id)}
                <div
                    class="mr-0.5 flex flex-auto items-center justify-between rounded-sm bg-zinc-400 p-0.5 pl-1 text-[0.7rem] font-black leading-[1-rem] hover:bg-gray-500 hover:shadow"
                    animate:flip={{ duration: flipDurationMs }}
                >
                    <div>
                        <input
                            class={`w-10 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-gray-900`}
                            placeholder="레벨"
                            bind:value={item.level}
                        />
                        <input
                            class={`w-28 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-gray-900`}
                            placeholder="레이드명"
                            bind:value={item.raidName}
                        />
                        <input
                            class={`w-14 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-gray-900`}
                            placeholder="난이도"
                            bind:value={item.difficulty}
                        />
                        <input
                            class={`w-10 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-gray-900`}
                            placeholder="관문"
                            bind:value={item.gate}
                        />
                        <input
                            class={`w-12 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-gray-900`}
                            placeholder="골드"
                            bind:value={item.reward}
                        />
                        <input
                            class={`w-7 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-gray-900`}
                            placeholder="타입"
                            type="number"
                            min="1"
                            max="2"
                            maxlength="2"
                            bind:value={item.resetType}
                        />
                    </div>
                    <button class="right-0.5 mr-1" onclick={() => handleDelRaid(item)}>
                        <UserRemoveOutline class="remove-btn" color="#ff0000" />
                    </button>
                </div>
            {/each}
        </section>
    </div>
</div>
