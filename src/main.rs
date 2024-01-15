use std::process::exit;

mod constants;
use crate::constants::DEFAULT_FEN_STRING;

mod board;
use crate::board::BoardState;

mod piece_parsing;
use crate::piece_parsing::parse_bitboards_into_vector;

mod move_generation;
use crate::move_generation::generate_pawn_moves;

fn main() {
    let board = BoardState::from_fen(DEFAULT_FEN_STRING).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });

    // generate pseudo legal moves for black pawns
    let pawn_moves = generate_pawn_moves(
        parse_bitboards_into_vector(1, board.bb_pieces[1][0]),
        board.bb_colors[1],
        board.bb_colors[0],
    );
}
