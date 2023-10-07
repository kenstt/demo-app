use core::tic_tac_toe::{Game, Symbol};

#[test]
fn test_game1_for_x_win() {          // 測試X贏的情境
    let mut game = Game::default();
    game.play(3).unwrap();           // O
    game.play(1).unwrap();           // X
    game.play(2).unwrap();           // O
    let err = game.play(1);          // 下到非空的格子
    assert!(err.is_err());  // 應該會回傳錯誤

    game.play(4).unwrap();           // X
    game.play(5).unwrap();           // O
    assert!(!game.is_over); // 確定分出勝負前的狀態
    assert_eq!(game.winner, None);   // 棋局尚未結束，且尚無贏家
    game.play(7).unwrap();           // X -> 連成一線 1, 4, 7
    assert!(game.is_over);  // 棋局狀態為結束
    assert_eq!(game.winner, Some(Symbol::X)); // 棋局贏家為 O

    let err = game.play(6);
    assert!(err.is_err());
}

#[test]
fn test_game2_for_draw() {           // 模擬平手情境
    let mut game = Game::default();
    game.play(1).unwrap();           // O
    game.play(4).unwrap();           // X
    game.play(2).unwrap();           // O
    game.play(5).unwrap();           // X
    game.play(6).unwrap();           // O
    game.play(3).unwrap();           // X
    game.play(7).unwrap();           // O
    game.play(8).unwrap();           // X
    assert!(!game.is_over);
    game.play(9).unwrap();           // O
    assert!(game.is_over);  // 棋局已結束
    assert_eq!(game.winner, None);   // 但無玩家勝出
}

// 把game.cells的pub拿掉，測試會失敗
#[test]
fn test_access_game_cell() {
    let mut game = Game::default();
    game.cells = [
        None, None, None,
        None, None, None,
        None, None, None,
    ];
}