use crate::constants::{BitBoard, BOARD_SQUARES, PAWN_ATTACK_SQUARES};

pub fn generate_pawn_moves(
    pawns: Vec<(u32, u32)>,
    bb_friendly_pieces: BitBoard,
    bb_enemy_pieces: BitBoard,
) -> Vec<BitBoard> {
    let mut bb_moves_vec: Vec<BitBoard> = vec![];

    let bb_fullboard = bb_friendly_pieces | bb_enemy_pieces;

    for pawn in pawns.iter() {
        let mut bb_pawn_moves: BitBoard = 0;

        let attack_squares = PAWN_ATTACK_SQUARES[pawn.0 as usize][pawn.1 as usize];

        // pseudo legal pawn forward moves
        if pawn.0 == 0 {
            // if pawn can move 1 square forward
            if bb_fullboard | BOARD_SQUARES[pawn.1 as usize] >> 8 != bb_fullboard {
                bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] >> 8;
            }
            // if pawn can move 2 squares forward
            if bb_fullboard | BOARD_SQUARES[pawn.1 as usize] >> 16 != bb_fullboard {
                bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] >> 16;
            }
        } else {
            // if pawn can move 1 square forward
            if bb_fullboard | BOARD_SQUARES[pawn.1 as usize] << 8 != bb_fullboard {
                bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] << 8;
            }
            // if pawn can move 2 squares forward
            if bb_fullboard | BOARD_SQUARES[pawn.1 as usize] << 16 != bb_fullboard {
                bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] << 16;
            }
        }

        // pseudo legal pawn attacks
        if bb_fullboard | attack_squares != bb_fullboard
            || bb_friendly_pieces | attack_squares == bb_friendly_pieces
        {
            // all attack squares are empty or
            // all attack squares are occupied by friendly pieces
        } else if bb_enemy_pieces | attack_squares == bb_enemy_pieces {
            // all attack squares are occupied by enemy pieces
            bb_pawn_moves |= attack_squares;
        } else {
            let mut attack_squares_copy = attack_squares.clone();

            while attack_squares_copy != 0 {
                let attack_square_index = attack_squares_copy.trailing_zeros();

                // if picked attacked square empty or not
                if bb_fullboard | BOARD_SQUARES[attack_square_index as usize] == bb_fullboard {
                    if bb_friendly_pieces | BOARD_SQUARES[attack_square_index as usize]
                        != bb_friendly_pieces
                    {
                        bb_pawn_moves |= BOARD_SQUARES[attack_square_index as usize];
                    }
                }

                attack_squares_copy ^= BOARD_SQUARES[attack_square_index as usize];
            }
        }

        bb_moves_vec.push(bb_pawn_moves);
    }

    bb_moves_vec
}
