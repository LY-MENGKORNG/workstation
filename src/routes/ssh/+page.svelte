<script lang="ts">
    import { Button, buttonVariants } from "$lib/components/ui/button";
    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
    } from "$lib/components/ui/card";
    import * as Item from "$lib/components/ui/item";
    import * as Dialog from "$lib/components/ui/dialog";
    import {
        parseSSHConfig,
        readConfigFile,
        listSSHKeyFiles,
    } from "$lib/hooks/shell";

    import { onMount } from "svelte";

    let keyFiles = $state<string[]>([]);
    onMount(async () => {
        keyFiles = await listSSHKeyFiles();
    });
</script>

<div class="">
    <div class="flex items-center justify-between">
        <h1 class="text-3xl">SSH Key Files</h1>
        <Dialog.Root>
            <Dialog.Trigger
                type="button"
                class={"flex items-center gap-2 " +
                    buttonVariants({ variant: "default" })}
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="lucide-icon lucide lucide-plus"
                    ><!----><path d="M5 12h14"></path><!----><path d="M12 5v14"
                    ></path><!----><!----><!----></svg
                > <span>Add</span>
            </Dialog.Trigger>
            <Dialog.Content>
                <Dialog.Header>
                    <Dialog.Title>Create new SSH Key</Dialog.Title>
                    <Dialog.Description
                        >Select an SSH key file to add.</Dialog.Description
                    >
                </Dialog.Header>
            </Dialog.Content>
        </Dialog.Root>
    </div>

    <Card>
        <CardContent>
            <Item.Group>
                {#each keyFiles as keyFile (keyFile)}
                    <Item.Root>
                        <Item.Content>
                            <Item.Title>
                                {keyFile}
                            </Item.Title>
                        </Item.Content>
                    </Item.Root>
                {/each}
            </Item.Group>
        </CardContent>
    </Card>
</div>
