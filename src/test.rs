use crate::{
    board::{BoardState, Color, Piece},
    constants::{BOARD_SQUARES, DEFAULT_FEN_STRING},
};

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
    assert!(board.is_in_check(Color::Black))
}

#[test]
fn encoding_test() {
    let board = BoardState::from_fen(DEFAULT_FEN_STRING).expect("Fail during board setup");

    let moves = board.encode_move(62, 52, Piece::None);
    // choose a white right knight move

    assert_eq!(moves, 27966);
}

#[test]
fn decoding_test() {
    let board = BoardState::from_fen(DEFAULT_FEN_STRING).expect("Fail during board setup");

    let moves = board.decode_move(board.generate_moves_by_color(&Color::White)[18]);
    // choose a white right knight move

    assert_eq!(moves.0, BOARD_SQUARES[62]);
    assert_eq!(moves.1, BOARD_SQUARES[45]);
    assert!(matches!(moves.2, Piece::Knight));
    assert!(matches!(moves.3, Color::White));
    assert!(matches!(moves.4, Piece::None));
}
