import { httpClient } from './ky';
import type { Game, GameSet } from '../model/tic_tac_toe';

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
