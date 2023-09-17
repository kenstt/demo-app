use core::tic_tac_toe::{Game, Cell};

fn main() {
    let mut game = Game::default();
    println!("{}", game);
    loop {                               // 開始迴圈
        println!("請輸入數字 1 and 9");    // 提示用戶輸入
        let num = 1;               // 取得用戶輸入
        game.play(num);                  // 處理遊戲邏輯
        if game.is_over { break; }       // 若遊戲結束，離開迴圈
    }
    println!("{}", game);                // 印出結果
}