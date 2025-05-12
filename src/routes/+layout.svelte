<script lang="ts">
    import "@fontsource-variable/noto-sans-kr";
    import "@fontsource/nanum-gothic-coding";
    import "../app.css";
    import { getAllWindows, getCurrentWindow } from "@tauri-apps/api/window";
    import { onDestroy } from "svelte";

    import { appStore } from "../stores/appStore";

    import { UserSettingsType } from "$lib/types";

    const _appWindow = getCurrentWindow();
    const _allWindow = getAllWindows();
    let userSettings: UserSettingsType = $state(new UserSettingsType());

    // + Subscribe to store
    const unsubscribe = appStore.subscribe((value) => {
        userSettings = value.userSettings;
    });

    $effect.pre(() => {
        console.log(
            "%c+layout.svelte $effect.pre",
            "color:white; font-style:bold; background-color:blue; padding:3px; border-radius:4px; font-size:12px;"
        );
        // console.log(appWindow);
        // console.log(allWindow);
    });

    onDestroy(() => {
        unsubscribe(); // ! Cleanup on unmount
    });

    let isHovered: boolean = $state(false);
    function handleMouseEnter() {
        if (!userSettings.focus_border_enabled) return;
        isHovered = true;
        setTimeout(() => (isHovered = false), 500); // ? 0.5초 후 테두리 제거
    }
</script>

<svelte:window on:contextmenu|preventDefault />
<!-- svelte-ignore slot_element_deprecated -->
<div role="presentation" class="container flex h-screen flex-auto flex-col" onmouseenter={handleMouseEnter}>
    <slot />
    <div class={`${isHovered && "border-effect"} container pointer-events-none absolute h-screen`}></div>
</div>

<style>
    .border-effect {
        border: 1px solid transparent;
        pointer-events: none;
        animation: glow 0.5s ease-in-out;
    }

    @keyframes glow {
        0% {
            border-color: rgba(0, 150, 255, 0);
            box-shadow: 0 0 0px rgba(0, 150, 255, 0);
            opacity: 0;
        }
        20% {
            border-color: rgba(0, 150, 255, 0.5);
            box-shadow: 0 0 8px rgba(0, 150, 255, 0.5);
            opacity: 0.8;
        }
        50% {
            border-color: rgba(0, 150, 255, 0.9);
            box-shadow: 0 0 15px rgba(0, 150, 255, 1);
            opacity: 1;
        }
        80% {
            border-color: rgba(0, 150, 255, 0.5);
            box-shadow: 0 0 8px rgba(0, 150, 255, 0.5);
            opacity: 0.8;
        }
        100% {
            border-color: rgba(0, 150, 255, 0);
            box-shadow: 0 0 0px rgba(0, 150, 255, 0);
            opacity: 0;
        }
    }
</style>
