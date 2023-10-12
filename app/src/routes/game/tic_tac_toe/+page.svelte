<script lang="ts">
  import { api } from '../../../api';
  import type { ErrorResponse } from '../../../model/tic_tac_toe';
  import { emptyGame } from '../../../model/tic_tac_toe';
  import { onDestroy, onMount } from "svelte";
  import { wsClient } from "../../../api/ws_client";
  import { getNotificationsContext } from 'svelte-notifications';
  const { addNotification } = getNotificationsContext();
  import type { Event } from '@tauri-apps/api/event'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api/tauri';

  wsClient().onmessage = (e) => {
    addNotification({
      text: e.data.match(/>:(.*)/)[1],
      position: 'bottom-right',
      type: 'info',
      removeAfter: 4000,
    });
  };

  let isOffline = false;
  $: isOffline ? (gameSet = emptyGame()) : (gameSet = emptyGame());

  let error: ErrorResponse | null = null;
  let gameSet = emptyGame();
  $: wonLine = gameSet[1].won_line;
  $: game = gameSet[1];
  $: gameId = gameSet[0];
  $: errorMessage = error ? error?.message + (error?.details ? `，${error?.details}` : '') : '';
  let id: number = 1;

  const newGame = async () => {
    let now = performance.now();
    gameSet = isOffline ? await api.ticTacToeOffline.newGame() : await api.ticTacToe.newGame();
    console.log(`took ${performance.now() - now} ms`); // 測量經過時間 performance
    error = null;
  };

  const playGame = async (index: number) => {
    try {
      gameSet = isOffline
        ? await api.ticTacToeOffline.play(gameId, index)
        : await api.ticTacToe.play(gameId, index);
      error = null;
    } catch (e: unknown) {
      error = e as ErrorResponse;
    }
  };
  const goto = async (id: number) => {
    error = null;
    try {
      gameSet = isOffline
        ? await api.ticTacToeOffline.getGame(id)
        : await api.ticTacToe.getGame(id);
    } catch (e) {
      // console.log(e);
      error = e as ErrorResponse;
    }
  };
  const deleteGame = async () => {
    error = null;
    isOffline
      ? await api.ticTacToeOffline.deleteGame(gameId)
      : await api.ticTacToe.deleteGame(gameId);
    gameSet = emptyGame();
  };
  const onInput = (e: Event) => {
    // console.log(e);
    const target = e.target as HTMLInputElement;
    id = Number(target.value);
  };

  let unlisten = () => { }
  const subscribe = async () => {
    if (typeof window !== 'undefined' && window.__TAURI_IPC__) {
      unlisten();
      unlisten = await listen('message', (event: Event<string>) => {
        addNotification({
          text: event.payload,
          position: 'top-right',
          type: 'success',
          removeAfter: 3000,
        });
      });
      console.log('已訂閱tauri message');
    }
  };
  const unsubscribe = () => {
    if (typeof window !== 'undefined' && window.__TAURI_IPC__) {
      unlisten();
      console.log('已取消訂閱tauri message');
    }
  };
  const stopPolling = () => {
    if (typeof window !== 'undefined' && window.__TAURI_IPC__) {
      invoke('stop_polling_message');
      console.log('已停止tauri message服務');
    }
  };
  const startPolling = async () => {
    if (typeof window !== 'undefined' && window.__TAURI_IPC__) {
      await invoke('polling_message');
      console.log('已啟動tauri message服務');
    }
  };
  onMount(async () => {
    await newGame();
    await subscribe();
    await startPolling();
  });

  onDestroy(() => {
    unsubscribe();
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
      on:click={() => goto(id)}>GO
    </button
    >
    <label class="relative inline-flex items-end cursor-pointer">
      <input type="checkbox" value="" class="sr-only peer" bind:checked={isOffline}/>
      <div
        class="w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
      />
      <span class="ml-3 text-sm font-medium text-gray-900 dark:text-gray-800"
      >{isOffline ? '本機' : '線上'}</span
      >
    </label>
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

<div class="grid grid-cols-4 justify-center items-baseline gap-3">
  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg text-2xl"
    on:click={startPolling}
  >
    啟動訊息服務
  </button>
  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg text-2xl"
    on:click={stopPolling}
  >
    停止訊息服務
  </button>
  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg text-2xl"
    on:click={subscribe}
  >
    訂閱訊息
  </button>
  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg text-2xl"
    on:click={unsubscribe}
  >
    取消訂閱訊息
  </button>
</div>
