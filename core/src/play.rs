use core::tic_tac_toe::{Game, Cell};

fn main() {
    let game = Game {
        cells: [Cell::Empty; 9]
    };
    println!("{:?}", game);
}