use std::process::exit;

mod board;
mod constants;
mod magic;
mod move_generation;
mod piece_parsing;
mod utils;

use board::BoardState;
use constants::DEFAULT_FEN_STRING;

fn main() {
    let _board = BoardState::from_fen(DEFAULT_FEN_STRING).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });
}

#[cfg(test)]
mod test;
