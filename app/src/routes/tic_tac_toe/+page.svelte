<script lang="ts">
  import { api } from "../../api";

  let newGame = api.ticTacToe.newGame();
</script>

<div>
  {#await newGame}
    <p>...loading</p>
  {:then value}
    局號：{value[0]}，狀態：{value[1].is_over ? '結束' : '進行中'} ，贏家：{value[1].winner ?? '無'}
  {:catch error}
    發生錯誤：{error}
  {/await }
</div>