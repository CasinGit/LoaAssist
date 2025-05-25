<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import Database from "@tauri-apps/plugin-sql";
    import { Checkbox } from "flowbite-svelte";
    import { onDestroy, onMount } from "svelte";
    import { flip } from "svelte/animate";
    import { dndzone } from "svelte-dnd-action";

    import { appStore, loadLiveDB, setGold } from "../../stores/appStore";

    import { ClassType, UserSettingsType, type ExtendsRaidType } from "$lib/types";
    import { invoke } from "$lib/utils/invoke";

    const flipDurationMs = 200; // * dnd duration 딜레이
    let liveDbName: string = $state("");
    let raidsTable: ExtendsRaidType[] = $state([]);
    let userSettings: UserSettingsType = $state(new UserSettingsType());

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        liveDbName = value.liveDbName;
        raidsTable = value.raidsTable;
        userSettings = value.userSettings;
    });

    let unsubscribeListen: () => void;
    listen("on:reloadTable", (event) => {
        console.log("수신한 데이터:", event.payload);
        if ((event.payload as { table: string }).table === "raidsTable") {
            loadLiveDB();
        }
    }).then((unlisten) => {
        unsubscribeListen = unlisten;
    });

    $effect(() => {
        // console.log($inspect(raidsTable));
    });

    onMount(() => {
        try {
            loadLiveDB();
        } catch (error) {
            console.log(error);
        }
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on WorkSheet.svelte component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        unsubscribe(); // ! Cleanup on unmount
        unsubscribeListen(); // ! Cleanup on Listen Event
    });

    // + 숙제 체크 핸들러
    async function handleOnChange(e: any, item: ExtendsRaidType) {
        // console.log("체크 실행됨", e.target.checked, item);

        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            await db.execute(`UPDATE live_raids SET complete = ? WHERE id = ?`, [Number(item.complete), item.id]);

            await loadLiveDB(true);

            if (item.complete) {
                const gold = await invoke("increment_gold", { value: item.reward! });
                setGold(gold);
            } else {
                const gold = await invoke("decrement_gold", { value: item.reward! });
                setGold(gold);
            }
        } catch (error) {
            console.error("Transaction failed:", error);
        } finally {
            await db.close();
        }
    }
</script>

<section class="space-y-1" use:dndzone={{ items: raidsTable, flipDurationMs, dragDisabled: true }}>
    {#each raidsTable as item (item.id)}
        <div
            class="group flex items-center rounded-md bg-zinc-400 p-0.5 text-[0.7rem] font-black leading-[1-rem] text-gray-900 hover:bg-gray-500 hover:shadow"
            animate:flip={{ duration: flipDurationMs }}
        >
            <Checkbox
                color="teal"
                class="ml-[0.05rem] mr-1 size-3"
                bind:checked={item.complete}
                on:change={(e) => handleOnChange(e, item)}
            />
            {#if userSettings.class_image}
                <img class="mr-0.5 size-4" src={`/images/classes/${ClassType[item.class as any]}.svg`} alt="class" />
            {/if}
            <p class={`text-${item.color}-500 text-shadow mr-1`}>{item.charName}</p>
            <p class="mr-1 text-black">{item.raidName} {item.difficulty}</p>
            <p class="mr-1 text-black">{item.gate}관문</p>
            <p class="text-shadow mr-1 flex flex-auto justify-end text-right text-amber-300">{item.reward}</p>
        </div>
    {/each}
</section>
