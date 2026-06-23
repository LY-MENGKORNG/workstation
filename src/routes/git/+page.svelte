<script lang="ts">
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import * as Item from "$lib/components/ui/item";
	import * as C from "$lib/components/ui/card";
	import * as D from "$lib/components/ui/dialog";
	import {
		readGitConfig,
		parseGitHosts,
		testGitConn,
		type GitConfig,
		type GitHost,
	} from "$lib/hooks/shell";
	import { CircleCheck, CircleX, RotateCcw, GitBranch } from "@lucide/svelte/icons";
	import { onMount } from "svelte";
	import Input from "$lib/components/ui/input/input.svelte";

	let gitConfig = $state<GitConfig>({ entries: [], raw: "" });
	let gitHosts = $state<GitHost[]>([]);
	let connectionResults = $state<Record<string, boolean | null>>({});
	let selectedHost = $state<string | null>(null);

	onMount(async () => {
		gitConfig = await readGitConfig();
		gitHosts = await parseGitHosts();
		for (const host of gitHosts) {
			connectionResults[host.host] = null;
		}
	});

	async function handleTestConn(host: string) {
		connectionResults[host] = null;
		const result = await testGitConn(host);
		connectionResults[host] = result;
	}

	async function handleTestAll() {
		for (const host of gitHosts) {
			connectionResults[host.host] = null;
		}
		for (const host of gitHosts) {
			const result = await testGitConn(host.host);
			connectionResults[host.host] = result;
		}
	}

	const userEntries = $derived(
		gitConfig.entries.filter((e) => e.section === "user")
	);
	const coreEntries = $derived(
		gitConfig.entries.filter((e) => e.section === "core")
	);
	const otherEntries = $derived(
		gitConfig.entries.filter(
			(e) => e.section !== "user" && e.section !== "core"
		)
	);

	$effect(() => {
		// Reactively respond to connection results
	});
</script>

