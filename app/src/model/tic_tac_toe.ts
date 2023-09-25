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
}

export type GameSet = [number, Game]; // POST 回傳資料結構

export const emptyGame = (): GameSet => [
  0,
  {
    cells: [null, null, null, null, null, null, null, null, null],
    is_over: false,
    winner: null,
  },
];

export type ErrorResponse = {
  message: string;
  details?: string;
};
