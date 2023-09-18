use std::fmt::{Display, Formatter};

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
}

impl Game {
    // 替Game加上方法
    pub fn play(&mut self, num: usize) {   // 這裡的方法參數為自己與外部輸入值
        let index = num - 1;
        self.cells[index] = Some(Symbol::O);
    }
}

impl Default for Game {
    // 替Game實作預設(無參數建構式)
    fn default() -> Self {     // 這裡的大寫Self指的是Self的Type，就是指Game
        Self {                 // 所以這邊的兩個Self換成Game也是一樣的結果
            cells: [None; 9],
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