<div class="flex flex-col w-full max-w-5xl mx-auto space-y-6">
	<div class="flex items-center justify-between w-full">
		<h1 class="text-3xl">Git Configuration</h1>
		<Button onclick={handleTestAll} variant="outline">
			<RotateCcw class="size-4 mr-1" />
			Test All Connections
		</Button>
	</div>

	<!-- User Config -->
	<C.Card class="w-full bg-transparent rounded-none">
		<C.CardHeader>
			<C.CardTitle>User Configuration</C.CardTitle>
			<C.CardDescription>Global git user settings</C.CardDescription>
		</C.CardHeader>
		<C.CardContent>
			<Item.Group>
				{#each userEntries as entry (entry.key)}
					<Item.Root variant="outline">
						<Item.Content>
							<Item.Title class="font-mono text-sm">
								{entry.key}
							</Item.Title>
							<Item.Description class="text-xs text-muted-foreground">
								{entry.value}
							</Item.Description>
						</Item.Content>
						<Item.Actions>
							<Input value={entry.value} class="h-8 w-48 text-sm" />
						</Item.Actions>
					</Item.Root>
				{/each}
				{#if userEntries.length === 0}
					<Item.Root variant="outline" class="border-dashed">
						<Item.Content>
							<Item.Title class="text-muted-foreground text-sm">
								No user config found. Set up with:
								<code class="text-xs ml-1">git config --global user.name "Your Name"</code>
							</Item.Title>
						</Item.Content>
					</Item.Root>
				{/if}
			</Item.Group>
		</C.CardContent>
	</C.Card>

	<!-- Git Hosts -->
	<C.Card class="w-full bg-transparent rounded-none">
		<C.CardHeader>
			<div class="flex items-center justify-between">
				<div>
					<C.CardTitle>Remote Hosts</C.CardTitle>
					<C.CardDescription>SSH hosts configured for git operations</C.CardDescription>
				</div>
				<GitBranch class="size-5 text-muted-foreground" />
			</div>
		</C.CardHeader>
		<C.CardContent>
			<Item.Group>
				{#each gitHosts as host (host.host)}
					<Item.Root variant="outline">
						<Item.Content>
							<div class="flex items-center gap-3">
								<div
									class="flex items-center justify-center w-5 h-5 rounded-full border transition-colors
										{connectionResults[host.host] === true
											? 'border-green-500 text-green-500'
											: connectionResults[host.host] === false
												? 'border-red-500 text-red-500'
												: 'border-muted-foreground/30 text-muted-foreground/30'}"
								>
									{#if connectionResults[host.host] === true}
										<CircleCheck class="size-4" />
									{:else if connectionResults[host.host] === false}
										<CircleX class="size-4" />
									{/if}
								</div>
								<div>
									<Item.Title class="font-mono text-sm">
										{host.host}
									</Item.Title>
									<Item.Description class="text-xs text-muted-foreground">
										{host.hostname} • {host.user}
									</Item.Description>
								</div>
							</div>
						</Item.Content>
						<Item.Actions>
							<span class="text-xs text-muted-foreground font-mono truncate max-w-[150px]">
								{host.identityFile}
							</span>
							<Button
								variant="ghost"
								size="icon"
								class="rounded-full"
								onclick={() => handleTestConn(host.host)}
								title="Test connection"
							>
								<RotateCcw class="size-3.5" />
							</Button>
						</Item.Actions>
					</Item.Root>
				{/each}
				{#if gitHosts.length === 0}
					<Item.Root variant="outline" class="border-dashed">
						<Item.Content>
							<Item.Title class="text-muted-foreground text-sm">
								No SSH hosts configured for git. Add hosts to your ~/.ssh/config
							</Item.Title>
						</Item.Content>
					</Item.Root>
				{/if}
			</Item.Group>
		</C.CardContent>
	</C.Card>

	<!-- Core Config -->
	<C.Card class="w-full bg-transparent rounded-none">
		<C.CardHeader>
			<C.CardTitle>Core Settings</C.CardTitle>
			<C.CardDescription>Git core configuration</C.CardDescription>
		</C.CardHeader>
		<C.CardContent>
			<Item.Group>
				{#each coreEntries as entry (entry.key)}
					<Item.Root variant="outline">
						<Item.Content>
							<Item.Title class="font-mono text-sm">
								{entry.key}
							</Item.Title>
							<Item.Description class="text-xs text-muted-foreground">
								{entry.value}
							</Item.Description>
						</Item.Content>
					</Item.Root>
				{/each}
			</Item.Group>
		</C.CardContent>
	</C.Card>

	<!-- Other config -->
	{#if otherEntries.length > 0}
		<C.Card class="w-full bg-transparent rounded-none">
			<C.CardHeader>
				<C.CardTitle>Other Settings</C.CardTitle>
				<C.CardDescription>Additional git configuration</C.CardDescription>
			</C.CardHeader>
			<C.CardContent>
				<Item.Group>
					{#each otherEntries as entry (entry.key)}
						<Item.Root variant="outline">
							<Item.Content>
								<Item.Title class="font-mono text-sm">
									{entry.key}
								</Item.Title>
								<Item.Description class="text-xs text-muted-foreground">
									{entry.value}
								</Item.Description>
							</Item.Content>
						</Item.Root>
					{/each}
				</Item.Group>
			</C.CardContent>
		</C.Card>
	{/if}

	<!-- Config Details Dialog -->
	<D.Root>
		<D.Trigger type="button" class={buttonVariants({ variant: "ghost", class: "w-full mt-2" })}>
			View Raw Config
		</D.Trigger>
		<D.Content class="max-w-2xl">
			<D.Header>
				<D.Title>Raw Git Config</D.Title>
				<D.Description>Global git configuration (~/.gitconfig)</D.Description>
			</D.Header>
			<pre class="bg-muted p-4 rounded-md text-xs font-mono overflow-auto max-h-[400px] whitespace-pre-wrap">
				{gitConfig.raw || "No global git config found"}
			</pre>
			<D.Footer>
				<D.Close class={buttonVariants({ variant: "outline" })}>Close</D.Close>
			</D.Footer>
		</D.Content>
	</D.Root>
</div>
