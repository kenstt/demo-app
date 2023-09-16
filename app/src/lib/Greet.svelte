<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }

  let data = [];
  const callMyApi = async () => {
    const response = await fetch("https://dummy.restapiexample.com/api/v1/employees");
    const body = await response.json();
    data = body.data;
    console.log(data);    // 我們直接看資料的長相，再回來調整程式碼
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
  <button on:click={callMyApi}>Call my API</button>
  <ul>
    {#each data as item}
      <li>{item.employee_name}</li>
    {/each}
  </ul>
</div>