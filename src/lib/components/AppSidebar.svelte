<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import { page } from "$app/stores";
	import HomeIcon from "@lucide/svelte/icons/house";
	import TerminalIcon from "@lucide/svelte/icons/terminal";
	import GitBranchIcon from "@lucide/svelte/icons/git-branch";
	import ModeToggle from "./ModeToggle.svelte";

	const navItems = [
		{ title: "Home", url: "/", icon: HomeIcon },
		{ title: "SSH Config", url: "/ssh", icon: TerminalIcon },
		{ title: "Git Config", url: "/git", icon: GitBranchIcon },
	];
</script>

<Sidebar.Root collapsible="icon">
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton size="lg">
					{#snippet child({ props })}
						<a href="/" {...props}>
							<div
								class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
							>
								<TerminalIcon class="size-4" />
							</div>
							<span class="truncate font-semibold">Workstation</span>
						</a>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Navigation</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each navItems as item (item.title)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton
								isActive={$page.url.pathname === item.url}
								tooltipContent={item.title}
							>
								{#snippet child({ props })}
									<a href={item.url} {...props}>
										<item.icon />
										<span>{item.title}</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<div class="flex items-center justify-center p-2">
					<ModeToggle />
				</div>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
