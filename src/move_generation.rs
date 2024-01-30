use crate::board::{trailing_zeros, Bitboard};
use crate::constants::{BOARD_SQUARES, KING_MOVES, KNIGHT_MOVES, PAWN_ATTACK_SQUARES};

pub fn generate_pawn_moves(
    pawns: Vec<(u32, u32)>,
    bb_friendly_pieces: Bitboard,
    bb_enemy_pieces: Bitboard,
    bb_en_passant: Bitboard,
) -> Vec<Bitboard> {
    let mut bb_moves_vec: Vec<Bitboard> = vec![];

    let bb_fullboard = bb_friendly_pieces | bb_enemy_pieces;

    for pawn in pawns.iter() {
        let mut bb_pawn_moves: Bitboard = 0;

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

pub fn generate_king_moves(bb_king: Vec<(u32, u32)>, bb_friendly_pieces: Bitboard) -> Bitboard {
    let bb_moves: Bitboard = KING_MOVES[bb_king[0].1 as usize];

    (bb_moves ^ bb_friendly_pieces) & bb_moves
}

pub fn generate_knight_moves(
    knights: Vec<(u32, u32)>,
    bb_friendly_pieces: Bitboard,
) -> Vec<Bitboard> {
    let mut bb_moves_vec: Vec<Bitboard> = vec![];

    for knight in knights.iter() {
        let bb_moves: Bitboard = KNIGHT_MOVES[knight.1 as usize];

        bb_moves_vec.push((bb_moves ^ bb_friendly_pieces) & bb_moves);
    }

    bb_moves_vec
}

pub fn generate_rook_attacks_on_the_fly(square: u8, blockers: Bitboard) -> Bitboard {
    let mut attacks: Bitboard = 0;

    let rank: u8 = square / 8;
    let file: u8 = square % 8;

    for r in (0..rank).rev() {
        attacks |= BOARD_SQUARES[r as usize * 8 + file as usize];
        if BOARD_SQUARES[r as usize * 8 + file as usize] & blockers != 0 {
            break;
        }
    }
    for r in rank + 1..8 {
        attacks |= BOARD_SQUARES[r as usize * 8 + file as usize];
        if BOARD_SQUARES[r as usize * 8 + file as usize] & blockers != 0 {
            break;
        }
    }

    for f in (0..file).rev() {
        attacks |= BOARD_SQUARES[rank as usize * 8 + f as usize];
        if BOARD_SQUARES[rank as usize * 8 + f as usize] & blockers != 0 {
            break;
        }
    }

    for f in file + 1..8 {
        attacks |= BOARD_SQUARES[rank as usize * 8 + f as usize];

        if BOARD_SQUARES[rank as usize * 8 + f as usize] & blockers != 0 {
            break;
        }
    }

    attacks
}

pub fn set_occupancies(index: u32, bits_in_mask: u8, attack_mask: Bitboard) -> Bitboard {
    let mut attack_map: Bitboard = attack_mask.clone();
    let mut occupancy: Bitboard = 0;

    for count in 0..bits_in_mask {
        let square: Bitboard = BOARD_SQUARES[trailing_zeros(attack_map) as usize];

        attack_map ^= square;

        if index as u64 & (1u64 << count) != 0 {
            occupancy |= square;
        }
    }

    occupancy
}
