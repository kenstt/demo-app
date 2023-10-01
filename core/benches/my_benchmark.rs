use criterion::{black_box, criterion_group, criterion_main, Criterion};

use core::tic_tac_toe::{Game, Symbol};

fn test_check_winner() {
    let mut game = Game::default();
    game.cells = [
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
        Some(Symbol::X), Some(Symbol::X), None,
        None, None, None,
    ];
    assert_eq!(game.check_winner(), Some(Symbol::O));
    assert_eq!(game.won_line, Some([1, 2, 3]));

    game.cells = [
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
        Some(Symbol::X), Some(Symbol::X), None,
        None, None, None,
    ];
    assert_eq!(game.check_winner(), None);
    assert_eq!(game.won_line, None);

    game.cells = [
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
        Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
    ];
    assert_eq!(game.check_winner(), None);
    assert_eq!(game.won_line, None);

    game.cells = [
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
        Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::O),
    ];
    assert_eq!(game.check_winner(), Some(Symbol::O));
    assert_eq!(game.won_line, Some([7, 8, 9]));

    game.cells = [
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
        Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
        Some(Symbol::O), Some(Symbol::X), Some(Symbol::O),
    ];
    assert_eq!(game.check_winner(), None);
    assert_eq!(game.won_line, None);

    game.cells = [
        Some(Symbol::O), Some(Symbol::O), Some(Symbol::X),
        Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
        Some(Symbol::X), Some(Symbol::X), Some(Symbol::O),
    ];
    assert_eq!(game.check_winner(), Some(Symbol::X));
    assert_eq!(game.won_line, Some([3, 5, 7]));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("check winner", |b| b.iter(|| test_check_winner()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);