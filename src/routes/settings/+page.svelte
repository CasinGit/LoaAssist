<script lang="ts">
    import { emit } from "@tauri-apps/api/event";
    import Database from "@tauri-apps/plugin-sql";
    import { onDestroy, onMount } from "svelte";

    import { appStore, loadLiveDB } from "../../stores/appStore";

    import CharacterList from "$lib/components/settings/CharacterList.svelte";
    import GroupRaidList from "$lib/components/settings/GroupRaidList.svelte";
    import WorkList from "$lib/components/settings/WorkList.svelte";
    import type { CharacterType, ExtendsRaidType, RaidType } from "$lib/types";

    let liveDbName: string = $state("");
    let charTable: CharacterType[] = $state([]);
    let groupRaidTable: RaidType[] = $state([]);
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
        // console.log($inspect(groupRaidTable));
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

            groupRaidTable = await db.select(`
                SELECT 
                    cr.*, 
                    COALESCE(SUM(dr.reward), 0) AS reward
                FROM 
                    group_raids AS cr
                LEFT JOIN 
                    group_raids_map AS crm ON cr.raidId = crm.groupId
                LEFT JOIN 
                    default_raids AS dr ON crm.raidId = dr.raidId
                GROUP BY 
                    cr.raidId, cr.priority, cr.level, cr.raidName, 
                    cr.difficulty, cr.gate, cr.reward, cr.resetType
                ORDER BY 
                    cr.priority;
            `);
            groupRaidTable = groupRaidTable.map((item) => ({ ...item, id: item.raidId }));

            unionRaidTable = await db.select(`
                SELECT 
                    dr.raidId,
                    dr.priority,
                    dr.level,
                    dr.raidName,
                    dr.difficulty,
                    dr.gate,
                    dr.reward,
                    dr.resetType,
                    'default' AS source,
                    1 AS sort_order
                FROM 
                    default_raids AS dr

                UNION ALL

                SELECT 
                    cr.raidId,
                    cr.priority,
                    cr.level,
                    cr.raidName,
                    cr.difficulty,
                    cr.gate,
                    COALESCE(SUM(dr.reward), 0) AS reward,
                    cr.resetType,
                    'group' AS source,
                    0 AS sort_order
                FROM 
                    group_raids AS cr
                LEFT JOIN 
                    group_raids_map AS crm ON cr.raidId = crm.groupId
                LEFT JOIN 
                    default_raids AS dr ON crm.raidId = dr.raidId
                GROUP BY 
                    cr.raidId, cr.priority, cr.level, cr.raidName, 
                    cr.difficulty, cr.gate, cr.resetType
                ORDER BY 
                    sort_order, priority;
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

    <GroupRaidList bind:liveDbName bind:groupRaidTable {reloadAllTable} />

    <WorkList bind:raidsTable bind:liveDbName bind:charTable bind:unionRaidTable {reloadAllTable} />
</div>
