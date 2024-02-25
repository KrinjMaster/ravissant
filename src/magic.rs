// i decided not to leave magics generationg
// dunno if you did want to see it here thou
use crate::{
    board::Bitboard,
    constants::{BISHOP_MOVES, ROOK_MOVES},
};

#[derive(Debug, Copy, Clone)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

pub fn get_bishop_move(magic_entry: MagicEntry, bb_blockers: Bitboard) -> Bitboard {
    let blockers = bb_blockers & magic_entry.mask;
    let index = (blockers.wrapping_mul(magic_entry.magic)) >> (magic_entry.shift);

    BISHOP_MOVES[(magic_entry.offset + index as u32) as usize]
}

pub fn get_rook_move(magic_entry: MagicEntry, bb_blockers: Bitboard) -> Bitboard {
    let blockers = bb_blockers & magic_entry.mask;
    let index = (blockers.wrapping_mul(magic_entry.magic)) >> (magic_entry.shift);

    ROOK_MOVES[(magic_entry.offset + index as u32) as usize]
}
