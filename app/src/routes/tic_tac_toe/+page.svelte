<script lang="ts">
  import { api } from "../../api";
  import type { ErrorResponse } from '../../model/tic_tac_toe';
  import { emptyGame } from "../../model/tic_tac_toe";
  import { onMount } from "svelte";

  let gameSet = emptyGame();    // 在model裡新增一個fn建立空白物件，讓下面標籤中的資料綁定不報錯。
  const newGame = async () => {    // 把呼叫api包成這裡用的function
    gameSet = await api.ticTacToe.newGame();
    // newGame2 = api.ticTacToe.newGame();
  }

  let error: string | null = null;
  const playGame = async (index: number) => {
    try {
      gameSet = await api.ticTacToe.play(gameSet[0], index);
      error = null;
    } catch (e: unknown) {
      let err = e as ErrorResponse;
      let msg = err.message;
      if (err.details) {
        msg += `: ${err.details}`;
      }
      error = msg;
    }
  }

  onMount(async () => {
    await newGame();            // 初始化先從server取得新局
  })

  // let newGame2 = api.ticTacToe.newGame();
  // let playGame2 = (id, step) => {
  //   newGame2 = api.ticTacToe.play(id, step);
  // }

</script>

<button
  class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
  on:click={newGame}
> 新遊戲
</button>

<h2 class="font-bold py-2 px-4 rounded text-2xl">
  局號：{gameSet[0]}，
  {#if gameSet[1].winner}
    遊戲結束，贏家：{gameSet[1].winner}！
  {:else if gameSet[1].is_over && !gameSet[1].winner}
    遊戲結束：平手！
  {:else}
    遊戲正在進行中...
  {/if}
  <span class="text-red-500 text-lg">  {error ?? ''}  </span>
</h2>

<div class="w-96 grid grid-cols-3">
  {#each gameSet[1].cells as symbol, index}
    <button
      class="h-32 text-9xl text-amber-500 border-2 border-amber-500 rounded-md"
      on:click={() => playGame(index+1)}
    >{symbol ?? ' '}</button>
  {/each}
</div>


<!--{#await newGame2}-->
<!--  <p>...loading</p>-->
<!--{:then value}-->
<!--  <div> 第{value[0]}局</div>-->
<!--  <div class="grid grid-cols-3">-->
<!--    {#each value[1].cells as symbol, index}-->
<!--      <div>-->
<!--        <button on:click={playGame2.bind(this, value[0], index+1)}> {index}: {symbol}</button>-->
<!--      </div>-->
<!--    {/each}-->
<!--  </div>-->
<!--{:catch error}-->
<!--  發生錯誤：{error.message} {error.details}-->
<!--{/await }-->
