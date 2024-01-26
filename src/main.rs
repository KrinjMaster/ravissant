use std::process::exit;

mod constants;
mod magic;
use board::print_bitboard;
use move_generation::generate_king_moves;

use crate::constants::{BOARD_SQUARES, DEFAULT_FEN_STRING};

mod board;
use crate::board::{BoardState, Color, Piece};

mod piece_parsing;
use crate::piece_parsing::parse_bitboards;

mod move_generation;
use crate::move_generation::{generate_knight_moves, generate_pawn_moves};

fn main() {
    let board = BoardState::from_fen(DEFAULT_FEN_STRING).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });

    // generate pseudo legal moves
    let white_pawn_moves = generate_pawn_moves(
        parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Pawn)),
        board.get_color_bb(Color::White),
        board.get_color_bb(Color::Black),
        board.bb_en_passant,
    );

    let _white_king_moves = generate_king_moves(
        parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::King)),
        board.get_color_bb(Color::White),
    );

    let white_knight_moves = generate_knight_moves(
        parse_bitboards(
            Color::White,
            board.get_piece_bb(Color::White, Piece::Knight),
        ),
        board.get_color_bb(Color::White),
    );

    print_bitboard(white_pawn_moves[1]);
}
