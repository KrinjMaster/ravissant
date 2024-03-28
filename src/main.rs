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
use constants::DEFAULT_FEN_STRING;

use crate::board::Color;
// use search::negamax;
// use utils::print_bitboard;

fn main() {
    // let _board = BoardState::from_fen(DEFAULT_FEN_STRING).unwrap_or_else(|err| {
    //     println!("{}", err);
    //     exit(1);
    // });
    let board =
        BoardState::from_fen("rnbqkbnr/ppppp1pp/8/5p1Q/4P3/8/PPPP1PPP/RNB1KBNR w KQkq f6 0 1")
            .expect("Fail during board setup");
    println!("{}", board.is_in_check(Color::Black));
}

#[cfg(test)]
mod test;
