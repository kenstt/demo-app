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

pub trait TicTacToeService: Clone + Send + Sync {
    fn new_game(&self) -> Result<(usize, Game), Error>;
    fn get(&self, id: usize) -> Result<Game, Error>;
    fn play(&self, id: usize, num: usize) -> Result<Game, Error>;
    fn delete(&self, id: usize) -> Result<(), Error>;
}


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct InMemoryTicTacToeService {
    // 定義InMemory服務
    games: Arc<Mutex<HashMap<usize, Game>>>, // 這什麼型別!?，請看下方補充教材
}

impl InMemoryTicTacToeService {
    // 實作建構式，這次我們改用new
    pub fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryTicTacToeService {
    fn default() -> Self {
        Self {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl TicTacToeService for InMemoryTicTacToeService {
    fn new_game(&self) -> Result<(usize, Game), Error> {
        let mut games = self.games
            .lock()                // LockResult<MutexGuard<HashMap<…>>>
            .unwrap();             // MutexGuard<HashMap<…>>
        // let id = games.len() + 1; 原寫法有問題
        let id = if games.iter().count() == 0 {
            1                                                   // 不加; 視為 if 的回傳值
        } else {
            games.iter().max_by_key(|(k, _)| *k).unwrap().0 + 1 // 不加; 視為 if 的回傳值
        };
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
        let mut games = self.games.lock().unwrap();
        match games.remove(&id) {         // remove 會回傳Some(Game)
            Some(_) => Ok(()),            // Reulst的Ok型別是()
            None => Err(Error::NotFound),
        }                                 // 注意這裡沒;結尾，所以表示match表達式結果直接回傳
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let service = InMemoryTicTacToeService::new();
        let (id, game) = service.new_game().unwrap();  // unwrap取得Result內容
        assert_eq!(id, 1);
        assert!(!game.is_over);
        let is_empty = game.cells.iter().all(|&x| x.is_none()); // 驗證每一格都是空的
        // let is_empty = game.cells.iter().all(|x| *x == None); // 或是這樣寫
        assert!(is_empty);
    }

    #[test]
    fn test_get() {                                    // 測試 Happy Path
        let service = InMemoryTicTacToeService::new();
        let _ = service.new_game();                         // 新局的變數我們不需要，直接 _ 丟棄
        let game = service.get(1).unwrap();            // 透過 get 取得 game
        assert!(!game.is_over);               // 簡單驗一下內容
        let is_empty = game.cells.iter().all(|x| x.is_none());
        assert!(is_empty);
    }

    #[test]
    fn test_get_none() {                               // 測試 None 會回傳Error
        let service = InMemoryTicTacToeService::new();
        let game = service.get(10);                    // Result<Game, Error>
        assert!(game.is_err());               // 驗證回傳Error
        assert_eq!(game.err(), Some(Error::NotFound)); // 驗證Error類別
    }

    #[test]
    fn test_play() {
        let service = InMemoryTicTacToeService::new();
        let _ = service.new_game();                      // 建立一筆id=1的遊戲局
        let game = service.play(1, 1).unwrap();     //呼叫 play
        assert_eq!(game.cells[0], Some(core::tic_tac_toe::Symbol::O));

        let game = service.get(1).unwrap();         // 透過 get 確認修改是否回存
        assert_eq!(game.cells[0], Some(core::tic_tac_toe::Symbol::O));
    }

    #[test]
    fn test_play_two_round() {
        let service = InMemoryTicTacToeService::new();
        let _ = service.new_game();
        let game = service.play(1, 1).unwrap();
        let game = if game.cells[3] == Some(core::tic_tac_toe::Symbol::X) {
            service.play(1, 2).unwrap()
        } else {
            service.play(1, 3).unwrap()
        };
        let steps = game.cells.iter().filter(|x| x.is_some()).count();
        assert_eq!(steps, 4);   // 驗證執行完含電腦總步數是4
    }

    #[test]
    fn test_delete() {
        let service = InMemoryTicTacToeService::new();
        let _ = service.new_game();                         // 先製造1個才有東西刪除
        let result = service.delete(1);                // 執行刪除
        assert!(result.is_ok());              // 驗證執行成功
        let game = service.get(1);                     // 覆驗已無編號1資料
        assert!(game.is_err());
        assert_eq!(game.err(), Some(Error::NotFound));
    }

    #[test]
    fn test_delete_none() {
        let service = InMemoryTicTacToeService::new();
        let result = service.delete(10);                // 刪除不存在資料
        assert!(result.is_err());              // 如預期報錯
        assert_eq!(result.err(), Some(Error::NotFound));
    }
}

