import type { TicTacToeApi } from './tic_tac_toe';
import { ticTacToeApi, ticTacToeApiTauri } from './tic_tac_toe';

export interface Api {
  ticTacToe: TicTacToeApi;
}

export const api: Api = {
  ticTacToe: ticTacToeApiTauri,
};
