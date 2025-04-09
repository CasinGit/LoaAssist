<script lang="ts">
    type Anser1 = "reset" | "cancel";
    type Anser2 = "twoWeek" | "week";

    import { CloseOutline } from "flowbite-svelte-icons";
    import { onDestroy, onMount } from "svelte";

    import { invoke } from "$lib/utils/invoke";

    const playSystemSound = () => invoke("play_system_sound", { sound: "Background" });

    let {
        isOpen,
        closeWorkResetModal,
        workSheetReset
    }: {
        isOpen: boolean;
        closeWorkResetModal: () => void;
        workSheetReset: (resetType: "twoWeek" | "week") => Promise<void>;
    } = $props();

    let step: number = $state(0);
    let isBlinking: boolean = $state(false);

    let anser1: Anser1 = $state("cancel");
    let anser1_btn1: HTMLButtonElement | null = $state(null);
    let anser1_btn2: HTMLButtonElement | null = $state(null);

    let anser2: Anser2 = $state("week");
    let anser2_btn1: HTMLButtonElement | null = $state(null);
    let anser2_btn2: HTMLButtonElement | null = $state(null);

    $effect.pre(() => {
        // console.log(anser1);
        // console.log(anser2);
    });

    onMount(() => {
        playSystemSound();
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });

    function handleKeydown(e: KeyboardEvent) {
        console.log(e.key);
        if (e.key === "Tab") {
            if (step === 0) {
                // ? Modal Step이 0 이라면 실행
                if (anser1 === "cancel") {
                    anser1 = "reset";
                    anser1_btn1?.focus();
                } else {
                    anser1 = "cancel";
                    anser1_btn2?.focus();
                }
            }
            if (step === 1) {
                // ? Modal Step이 1 이라면 실행
                if (anser2 === "week") {
                    anser2 = "twoWeek";
                    anser2_btn2?.focus();
                } else {
                    anser2 = "week";
                    anser2_btn1?.focus();
                }
            }
        } else if (e.key === "Escape") {
            closeWorkResetModal();
        } else if (e.key === "Enter") {
            if (step === 0) {
                selectAnser1(anser1);
            } else if (step === 1) {
                selectAnser2(anser2);
            }
        }
    }

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

    function backdropClick() {
        playSystemSound();
        blinkModal();
    }

    function selectAnser1(anser: Anser1) {
        if (anser === "cancel") return closeWorkResetModal();
        anser1 = anser;
        step++;
        playSystemSound();
    }

    async function selectAnser2(anser: Anser2) {
        anser2 = anser;
        console.log(anser1);
        console.log(anser2);
        await workSheetReset(anser);
        closeWorkResetModal();
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
    <div class="mb-2 flex flex-1 justify-between">
        <h1 class="text-base">초기화</h1>
        <CloseOutline class="cursor-pointer" onclick={closeWorkResetModal} />
    </div>
    {#if step === 0}
        <p class="mb-3 text-sm">숙제표를 수동으로 초기화 하시겠습니까?</p>
        <div class="flex justify-end gap-2">
            <button
                class={`${anser1 === "reset" && "ring-[3px]"} focus group relative inline-flex items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-cyan-500 to-blue-500 p-0.5 text-sm font-medium text-gray-900 hover:text-white focus:outline-none focus:ring-4 focus:ring-cyan-200 group-hover:from-cyan-500 group-hover:to-blue-500`}
                bind:this={anser1_btn1}
                onclick={() => selectAnser1("reset")}
            >
                <span
                    class="relative rounded-md bg-white px-2 py-1 text-black transition-all duration-75 ease-in group-hover:bg-transparent group-hover:text-white"
                >
                    초기화
                </span>
            </button>
            <button
                class={`${anser1 === "cancel" && "ring-[3px]"} focus group relative inline-flex items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-cyan-500 to-blue-500 p-0.5 text-sm font-medium text-gray-900 hover:text-white focus:outline-none focus:ring-4 focus:ring-cyan-200 group-hover:from-cyan-500 group-hover:to-blue-500`}
                bind:this={anser1_btn2}
                onclick={() => selectAnser1("cancel")}
            >
                <span
                    class="relative rounded-md bg-white px-2 py-1 text-black transition-all duration-75 ease-in group-hover:bg-transparent group-hover:text-white"
                >
                    취소
                </span>
            </button>
        </div>
    {/if}
    {#if step === 1}
        <p class="mb-3 text-sm">2주 쿨타임 레이드도 초기화 하시겠습니까?</p>
        <div class="flex justify-end gap-2">
            <button
                class={`${anser2 === "week" && "ring-[3px]"} focus group relative inline-flex items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-cyan-500 to-blue-500 p-0.5 text-sm font-medium text-gray-900 hover:text-white focus:outline-none focus:ring-4 focus:ring-cyan-200 group-hover:from-cyan-500 group-hover:to-blue-500`}
                bind:this={anser2_btn1}
                onclick={() => selectAnser2("week")}
            >
                <span
                    class="relative rounded-md bg-white px-2 py-1 text-black transition-all duration-75 ease-in group-hover:bg-transparent group-hover:text-white"
                >
                    아니오
                </span>
            </button>
            <button
                class={`${anser2 === "twoWeek" && "ring-[3px]"} focus group relative inline-flex items-center justify-center overflow-hidden rounded-lg bg-gradient-to-br from-cyan-500 to-blue-500 p-0.5 text-sm font-medium text-gray-900 hover:text-white focus:outline-none focus:ring-4 focus:ring-cyan-200 group-hover:from-cyan-500 group-hover:to-blue-500`}
                bind:this={anser2_btn2}
                onclick={() => selectAnser2("twoWeek")}
            >
                <span
                    class="relative rounded-md bg-white px-2 py-1 text-black transition-all duration-75 ease-in group-hover:bg-transparent group-hover:text-white"
                >
                    예
                </span>
            </button>
        </div>
    {/if}
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
        width: 280px;
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
