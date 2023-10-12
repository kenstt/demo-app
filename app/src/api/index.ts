import type { TicTacToeApi } from './tic_tac_toe';
import { ticTacToeApi, ticTacToeApiTauri, ticTacToeApiTauriOffline, ticTacToeApiWasm, } from './tic_tac_toe';
import { login } from './auth';

export interface Api {
  ticTacToe: TicTacToeApi;
  ticTacToeOffline: TicTacToeApi;
  login: (username: string, password: string) => Promise<void>;
}

const httpApi: Api = {
  ticTacToe: ticTacToeApi,
  ticTacToeOffline: ticTacToeApiWasm,
  login,
};

const tauriApi: Api = {
  ticTacToe: ticTacToeApiTauri,
  ticTacToeOffline: ticTacToeApiTauriOffline,
  login,
};

// @ts-ignore
export const api: Api = (typeof window !== 'undefined' && window?.__TAURI_IPC__) ? tauriApi : httpApi;
