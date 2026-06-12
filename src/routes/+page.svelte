<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  let sshKeyFiles = $state<string[]>([]);

  async function listSSHKeyFiles() {
    sshKeyFiles = (await invoke("list_pub_key_files")) as string[];
  }

  onMount(listSSHKeyFiles);
</script>

<h1>SSH Key Files</h1>
<ul>
  {#each sshKeyFiles as file}
    <li>{file}</li>
  {/each}
</ul>
