<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import Beach_Regular from "svelte-fluentui-icons/icons/Beach_Regular.svelte";

    document.body.className = "antialiased bg-white text-slate-600 text-base";

    let title = "Tauri Template";
    let version = "0.1.0";

    appWindow.setTitle(title + " " + version);
</script>

<div class="flex items-center h-screen place-content-center">
    <div class="flex flex-col items-center space-y-6">
        <Beach_Regular size={180} class="text-blue-600" />
        <h1 class="text-4xl font-bold">Hello Tauri</h1>
        <p>Vite + Svelte + TS + Tauri + Tailwindcss</p>
    </div>
</div>
