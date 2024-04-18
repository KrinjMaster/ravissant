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

use search::negamax;

fn main() {
    let mut board = BoardState::from_fen(DEFAULT_FEN_STRING).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });

    let score = negamax(&mut board, 3);

    println!("{}", score);
}

#[cfg(test)]
mod test;
