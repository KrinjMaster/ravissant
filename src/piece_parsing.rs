use crate::constants::{BitBoard, BOARD_SQUARES};

pub fn parse_bitboards_into_vector(color: u32, bitboards: BitBoard) -> Vec<(u32, u32)> {
    let mut pieces_vector: Vec<(u32, u32)> = vec![];

    let mut bb_pieces: u64 = bitboards.clone();

    while bb_pieces != 0 {
        let square_index: u32 = bb_pieces.trailing_zeros();

        pieces_vector.push((color, square_index));

        bb_pieces ^= BOARD_SQUARES[square_index as usize];
    }

    pieces_vector
}
