use rand::prelude::SliceRandom;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

const SYMBOLS: [Symbol; 2] = [Symbol::O, Symbol::X];

/// 井字遊戲的錯誤類型。
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("錯誤的格號，請填1至9間的數字！")]
    WrongStep,
    /// 格子已被劃記。
    #[error("已非空格，不能再次畫記！")]
    AlreadyOccupied,
    /// 遊戲已結束。
    #[error("遊戲已結束，無法操作！")]
    GameOver,
}

/// 井字遊戲的符號。
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Symbol {
    O,
    X,
}

/// 井字遊戲棋局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// 棋盤格子，每個格子可能是空的，或是被劃記的符號。
    pub cells: [Option<Symbol>; 9],
    /// 遊戲是否結束。
    pub is_over: bool,
    /// 贏家，若無則為None，若有則為Some(Symbol)。
    pub winner: Option<Symbol>,
    /// 贏的連線，如果沒有則為`None`，如果有則為九宮格位置（1~9）
    pub won_line: Option<[u8; 3]>,
}

// 如果你想要實作getter的話，這裡是範例參考：
impl Game {
    /// 取得九宮格的格子，每個格子可能是空格，或是`O`或`X`
    pub fn cells(&self) -> [Option<Symbol>; 9] { self.cells }
    /// 取得遊戲是否結束
    pub fn is_over(&self) -> bool { self.is_over }
    /// 取得贏家，如果沒有則為`None`，如果有則為`O`或`X`
    pub fn winner(&self) -> Option<Symbol> { self.winner }
}

impl Game {
    /// 與電腦對戰，`num`為玩家下的格號，1~9分別代表[九宮格](https://zh.wikipedia.org/zh-tw/%E4%B9%9D%E5%AE%AE%E6%A0%BC)的位置，
    /// 玩家下完後，電腦會隨機選擇一個空格劃記，直到遊戲結束，
    /// 若玩家下的格號已被劃記，會回傳錯誤[`Error::AlreadyOccupied`]，
    /// 若遊戲已結束，會回傳錯誤[`Error::GameOver`]，
    /// 若一切正常，回傳`Ok(())`。
    pub fn play_with_counter(&mut self, num: usize) -> Result<(), Error> {
        self.play(num)?;         // unwrap or return Error
        if self.is_over {        // 結束就離開
            return Ok(());
        }

        let mut indices: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // 格子的範圍
        // let mut indices: [usize; 9] = (1..=9)            // 同上一行，進階寫法
        //     .collect::<Vec<_>>().try_into().unwrap();
        indices.shuffle(&mut rand::thread_rng());        // 打亂順序
        for index in indices.iter().enumerate() {        // 逐一檢查可否下
            let num = index.1;                           // 取出格號
            if self.cells[num - 1].is_some() {           // 檢查該格是否已劃記
                continue;                                // 若已劃記便跳至下一格 (next for)
            }
            self.play(*num)?;                            // 格號為空直接劃記該格
            break;                                       // 中斷整個for (不然會全填滿)
        }
        Ok(())
    }

    /// 取得目前步數，用於判斷現在是輪到哪一方符號。
    /// 以下範例，預設第一手`O`，所以第一步後，步數為1，後續依下棋次數遞增：
    /// ```
    /// # use core::tic_tac_toe::{Game, Symbol};
    /// let mut game = Game::default();
    /// assert_eq!(game.current_step(), 0);
    /// game.play(1).unwrap();
    /// assert_eq!(game.current_step(), 1);
    /// game.play(2).unwrap();
    /// assert_eq!(game.current_step(), 2);
    /// ```
    pub fn current_step(&self) -> usize {
        self.cells.iter().filter(|x| x.is_some()).count() // 算步數
    }

