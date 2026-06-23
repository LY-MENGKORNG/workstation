<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import "../app.css";
	import { ModeWatcher } from "mode-watcher";
	import AppSidebar from "$lib/components/AppSidebar.svelte";
	let { children } = $props();
</script>

<ModeWatcher defaultMode="system" />

<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Inset>
		<div class="flex items-center gap-2 border-b px-4 py-2">
			<Sidebar.Trigger />
		</div>
		<main class="flex-1 p-4 pt-8">
			{@render children?.()}
		</main>
	</Sidebar.Inset>
</Sidebar.Provider>
