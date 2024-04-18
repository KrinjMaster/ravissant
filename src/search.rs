use crate::{
    board::BoardState,
    eval::{evaluate, CHECKMATE, DRAW},
};
use std::process::exit;

pub fn negamax(board: &mut BoardState, depth: u8) -> i32 {
    let mut max: i32 = -100_000_000;

    if depth == 0 {
        let score = evaluate(board);

        return score;
    }

    let moves = board.generate_moves_by_color(&board.to_move);

    if moves.is_empty() {
        if board.is_in_check(&board.to_move) {
            return CHECKMATE;
        }

        return DRAW;
    }

    for piece_move in moves.iter() {
        let (_, _, _, color, _, _) = board.decode_move(*piece_move).unwrap_or_else(|err| {
            println!("{}", err);
            exit(1);
        });

        board.make_move(*piece_move);

        if board.is_in_check(&color) {
            let _ = board.undo_move();
            continue;
        }

        let score = negamax(board, depth - 1);

        if -score > max {
            max = -score;
        }

        let _ = board.undo_move();
    }

    max
}
