use std::process::exit;

mod constants;
use move_generation::generate_king_moves;

use crate::constants::DEFAULT_FEN_STRING;

mod board;
use crate::board::BoardState;

mod piece_parsing;
use crate::piece_parsing::parse_bitboards_into_vector;

mod move_generation;
use crate::move_generation::{generate_knight_moves, generate_pawn_moves};

fn main() {
    let board = BoardState::from_fen(DEFAULT_FEN_STRING).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });

    // generate pseudo legal moves
    let white_pawn_moves = generate_pawn_moves(
        parse_bitboards_into_vector(0, board.bb_pieces[0][0]),
        board.bb_colors[0],
        board.bb_colors[1],
        board.bb_en_passant,
    );

    let white_king_moves = generate_king_moves(
        parse_bitboards_into_vector(0, board.bb_pieces[0][5]),
        board.bb_colors[1],
    );

    let white_knight_moves = generate_knight_moves(
        parse_bitboards_into_vector(0, board.bb_pieces[0][2]),
        board.bb_colors[0],
    );
}
