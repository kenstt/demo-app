use std::fmt::{Display, Formatter};

pub enum Player {
    A,
    B,
}

#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Empty,
    O,
    X,
}

#[derive(Debug)]
pub struct Game {
    pub cells: [Cell; 9],
    pub is_over: bool,
    pub winner: Option<Cell>,
}

impl Game {
    // 替Game加上方法
    pub fn play(&mut self, num: usize) {   // 這裡的方法參數為自己與外部輸入值
        let index = num - 1;
        self.cells[index] = Cell::O;
    }
}

impl Default for Game {
    // 替Game實作預設(無參數建構式)
    fn default() -> Self {     // 這裡的大寫Self指的是Self的Type，就是指Game
        Self {                 // 所以這邊的兩個Self換成Game也是一樣的結果
            cells: [Cell::Empty; 9],
            is_over: false,
            winner: None,
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
            self.cells[0], self.cells[1], self.cells[2],
            self.cells[3], self.cells[4], self.cells[5],
            self.cells[6], self.cells[7], self.cells[8],
        )
    }
}

impl Display for Cell {
    // 替 Cell 實現 Display 這個 trait
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::O => write!(f, "O"),
            Cell::X => write!(f, "X"),
        }
    }
}