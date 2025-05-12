<script lang="ts">
    import { message } from "@tauri-apps/plugin-dialog";
    import Database from "@tauri-apps/plugin-sql";
    import { UploadSolid, UserAddSolid, UserRemoveOutline } from "flowbite-svelte-icons";
    import { flip } from "svelte/animate";
    import { dndzone, type DndEvent } from "svelte-dnd-action";

    import ClassList from "$lib/data/class_list.json";
    import ColorList from "$lib/data/color_list.json";
    import type { CharacterType } from "$lib/types";
    import { generateUniqueId } from "$lib/utils/utils";

    const flipDurationMs = 200; // * dnd duration 딜레이

    // ? Component props
    let {
        liveDbName = $bindable(),
        charTable = $bindable(),
        reloadAllTable
    }: {
        liveDbName: string;
        charTable: CharacterType[];
        reloadAllTable: () => Promise<void>;
    } = $props();

    $effect(() => {
        // console.log($inspect("charTable", charTable));
    });

    // + 캐릭터 행 순서 Moved
    function handleDndConsider(e: CustomEvent<DndEvent<CharacterType>>) {
        console.log("Character Dnd Moved");
        charTable = e.detail.items;
    }

    // + 캐릭터 행 순서 Move End
    function handleDndFinalize(e: CustomEvent<DndEvent<CharacterType>>) {
        console.log("Character Dnd Move End");
        charTable = e.detail.items;
    }

    // + 캐릭터 행 추가 핸들러
    function handleAddChar() {
        const charIdx = charTable.length + 1;
        const uniqueId = generateUniqueId();
        console.log(uniqueId);

        charTable.push({
            charId: uniqueId,
            priority: charIdx,
            charName: "",
            class: "",
            color: "slate",
            id: uniqueId
        });
    }

    // + 캐릭터 행 삭제 핸들러
    function handleDelChar(item: CharacterType) {
        // console.log(item);

        const targetIndex = charTable.findIndex((e) => e.charId === item.charId);
        console.log(targetIndex);

        charTable.splice(targetIndex, 1);
        console.log(charTable);
    }

    // + 캐릭터 리스트 저장 핸들러
    async function handleSaveChar() {
        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            await db.execute("BEGIN TRANSACTION");

            await db.execute(`DELETE FROM characters`);

            let index: number = 1;
            for (const item of charTable) {
                if (!item.charName || !item.class || !item.color) continue;

                item.priority = index;

                await db.execute(
                    "INSERT INTO characters (charId, priority, charName, class, color) VALUES ($1, $2, $3, $4, $5)",
                    [item.charId, item.priority, item.charName, item.class, item.color]
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
                "캐릭터 데이터를 저장하는 중 문제가 발생했습니다.\n데이터를 확인한 후, 문제가 되는 항목을 수정하거나 제거한 뒤 다시 시도해 주세요.\n\n(문제가 계속 발생할 경우, 프로그램을 재시작하거나 개발자에게 문의해 주세요.)",
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
</script>

<div class="p-1">
    <div class="flex gap-1 text-center">
        <p class="mb-1 text-sm">※ 캐릭터 리스트</p>
        <button onclick={handleAddChar}>
            <UserAddSolid />
        </button>
        <button onclick={handleSaveChar}>
            <UploadSolid />
        </button>
    </div>
    <div class="h-auto max-h-44 w-auto overflow-y-auto">
        <section
            class="space-y-1"
            onconsider={handleDndConsider}
            onfinalize={handleDndFinalize}
            use:dndzone={{ items: charTable, flipDurationMs, dropFromOthersDisabled: true }}
        >
            {#each charTable as row (row.id)}
                <div
                    class="mr-0.5 flex flex-auto items-center justify-between rounded-sm bg-zinc-400 p-0.5 pl-1 text-[0.7rem] font-black leading-[1-rem] hover:bg-gray-500 hover:shadow"
                    animate:flip={{ duration: flipDurationMs }}
                >
                    <div>
                        <input
                            class={`w-28 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-${row.color}-500 text-shadow`}
                            bind:value={row.charName}
                        />

                        <select
                            class="w-auto rounded-sm bg-slate-100 p-0 !pr-8 pl-0.5 text-xs text-gray-900"
                            name="class"
                            id="class"
                            bind:value={row.class}
                        >
                            {#each ClassList as item}
                                <option class="font-bold text-black" value={item}>{item}</option>
                            {/each}
                        </select>

                        <select
                            class={`w-auto rounded-sm bg-slate-100 p-0 !pr-8 pl-0.5 text-xs text-${row.color}-500 text-shadow`}
                            name="color"
                            id="color"
                            bind:value={row.color}
                        >
                            {#each ColorList as item}
                                <option class="font-bold text-black" value={item}>{item}</option>
                            {/each}
                        </select>
                    </div>
                    <button class="right-0.5 mr-1" onclick={() => handleDelChar(row)}>
                        <UserRemoveOutline class="remove-btn" color="#ff0000" />
                    </button>
                </div>
            {/each}
        </section>
    </div>
</div>
