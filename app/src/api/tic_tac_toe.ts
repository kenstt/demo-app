export interface TicTacToeApi {  // 定義本模組介面
  newGame: () => Promise<any>;   // todo: 等等再寫model
}

const newGame = async (): Promise<any> => {
  let response = await fetch("http://localhost:3030/tic_tac_toe", {
    method: "POST",
  });                // 使用es原生fetch呼叫rest api

  if (response.ok) {
    let data = await response.json();
    return Promise.resolve(data);
  }  // todo: error handling
}

export const ticTacToeApi: TicTacToeApi = {
  newGame,
}
