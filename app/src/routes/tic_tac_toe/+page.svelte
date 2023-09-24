<script lang="ts">
  import {api} from "../../api";
  let newGame = api.ticTacToe.newGame();
</script>

<div>
  {#await newGame}
    <p>...loading</p>
  {:then value}
    遊戲：{value}
  {:catch error}
    發生錯誤：{error}
  {/await }
</div>