import type { TicTacToeApi } from './tic_tac_toe';
import { ticTacToeApi, ticTacToeApiTauri } from './tic_tac_toe';

export interface Api {
  ticTacToe: TicTacToeApi;
}

const httpApi: Api = {    // 把之前 api rename 成 httpApi 區別 tauri
  ticTacToe: ticTacToeApi,
};

const tauriApi: Api = {    // 這部分沒有動
  ticTacToe: ticTacToeApiTauri,
};

export const api: Api = window.__TAURI_IPC__ ? tauriApi : httpApi;
