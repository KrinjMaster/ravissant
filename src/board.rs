use crate::{
    constants::{BOARD_SQUARES, FIFTH_RANK, FOURTH_RANK},
    move_generation::{
        generate_bishop_moves, generate_king_moves, generate_knight_moves, generate_pawn_moves,
        generate_queen_moves, generate_rook_moves,
    },
    piece_parsing::parse_bitboards,
};

pub type Move = u16;
pub type Bitboard = u64;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
    None = 6,
}

#[derive(Debug, Clone)]
pub struct BoardState {
    pub bb_pieces: [[Bitboard; 6]; 2],
    pub bb_colors: [Bitboard; 2],
    pub bb_fullboard: Bitboard,
    pub to_move: Color,
    pub bb_castling_rigths: [[Bitboard; 2]; 2],
    pub bb_en_passant: Bitboard,
    pub halfmove: u32,
    pub fullmove: u32,
    pub move_history: Vec<Move>,
}

impl BoardState {
    pub fn encode_move(&self, from_bb: u8, to_bb: u8, capture: Piece) -> u16 {
        from_bb as u16 | ((to_bb as u16) << 6) | ((capture as u16) << 12)
    }
    pub fn decode_move(&self, piece_move: Move) -> (Bitboard, Bitboard, Piece, Color, Piece, bool) {
        let start_bb: Bitboard =
            BOARD_SQUARES[((piece_move & !(1 << 7 | 1 << 8 | 1 << 6)) as u8) as usize];
        let end_bb: Bitboard =
            BOARD_SQUARES[((piece_move >> 6 & !(1 << 7 | 1 << 8)) as u8) as usize];
        let captured_piece_index: u8 = ((piece_move) >> 12 & (1 | 1 << 1 | 1 << 2)) as u8;
        let captured_piece = match captured_piece_index {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => Piece::None,
        };
        let is_promotion: bool = (piece_move >> 15 & 1) as u8 == 1;
        let mut piece: Piece = Piece::None;
        let mut color: Color = Color::White;

        for color_index in 0..1 {
            for piece_index in 0..5 {
                if self.bb_pieces[color_index as usize][piece_index as usize] & start_bb != 0 {
                    piece = match piece_index {
                        0 => Piece::Pawn,
                        1 => Piece::Knight,
                        2 => Piece::Bishop,
                        3 => Piece::Rook,
                        4 => Piece::Queen,
                        5 => Piece::King,
                        _ => Piece::None,
                    };

                    if color_index == 0 {
                        color = Color::White;
                    } else {
                        color = Color::Black;
                    }
                }
            }
        }

        (start_bb, end_bb, piece, color, captured_piece, is_promotion)
    }
    pub fn is_in_check(self, color: Color) -> bool {
        let opposite_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        let all_attacks = self
            .generate_moves_by_color(&opposite_color)
            .iter()
            .fold(0, |acc, cur| {
                acc | BOARD_SQUARES[((cur >> 6 & !(1 << 8 | 1 << 7)) as u8) as usize]
            });

        all_attacks & self.get_piece_bb(color, Piece::King) != 0
    }

