use rand::prelude::SliceRandom;
use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("已非空格，不能再次畫記！")]
    AlreadyOccupied,
    #[error("遊戲已結束，無法操作！")]
    GameOver,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Symbol {
    O,
    X,
}

#[derive(Debug)]
pub struct Game {
    pub cells: [Option<Symbol>; 9],
    pub is_over: bool,
    pub winner: Option<Symbol>,
    pub symbols: [Symbol; 2],   // 交替下棋用
}

impl Game {
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

    pub fn current_step(&self) -> usize {
        self.cells.iter().filter(|x| x.is_some()).count() // 算步數
    }

    pub fn play(&mut self, num: usize) -> Result<(), Error> {
        if self.is_over {                    // 一開始先判斷遊戲結束就報錯
            return Err(Error::GameOver);
        }
        let index = num - 1;
        if self.cells[index].is_some() {
            return Err(Error::AlreadyOccupied);
        }
        let symbol = self.symbols[self.current_step() % 2];
        self.cells[index] = Some(symbol);
        self.check_over();                   // 玩家每一步結束後判斷是否結束
        Ok(())
    }

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

    pub fn check_winner(&mut self) -> Option<Symbol> {
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
            match line {
                [Some(Symbol::O), Some(Symbol::O), Some(Symbol::O)] => return Some(Symbol::O),
                [Some(Symbol::X), Some(Symbol::X), Some(Symbol::X)] => return Some(Symbol::X),
                _ => (),
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
            symbols: [Symbol::O, Symbol::X], // 未來開心的話可以改順序，或加上奇怪的符號(?)△☆★ （？
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

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), None,
            None, None, None,
        ];
        assert_eq!(game.check_winner(), None);

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
        ];
        assert_eq!(game.check_winner(), None);

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
        ];
        assert_eq!(game.check_winner(), Some(Symbol::O));

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::O), Some(Symbol::X), Some(Symbol::O),
        ];
        assert_eq!(game.check_winner(), None);

        game.cells = [
            Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
            Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
        ];
        assert_eq!(game.check_winner(), Some(Symbol::X));
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
}