<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import init, { hello_async } from '../../../wasm/pkg/wasm'
  import { onMount } from "svelte";

  let name = '';
  let greetMsg = '';
  let hello = null;

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // greetMsg = await invoke('greet', { name });
    try {
      let s = await hello_async();
      console.log(s);
    } catch (e) {
      console.log(e);
    }
  }

  onMount(async () => {
    await init();
    hello = hello_async;
  });
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