    pub fn make_move(&mut self, encoded_move: Move) {
        let (start_pos, end_pos, piece, color, _, _) = self.decode_move(encoded_move);

        // delete piece on the move square if there is one
        // for index in 0..6 {
        //     // opposite color
        //     match color {
        //         Color::White => {
        //             if self.bb_pieces[Color::Black as usize][index] & end_pos != 0 {
        //                 self.bb_pieces[Color::Black as usize][index] &= !end_pos;
        //                 match index {
        //                     0 => captured_piece = Piece::Pawn,
        //                     1 => captured_piece = Piece::Knight,
        //                     2 => captured_piece = Piece::Bishop,
        //                     3 => captured_piece = Piece::Rook,
        //                     4 => captured_piece = Piece::Queen,
        //                     5 => captured_piece = Piece::King,
        //                     _ => (),
        //                 };
        //             }
        //         }
        //         Color::Black => {
        //             if self.bb_pieces[Color::White as usize][index] & end_pos != 0 {
        //                 self.bb_pieces[Color::White as usize][index] &= !end_pos;
        //                 match index {
        //                     0 => captured_piece = Piece::Pawn,
        //                     1 => captured_piece = Piece::Knight,
        //                     2 => captured_piece = Piece::Bishop,
        //                     3 => captured_piece = Piece::Rook,
        //                     4 => captured_piece = Piece::Queen,
        //                     5 => captured_piece = Piece::King,
        //                     _ => (),
        //                 };
        //             }
        //         }
        //     }
        // }

        // delete piece from color bitboards
        match color {
            Color::White => {
                self.bb_colors[Color::White as usize] &= !start_pos;
                self.bb_colors[Color::White as usize] |= end_pos;

                self.bb_colors[Color::Black as usize] &= !start_pos;
                self.bb_colors[Color::Black as usize] &= !end_pos;
            }
            Color::Black => {
                self.bb_colors[Color::Black as usize] &= !start_pos;
                self.bb_colors[Color::Black as usize] |= end_pos;

                self.bb_colors[Color::White as usize] &= !start_pos;
                self.bb_colors[Color::White as usize] &= !end_pos;
            }
        }

        // new fullboard
        self.bb_fullboard =
            self.bb_colors[Color::White as usize] | self.bb_colors[Color::Black as usize];

        // check for castling and en passant
        match piece {
            Piece::King => {
                self.bb_castling_rigths[color as usize][1] = 0;
                self.bb_castling_rigths[color as usize][0] = 0;
            }
            Piece::Rook => {
                self.bb_castling_rigths[color as usize][1] &= !start_pos;
                self.bb_castling_rigths[color as usize][0] &= !start_pos;
            }
            Piece::Pawn => {
                if start_pos >> 16 == end_pos && matches!(color, Color::White) {
                    self.bb_en_passant |= start_pos >> 8;
                }
                if start_pos << 16 == end_pos && matches!(color, Color::Black) {
                    self.bb_en_passant |= start_pos << 8;
                }
            }
            _ => (),
        }

        // make a move
        self.bb_pieces[color as usize][piece as usize] |= end_pos;
        self.bb_colors[color as usize] |= end_pos;
        self.bb_fullboard |= end_pos;

        self.move_history.push(encoded_move);

        // if black to move
        if self.halfmove == 1 {
            self.to_move = Color::White;
            self.halfmove = 0;
            self.fullmove += 1;
        } else {
            self.to_move = Color::Black;
            self.halfmove = 1;
        }
    }

    pub fn undo_move(&mut self) -> Result<(), &str> {
        let last_move = self.move_history.pop().expect("No more moves found!");

        let (start_pos, end_pos, piece, color, captured_piece, is_promotion) =
            self.decode_move(last_move);

        let opposite_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        // check for castling and castling avaliability
        match piece {
            Piece::Pawn => {
                match color {
                    Color::White => {
                        // if piece was previosly of fourth rank
                        if start_pos & FOURTH_RANK != 0 {
                            self.bb_en_passant |= start_pos << 8;
                        }
                    }
                    Color::Black => {
                        // if piece was previosly of fifth rank
                        if start_pos & FIFTH_RANK != 0 {
                            self.bb_en_passant |= start_pos >> 8;
                        }
                    }
                }
            }
            Piece::Rook => {
                for index in 0..2 {
                    if (self.bb_castling_rigths[color as usize][index] | start_pos).count_ones()
                        == 1
                    {
                        self.bb_castling_rigths[color as usize][index] = start_pos;
                    }
                }
            }
            Piece::King => {
                if matches!(color, Color::White) {
                    // white kingside
                    self.bb_castling_rigths[color as usize][0] =
                        self.bb_pieces[color as usize][Piece::Rook as usize] & BOARD_SQUARES[63];
                    // white queenside
                    self.bb_castling_rigths[color as usize][1] =
                        self.bb_pieces[color as usize][Piece::Rook as usize] & BOARD_SQUARES[56];
                } else {
                    // black kingside
                    self.bb_castling_rigths[color as usize][0] =
                        self.bb_pieces[color as usize][Piece::Rook as usize] & BOARD_SQUARES[0];
                    // black queenside
                    self.bb_castling_rigths[color as usize][1] =
                        self.bb_pieces[color as usize][Piece::Rook as usize] & BOARD_SQUARES[7];
                }
            }
            _ => (),
        }

        // undo move
        self.bb_pieces[color as usize][piece as usize] |= start_pos;
        self.bb_pieces[color as usize][piece as usize] ^= end_pos;

        // undo move in colors bb
        self.bb_colors[color as usize] |= start_pos;
        self.bb_colors[color as usize] ^= end_pos;

        // if move captured piece
        if !matches!(captured_piece, Piece::None) {
            // if captured piece is not empty
            self.bb_pieces[opposite_color as usize][captured_piece as usize] |= end_pos;
            self.bb_colors[opposite_color as usize] |= end_pos;
            self.bb_fullboard |= end_pos;
        }

        // undo move in fullboard
        self.bb_fullboard =
            self.bb_colors[color as usize] | self.bb_colors[opposite_color as usize];

        // halfmove undo
        if self.halfmove == 0 {
            // undo fullmove count only if white currently to move
            self.halfmove = 1;
            self.fullmove -= 1;
        } else {
            self.halfmove = 0;
        }

        self.to_move = opposite_color;

        Ok(())
    }

