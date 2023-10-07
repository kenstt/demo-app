import type { TicTacToeApi } from './tic_tac_toe';
import { ticTacToeApi, ticTacToeApiTauri, ticTacToeApiTauriOffline, ticTacToeApiWasm, } from './tic_tac_toe';

export interface Api {
  ticTacToe: TicTacToeApi;
  ticTacToeOffline: TicTacToeApi;

}

const httpApi: Api = {
  ticTacToe: ticTacToeApi,
  ticTacToeOffline: ticTacToeApiWasm,
};

const tauriApi: Api = {
  ticTacToe: ticTacToeApiTauri,
  ticTacToeOffline: ticTacToeApiTauriOffline,
};

// @ts-ignore
export const api: Api = (typeof window !== 'undefined' && window?.__TAURI_IPC__) ? tauriApi : httpApi;
