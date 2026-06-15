<script lang="ts">
  import ModeToggle from "$lib/components/modeToggle.svelte";
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";
  let { children } = $props();
</script>

<ModeWatcher />
<main class="p-4 min-h-screen w-full flex flex-col relative">
  {@render children?.()}
  <ModeToggle class="fixed bottom-4 right-4" />
</main>
