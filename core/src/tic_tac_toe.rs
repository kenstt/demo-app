use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("已非空格，不能再次畫記！")]
    AlreadyOccupied,
    #[error("遊戲已結束，無法操作！")]
    GameOver,
}

#[derive(Copy, Clone, Debug)]
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
    pub fn current_step(&self) -> usize {
        self.cells.iter().filter(|x| x.is_some()).count() // 算步數
    }

    pub fn play(&mut self, num: usize) -> Result<(), Error>{
        let index = num - 1;
        if self.cells[index].is_some() {
            return Err(Error::AlreadyOccupied);
        }
        let symbol = self.symbols[self.current_step() % 2];
        self.cells[index] = Some(symbol);
        Ok(())
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