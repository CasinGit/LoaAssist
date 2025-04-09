<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import { appStore } from "../../stores/appStore";

    let gold: number = $state(0);
    let totalReward: number = $state(0);
    let remainingReward: number = $state(0);
    let totalRaids: number = $state(0);

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        gold = value.gold;
        totalReward = value.totalReward;
        remainingReward = value.remainingReward;
        totalRaids = value.totalRaids;
    });

    onMount(() => {});

    onDestroy(() => {
        console.log(
            "%cCleanup completed on Dashboard.svelte component unmount.",
            "color: white; font-style: italic; background-color: red;padding: 3px; border-radius: 4px; font-size:12px"
        );
        unsubscribe(); // ! Cleanup on unmount
    });
</script>

<div>
    <p>레이드 갯수 : {totalRaids}</p>
    <p>현재 골드 : {gold}</p>
    <p>총 골드 : {totalReward}</p>
    <p>남은 골드 : {remainingReward}</p>
</div>
