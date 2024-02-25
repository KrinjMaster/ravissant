// pieces value
// from https://www.chessprogramming.org/Simplified_Evaluation_Function

use crate::{
    board::{BoardState, Color, Piece},
    move_generation::generate_king_moves,
    piece_parsing::parse_bitboards,
};

pub const WHITE_CHECKMATE: i32 = 10_000;
pub const DRAW: i32 = 10_000;
pub const BLACK_CHECKMATE: i32 = -10_000;

pub const PAWN: i32 = 100;
pub const KNIGHT: i32 = 310;
pub const BISHOP: i32 = 330;
pub const ROOK: i32 = 500;
pub const QUEEN: i32 = 900;
pub const KING: i32 = 20_000;

pub const ENDGAME_PAWN: i32 = 200;
pub const ENDGAME_KNIGHT: i32 = 280;
pub const ENDGAME_BISHOP: i32 = 300;
pub const ENDGAME_ROOK: i32 = 600;
pub const ENDGAME_QUEEN: i32 = 920;
pub const ENDGAME_KING: i32 = 20_000;

// bonus for both black and white pawns in both opening (and middlegame) and endgame
pub const WHITE_PAWN_MIDDLEGAME_BONUS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0, 0,
    20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50, 50,
    50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const WHITE_PAWN_ENDGAME_BONUS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0,
    0, -10, -5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20,
    10, 10, 50, 50, 50, 50, 50, 50, 50, 50,
];

pub const BLACK_PAWN_MIDDLEGAME_BONUS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const BLACK_PAWN_ENDGAME_BONUS: [i32; 64] = [
    50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5, 10, 25, 25, 10, 5, 5, 0,
    0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20, -20, 10, 10, 5, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// bonus for both white and black knights
pub const WHITE_KNIGHT_BONUS: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, 20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

pub const BLACK_KNIGHT_BONUS: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 5, 5, 0, -20, -40, -30, 5, 10, 15, 15, 10,
    5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 10, 15, 15, 10,
    0, -30, -40, -20, 0, 0, 0, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

// bonus for both white and black bishops
pub const WHITE_BISHOP_BONUS: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

pub const BLACK_BISHOP_BONUS: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 5, 0, 0, 0, 0, 5, -10, -10, 10, 10, 10, 10, 10,
    10, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 0, 0, 0, 0, 0, 0, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

// bonus for both black and white rooks
pub const WHITE_ROOK_BONUS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

pub const BLACK_ROOK_BONUS: [i32; 64] = [
    0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0,
    0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5, 0, 0,
    0, 0, 0, 0, 0, 0,
];

// bonus for both black and white queens
pub const WHITE_QUEEN_BONUS: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

pub const BLACK_QUEEN_BONUS: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 5, 0, 0, -10, -10, 5, 5, 5, 5, 5, 0, -10,
    0, 0, 5, 5, 5, 5, 0, -5, -5, 0, 5, 5, 5, 5, 0, -5, -10, 0, 5, 5, 5, 5, 0, -10, -10, 0, 0, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

// bonus for both black and white kind in both opening ( middlegame ) and endgame
pub const WHITE_KING_MIDDLEGAME_BONUS: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

pub const WHITE_KING_ENDGAME_BONUS: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30,
    -30, -50,
];

pub const BLACK_KING_MIDDLEGAME_BONUS: [i32; 64] = [
    20, 30, 10, 0, 0, 10, 30, 20, 20, 20, 0, 0, 0, 0, 20, 20, -10, -20, -20, -20, -20, -20, -20,
    -10, -20, -30, -30, -40, -40, -30, -30, -20, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
    -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50,
    -40, -40, -30,
];

pub const BLACK_KING_ENDGAME_BONUS: [i32; 64] = [
    -50, -30, -30, -30, -30, -30, -30, -50, -30, -30, 0, 0, 0, 0, -30, -30, -30, -10, 20, 30, 30,
    20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10,
    20, 30, 30, 20, -10, -30, -30, -20, -10, 0, 0, -10, -20, -30, -50, -40, -30, -20, -20, -30,
    -40, -50,
];

