<script lang="ts">
    import { emit } from "@tauri-apps/api/event";
    import Database from "@tauri-apps/plugin-sql";
    import { onDestroy, onMount } from "svelte";

    import { appStore, loadLiveDB } from "../../stores/appStore";

    import CharacterList from "$lib/components/settings/CharacterList.svelte";
    import CustomRaidList from "$lib/components/settings/CustomRaidList.svelte";
    import WorkList from "$lib/components/settings/WorkList.svelte";
    import type { CharacterType, ExtendsRaidType, RaidType } from "$lib/types";

    let liveDbName: string = $state("");
    let charTable: CharacterType[] = $state([]);
    let customRaidTable: RaidType[] = $state([]);
    let unionRaidTable: RaidType[] = $state([]);
    let raidsTable: ExtendsRaidType[] = $state([]);

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        liveDbName = value.liveDbName;
        raidsTable = value.raidsTable;
    });

    $effect(() => {
        // console.log($inspect(liveDB));
        // console.log($inspect(charDB));
        // console.log($inspect(customRaidTable));
        // console.log($inspect(unionRaidTable));
    });

    onMount(() => {
        console.log(
            "%csettings +page.svelte onMount",
            "color:white; font-style:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
        );

        reloadAllTable();
    });

    onDestroy(() => {
        console.log(
            "%cCleanup completed on component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        unsubscribe(); // ! Cleanup on unmount
    });

    async function reloadAllTable() {
        await loadLiveDB(); // ! 이 구문이 필요한지 나중에 검증 필요함

        const db = await Database.load(`sqlite:${liveDbName}.db`);
        try {
            charTable = await db.select("SELECT * FROM characters ORDER BY priority");
            charTable = charTable.map((item) => ({ ...item, id: item.charId }));

            customRaidTable = await db.select("SELECT * FROM custom_raids ORDER BY priority");
            customRaidTable = customRaidTable.map((item) => ({ ...item, id: item.raidId }));

            unionRaidTable = await db.select(`
                SELECT *, 'default' AS source FROM default_raids
                UNION
                SELECT *, 'custom' AS source FROM custom_raids
                ORDER BY source, priority
            `);
            unionRaidTable = unionRaidTable.map((item) => ({ ...item, id: item.raidId }));
        } catch (error) {
            console.error("Transaction failed:", error);
        } finally {
            await db.close();
            emit("on:reloadTable", { table: "raidsTable" });
        }
    }
</script>

<div class="flex flex-auto flex-col overflow-y-auto bg-neutral-800 bg-opacity-70">
    <CharacterList bind:liveDbName bind:charTable {reloadAllTable} />

    <CustomRaidList bind:liveDbName bind:customRaidTable {reloadAllTable} />

    <WorkList bind:raidsTable bind:liveDbName bind:charTable bind:unionRaidTable {reloadAllTable} />
</div>
