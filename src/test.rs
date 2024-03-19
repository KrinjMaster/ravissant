use crate::{board::BoardState, constants::DEFAULT_FEN_STRING};

#[test]
fn from_fen_ok() {
    assert!(BoardState::from_fen(DEFAULT_FEN_STRING).is_ok())
}

#[test]
fn initial_moves() {
    let board = BoardState::from_fen(DEFAULT_FEN_STRING).expect("Fail during board setup");
    assert_eq!(
        board.generate_moves_by_color(&board.to_move).len(),
        (8 * 2) + (2 * 2)
    )
}

#[test]
fn in_check() {
    let board =
        BoardState::from_fen("rnbqkbnr/ppppp1pp/8/5p1Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 0 1")
            .expect("Fail during board setup");
    assert!(board.is_in_check())
}
