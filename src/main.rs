use std::process::exit;
mod board;
mod constants;

fn main() {
    let board = board::BoardState::from_fen(constants::DEFAULT_FEN_STRING).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });
}
