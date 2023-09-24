<script lang="ts">
  import {api} from "../../api";
  import {emptyGame} from "../../model/tic_tac_toe";
  import {onMount} from "svelte";

  let gameSet = emptyGame();    // 在model裡新增一個fn建立空白物件，讓下面標籤中的資料綁定不報錯。
  const newGame = async () => {    // 把呼叫api包成這裡用的function
    gameSet = await api.ticTacToe.newGame();
  }

  const playGame = async (index: number) => {
    gameSet = await api.ticTacToe.play(gameSet[0], index);
  }

  onMount(async () => {
    await newGame();            // 初始化先從server取得新局
  })
</script>

<button on:click={newGame}>新遊戲</button>
<div>
  局號：{gameSet[0]}，狀態：{gameSet[1].is_over ? '結束' : '進行中'}，贏家：{gameSet[1].winner ?? '無'}
</div>
<div class="grid grid-cols-3">
  {#each gameSet[1].cells as symbol, index}
    <button on:click={playGame.bind(this, index+1)}>{index}: {symbol}</button>
  {/each}
</div>
