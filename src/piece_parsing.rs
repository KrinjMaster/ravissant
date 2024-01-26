use crate::board::{Bitboard, Color};
use crate::constants::BOARD_SQUARES;

pub fn parse_bitboards(color: Color, bitboards: Bitboard) -> Vec<(u32, u32)> {
    let piece_color = match color {
        Color::White => 0,
        Color::Black => 1,
    };

    let mut pieces_vector: Vec<(u32, u32)> = vec![];

    let mut bb_pieces: u64 = bitboards.clone();

    while bb_pieces != 0 {
        let square_index: u32 = bb_pieces.trailing_zeros();

        pieces_vector.push((piece_color, square_index));

        bb_pieces ^= BOARD_SQUARES[square_index as usize];
    }

    pieces_vector
}
