<script lang="ts">
  import { wsClient } from '../api/ws_client';

  let message = '';
  let responseMessage = '';
  let ws = wsClient();
  ws.onmessage = function (event) {
    console.log('Message from server ', event.data);
    responseMessage = event.data;
  };
  async function greet() {
    ws.send(message);
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter message" bind:value={message} />
    <button type="submit">Greet</button>
  </form>
  <p>{responseMessage}</p>
</div>