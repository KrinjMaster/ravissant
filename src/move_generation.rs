use crate::constants::{BitBoard, BOARD_SQUARES, KING_MOVES, KNIGHT_MOVES, PAWN_ATTACK_SQUARES};

// rewrite whole fucking pawn move generation

pub fn generate_pawn_moves(
    pawns: Vec<(u32, u32)>,
    bb_friendly_pieces: BitBoard,
    bb_enemy_pieces: BitBoard,
    bb_en_passant: BitBoard,
) -> Vec<BitBoard> {
    let mut bb_moves_vec: Vec<BitBoard> = vec![];

    let bb_fullboard = bb_friendly_pieces | bb_enemy_pieces;

    for pawn in pawns.iter() {
        let mut bb_pawn_moves: BitBoard = 0;

        let attack_squares = PAWN_ATTACK_SQUARES[pawn.0 as usize][pawn.1 as usize];

        // if pawn can attack
        bb_pawn_moves |= (attack_squares ^ bb_friendly_pieces) & (attack_squares & bb_enemy_pieces);

        //  moves generation forward and en passant
        if pawn.0 == 0 {
            // white pawn
            if (BOARD_SQUARES[pawn.1 as usize] >> 8) | bb_fullboard != bb_fullboard {
                bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] >> 8;
            }

            // if pawn is on 2nd rank
            if pawn.1 >= 48 && pawn.1 <= 55 {
                if (BOARD_SQUARES[pawn.1 as usize] >> 16) | bb_fullboard != bb_fullboard {
                    bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] >> 16;
                }
            }

            // en passant
            if bb_enemy_pieces | (bb_en_passant << 8) == bb_enemy_pieces
                && bb_en_passant | attack_squares == attack_squares
            {
                // black pawn can get passanted
                bb_pawn_moves |= bb_en_passant;
            }
        } else {
            // black pawn
            if (BOARD_SQUARES[pawn.1 as usize] << 8) | bb_fullboard != bb_fullboard {
                bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] << 8;
            }

            // if pawn is on 7th rank
            if pawn.1 >= 8 && pawn.1 <= 15 {
                if (BOARD_SQUARES[pawn.1 as usize] << 16) | bb_fullboard != bb_fullboard {
                    bb_pawn_moves |= BOARD_SQUARES[pawn.1 as usize] << 16;
                }
            }

            // en passant
            if bb_enemy_pieces | (bb_en_passant >> 8) == bb_enemy_pieces
                && bb_en_passant | attack_squares == attack_squares
            {
                // white pawn can get passanted
                bb_pawn_moves |= bb_en_passant;
            }
        }

        bb_moves_vec.push(bb_pawn_moves);
    }

    bb_moves_vec
}

pub fn generate_king_moves(bb_king: Vec<(u32, u32)>, bb_friendly_pieces: BitBoard) -> BitBoard {
    let bb_moves: BitBoard = KING_MOVES[bb_king[0].1 as usize];

    (bb_moves ^ bb_friendly_pieces) & bb_moves
}

pub fn generate_knight_moves(
    knights: Vec<(u32, u32)>,
    bb_friendly_pieces: BitBoard,
) -> Vec<BitBoard> {
    let mut bb_moves_vec: Vec<BitBoard> = vec![];

    for knight in knights.iter() {
        let bb_moves: BitBoard = KNIGHT_MOVES[knight.1 as usize];

        bb_moves_vec.push((bb_moves ^ bb_friendly_pieces) & bb_moves);
    }

    bb_moves_vec
}
