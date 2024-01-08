// i only did pieces inserting everything still left when parsing fen string
use crate::constants::BOARD_SQUARES;

pub type BitBoard = u64;

#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    pub bitboards: [[BitBoard; 6]; 2],
}

impl BoardState {
    pub fn from_fen(fen_string: &str) -> Result<BoardState, &str> {
        let fen: Vec<&str> = fen_string.split_whitespace().collect();

        if fen.len() != 6 {
            return Err("Incorrect fen string number of data!");
        }

        // piece position
        // bitboards for future piece inserting

        // white pieces
        let mut bb_white_pawns: BitBoard = 0;
        let mut bb_white_knights: BitBoard = 0;
        let mut bb_white_bishops: BitBoard = 0;
        let mut bb_white_rooks: BitBoard = 0;
        let mut bb_white_queens: BitBoard = 0;
        let mut bb_white_king: BitBoard = 0;
        // black pieces
        let mut bb_black_pawns: BitBoard = 0;
        let mut bb_black_knights: BitBoard = 0;
        let mut bb_black_bishops: BitBoard = 0;
        let mut bb_black_rooks: BitBoard = 0;
        let mut bb_black_queens: BitBoard = 0;
        let mut bb_black_king: BitBoard = 0;

        let fen_pieces: Vec<&str> = fen[0].split("/").collect();

        if fen_pieces.len() != 8 {
            return Err("Incorrect pieces placement in the fen string!");
        }

        // not permanent solution (maybe)
        let mut white_king_count = 0;
        let mut black_king_count = 0;

        for row in 0..fen_pieces.len() {
            let mut col = 0;

            for char in fen_pieces[row].chars() {
                if char.is_numeric() {
                    col += char.to_digit(10).unwrap();
                } else {
                    match char {
                        'P' => {
                            bb_white_pawns =
                                bb_white_pawns | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'N' => {
                            bb_white_knights =
                                bb_white_knights | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'B' => {
                            bb_white_bishops =
                                bb_white_bishops | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'R' => {
                            bb_white_rooks =
                                bb_white_rooks | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'Q' => {
                            bb_white_queens =
                                bb_white_queens | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'K' => {
                            if white_king_count > 1 {
                                return Err(
                                    "Incorrect fen string! More than 1 white king on the board!",
                                );
                            }

                            bb_white_king =
                                bb_white_king | BOARD_SQUARES[8 * row as usize + col as usize];
                            white_king_count += 1;
                        }
                        'p' => {
                            bb_black_pawns =
                                bb_black_pawns | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'n' => {
                            bb_black_knights =
                                bb_black_knights | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'b' => {
                            bb_black_bishops =
                                bb_black_bishops | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'r' => {
                            bb_black_rooks =
                                bb_black_rooks | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'q' => {
                            bb_black_queens =
                                bb_black_queens | BOARD_SQUARES[8 * row as usize + col as usize]
                        }
                        'k' => {
                            if black_king_count > 1 {
                                return Err(
                                    "Incorrect fen string! More than 1 black king on the board!",
                                );
                            }

                            bb_black_king =
                                bb_black_king | BOARD_SQUARES[8 * row as usize + col as usize];

                            black_king_count += 1;
                        }
                        _ => return Err("Incorrect piece in fen string!"),
                    }
                    col += 1;
                }
            }
        }

        Ok(BoardState {
            bitboards: [
                [
                    bb_white_pawns,
                    bb_white_knights,
                    bb_white_bishops,
                    bb_white_rooks,
                    bb_white_queens,
                    bb_white_king,
                ],
                [
                    bb_black_pawns,
                    bb_black_knights,
                    bb_black_bishops,
                    bb_black_rooks,
                    bb_black_queens,
                    bb_black_king,
                ],
            ],
        })
    }
}
