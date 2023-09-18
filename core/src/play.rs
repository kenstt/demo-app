use core::tic_tac_toe::Game;

fn main() {
    let mut game = Game::default();                // 遊戲內容會變動，故給 mut
    println!("{}", game);
    loop {
        println!("請輸入數字 1 and 9");

        let mut input = String::new();                  // 存放 User 輸入的變數

        let _ = std::io::stdin()                        // 讀取輸入
            .read_line(&mut input);                     // Result<usize> 回傳結果是讀取的Bytes

        match input.trim().parse::<usize>() {           // 嘗試轉換輸入內容為數字
            Ok(num) => {                                // 轉換成功
                if num < 1 || num > 9 {                 // 檢核輸入範圍不符
                    println!("數字範圍錯誤，請輸入數字 1 ~ 9");
                    continue;                           // 提示並跳下一輪請User重新輸入
                }
                println!("你輸入的是: {}", num);          // Debug 用 (?)
                let round = game.play_with_counter(num);
                if round.is_err() {                     // 加上遇到錯誤便打印出來
                    println!("錯誤：{}", round.err().unwrap());
                    continue;
                }
            }
            Err(_) => {                                  // 解析錯誤
                println!("輸入內容錯誤：請輸入數字 1 ~ 9：");
                continue;                                // 提示並跳下一輪請User重新輸入
            }
        };

        println!("{}", game);                            // 印出運行後結果

        if game.is_over {                                // 處理遊戲結束
            if game.winner.is_some() {                   // 顯示贏家或平手
                println!("遊戲結束：贏家是：{}", game.winner.unwrap());
            } else {
                println!("遊戲結束：平手");
            }
            break;                                       // 結束遊戲
        }
    }
    println!("{}", game);
}
