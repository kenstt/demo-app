<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let name = '';
  let greetMsg = '';

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke('greet', { name });
  }

  const test_api = async () => {
    const res = await invoke('http_test');
    console.log(res);
  }
</script>

<button
  class="bg-blue-400 hover:bg-blue-500 text-white font-bold py-2 px-4 rounded"
  on:click={test_api}> 測試tauri reqwest
</button>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
