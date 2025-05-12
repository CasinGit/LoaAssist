<script lang="ts">
    import { message } from "@tauri-apps/plugin-dialog";
    import Database from "@tauri-apps/plugin-sql";
    import { GridPlusSolid, UploadSolid } from "flowbite-svelte-icons";
    import { onDestroy } from "svelte";
    import { flip } from "svelte/animate";
    import { dndzone, type DndEvent } from "svelte-dnd-action";

    import { appStore, openGroupEditModal, closeGroupEditModal } from "../../../stores/appStore";
    import GroupEditModal from "../modal/GroupEditModal.svelte";

    import type { RaidType } from "$lib/types";
    import { generateUniqueId } from "$lib/utils/utils";

    const flipDurationMs = 200; // * dnd duration 딜레이

    // ? Component props
    let {
        liveDbName = $bindable(),
        groupRaidTable = $bindable(),
        reloadAllTable
    }: {
        liveDbName: string;
        groupRaidTable: RaidType[];
        reloadAllTable: () => Promise<void>;
    } = $props();

    let groupEditModalOpen: boolean = $state(false);
    let selectGroupRaid: RaidType = $state()!;

    $effect(() => {
        // console.log($inspect(groupRaidTable));
    });

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        groupEditModalOpen = value.groupEditModalOpen;
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        unsubscribe(); // ! Cleanup on unmount
    });

    // + 그룹 레이드 행 순서 Moved
    function handleDndConsider(e: CustomEvent<DndEvent<RaidType>>) {
        console.log("GroupRaid Dnd Moved");
        groupRaidTable = e.detail.items;
    }

    // + 그룹 레이드 행 순서 Move End
    async function handleDndFinalize(e: CustomEvent<DndEvent<RaidType>>) {
        console.log("GroupRaid Dnd Move End");
        groupRaidTable = e.detail.items;
    }

    // + 그룹 레이드 행 추가 핸들러
    async function handleAddRaid() {
        const raidIdx = groupRaidTable.length + 1;
        const uniqueId = generateUniqueId();
        // console.log(uniqueId);

        let tempRaid: RaidType = {
            id: uniqueId,
            raidId: uniqueId,
            priority: raidIdx,
            level: null,
            raidName: null,
            difficulty: null,
            gate: null,
            reward: null,
            resetType: 1
        };

        selectGroupRaid = tempRaid;
        openGroupEditModal();
    }

    // + 그룹 레이드 리스트 저장 핸들러
    async function handleSaveRaid() {
        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            await db.execute("BEGIN TRANSACTION");

            await db.execute(`DELETE FROM group_raids`);

            let index: number = 1;
            for (const item of groupRaidTable) {
                if (!item.level || !item.raidName || !item.difficulty || !item.gate || !item.reward) continue;

                item.priority = index;

                await db.execute(
                    "INSERT INTO group_raids (raidId, priority, level, raidName, difficulty, gate, reward, resetType) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
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

            message(
                "그룹 레이드 데이터를 저장하는 중 문제가 발생했습니다.\n데이터를 확인한 후, 문제가 되는 항목을 수정하거나 제거한 뒤 다시 시도해 주세요.\n\n(문제가 계속 발생할 경우, 프로그램을 재시작하거나 개발자에게 문의해 주세요.)",
                {
                    title: "DB Transaction Error",
                    kind: "error"
                }
            );
        } finally {
            await db.close();
            await reloadAllTable();
        }
    }

    function handleEditRaid(item: RaidType) {
        // todo GroupEditModal 오픈 코드
        // console.log(item);
        selectGroupRaid = item;
        openGroupEditModal();
    }
</script>

<div class="p-1">
    <div class="flex gap-1 text-center">
        <p class="mb-1 text-sm">※ 그룹 레이드 추가/수정</p>
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
            use:dndzone={{ items: groupRaidTable, flipDurationMs, dropFromOthersDisabled: true }}
            onconsider={handleDndConsider}
            onfinalize={handleDndFinalize}
        >
            {#each groupRaidTable as item (item.id)}
                <div
                    class="mr-0.5 flex flex-auto items-center justify-between rounded-sm bg-zinc-400 p-0.5 pl-1 text-[0.7rem] font-black leading-[1-rem] hover:bg-gray-500 hover:shadow"
                    animate:flip={{ duration: flipDurationMs }}
                    role="button"
                    tabindex="0"
                    ondblclick={() => handleEditRaid(item)}
                >
                    <div class="flex gap-0.5">
                        <p
                            class={`w-10 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs ${item.level ? "text-gray-900" : "text-gray-500"}`}
                        >
                            {item.level ?? "레벨"}
                        </p>
                        <p
                            class={`w-28 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs ${item.raidName ? "text-gray-900" : "text-gray-500"}`}
                        >
                            {item.raidName ?? "레이드명"}
                        </p>
                        <p
                            class={`w-14 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs ${item.difficulty ? "text-gray-900" : "text-gray-500"}`}
                        >
                            {item.difficulty ?? "난이도"}
                        </p>
                        <p
                            class={`w-10 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs ${item.gate ? "text-gray-900" : "text-gray-500"}`}
                        >
                            {item.gate ?? "관문"}
                        </p>
                        <p
                            class={`w-12 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs ${item.reward ? "text-gray-900" : "text-gray-500"}`}
                        >
                            {item.reward ?? "골드"}
                        </p>
                        <p
                            class={`w-7 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs ${item.resetType ? "text-gray-900" : "text-gray-500"}`}
                        >
                            {item.resetType ?? "타입"}
                        </p>
                    </div>
                </div>
            {/each}
        </section>
    </div>
    {#if groupEditModalOpen}
        <GroupEditModal
            isOpen={groupEditModalOpen}
            {closeGroupEditModal}
            bind:item={selectGroupRaid}
            {groupRaidTable}
            {reloadAllTable}
            {handleSaveRaid}
        />
    {/if}
</div>
