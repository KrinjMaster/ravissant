use crate::constants::BOARD_SQUARES;

// board when encoded looks something like this when printed
// 1 . . . . . . . .
// 2 . . . . . . . .
// 3 . . . . . . . .
// 4 . . . . . . . .
// 5 . . . . . . . .
// 6 . . . . . . . .
// 7 . . . . . . . .
// 8 . . . . . . . .
//   h g f e d c b a

pub fn print_bitboard(bb: Bitboard) {
    let formatted_bb: String = format!("{:064b}", bb);
    let form_bb: Vec<&str> = formatted_bb
        .split("")
        .filter(|&str| !str.is_empty())
        .collect();

    for rank in 0..8 {
        let rank: String = match rank {
            0 => "h  ".to_string(),
            1 => "g  ".to_string(),
            2 => "f  ".to_string(),
            3 => "e  ".to_string(),
            4 => "d  ".to_string(),
            5 => "c  ".to_string(),
            6 => "b  ".to_string(),
            7 => "a  ".to_string(),
            _ => continue,
        } + &form_bb[(7 - rank) * 8..(7 - rank) * 8 + 8]
            .join(" ")
            .chars()
            .rev()
            .collect::<String>();

        println!("{}", rank);
    }
    println!("\n   1 2 3 4 5 6 7 8\n");
    println!("biboard is: {}", bb);
}

pub type Bitboard = u64;

#[derive(Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Debug)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    pub bb_pieces: [[Bitboard; 6]; 2],
    pub bb_colors: [Bitboard; 2],
    pub bb_fullboard: Bitboard,
    pub bb_to_move: Bitboard,
    pub bb_castling_rigths: [[Bitboard; 2]; 2],
    pub bb_en_passant: Bitboard,
    pub halfmove: u32,
    pub fullmove: u32,
}

impl BoardState {
    pub fn get_piece_bb(&self, color: Color, piece: Piece) -> Bitboard {
        self.bb_pieces[color as usize][piece as usize]
    }

    pub fn get_color_bb(&self, color: Color) -> Bitboard {
        self.bb_colors[color as usize]
    }

