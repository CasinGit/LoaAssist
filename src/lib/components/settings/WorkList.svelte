<script lang="ts">
    import { message } from "@tauri-apps/plugin-dialog";
    import Database from "@tauri-apps/plugin-sql";
    import { GridPlusSolid, UploadSolid, UserRemoveOutline } from "flowbite-svelte-icons";
    import { flip } from "svelte/animate";
    import { dndzone, type DndEvent } from "svelte-dnd-action";

    import { type CharacterType, type ExtendsRaidType, type RaidType } from "$lib/types";

    const flipDurationMs = 200; // * dnd duration 딜레이

    // ? Component props
    let {
        raidsTable = $bindable(),
        liveDbName = $bindable(),
        charTable = $bindable(),
        unionRaidTable = $bindable(),
        reloadAllTable
    }: {
        raidsTable: ExtendsRaidType[];
        liveDbName: string;
        charTable: CharacterType[];
        unionRaidTable: RaidType[];
        reloadAllTable: () => Promise<void>;
    } = $props();

    $effect(() => {
        // console.log($inspect("raidsTable", raidsTable));
        // console.log($inspect("charTable", charTable));
        // console.log($inspect("unionRaidTable", unionRaidTable));
    });

    // + 레이드 행 순서 Moved
    function handleDndConsider(e: CustomEvent<DndEvent<ExtendsRaidType>>) {
        console.log("Raid Dnd Moved");
        raidsTable = e.detail.items;
    }

    // + 레이드 행 순서 Move End
    async function handleDndFinalize(e: CustomEvent<DndEvent<ExtendsRaidType>>) {
        console.log("Raid Dnd Move End");
        raidsTable = e.detail.items;
    }

    // + 레이드 행 추가 핸들러
    async function handleAddRaid() {
        const raidIdx = raidsTable.length + 1;

        raidsTable.push({
            id: raidIdx,
            charId: -1,
            raidType: "default",
            raidId: -1,
            complete: false,
            swap: 0,
            resetType: 1,
            charName: "",
            class: "",
            color: "",
            difficulty: "",
            gate: "",
            level: 0,
            priority: 0,
            raidName: "",
            reward: 0
        });
    }

    // + 레이드 행 삭제 핸들러
    function handleDelRaid(item: ExtendsRaidType) {
        console.log(item);

        const targetIndex = raidsTable.findIndex((e) => e.id === item.id);
        console.log(targetIndex);

        raidsTable.splice(targetIndex, 1);
        console.log(raidsTable);
    }

    // + 레이드 리스트 저장 핸들러
    async function handleSaveRaid() {
        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            await db.execute("BEGIN TRANSACTION");

            await db.execute(`DELETE FROM live_raids`);

            let index: number = 1;
            for (const item of raidsTable) {
                if (item.charId === -1 || item.raidId === -1) continue;

                await db.execute(
                    `INSERT INTO live_raids (id, charId, raidType, raidId, complete, swap, resetType) VALUES ($1, $2, $3, $4, $5, $6, $7)`,
                    [index, item.charId, item.raidType, item.raidId, Number(item.complete), item.swap, item.resetType]
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
                "레이드 데이터를 저장하는 중 문제가 발생했습니다.\n데이터를 확인한 후, 문제가 되는 항목을 수정하거나 제거한 뒤 다시 시도해 주세요.\n\n(문제가 계속 발생할 경우, 프로그램을 재시작하거나 개발자에게 문의해 주세요.)",
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

    // + 레이드에 지정된 캐릭터 변경 함수
    function updateChar(item: ExtendsRaidType, charId: string) {
        // console.log(item); // ? 업데이트 해야할 데이터
        // console.log(charId); // ? 선택한 캐릭터 charId

        const selected = charTable.find((arr) => arr.charId === Number(charId));
        // console.log(selected);

        if (selected) {
            item.charId = selected.charId;
            item.charName = selected.charName;
            item.class = selected.class;
            item.color = selected.color;
        }
    }

    // + 레이드 변경 함수
    function updateRaid(item: ExtendsRaidType, raidId: string) {
        // console.log(item); // ? 업데이트 해야할 데이터
        // console.log(raidId); // ? 선택한 레이드의 raidId

        const selected = unionRaidTable.find((arr) => arr.raidId === Number(raidId));
        // console.log(selected);

        if (selected) {
            item.raidId = selected.raidId;
            item.priority = selected.priority;
            item.level = selected.level;
            item.raidName = selected.raidName;
            item.difficulty = selected.difficulty;
            item.gate = selected.gate;
            item.reward = selected.reward;
            item.resetType = selected.resetType;
            item.raidType = selected.source;
        }
    }
</script>

<div class="p-1">
    <div class="flex gap-1 text-center">
        <p class="mb-1 text-sm">※ 레이드 추가/순서 변경</p>
        <button onclick={handleAddRaid}>
            <GridPlusSolid />
        </button>
        <button onclick={handleSaveRaid}>
            <UploadSolid />
        </button>
    </div>
    <div class="h-auto max-h-80 w-auto overflow-y-auto">
        <section
            class="space-y-1"
            onconsider={handleDndConsider}
            onfinalize={handleDndFinalize}
            use:dndzone={{ items: raidsTable, flipDurationMs, dropFromOthersDisabled: true }}
        >
            {#each raidsTable as row (row.id)}
                <div
                    class="mr-0.5 flex flex-auto items-center justify-between gap-0.5 rounded-sm bg-zinc-400 p-0.5 pl-1 text-[0.7rem] font-black leading-[1-rem] hover:bg-gray-500 hover:shadow"
                    animate:flip={{ duration: flipDurationMs }}
                >
                    <!--  -->
                    <!-- 캐릭터 선택 -->
                    <select
                        class={`w-auto rounded-sm bg-slate-100 p-0 !pr-8 pl-0.5 text-xs text-${row.color}-500 text-shadow`}
                        name="charSel"
                        id="charSel"
                        bind:value={row.charId}
                        onchange={(e) => updateChar(row, e.currentTarget.value)}
                    >
                        {#each charTable as item}
                            <option class="font-bold text-black" value={item.charId}>{item.charName}</option>
                        {/each}
                    </select>
                    <!--  -->
                    <!-- 레이드 선택 -->
                    <select
                        class={`w-auto rounded-sm bg-slate-100 p-0 !pr-8 pl-0.5 text-xs text-black`}
                        name="raidSel"
                        id="raidSel"
                        bind:value={row.raidId}
                        onchange={(e) => updateRaid(row, e.currentTarget.value)}
                    >
                        {#each unionRaidTable as item}
                            <option
                                class={`font-bold text-black ${item.source === "group" && "bg-slate-400"}`}
                                value={item.raidId}
                            >
                                {item.raidName}
                                {item.difficulty}
                                {item.gate}관문
                            </option>
                        {/each}
                    </select>
                    <p class="text-shadow mr-1 flex flex-auto justify-end text-right text-amber-300">
                        {row.reward}
                    </p>
                    <button class="right-0.5 mr-1" onclick={() => handleDelRaid(row)}>
                        <UserRemoveOutline class="remove-btn" color="#ff0000" />
                    </button>
                </div>
            {/each}
        </section>
    </div>
</div>
