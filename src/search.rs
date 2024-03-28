use crate::{
    board::{BoardState, Color, Move},
    eval::evaluate,
};

pub fn negamax(board: &mut BoardState, depth: u8, moves: Vec<Move>) -> (i32, Move) {
    let mut max: i32 = -100_000;
    let mut best_move: Move = 0;

    if depth == 0 {
        let score = evaluate(board);

        return (score, best_move);
    }

    for piece_move in moves.iter() {
        let (_, _, _, color, _, _) = board.decode_move(*piece_move, false);
        board.make_move(*piece_move);

        let opposite_color: Color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        if board.clone().is_in_check(opposite_color) {
            let _ = board.undo_move();
            continue;
        }

        let score = -(negamax(
            board,
            depth - 1,
            board.generate_moves_by_color(&board.to_move),
        )
        .0);

        if score > max {
            best_move = *piece_move;
            max = score;
        }

        let _ = board.undo_move();
    }

    (max, best_move)
}
