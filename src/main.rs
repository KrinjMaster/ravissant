use std::process::exit;

mod board;
mod constants;
mod eval;
mod magic;
mod move_generation;
mod piece_parsing;
mod search;
mod utils;

use board::BoardState;
use board::Color;
use constants::DEFAULT_FEN_STRING;
use search::negamax;
use utils::print_bitboard;

use crate::eval::evaluate;

fn main() {
    let mut board =
        BoardState::from_fen("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 1")
            .unwrap_or_else(|err| {
                println!("{}", err);
                exit(1);
            });

    let moves = board.generate_moves_by_color(&Color::White);

    // for piece_move in moves.iter() {
    //     if piece_move.1 & board.get_color_bb(Color::Black) != 0 {
    //         board.make_move(&piece_move.2, &piece_move.3, (piece_move.0, piece_move.1));
    //     }
    // }

    // println!("{}", evaluate(&board));

    let best_score = negamax(&mut board, 2, moves);

    println!("best score: {}\n", best_score.0);
    print_bitboard(best_score.1 .0);
    print_bitboard(best_score.1 .1);

    // if it searches 3+ moves ahead it is starting goin crazy like 1860 score wtf??
}
