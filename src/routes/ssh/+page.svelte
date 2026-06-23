<script lang="ts">
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import * as Item from "$lib/components/ui/item";
	import * as D from "$lib/components/ui/dialog";
	import {
		parseSSHConfig,
		readConfigFile,
		listSSHKeyFiles,
		getConfByFilename,
		testConn,
		writeSSHConfig,
		toggleSSHEntry,
		type Conf,
	} from "$lib/hooks/shell";
	import { FilePen, Siren, CircleCheck, CircleX, RotateCcw } from "@lucide/svelte/icons";
	import { onMount } from "svelte";
	import * as C from "$lib/components/ui/card";
	import Input from "$lib/components/ui/input/input.svelte";
	import Textarea from "$lib/components/ui/textarea/textarea.svelte";

	type ConfResult = {
		connected: boolean;
		error?: string;
	} & Conf;

	let keyFiles = $state<string[]>([]);
	let sshConfig = $state<Conf[]>([]);
	let configText = $state<string>("");
	let conf = $state<Conf>();
	let confResults = $state<ConfResult[]>([]);

	onMount(async () => {
		keyFiles = await listSSHKeyFiles();
		configText = await readConfigFile();
		sshConfig = await parseSSHConfig(await readConfigFile());
		conf = await getConfByFilename(keyFiles[0]);

		for (const keyFile of sshConfig) {
			const result = (await testConn(keyFile.host)) as ConfResult;
			confResults.push(result);
		}
	});

	async function handleSaveConfig() {
		await writeSSHConfig(configText);
		sshConfig = await parseSSHConfig(await readConfigFile());
	}

	async function handleToggle(host: string, enabled: boolean) {
		await toggleSSHEntry(host, enabled);
		configText = await readConfigFile();
		sshConfig = await parseSSHConfig(configText);
	}

	async function handleRetest(index: number) {
		const result = (await testConn(sshConfig[index].host)) as ConfResult;
		confResults[index] = result;
	}
</script>

<div class="flex flex-col w-full max-w-5xl mx-auto space-y-4">
	<div class="flex items-center justify-between w-full">
		<h1 class="text-3xl">SSH Configuration</h1>
		<div class="flex items-center gap-2">
			<D.Root>
				<form>
					<D.Trigger
						type="button"
						class={buttonVariants({ variant: "outline" })}
					>
						Edit Config
					</D.Trigger>
					<D.Content class="max-w-2xl">
						<D.Header>
							<D.Title>Edit SSH Config</D.Title>
							<D.Description>
								Edit your SSH configuration file (~/.ssh/config).
							</D.Description>
						</D.Header>
						<div>
							<Textarea bind:value={configText} class="min-h-[300px] font-mono text-sm" />
						</div>
						<D.Footer>
							<D.Close
								type="button"
								class={buttonVariants({ variant: "outline" })}
							>
								Cancel
							</D.Close>
							<D.Close type="submit" class={buttonVariants()}>
								<Button type="button" onclick={handleSaveConfig}>
									Save changes
								</Button>
							</D.Close>
						</D.Footer>
					</D.Content>
				</form>
			</D.Root>
		</div>
	</div>

	<C.Card class="w-full bg-transparent rounded-none">
		<C.CardContent>
			<Item.Group class="mt-4">
				{#each sshConfig as entry, i (entry.host)}
					<Item.Root variant="outline">
						<Item.Content>
							<div class="flex items-center gap-3">
								<button
									type="button"
									class="flex items-center justify-center w-5 h-5 rounded border transition-colors
										{confResults[i]?.connected
											? 'border-green-500 text-green-500'
											: 'border-muted-foreground/30 text-muted-foreground/30'}"
									onclick={() => handleRetest(i)}
									title="Test connection"
								>
									{#if confResults[i]?.connected}
										<CircleCheck class="size-4" />
									{:else}
										<CircleX class="size-4" />
									{/if}
								</button>
								<div>
									<Item.Title class="font-mono text-sm">
										{entry.host}
									</Item.Title>
									<Item.Description class="text-xs text-muted-foreground">
										{entry.hostname} • {entry.user}
									</Item.Description>
								</div>
							</div>
						</Item.Content>
						<Item.Actions>
							<span class="text-xs text-muted-foreground font-mono truncate max-w-[200px]">
								{entry.identityFile}
							</span>
							<div class="flex items-center gap-1">
								<button
									type="button"
									class="inline-flex items-center justify-center rounded-md p-1.5 text-muted-foreground hover:text-foreground transition-colors"
									title="Retest connection"
									onclick={() => handleRetest(i)}
								>
									<RotateCcw class="size-3.5" />
								</button>
								<D.Root>
									<D.Trigger type="button" class={buttonVariants({ variant: "ghost", size: "icon", class: "rounded-full" })}>
										<FilePen class="size-3.5" />
									</D.Trigger>
									<D.Content>
										<D.Header>
											<D.Title>Edit Host: {entry.host}</D.Title>
											<D.Description>Modify connection details</D.Description>
										</D.Header>
										<div class="grid gap-3 py-4">
											<div class="grid gap-1">
												<label class="text-sm font-medium" for="edit-host">Host</label>
												<Input id="edit-host" value={entry.host} disabled />
											</div>
											<div class="grid gap-1">
												<label class="text-sm font-medium" for="edit-hostname">HostName</label>
												<Input id="edit-hostname" value={entry.hostname} />
											</div>
											<div class="grid gap-1">
												<label class="text-sm font-medium" for="edit-user">User</label>
												<Input id="edit-user" value={entry.user} />
											</div>
											<div class="grid gap-1">
												<label class="text-sm font-medium" for="edit-identity">IdentityFile</label>
												<Input id="edit-identity" value={entry.identityFile} />
											</div>
										</div>
										<D.Footer>
											<D.Close class={buttonVariants({ variant: "outline" })}>Close</D.Close>
										</D.Footer>
									</D.Content>
								</D.Root>
							</div>
						</Item.Actions>
					</Item.Root>
				{/each}

				<Item.Root variant="outline" class="border-dashed">
					<Item.Content>
						<Item.Title class="font-mono text-muted-foreground text-sm">
							Add new host configuration...
						</Item.Title>
					</Item.Content>
				</Item.Root>
			</Item.Group>
		</C.CardContent>
	</C.Card>
</div>
