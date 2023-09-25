export type Symbol = "X" | "O" | null;

export interface Game {                   // 遊戲資料結構
  cells: [
    Symbol, Symbol, Symbol,
    Symbol, Symbol, Symbol,
    Symbol, Symbol, Symbol,
  ],
  is_over: boolean,
  winner?: Symbol,
}

export type GameSet = [number, Game];    // POST 回傳資料結構

export const emptyGame = (): GameSet =>  [0, {
  cells: [
    null, null, null,
    null, null, null,
    null, null, null,
  ],
  is_over: false,
  winner: null,
}];

export type ErrorResponse = {
  message: string;
  details?: string;
};