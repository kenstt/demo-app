export type GameSymbol = 'X' | 'O' | null;

export interface Game {
  // 遊戲資料結構
  cells: [
    GameSymbol,
    GameSymbol,
    GameSymbol,
    GameSymbol,
    GameSymbol,
    GameSymbol,
    GameSymbol,
    GameSymbol,
    GameSymbol
  ];
  is_over: boolean;
  winner?: GameSymbol;
  won_line?: Array<number>;
}

export type GameSet = [number, Game]; // POST 回傳資料結構

export const emptyGame = (): GameSet => [
  0,
  {
    cells: [null, null, null, null, null, null, null, null, null],
    is_over: false,
    winner: null,
    won_line: [],
  },
];

export type ErrorResponse = {
  message: string;
  details?: string;
};

export interface WasmResponse<T> { // T 可以換成不同的類別
  Ok: T;
  Err: ErrorResponse;              // 包成我們之前的錯誤訊息格式
}