    pub fn from_fen(fen_string: &str) -> Result<BoardState, &str> {
        let fen: Vec<&str> = fen_string.split_whitespace().collect();

        if fen.len() != 6 {
            return Err("Incorrect fen string!");
        }

        // pieces position parsing
        let mut bb_white_pawns: Bitboard = 0;
        let mut bb_white_knights: Bitboard = 0;
        let mut bb_white_bishops: Bitboard = 0;
        let mut bb_white_rooks: Bitboard = 0;
        let mut bb_white_queens: Bitboard = 0;
        let mut bb_white_king: Bitboard = 0;
        let mut bb_black_pawns: Bitboard = 0;
        let mut bb_black_knights: Bitboard = 0;
        let mut bb_black_bishops: Bitboard = 0;
        let mut bb_black_rooks: Bitboard = 0;
        let mut bb_black_queens: Bitboard = 0;
        let mut bb_black_king: Bitboard = 0;

        let fen_pieces: Vec<&str> = fen[0].split("/").collect();

        if fen_pieces.len() != 8 {
            return Err("Incorrect pieces placement in the fen string!");
        }

        // not permanent solution (maybe)
        let mut white_king_count = 0;
        let mut black_king_count = 0;
        //

        for row in 0..fen_pieces.len() {
            let mut col = 0;

            for char in fen_pieces[row].chars() {
                if char.is_numeric() {
                    col += char.to_digit(10).unwrap();
                } else {
                    match char {
                        'P' => bb_white_pawns |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'N' => bb_white_knights |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'B' => bb_white_bishops |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'R' => bb_white_rooks |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'Q' => bb_white_queens |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'K' => {
                            if white_king_count > 1 {
                                return Err(
                                    "Incorrect fen string! More than 1 white king on the board!",
                                );
                            }

                            bb_white_king |= BOARD_SQUARES[8 * row as usize + col as usize];
                            white_king_count += 1;
                        }
                        'p' => bb_black_pawns |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'n' => bb_black_knights |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'b' => bb_black_bishops |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'r' => bb_black_rooks |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'q' => bb_black_queens |= BOARD_SQUARES[8 * row as usize + col as usize],
                        'k' => {
                            if black_king_count > 1 {
                                return Err(
                                    "Incorrect fen string! More than 1 black king on the board!",
                                );
                            }

                            bb_black_king |= BOARD_SQUARES[8 * row as usize + col as usize];

                            black_king_count += 1;
                        }
                        _ => return Err("Incorrect piece in fen string!"),
                    }
                    col += 1;
                }
            }
        }

        // parsing move to move
        let to_move_fen: &str = fen[1];
        let mut bb_to_move: Bitboard = 0;

        match to_move_fen {
            "w" => bb_to_move = 0,
            "b" => bb_to_move = 1,
            _ => return Err("Incorrect to move color in fen string!"),
        }

        // castling rights parsing
        let castling_rigths_fen: &str = fen[2];

        let mut bb_castling_white_kingside: Bitboard = 0;
        let mut bb_castling_white_queenside: Bitboard = 0;
        let mut bb_castling_black_kingside: Bitboard = 0;
        let mut bb_castling_black_queenside: Bitboard = 0;

        for char in castling_rigths_fen.chars() {
            match char {
                'K' => bb_castling_white_kingside = BOARD_SQUARES[0],
                'Q' => bb_castling_white_queenside = BOARD_SQUARES[7],
                'k' => bb_castling_black_kingside = BOARD_SQUARES[56],
                'q' => bb_castling_black_queenside = BOARD_SQUARES[63],
                _ => {
                    if char != '-' {
                        println!("{}", castling_rigths_fen);
                        return Err("Incorrect castlings rights in fen string!");
                    }
                }
            }
        }

        // en passant square parsing
        let mut en_passant: Vec<usize> = vec![];
        let mut bb_en_passant: Bitboard = 0;

        if fen[3].len() > 2 {
            return Err("Incorrect length of en passant square in fen string!");
        }

        for char in fen[3].chars() {
            match char {
                'a' => en_passant.push(0),
                'b' => en_passant.push(1),
                'c' => en_passant.push(2),
                'd' => en_passant.push(3),
                'e' => en_passant.push(4),
                'f' => en_passant.push(5),
                'g' => en_passant.push(6),
                'h' => en_passant.push(7),
                _ => {
                    if char.is_numeric() && en_passant.len() == 1 {
                        en_passant.push(char.to_digit(10).unwrap() as usize - 1);
                    } else if char != '-' {
                        return Err("Incorrect en passant square position in fen string!");
                    }
                }
            }
        }

        if en_passant.len() == 2 {
            bb_en_passant |= BOARD_SQUARES[63 - (en_passant[1] * 8 + (7 - en_passant[0]))];
        }

        // halfmove parsing
        let mut halfmove = 0;

        match fen[4].parse::<u32>() {
            Ok(int) => halfmove = int,
            Err(_) => return Err("Incorrect halfmove count in a fen string!"),
        };

        if halfmove > 1 {
            return Err("Incorrect halfmove count in a fen string!");
        }

        // fullmove parsing
        let mut fullmove = 0;

        match fen[5].parse::<u32>() {
            Ok(int) => fullmove = int,
            Err(_) => return Err("Incorrect halfmove count in a fen string!"),
        };

        Ok(BoardState {
            bb_pieces: [
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
            bb_colors: [
                bb_white_pawns
                    | bb_white_knights
                    | bb_white_bishops
                    | bb_white_rooks
                    | bb_white_queens
                    | bb_white_king,
                bb_black_pawns
                    | bb_black_knights
                    | bb_black_bishops
                    | bb_black_rooks
                    | bb_black_queens
                    | bb_black_king,
            ],
            bb_fullboard: bb_white_pawns
                | bb_white_knights
                | bb_white_bishops
                | bb_white_rooks
                | bb_white_queens
                | bb_white_king
                | bb_black_pawns
                | bb_black_knights
                | bb_black_bishops
                | bb_black_rooks
                | bb_black_queens
                | bb_black_king,
            bb_to_move,
            bb_castling_rigths: [
                [bb_castling_white_kingside, bb_castling_white_queenside],
                [bb_castling_black_kingside, bb_castling_black_queenside],
            ],
            bb_en_passant,
            halfmove,
            fullmove,
        })
    }
}
