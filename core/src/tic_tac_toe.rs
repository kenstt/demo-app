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
}