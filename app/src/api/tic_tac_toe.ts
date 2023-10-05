import { httpClient } from './ky';
import type { WasmResponse, Game, GameSet } from '../model/tic_tac_toe'
import { invoke } from "@tauri-apps/api/tauri";
import init, { new_game, get_game, play_game, delete_game } from '../../../wasm/pkg/wasm'

export interface TicTacToeApi {
  // 定義本模組介面
  newGame: () => Promise<GameSet>;
  getGame: (id: number) => Promise<GameSet>;
  play: (id: number, step: number) => Promise<GameSet>;
  deleteGame: (id: number) => Promise<void>;
}

const newGame = async (): Promise<GameSet> => {
  const response = await httpClient().post('tic_tac_toe');
  // const response = await fetch('http://localhost:3030/tic_tac_toe', {
  //   method: 'POST',
  // }); // 使用es原生fetch呼叫rest api

  if (response.ok) {
    return await response.json();
  } else {
    return Promise.reject(await response.json());
  }
};

const play = async (id: number, step: number): Promise<GameSet> => {
  const response = await httpClient().put(`tic_tac_toe/${id}/${step}`);
  // const response = await fetch(`http://localhost:3030/tic_tac_toe/${id}/${step}`, {
  //   method: 'PUT',
  // });

  if (response.ok) {
    const data = (await response.json()) as Game;
    return [id, data];
  } else {
    return Promise.reject(await response.json());
  }
};

const getGame = async (id: number): Promise<GameSet> => {
  const response = await httpClient().get(`tic_tac_toe/${id}`);

  if (response.ok) {
    const data = (await response.json()) as Game;
    return [id, data];
  } else {
    return Promise.reject(await response.json());
  }
};

const deleteGame = async (id: number): Promise<void> => {
  const response = await httpClient().delete(`tic_tac_toe/${id}`);

  if (!response.ok) {
    return Promise.reject(await response.json());
  }
};

export const ticTacToeApi: TicTacToeApi = {
  newGame,
  play,
  getGame,
  deleteGame,
};

const getGameTauri = async (id: number, isOffline: boolean = false): Promise<GameSet> => {
  try {
    let method: string = isOffline ? 'get_game_e' : 'get_game_grpc';
    const game = await invoke(method, {id});
    return [id, game as Game];                       // 組 GameSet
  } catch (e) {                                      // 補捉rust的Err(e)
    return Promise.reject(e);
  }
};

const newGameTauri = async (isOffline: boolean = false): Promise<GameSet> => {
  try {
    let method: string = isOffline ? 'new_game_e' : 'new_game_grpc';
    const gameSet = await invoke(method);
    return gameSet as GameSet;
  } catch (e) {
    return Promise.reject(e);
  }
};

const playGameTauri = async (
  id: number, num: number, isOffline: boolean = false
): Promise<GameSet> => {
  try {
    let method: string = isOffline ? 'play_game_e' : 'play_game_grpc';
    const game = await invoke(method, {id, num});
    return [id, game as Game];
  } catch (e) {
    return Promise.reject(e);
  }
};

const deleteGameTauri = async (id: number, isOffline: boolean = false): Promise<void> => {
  try {
    let method: string = isOffline ? 'delete_game_e' : 'delete_game_grpc';
    await invoke(method, {id});
  } catch (e) {
    return Promise.reject(e);
  }
};

export const ticTacToeApiTauri: TicTacToeApi = { // 實現與http同樣介面
  deleteGame: deleteGameTauri,
  getGame: getGameTauri,
  newGame: newGameTauri,
  play: playGameTauri,
};

/** 離線模式 */
export const ticTacToeApiTauriOffline: TicTacToeApi = {
  deleteGame: (id) => deleteGameTauri(id, true),
  getGame: (id) => getGameTauri(id, true),
  newGame: () => newGameTauri(true),
  play: (id, num) => playGameTauri(id, num, true),
};

export const ticTacToeApiWasm: TicTacToeApi = {
  async newGame(): Promise<GameSet> {
    await init();
    let result: WasmResponse<GameSet> = new_game();
    if (result.Ok) {
      return result.Ok;
    } else {
      return Promise.reject(result.Err);
    }
  },
  async getGame(id: number): Promise<GameSet> {
    await init();
    let result: WasmResponse<Game> = get_game(id);
    if (result.Ok) {
      return [id, result.Ok];
    } else {
      return Promise.reject(result.Err);
    }
  },
  async play(id: number, num: number): Promise<GameSet> {
    await init();
    let result: WasmResponse<Game> = play_game(id, num);
    if (result.Ok) {
      return [id, result.Ok];
    } else {
      return Promise.reject(result.Err);
    }
  },
  async deleteGame(id: number): Promise<unknown> {
    await init();    // 初始化wasm後，才可以使用wasm的fn
    let result: WasmResponse<unknown> = delete_game(id);
    if (!result.Ok) {
      return Promise.reject(result.Err);
    }
  },
};