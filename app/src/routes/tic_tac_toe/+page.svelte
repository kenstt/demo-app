<script lang="ts">
  import { api } from '../../api';
  import type { ErrorResponse } from '../../model/tic_tac_toe';
  import { emptyGame } from '../../model/tic_tac_toe';
  import { onMount } from 'svelte';

  let error: ErrorResponse | null = null;
  let gameSet = emptyGame();
  $: wonLine = gameSet[1].won_line;
  $: game = gameSet[1];
  $: gameId = gameSet[0];
  $: errorMessage = error ? error?.message + (error?.details ? `，${error?.details}` : '') : '';
  let id: number = 1;

  const newGame = async () => {
    error = null;
    gameSet = await api.ticTacToe.newGame();
  };

  const playGame = async (index: number) => {
    try {
      gameSet = await api.ticTacToe.play(gameId, index);
      error = null;
    } catch (e: unknown) {
      error = e as ErrorResponse;
    }
  };
  const goto = async (id: number) => {
    error = null;
    try {
      gameSet = await api.ticTacToe.getGame(id);
    } catch (e) {
      // console.log(e);
      error = e as ErrorResponse;
    }
  };
  const deleteGame = async () => {
    error = null;
    await api.ticTacToe.deleteGame(gameId);
    gameSet = emptyGame();
  };
  const onInput = (e: Event) => {
    // console.log(e);
    const target = e.target as HTMLInputElement;
    id = Number(target.value);
  };
  onMount(async () => {
    await newGame();
  });
</script>

<div class="grid grid-cols-4 justify-center items-baseline gap-3">
  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg text-2xl"
    on:click={newGame}
  >
    新遊戲
  </button>
  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg text-2xl"
    on:click={deleteGame}
  >
    刪除此局
  </button>
  <div class="px-5 text-2xl col-span-2">
    <span> 跳至第 </span>
    <input
      class="w-16 border-2 border-blue-500 rounded-md h-10 text-center text-2xl"
      on:keydown={(e) => e.key === 'Enter' && goto(id)}
      value={id} on:input={onInput}
    />
    <span>筆</span>
    <button
      class="border-blue-500 hover:bg-blue-700 text-blue-500 border-2 font-bold py-2 px-4 rounded-lg text-lg h-12"
      on:click={() => goto(id)}>GO</button
    >
  </div>
</div>

<h2 class="font-bold py-2 px-4 rounded text-2xl">
  局號：{gameId}，
  {#if !gameId}
    請先開啟新遊戲！
  {:else if game.winner}
    遊戲結束，贏家：{game.winner}！
  {:else if game.is_over && !game.winner}
    遊戲結束：平手！
  {:else}
    遊戲正在進行中...
  {/if}
  <span class="text-red-500 text-lg"> {errorMessage} </span>
</h2>

<div class="w-96 grid grid-cols-3">
  {#each game.cells as symbol, index}
    <button
      class="h-32 text-9xl text-amber-500 border-2 border-amber-500 rounded-md hover:bg-amber-100 hover:text-white"
      class:text-blue-500={wonLine?.includes(index + 1)}
      class:bg-amber-100={wonLine?.includes(index + 1)}
      on:click={() => playGame(index + 1)}
    >{symbol ?? ' '}
    </button>
  {/each}
</div>