    /// 下棋，`num`為指定劃記的格號，1~9分別代表九宮格的位置，
    /// 若指定的格號已被劃記，會回傳錯誤[`Error::AlreadyOccupied`]，
    /// 若遊戲已結束，會回傳錯誤[`Error::GameOver`]，
    /// 若無報錯，會將格號劃記為當前劃記符號，並檢查遊戲是否結束，
    /// 若一切正常，回傳`Ok(())`。
    /// 範例，以下在第1格下棋後，預設第一手`O`，所以第1格（陣列索引第0項）會畫上`O`：
    /// ```
    /// # use core::tic_tac_toe::{Game, Symbol};
    /// let mut game = Game::default();
    /// game.play(1).unwrap();
    /// assert_eq!(game.cells[0], Some(Symbol::O));
    /// ```
    /// 如果我們再下第二手，預設第二手`X`，所以下例第8格（陣列索引第7項）會畫上`X`：
    /// ```
    /// # use core::tic_tac_toe::{Game, Symbol};
    /// # let mut game = Game::default();
    /// # game.play(1).unwrap();
    /// # assert_eq!(game.cells[0], Some(Symbol::O));
    /// game.play(8).unwrap();
    /// assert_eq!(game.cells[7], Some(Symbol::X));
    /// ```
    pub fn play(&mut self, num: usize) -> Result<(), Error> {
        if self.is_over {                    // 一開始先判斷遊戲結束就報錯
            return Err(Error::GameOver);
        }
        if !(1..=9).contains(&num) {
            return Err(Error::WrongStep);
        }
        let index = num - 1;
        if self.cells[index].is_some() {
            return Err(Error::AlreadyOccupied);
        }
        let symbol = SYMBOLS[self.current_step() % 2];
        self.cells[index] = Some(symbol);
        self.check_over();                   // 玩家每一步結束後判斷是否結束
        Ok(())
    }

    /// 檢查遊戲是否結束，若結束則設定`is_over`為`true`，並設定`winner`為贏家。
    /// ```
    /// # use core::tic_tac_toe::{Game, Symbol};
    /// # let mut game = Game::default();
    /// game.cells = [
    ///     Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
    ///     Some(Symbol::X), Some(Symbol::X), None,
    ///     None,            None,            None,
    /// ];
    /// game.check_over();
    /// assert_eq!(game.is_over, true);
    /// assert_eq!(game.winner, Some(Symbol::O));
    /// ```
    /// 和局範例：
    /// ```
    /// # use core::tic_tac_toe::{Game, Symbol};
    /// # let mut game = Game::default();
    /// game.cells = [
    ///     Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
    ///     Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
    ///     Some(Symbol::O), Some(Symbol::X), Some(Symbol::O),
    /// ];
    /// game.check_over();
    /// assert_eq!(game.is_over, true);
    /// assert_eq!(game.winner, None);
    /// ```
    /// 未結束範例：
    /// ```
    /// # use core::tic_tac_toe::{Game, Symbol};
    /// # let mut game = Game::default();
    /// game.cells = [
    ///     Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
    ///     Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
    ///     Some(Symbol::O), None,            None,
    /// ];
    /// game.check_over();
    /// assert_eq!(game.is_over, false);
    /// assert_eq!(game.winner, None);
    /// ```
    pub fn check_over(&mut self) {
        let winner = self.check_winner();        // 檢查贏家，邏輯比較複雜，另外寫個fn
        match winner {                                // 匹配玩家所有可能
            Some(_) => {                              // 情境一：非None
                self.is_over = true;                  //   註記遊戲結束
                self.winner = winner;                 //   紀錄贏家
            }
            None => {                                 // 情境二：無贏家
                if self.cells.iter().all(|x| x.is_some()) { // 若格式已填滿
                    self.is_over = true;              // 遊戲結束 (平手)
                }
            }
        }
    }

    /// 檢查贏家，
    /// 這個函式會檢查所有可能的連線情境，若有連線則回傳贏家`Some(Symbol)`，
    /// 若無則回傳[`None`]。
    /// ```
    /// # use core::tic_tac_toe::{Game, Symbol};
    /// # let mut game = Game::default();
    /// game.cells = [
    ///     Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
    ///     Some(Symbol::X), Some(Symbol::X), None,
    ///     None,            None,            None,
    /// ];
    /// assert_eq!(game.check_winner(), Some(Symbol::O));
    /// ```
    pub fn check_winner(&mut self) -> Option<Symbol> {
        self.won_line = None;
        let win_patterns = [                 // 連線的index情境
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // 橫
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // 直
            [0, 4, 8], [2, 4, 6],            // 斜
        ];
        for idx in win_patterns.iter() {     // 用for 逐項檢查上面8條線
            let line = [                     // 把資料代入
                self.cells[idx[0]],
                self.cells[idx[1]],
                self.cells[idx[2]],
            ];
            let winner = match line {
                [Some(Symbol::O), Some(Symbol::O), Some(Symbol::O)] => {
                    Some(Symbol::O)
                }
                [Some(Symbol::X), Some(Symbol::X), Some(Symbol::X)] => {
                    Some(Symbol::X)
                }
                _ => None,
            };
            if winner.is_some() {
                self.won_line = Some(idx.iter()
                    .map(|x| u8::try_from(*x + 1).unwrap())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap());
                return winner;    // 已有winner，直接中斷比對，並回傳比對結果
            }
        }
        None                                  // 檢查完無符合條件回傳無
    }
}

