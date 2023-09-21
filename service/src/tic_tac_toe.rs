use core::tic_tac_toe::Game;

#[derive(Debug)]
pub enum Error {
    GameRules(String),
    GameOver,
    NotFound,
    Unknown,
}

pub trait TicTacToeService {
    fn new(&self) -> Result<(usize, Game), Error>;
    fn get(&self, id: usize) -> Result<Game, Error>;
    fn play(&self, id: usize, num: usize) -> Result<Game, Error>;
    fn delete(&self, id: usize) -> Result<(), Error>;
}


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct InMemoryTicTacToeService {
    // 定義InMemory服務
    games: Arc<Mutex<HashMap<usize, Game>>>, // 這什麼型別!?，請看下方補充教材
}

impl InMemoryTicTacToeService {
    // 實作建構式，這次我們改用new
    fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl TicTacToeService for InMemoryTicTacToeService {
    fn new(&self) -> Result<(usize, Game), Error> {
        let mut games = self.games
            .lock()                // LockResult<MutexGuard<HashMap<…>>>
            .unwrap();             // MutexGuard<HashMap<…>>
        let id = games.len() + 1;  // 遞增序號
        let game = Game::default();
        games.insert(id, game.clone());    // HashMap新增 key/value方式
        Ok((id, game))
    }

    fn get(&self, id: usize) -> Result<Game, Error> {
        todo!()
    }

    fn play(&self, id: usize, num: usize) -> Result<Game, Error> {
        todo!()
    }

    fn delete(&self, id: usize) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let service = InMemoryTicTacToeService::new();
        let (id, game) = service.new().unwrap();  // unwrap取得Result內容
        assert_eq!(id, 1);
        assert_eq!(game.is_over, false);
        let is_empty = game.cells.iter().all(|&x| x == None); // 驗證每一格都是空的
        // let is_empty = game.cells.iter().all(|x| *x == None); // 或是這樣寫
        assert_eq!(is_empty, true);
    }
}