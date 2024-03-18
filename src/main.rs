use std::process::exit;

mod board;
mod constants;
mod magic;
mod move_generation;
mod piece_parsing;
mod utils;

use board::BoardState;
use utils::print_bitboard;

use crate::board::Color;

fn main() {
    let board =
        BoardState::from_fen("rnbqkbnr/ppppp1pp/8/5p1Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq f6 0 1")
            .unwrap_or_else(|err| {
                println!("{}", err);
                exit(1);
            });

    let white_moves = board
        .generate_moves_by_color(&Color::White)
        .iter()
        .fold(0, |acc, cur| acc | cur.1);

    print_bitboard(white_moves);
    println!("{}", board.is_in_check());
}
