use crate::{board::Bitboard, constants::ROOK_ATTACKS};

#[derive(Debug, Copy, Clone)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

pub fn get_rook_move(magic_entry: MagicEntry, bb_blockers: Bitboard) -> Bitboard {
    let blockers = bb_blockers & magic_entry.mask;
    let index = (blockers.wrapping_mul(magic_entry.magic)) >> (magic_entry.shift);

    ROOK_ATTACKS[(magic_entry.offset + index as u32) as usize]
}
