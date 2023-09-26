<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let name = '';
  let greetMsg = '';

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke('greet', { name });
  }

  const get_game = async (id) => {
    const res = await invoke('get_game', {id});
    console.log(res);
  }
  const new_game = async () => {
    const res = await invoke('new_game');
    console.log(res);
  }
</script>

<button
  class="bg-blue-400 hover:bg-blue-500 text-white font-bold py-2 px-4 rounded"
  on:click={() => get_game(1)}>
  Get Game
</button>
<button
  class="bg-blue-400 hover:bg-blue-500 text-white font-bold py-2 px-4 rounded"
  on:click={() => new_game()}>
  NEW Game
</button>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
