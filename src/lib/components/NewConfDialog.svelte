<script lang="ts">
    import * as D from "$lib/components/ui/dialog";
    import * as IG from "$lib/components/ui/input-group";
    import { ArrowUpIcon, Plus } from "@lucide/svelte";
    import { buttonVariants } from "./ui/button";
    import Separator from "./ui/separator/separator.svelte";
    import Button from "./ui/button/button.svelte";
    import Label from "./ui/label/label.svelte";
    import Input from "./ui/input/input.svelte";
</script>

<D.Root>
    <D.Trigger
        type="button"
        class={"flex items-center gap-2 " +
            buttonVariants({ variant: "default" })}
    >
        <Plus />
        <span>Add</span>
    </D.Trigger>
    <D.Content>
        <D.Header>
            <D.Title class="text-2xl">Create new SSH Key</D.Title>
            <D.Description>
                Enter the name of the key file to add.
            </D.Description>
        </D.Header>
        <div class="flex flex-col gap-4">
            <IG.Root>
                <IG.Input placeholder="Enter file name e.g. id_github..." />
                <IG.Addon align="block-end">
                    <IG.Text class="ms-auto">52% used</IG.Text>
                    <Separator orientation="vertical" class="h-4!" />
                    <Label for="uploadFile">
                        <IG.Button
                            variant="default"
                            class="rounded-full"
                            size="icon-xs"
                        >
                            <ArrowUpIcon />
                            <span class="sr-only">Send</span>
                        </IG.Button>
                    </Label>
                    <Input id="uploadFile" type="file" style="display: none;" />
                </IG.Addon>
            </IG.Root>
            <Button type="submit">Save</Button>
        </div>
    </D.Content>
</D.Root>