pub fn evaluate(board: &BoardState) -> i32 {
    // for negamax, relative eval
    let color_const = match board.to_move {
        Color::White => 1,
        Color::Black => -1,
    };
    // if amount of pieces on the board except pawn are less that 4 + 2 kings
    if ((board.bb_fullboard ^ board.get_piece_bb(Color::White, Piece::Pawn))
        ^ board.get_piece_bb(Color::Black, Piece::Pawn))
    .count_ones()
        < 6
    {
        // engame
        // white pawns
        let white_pawn_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Pawn));

        let mut white_pawn_score = 0;

        for pos in white_pawn_position.iter() {
            white_pawn_score += ENDGAME_PAWN + WHITE_PAWN_ENDGAME_BONUS[pos.1 as usize];
        }

        // white knights
        let white_knight_position = parse_bitboards(
            Color::White,
            board.get_piece_bb(Color::White, Piece::Knight),
        );

        let mut white_knight_score = 0;

        for pos in white_knight_position.iter() {
            white_knight_score += ENDGAME_KNIGHT + WHITE_KNIGHT_BONUS[pos.1 as usize];
        }

        // white bishops
        let white_bishop_position = parse_bitboards(
            Color::White,
            board.get_piece_bb(Color::White, Piece::Bishop),
        );

        let mut white_bishop_score = 0;

        for pos in white_bishop_position.iter() {
            white_bishop_score += ENDGAME_BISHOP + WHITE_BISHOP_BONUS[pos.1 as usize];
        }

        // white rooks
        let white_rook_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Rook));

        let mut white_rook_score = 0;

        for pos in white_rook_position.iter() {
            white_rook_score += ENDGAME_ROOK + WHITE_ROOK_BONUS[pos.1 as usize];
        }

        // white queen
        let white_queen_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Queen));

        let mut white_queen_score = 0;

        for pos in white_queen_position.iter() {
            white_queen_score += ENDGAME_QUEEN + WHITE_QUEEN_BONUS[pos.1 as usize];
        }

        // white king
        let white_king_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Queen));

        let mut white_king_bonus = 0;

        for pos in white_king_position.iter() {
            white_king_bonus += WHITE_KING_ENDGAME_BONUS[pos.1 as usize];
        }

        let black_pawn_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Pawn));

        let mut black_pawn_score = 0;

        for pos in black_pawn_position.iter() {
            black_pawn_score += ENDGAME_PAWN + BLACK_PAWN_ENDGAME_BONUS[pos.1 as usize];
        }

        // black knights
        let black_knight_position = parse_bitboards(
            Color::Black,
            board.get_piece_bb(Color::Black, Piece::Knight),
        );

        let mut black_knight_score = 0;

        for pos in black_knight_position.iter() {
            black_knight_score += ENDGAME_KNIGHT + BLACK_KNIGHT_BONUS[pos.1 as usize];
        }

        // black bishops
        let black_bishop_position = parse_bitboards(
            Color::Black,
            board.get_piece_bb(Color::Black, Piece::Bishop),
        );

        let mut black_bishop_score = 0;

        for pos in black_bishop_position.iter() {
            black_bishop_score += ENDGAME_BISHOP + BLACK_BISHOP_BONUS[pos.1 as usize];
        }

        // black rooks
        let black_rook_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Rook));

        let mut black_rook_score = 0;

        for pos in black_rook_position.iter() {
            black_rook_score += ENDGAME_ROOK + BLACK_ROOK_BONUS[pos.1 as usize];
        }

        // black queen
        let black_queen_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Queen));

        let mut black_queen_score = 0;

        for pos in black_queen_position.iter() {
            black_queen_score += ENDGAME_QUEEN + BLACK_QUEEN_BONUS[pos.1 as usize];
        }

        // black king
        let black_king_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Queen));

        let mut black_king_bonus = 0;

        for pos in black_king_position.iter() {
            black_king_bonus += BLACK_KING_ENDGAME_BONUS[pos.1 as usize];
        }

        // other bonuses

        // bishop pair
        let white_bishop_pair_bonus =
            match board.get_piece_bb(Color::White, Piece::Bishop).count_ones() == 2 {
                true => 50,
                false => 0,
            };

        let black_bishop_pair_bonus =
            match board.get_piece_bb(Color::Black, Piece::Bishop).count_ones() == 2 {
                true => 50,
                false => 0,
            };
        ((white_pawn_score
            + white_knight_score
            + white_bishop_score
            + white_rook_score
            + white_queen_score
            + white_king_bonus
            + white_bishop_pair_bonus)
            - (black_pawn_score
                + black_knight_score
                + black_bishop_score
                + black_rook_score
                + black_queen_score
                + black_king_bonus
                + black_bishop_pair_bonus))
            * color_const
    } else {
        // middle game / opening
        // white pawns
        let white_pawn_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Pawn));

        let mut white_pawn_score = 0;

        for pos in white_pawn_position.iter() {
            white_pawn_score += PAWN + WHITE_PAWN_MIDDLEGAME_BONUS[pos.1 as usize];
        }

        // white knights
        let white_knight_position = parse_bitboards(
            Color::White,
            board.get_piece_bb(Color::White, Piece::Knight),
        );

        let mut white_knight_score = 0;

        for pos in white_knight_position.iter() {
            white_knight_score += KNIGHT + WHITE_KNIGHT_BONUS[pos.1 as usize];
        }

        // white bishops
        let white_bishop_position = parse_bitboards(
            Color::White,
            board.get_piece_bb(Color::White, Piece::Bishop),
        );

        let mut white_bishop_score = 0;

        for pos in white_bishop_position.iter() {
            white_bishop_score += BISHOP + WHITE_BISHOP_BONUS[pos.1 as usize];
        }

        // white rooks
        let white_rook_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Rook));

        let mut white_rook_score = 0;

        for pos in white_rook_position.iter() {
            white_rook_score += ROOK + WHITE_ROOK_BONUS[pos.1 as usize];
        }

        // white queen
        let white_queen_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Queen));

        let mut white_queen_score = 0;

        for pos in white_queen_position.iter() {
            white_queen_score += QUEEN + WHITE_QUEEN_BONUS[pos.1 as usize];
        }

        // white king
        let white_king_position =
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::Queen));

        let mut white_king_bonus = 0;

        for pos in white_king_position.iter() {
            white_king_bonus += WHITE_KING_MIDDLEGAME_BONUS[pos.1 as usize];
        }

        let black_pawn_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Pawn));

        let mut black_pawn_score = 0;

        for pos in black_pawn_position.iter() {
            black_pawn_score += PAWN + BLACK_PAWN_MIDDLEGAME_BONUS[pos.1 as usize];
        }

        // black knights
        let black_knight_position = parse_bitboards(
            Color::Black,
            board.get_piece_bb(Color::Black, Piece::Knight),
        );

        let mut black_knight_score = 0;

        for pos in black_knight_position.iter() {
            black_knight_score += KNIGHT + BLACK_KNIGHT_BONUS[pos.1 as usize];
        }

        // black bishops
        let black_bishop_position = parse_bitboards(
            Color::Black,
            board.get_piece_bb(Color::Black, Piece::Bishop),
        );

        let mut black_bishop_score = 0;

        for pos in black_bishop_position.iter() {
            black_bishop_score += BISHOP + BLACK_BISHOP_BONUS[pos.1 as usize];
        }

        // black rooks
        let black_rook_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Rook));

        let mut black_rook_score = 0;

        for pos in black_rook_position.iter() {
            black_rook_score += ROOK + BLACK_ROOK_BONUS[pos.1 as usize];
        }

        // black queen
        let black_queen_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Queen));

        let mut black_queen_score = 0;

        for pos in black_queen_position.iter() {
            black_queen_score += QUEEN + BLACK_QUEEN_BONUS[pos.1 as usize];
        }

        // black king
        let black_king_position =
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::Queen));

        let mut black_king_bonus = 0;

        for pos in black_king_position.iter() {
            black_king_bonus += BLACK_KING_MIDDLEGAME_BONUS[pos.1 as usize];
        }

        // other bonuses

        // bishop pair
        let white_bishop_pair_bonus =
            match board.get_piece_bb(Color::White, Piece::Bishop).count_ones() == 2 {
                true => 50,
                false => 0,
            };

        let black_bishop_pair_bonus =
            match board.get_piece_bb(Color::Black, Piece::Bishop).count_ones() == 2 {
                true => 50,
                false => 0,
            };

        // king safety (if covered with pawns) only in middlegame and opening
        let white_king_moves = generate_king_moves(
            parse_bitboards(Color::White, board.get_piece_bb(Color::White, Piece::King)),
            board.bb_fullboard,
        );

        let white_king_safety_bonus = match white_king_moves.len() {
            0 => 100,
            _ => {
                if (board.get_color_bb(Color::White) & white_king_moves[0].1) <= 2 {
                    100
                } else {
                    -20
                }
            }
        };

        let black_king_moves = generate_king_moves(
            parse_bitboards(Color::Black, board.get_piece_bb(Color::Black, Piece::King)),
            board.bb_fullboard,
        );

        let black_king_safety_bonus = match black_king_moves.len() {
            0 => 100,
            _ => {
                if (board.get_color_bb(Color::Black) & black_king_moves[0].1).count_ones() <= 2 {
                    100
                } else {
                    -20
                }
            }
        };

        // println!("white_pawn_score: {}", white_pawn_score);
        // println!("knight: {}", white_knight_score);
        // println!("bishop: {}", white_bishop_score);
        // println!("rook: {}", white_rook_score);
        // println!("queen: {}", white_queen_score);
        // println!("white k {}", white_king_bonus);
        // println!("bish pair {}", white_bishop_pair_bonus);
        // println!("king saf {}\n", white_king_safety_bonus);
        //
        // println!("black_pawn_score: {}", black_pawn_score);
        // println!("knight: {}", black_knight_score);
        // println!("bishop: {}", black_bishop_score);
        // println!("rook: {}", black_rook_score);
        // println!("queen: {}", black_queen_score);
        // println!("bl king{}", black_king_bonus);
        // println!("bih pair {}", black_bishop_pair_bonus);
        // println!("bl saf {}\n\n\n\n", black_king_safety_bonus);

        ((white_pawn_score
            + white_knight_score
            + white_bishop_score
            + white_rook_score
            + white_queen_score
            + white_king_bonus
            + white_bishop_pair_bonus
            + white_king_safety_bonus)
            - (black_pawn_score
                + black_knight_score
                + black_bishop_score
                + black_rook_score
                + black_queen_score
                + black_king_bonus
                + black_bishop_pair_bonus
                + black_king_safety_bonus))
            * color_const
    }
}
