<script lang="ts">
    import { platform } from "@tauri-apps/api/os";
    import { appWindow } from "@tauri-apps/api/window";
    import Dismiss_Regular from "svelte-fluentui-icons/icons/Dismiss_Regular.svelte";
    import Square_Regular from "svelte-fluentui-icons/icons/Square_Regular.svelte";
    import SquareMultiple_Regular from "svelte-fluentui-icons/icons/SquareMultiple_Regular.svelte";
    import LineHorizontal1_Regular from "svelte-fluentui-icons/icons/LineHorizontal1_Regular.svelte";
    import AppContent from "./AppContent.svelte";
    import { onMount } from "svelte";

    document.body.className =
        "antialiased bg-white text-slate-600 text-base select-none w-screen h-screen overflow-hidden";

    if (import.meta.env.DEV !== true)
        document.body.addEventListener("contextmenu", (e) =>
            e.preventDefault()
        );

    let title = "Tauri Template";
    let version = "0.1.0";

    appWindow.setTitle(title + " " + version);

    let isMaximized = false;

    function minimize() {
        appWindow.minimize();
    }
    function maximize() {
        appWindow.toggleMaximize();
        isMaximized = !isMaximized;
    }
    function close() {
        appWindow.close();
    }

    let ismacOS: Boolean = null;

    onMount(async () => {
        var x = await platform();
        ismacOS = x === "darwin";
    });
</script>

{#if ismacOS === true}
    <div class="w-screen h-screen overflow-hidden">
        <AppContent />
    </div>
{:else if ismacOS === false}
    <div class="flex flex-col w-screen h-screen">
        <div class="flex bg-slate-100">
            <div
                data-tauri-drag-region
                class="flex-1 mx-2 my-1.5 text-xs font-bold"
            >
                {title}
            </div>
            <div class="flex">
                <div
                    on:click={() => minimize()}
                    class="flex items-center justify-center w-8 bg-slate-200[0]
                hover:bg-slate-300 active:bg-slate-200"
                >
                    <LineHorizontal1_Regular class="mt-1" size={14} />
                </div>
                <div
                    on:click={() => maximize()}
                    class="flex items-center justify-center w-8
        bg-slate-200[0] hover:bg-slate-300 active:bg-slate-200"
                >
                    {#if isMaximized}
                        <SquareMultiple_Regular size={16} />
                    {:else}
                        <Square_Regular size={16} />
                    {/if}
                </div>
                <div
                    on:click={() => close()}
                    class="flex items-center justify-center w-8
    bg-slate-200[0] hover:bg-slate-300 active:bg-slate-200"
                >
                    <Dismiss_Regular size={18} />
                </div>
            </div>
        </div>
        <div class="flex-1 w-full h-full overflow-hidden">
            <AppContent />
        </div>
    </div>
{/if}

<style>
    :global(img) {
        pointer-events: none;
    }
</style>
