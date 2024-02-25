use crate::{
    board::BoardState,
    board::{Bitboard, Color, Piece},
    eval::evaluate,
};

pub fn negamax(
    board: &mut BoardState,
    depth: u8,
    moves: Vec<(Bitboard, Bitboard, Color, Piece)>,
) -> (i32, (Bitboard, Bitboard, Color, Piece)) {
    if depth == 0 {
        let score = evaluate(board);

        return (score, (0, 0, Color::White, Piece::None));
    }

    let mut max: i32 = -100_000;
    let mut best_move: (Bitboard, Bitboard, Color, Piece) = (0, 0, Color::White, Piece::None);

    for piece_move in moves.iter() {
        board.make_move(&piece_move.2, &piece_move.3, (piece_move.0, piece_move.1));

        let score = -negamax(
            board,
            depth - 1,
            board.generate_moves_by_color(&board.to_move),
        )
        .0;

        if score > max {
            best_move = *piece_move;
            max = score;
        }

        let _ = board.undo_move();
    }

    (max, best_move)
}
