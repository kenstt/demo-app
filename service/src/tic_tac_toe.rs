use core::tic_tac_toe::Game;

#[derive(Debug, PartialEq)]
pub enum Error {
    GameRules(String),
    GameOver,
    NotFound,
    Unknown,
}

impl From<core::tic_tac_toe::Error> for Error {
    // 實作訊息裡提到的 trait
    fn from(e: core::tic_tac_toe::Error) -> Self {
        match e {                                      // 把 mapping邏輯寫這
            core::tic_tac_toe::Error::GameOver => Error::GameOver,
            _ => Error::GameRules(e.to_string()),
        }
    }
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
        let games = self.games.lock().unwrap();
        games.get(&id).cloned().ok_or(Error::NotFound)
        // 或
        // games.get(&id).map(|game| game.clone()).ok_or(Error::NotFound)
        // 或
        // match games.get(&id) {
        //     Some(game) => Ok(game.clone()),
        //     None => Err(Error::NotFound),
        // }
    }

    fn play(&self, id: usize, num: usize) -> Result<Game, Error> {
        let mut games = self.games.lock().unwrap();
        let game = games.get_mut(&id).ok_or(Error::NotFound)?;
        game.play_with_counter(num)?;
        // 另一種寫法
        // game.play_with_counter(num).map_err(|e| match e {
        //     core::tic_tac_toe::Error::GameOver => Error::GameOver,
        //     core::tic_tac_toe::Error::AlreadyOccupied
        //     => Error::GameRules("AlreadyOccupied".to_string()),
        // })?;

        Ok(game.clone())
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

    #[test]
    fn test_get() {                                    // 測試 Happy Path
        let service = InMemoryTicTacToeService::new();
        let _ = service.new();                         // 新局的變數我們不需要，直接 _ 丟棄
        let game = service.get(1).unwrap();            // 透過 get 取得 game
        assert_eq!(game.is_over, false);               // 簡單驗一下內容
        let is_empty = game.cells.iter().all(|x| *x == None);
        assert_eq!(is_empty, true);
    }

    #[test]
    fn test_get_none() {                               // 測試 None 會回傳Error
        let service = InMemoryTicTacToeService::new();
        let game = service.get(10);                    // Result<Game, Error>
        assert_eq!(game.is_err(), true);               // 驗證回傳Error
        assert_eq!(game.err(), Some(Error::NotFound)); // 驗證Error類別
    }

    #[test]
    fn test_play() {
        let service = InMemoryTicTacToeService::new();
        let _ = service.new();                      // 建立一筆id=1的遊戲局
        let game = service.play(1, 1).unwrap();     //呼叫 play
        assert_eq!(game.cells[0], Some(core::tic_tac_toe::Symbol::O));

        let game = service.get(1).unwrap();         // 透過 get 確認修改是否回存
        assert_eq!(game.cells[0], Some(core::tic_tac_toe::Symbol::O));
    }

    #[test]
    fn test_play_two_round() {
        let service = InMemoryTicTacToeService::new();
        let _ = service.new();
        let game = service.play(1, 1).unwrap();
        let game = if game.cells[3] == Some(core::tic_tac_toe::Symbol::X) {
            service.play(1, 2).unwrap()
        } else {
            service.play(1, 3).unwrap()
        };
        let steps = game.cells.iter().filter(|x| x.is_some()).count();
        assert_eq!(steps, 4);   // 驗證執行完含電腦總步數是4
    }
}

