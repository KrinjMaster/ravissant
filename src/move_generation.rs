use crate::board::Bitboard;
use crate::constants::{
    BISHOP_MAGICS, BOARD_SQUARES, KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS, ROOK_MAGICS,
};
use crate::magic::{get_bishop_move, get_rook_move};

pub fn generate_pawn_moves(
    pawns: Vec<(u32, u32)>,
    bb_friendly_pieces: Bitboard,
    bb_enemy_pieces: Bitboard,
    bb_en_passant: Bitboard,
) -> Vec<(Bitboard, Bitboard)> {
    let mut bb_moves_vec: Vec<(Bitboard, Bitboard)> = vec![];

    let bb_fullboard = bb_friendly_pieces | bb_enemy_pieces;

    for pawn in pawns.iter() {
        let mut bb_pawn_moves: Bitboard = 0;

        let attack_squares = PAWN_ATTACKS[pawn.0 as usize][pawn.1 as usize];

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

        bb_moves_vec.push((BOARD_SQUARES[pawn.1 as usize], bb_pawn_moves));
    }

    bb_moves_vec
}

pub fn generate_king_moves(
    kings: Vec<(u32, u32)>,
    bb_friendly_pieces: Bitboard,
) -> Vec<(Bitboard, Bitboard)> {
    let mut bb_moves_vec: Vec<(Bitboard, Bitboard)> = vec![];

    for king in kings.iter() {
        let bb_moves: Bitboard = KING_ATTACKS[king.1 as usize];

        bb_moves_vec.push((
            BOARD_SQUARES[king.1 as usize],
            (bb_moves ^ bb_friendly_pieces) & bb_moves,
        ));
    }

    bb_moves_vec
}

pub fn generate_knight_moves(
    knights: Vec<(u32, u32)>,
    bb_friendly_pieces: Bitboard,
) -> Vec<(Bitboard, Bitboard)> {
    let mut bb_moves_vec: Vec<(Bitboard, Bitboard)> = vec![];

    for knight in knights.iter() {
        let bb_moves: Bitboard = KNIGHT_ATTACKS[knight.1 as usize];

        bb_moves_vec.push((
            BOARD_SQUARES[knight.1 as usize],
            (bb_moves ^ bb_friendly_pieces) & bb_moves,
        ));
    }

    bb_moves_vec
}

pub fn generate_rook_moves(
    rooks: Vec<(u32, u32)>,
    bb_friendly_pieces: Bitboard,
    bb_fullboard: Bitboard,
) -> Vec<(Bitboard, Bitboard)> {
    let mut bb_moves_vec: Vec<(Bitboard, Bitboard)> = vec![];

    for rook in rooks.iter() {
        bb_moves_vec.push((
            BOARD_SQUARES[rook.1 as usize],
            get_rook_move(ROOK_MAGICS[rook.1 as usize], bb_fullboard) & !bb_friendly_pieces,
        ));
    }

    bb_moves_vec
}

pub fn generate_bishop_moves(
    bishops: Vec<(u32, u32)>,
    bb_friendly_pieces: Bitboard,
    bb_fullboard: Bitboard,
) -> Vec<(Bitboard, Bitboard)> {
    let mut bb_moves_vec: Vec<(Bitboard, Bitboard)> = vec![];

    for bishop in bishops.iter() {
        bb_moves_vec.push((
            BOARD_SQUARES[bishop.1 as usize],
            get_bishop_move(BISHOP_MAGICS[bishop.1 as usize], bb_fullboard) & !bb_friendly_pieces,
        ));
    }

    bb_moves_vec
}

pub fn generate_quen_moves(
    queens: Vec<(u32, u32)>,
    bb_fullboard: Bitboard,
    bb_friendly_pieces: Bitboard,
) -> Vec<(Bitboard, Bitboard)> {
    let mut bb_moves_vec: Vec<(Bitboard, Bitboard)> = vec![];

    let rook_moves = generate_rook_moves(queens.clone(), bb_friendly_pieces, bb_fullboard);
    let bishop_moves = generate_bishop_moves(queens.clone(), bb_friendly_pieces, bb_fullboard);

    if rook_moves.len() != 0 && bishop_moves.len() != 0 {
        for index in 0..queens.len() {
            // rook_moves and bishop_moves are equal in size so this loop should work just fine
            bb_moves_vec.push((
                rook_moves[index].0,
                rook_moves[index].1 | bishop_moves[index].1,
            ));
        }
    }

    bb_moves_vec
}
