use std::process::exit;

mod board;
mod constants;
mod magic;
mod move_generation;
mod piece_parsing;
mod utils;

use board::{BoardState, Color, Piece};
use constants::DEFAULT_FEN_STRING;
use move_generation::{generate_king_moves, generate_knight_moves, generate_pawn_moves};
use piece_parsing::parse_bitboards;

fn main() {
    let board = BoardState::from_fen(DEFAULT_FEN_STRING).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1);
    });

    // generate pseudo legal moves
    let _white_pawn_moves = generate_pawn_moves(
        parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Pawn)),
        board.get_color_bb(Color::White),
        board.get_color_bb(Color::Black),
        board.bb_en_passant,
    );

    let _white_king_moves = generate_king_moves(
        parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::King)),
        board.get_color_bb(Color::White),
    );

    let _white_knight_moves = generate_knight_moves(
        parse_bitboards(
            Color::White,
            board.get_piece_bb(Color::White, Piece::Knight),
        ),
        board.get_color_bb(Color::White),
    );

    // let magic_entry = ROOK_MAGICS[4];
    // let occucancies = set_occupancies(4095, count_ones(magic_entry.mask), magic_entry.mask);
    //
    // print_bitboard(get_rook_move(magic_entry, occucancies));
}