    pub fn generate_moves_by_color(&self, color: &Color) -> Vec<Move> {
        let opposite_color: &Color = match *color {
            Color::White => &Color::Black,
            Color::Black => &Color::White,
        };

        let mut moves_vec: Vec<Move> = vec![];

        // generate pseudo legal moves
        let pawns_moves = generate_pawn_moves(
            parse_bitboards(*color, self.get_piece_bb(*color, Piece::Pawn)),
            self.get_color_bb(*color),
            self.get_color_bb(*opposite_color),
            self.bb_en_passant,
        );

        for piece_move in pawns_moves.iter() {
            let mut move_bb = piece_move.1;

            while move_bb != 0 {
                let least_sign_bit = move_bb.trailing_zeros();

                moves_vec.push(self.encode_move(
                    piece_move.0.trailing_zeros() as u8,
                    least_sign_bit as u8,
                    Piece::None,
                ));

                move_bb ^= BOARD_SQUARES[least_sign_bit as usize];
            }
        }

        let kings_moves = generate_king_moves(
            parse_bitboards(*color, self.get_piece_bb(*color, Piece::King)),
            self.get_color_bb(*color),
        );

        for king in kings_moves.iter() {
            let mut move_bb = king.1;

            while move_bb != 0 {
                let least_sign_bit = move_bb.trailing_zeros();

                moves_vec.push(self.encode_move(
                    king.0.trailing_zeros() as u8,
                    least_sign_bit as u8,
                    Piece::None,
                ));

                move_bb ^= BOARD_SQUARES[least_sign_bit as usize];
            }
        }

        let knights_moves = generate_knight_moves(
            parse_bitboards(*color, self.get_piece_bb(*color, Piece::Knight)),
            self.get_color_bb(*color),
        );

        for piece_move in knights_moves.iter() {
            let mut move_bb = piece_move.1;

            while move_bb != 0 {
                let least_sign_bit = move_bb.trailing_zeros();

                moves_vec.push(self.encode_move(
                    piece_move.0.trailing_zeros() as u8,
                    least_sign_bit as u8,
                    Piece::None,
                ));

                move_bb ^= BOARD_SQUARES[least_sign_bit as usize];
            }
        }

        let bishops_moves = generate_bishop_moves(
            parse_bitboards(*color, self.get_piece_bb(*color, Piece::Bishop)),
            self.get_color_bb(*color),
            self.bb_fullboard,
        );

        for piece_move in bishops_moves.iter() {
            let mut move_bb = piece_move.1;

            while move_bb != 0 {
                let least_sign_bit = move_bb.trailing_zeros();

                moves_vec.push(self.encode_move(
                    piece_move.0.trailing_zeros() as u8,
                    least_sign_bit as u8,
                    Piece::None,
                ));

                move_bb ^= BOARD_SQUARES[least_sign_bit as usize];
            }
        }

        let rooks_moves = generate_rook_moves(
            parse_bitboards(*color, self.get_piece_bb(*color, Piece::Rook)),
            self.bb_colors[*color as usize],
            self.bb_fullboard,
        );

        for piece_move in rooks_moves.iter() {
            let mut move_bb = piece_move.1;

            while move_bb != 0 {
                let least_sign_bit = move_bb.trailing_zeros();

                moves_vec.push(self.encode_move(
                    piece_move.0.trailing_zeros() as u8,
                    least_sign_bit as u8,
                    Piece::None,
                ));

                move_bb ^= BOARD_SQUARES[least_sign_bit as usize];
            }
        }

        let queens_moves = generate_queen_moves(
            parse_bitboards(*color, self.get_piece_bb(*color, Piece::Queen)),
            self.bb_fullboard,
            self.bb_colors[*color as usize],
        );

        for piece_move in queens_moves.iter() {
            let mut move_bb = piece_move.1;

            while move_bb != 0 {
                let least_sign_bit = move_bb.trailing_zeros();

                moves_vec.push(self.encode_move(
                    piece_move.0.trailing_zeros() as u8,
                    least_sign_bit as u8,
                    Piece::None,
                ));

                move_bb ^= BOARD_SQUARES[least_sign_bit as usize];
            }
        }

        moves_vec
    }

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

        let mut to_move = Color::White;

        match to_move_fen {
            "w" => to_move = Color::White,
            "b" => to_move = Color::Black,
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
                'K' => bb_castling_white_kingside = BOARD_SQUARES[63],
                'Q' => bb_castling_white_queenside = BOARD_SQUARES[56],
                'k' => bb_castling_black_kingside = BOARD_SQUARES[0],
                'q' => bb_castling_black_queenside = BOARD_SQUARES[7],
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
            to_move,
            bb_castling_rigths: [
                [bb_castling_white_kingside, bb_castling_white_queenside],
                [bb_castling_black_kingside, bb_castling_black_queenside],
            ],
            bb_en_passant,
            halfmove,
            fullmove,
            move_history: vec![],
        })
    }
}
