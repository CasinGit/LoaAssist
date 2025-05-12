<script lang="ts">
    import { message } from "@tauri-apps/plugin-dialog";
    import Database from "@tauri-apps/plugin-sql";
    import { Checkbox } from "flowbite-svelte";
    import { CloseOutline, ExclamationCircleOutline } from "flowbite-svelte-icons";
    import { onDestroy, onMount } from "svelte";

    import { liveDbName } from "../../../stores/appStore";

    import type { RaidType } from "$lib/types";
    import { invoke } from "$lib/utils/invoke";

    type RaidWithCheck = RaidType & { checked: boolean };

    const playSystemSound = () => invoke("play_system_sound", { sound: "Background" });

    let {
        isOpen,
        item = $bindable(),
        groupRaidTable = $bindable(),
        closeGroupEditModal,
        reloadAllTable,
        handleSaveRaid
    }: {
        isOpen: boolean;
        item: RaidType;
        groupRaidTable: RaidType[];
        closeGroupEditModal: () => void;
        reloadAllTable: () => Promise<void>;
        handleSaveRaid: () => Promise<void>;
    } = $props();

    let isBlinking: boolean = $state(false);
    let defaultRaids: RaidWithCheck[] = $state([]);

    $effect.pre(() => {
        // $inspect(defaultRaids);

        // * defaultRaids 데이터가 변경될 때마다 골드 합산
        item.reward = defaultRaids.filter((raid) => raid.checked).reduce((sum, raid) => sum + (raid.reward || 0), 0);

        item.level = getHighestLevelFromCheckedRaids(defaultRaids) ?? null;
    });

    onMount(async () => {
        playSystemSound();

        // console.log(item);
        if (!item) closeGroupEditModal();

        const db = await Database.load(`sqlite:${liveDbName}.db`);

        try {
            defaultRaids = await db.select("SELECT * FROM default_raids");

            const selectRaids: Array<{ groupId: number; raidId: number }> = await db.select(
                `SELECT * FROM group_raids_map WHERE groupId = '${item.id}'`
            );
            // console.log(selectRaids);

            if (selectRaids) {
                const selectedRaidIds = new Set(selectRaids.map((r) => r.raidId));

                defaultRaids = defaultRaids.map((raid) => ({
                    ...raid,
                    checked: selectedRaidIds.has(raid.raidId)
                }));
            }
        } catch (error) {
            console.error("Transaction failed:", error);
        } finally {
            await db.close();
        }
    });

    onDestroy(() => {});

    // + 창 blink 효과 함수
    function blinkModal() {
        isBlinking = true;
        setTimeout(() => {
            isBlinking = false;
        }, 150); // 짧은 깜빡임 효과
        setTimeout(() => {
            isBlinking = true;
        }, 300); // 짧은 깜빡임 효과
        setTimeout(() => {
            isBlinking = false;
        }, 450); // 짧은 깜빡임 효과
    }

    // + 백그라운드 클릭 시 실행
    function backdropClick() {
        playSystemSound();
        blinkModal();
    }

    // + 제일 높은 레벨을 추출하는 함수
    function getHighestLevelFromCheckedRaids(raids: RaidWithCheck[]): number | null {
        const levels = raids.filter((r) => r.checked && r.level !== null).map((r) => r.level as number); // 이제 null이 없으므로 as number 가능

        if (levels.length === 0) return null;
        return Math.max(...levels);
    }

    // + 사용자가 체크를 했을 때 실행
    function handleSelectRaid(e: Event, item: RaidWithCheck) {
        const input = e.currentTarget as HTMLInputElement;

        // * defaultRaids 내부에서 해당 raid의 checked 상태를 업데이트
        defaultRaids = defaultRaids.map((raid) =>
            raid.raidId === item.raidId ? { ...raid, checked: input.checked } : raid
        );
        // console.log(defaultRaids);
    }

    // + 그룹 저장 함수
    async function handleGroupSave() {
        if (!item.level || !item.raidName || !item.difficulty || !item.gate || !item.reward) {
            message("레벨, 레이드명, 난이도, 관문, 골드 항목은 비워둘 수 없습니다.", {
                title: "저장 불가",
                kind: "warning"
            });
            return;
        }

        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            // * 체크된 항목 삽입
            await db.execute("BEGIN TRANSACTION");
            await db.execute(`DELETE FROM group_raids_map WHERE groupId = $1`, [item.id]);

            for (const raid of defaultRaids) {
                if (!raid.checked) continue;

                await db.execute(`INSERT INTO group_raids_map (groupId, raidId) VALUES ($1, $2)`, [
                    item.id,
                    raid.raidId
                ]);
            }

            await db.execute("COMMIT");
            console.log("Transaction committed successfully.");

            const existingIndex = groupRaidTable.findIndex((r) => r.raidId === item.raidId);
            console.log(existingIndex);

            if (existingIndex !== -1) {
                // * 이미 존재하면 해당 항목 덮어쓰기만
                groupRaidTable[existingIndex] = { ...item };
            } else {
                // * 없으면 새로 추가
                groupRaidTable.push(item);
            }

            await handleSaveRaid();
        } catch (error) {
            // ! 오류가 발생하면 롤백
            await db.execute("ROLLBACK");
            console.error("Transaction failed and rolled back:", error);
        } finally {
            await db.close();
            closeGroupEditModal();
        }
    }

    // + 그룹 삭제 함수
    async function handleGroupDelete() {
        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            // * 그룹 레이드, 체크 항목 삭제
            await db.execute("BEGIN TRANSACTION");
            await db.execute(`DELETE FROM group_raids_map WHERE groupId = $1`, [item.id]);
            await db.execute(`DELETE FROM group_raids WHERE raidId = $1`, [item.id]);
            await db.execute("COMMIT");
            console.log("Transaction committed successfully.");
        } catch (error) {
            // ! 오류가 발생하면 롤백
            await db.execute("ROLLBACK");
            console.error("Transaction failed and rolled back:", error);
        } finally {
            await db.close();
            await reloadAllTable();
            closeGroupEditModal();
        }
    }