impl Default for Game {
    // 替Game實作預設(無參數建構式)
    fn default() -> Self {     // 這裡的大寫Self指的是Self的Type，就是指Game
        Self {                 // 所以這邊的兩個Self換成Game也是一樣的結果
            cells: [None; 9],
            is_over: false,
            winner: None,
            won_line: None,
        }
    }
}

impl Display for Game {
    // 替 Game 實作 Display 這個 trait
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
        {} | {} | {}
        ---------
        {} | {} | {}
        ---------
        {} | {} | {}
        ",
            show(self.cells[0]), show(self.cells[1]), show(self.cells[2]),
            show(self.cells[3]), show(self.cells[4]), show(self.cells[5]),
            show(self.cells[6]), show(self.cells[7]), show(self.cells[8]),
        )
    }
}

/// 顯示格子內容( for Display trait )，若格子為空，顯示空白，否則顯示格子內容。
fn show(cell: Option<Symbol>) -> String {
    match cell {
        Some(symbol) => format!("{}", symbol),
        None => " ".to_string(),
    }
}

impl Display for Symbol {
    // 替 Cell 實現 Display 這個 trait
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::O => write!(f, "O"),
            Symbol::X => write!(f, "X"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_winner() {
        let mut game = Game::default();
        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
            Some(Symbol::X), Some(Symbol::X), None,
            None, None, None,
        ];
        assert_eq!(game.check_winner(), Some(Symbol::O));
        assert_eq!(game.won_line, Some([1, 2, 3]));

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), None,
            None, None, None,
        ];
        assert_eq!(game.check_winner(), None);
        assert_eq!(game.won_line, None);

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
        ];
        assert_eq!(game.check_winner(), None);
        assert_eq!(game.won_line, None);

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
        ];
        assert_eq!(game.check_winner(), Some(Symbol::O));
        assert_eq!(game.won_line, Some([7, 8, 9]));

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::X), Some(Symbol::O),
        ];
        assert_eq!(game.check_winner(), None);
        assert_eq!(game.won_line, None);

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
        ];
        assert_eq!(game.check_winner(), Some(Symbol::X));
        assert_eq!(game.won_line, Some([3, 5, 7]));
    }

    #[test]
    fn test_check_over_having_winner() {
        let mut game = Game::default();
        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
            Some(Symbol::X), Some(Symbol::X), None,
            None, None, None,
        ];
        game.check_over();
        assert_eq!(game.is_over, true);
        assert_eq!(game.winner, Some(Symbol::O));
    }

    #[test]
    fn test_check_over_having_no_winner() {
        let mut game = Game::default();
        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::X), Some(Symbol::O),
        ];
        game.check_over();
        assert_eq!(game.is_over, true);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn test_check_over_for_not_over() {
        let mut game = Game::default();
        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), None, None,
        ];
        game.check_over();
        assert_eq!(game.is_over, false);
        assert_eq!(game.winner, None);
    }

    #[test]
    fn test_current_step() {
        let mut game = Game::default();
        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), None, None,
        ];
        assert_eq!(game.current_step(), 7);

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::X), None,
        ];

        assert_eq!(game.current_step(), 8);
    }

    #[test]
    fn test_play() {
        let mut game = Game::default();
        game.play(1).unwrap();
        assert_eq!(game.cells[0], Some(Symbol::O));
        assert_eq!(game.current_step(), 1);

        game.play(2).unwrap();
        assert_eq!(game.cells[1], Some(Symbol::X));
        assert_eq!(game.current_step(), 2);

        game.play(3).unwrap();
        assert_eq!(game.cells[2], Some(Symbol::O));
        assert_eq!(game.current_step(), 3);

        game.play(4).unwrap();
        assert_eq!(game.cells[3], Some(Symbol::X));
        assert_eq!(game.current_step(), 4);

        game.play(5).unwrap();
        assert_eq!(game.cells[4], Some(Symbol::O));
        assert_eq!(game.current_step(), 5);

        game.play(6).unwrap();
        assert_eq!(game.cells[5], Some(Symbol::X));
        assert_eq!(game.current_step(), 6);
    }

    #[test]
    fn test_play_with_counter() {
        let mut game = Game::default();
        game.play_with_counter(1).unwrap();
        assert_eq!(game.cells[0], Some(Symbol::O));
        assert_eq!(game.current_step(), 2);
    }

    use std::mem::size_of;

    #[test]
    fn the_size_of_game_is_15_bytes() {
        assert_eq!(size_of::<Game>(), 15);
    }
}