</script>

<div
    class="modal-backdrop"
    role="button"
    tabindex="0"
    onkeydown={(e) => (e.key === "Enter" || e.key === " " ? null : null)}
    onclick={backdropClick}
    class:show={isOpen}
></div>
<div class="modal-container {isBlinking ? 'blink' : ''}" class:show={isOpen}>
    <div class="flex flex-1 justify-between">
        <h1 class="text-base">"{item.id}" 그룹 수정</h1>
        <CloseOutline
            class="cursor-pointer"
            onclick={async () => {
                await reloadAllTable();
                closeGroupEditModal();
            }}
        />
    </div>

    <p class="mb-2 text-xs text-yellow-300">
        ⚠️숙제표 설정에서 저장 버튼을 누르지 않더라도, <span class="font-medium text-green-400">그룹 저장</span>을
        누르면 자동으로 그룹 레이드 정보가 저장됩니다.
    </p>

    <div class="mb-2 flex gap-0.5 font-bold">
        <input
            class={`w-10 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-gray-900 `}
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
            class={`text-shadow w-12 rounded-sm border border-zinc-300 bg-zinc-400 p-0 pl-0.5 text-xs text-amber-400 placeholder:text-red-700`}
            placeholder="골드"
            bind:value={item.reward}
            disabled
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

    <div class="mb-2 rounded-sm bg-zinc-400">
        <ul class="h-48 overflow-y-auto text-sm">
            {#each defaultRaids as item (item.raidId)}
                <li>
                    <div class="m-1 flex items-center rounded-sm hover:bg-zinc-300">
                        <Checkbox
                            id="default-raid-{item.raidId}"
                            color="purple"
                            class="ml-0.5 mr-1 size-2.5"
                            bind:checked={item.checked}
                            onchange={(e) => handleSelectRaid(e, item)}
                        />
                        <label
                            for="default-raid-{item.raidId}"
                            class="ms-1 w-full text-xs font-extrabold text-gray-900"
                        >
                            ({item.level})
                            {item.raidName}
                            {item.difficulty}
                            {item.gate}관문
                        </label>
                        <p class="text-shadow mr-1 ms-1 text-xs font-bold text-amber-300">
                            {item.reward}
                        </p>
                    </div>
                </li>
            {/each}
        </ul>
    </div>

    <div class="flex justify-between">
        <div class="flex gap-2">
            <button
                type="button"
                class="rounded-lg bg-gradient-to-r from-green-400 via-green-500 to-green-600 px-3 py-2 text-center text-sm font-medium text-white shadow-lg shadow-green-500/50 hover:bg-gradient-to-br focus:outline-none focus:ring-2 focus:ring-green-300"
                onclick={handleGroupSave}
            >
                그룹 저장
            </button>
            <button
                type="button"
                class="rounded-lg bg-gradient-to-r from-red-400 via-red-500 to-red-600 px-3 py-2 text-center text-sm font-medium text-white shadow-lg shadow-red-500/50 hover:bg-gradient-to-br focus:outline-none focus:ring-2 focus:ring-red-300"
                ondblclick={handleGroupDelete}
            >
                삭제
            </button>
        </div>
        <button
            type="button"
            class="rounded-lg bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 px-3 py-2 text-center text-sm font-medium text-white shadow-lg shadow-blue-500/50 hover:bg-gradient-to-br focus:outline-none focus:ring-2 focus:ring-blue-300"
            onclick={async () => {
                await reloadAllTable();
                closeGroupEditModal();
            }}
        >
            취소
        </button>
    </div>
</div>

<style>
    .modal-backdrop {
        cursor: no-drop;
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        display: none;
        z-index: 10;
    }
    .modal-container {
        position: fixed;
        width: auto;
        /* height: 400px; */
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: rgba(0, 0, 0, 0.85);
        padding: 5px 10px 10px 10px;
        border-radius: 5px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        z-index: 20;
        display: none;
        color: white;
    }
    .modal-backdrop.show,
    .modal-container.show {
        display: block;
    }
    .modal-container.blink {
        box-shadow: 0px 0px 5px rgba(255, 255, 255, 0.6);
        background-color: rgba(0, 0, 0, 0.75);
    }
</style>
