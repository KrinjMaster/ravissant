use crate::{
    constants::{BOARD_SQUARES, FIFTH_RANK, FOURTH_RANK},
    move_generation::{
        generate_bishop_moves, generate_king_moves, generate_knight_moves, generate_pawn_moves,
        generate_queen_moves, generate_rook_moves,
    },
    piece_parsing::parse_bitboards,
    utils::{number_to_color, number_to_piece, opposite_color},
};
use std::process::exit;

pub type EncodedMove = u32;
pub type Bitboard = u64;
pub type DecodedMove = (Bitboard, Bitboard, Piece, Color, Piece, bool);

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
    pub move_history: Vec<EncodedMove>,
}

impl BoardState {
    pub fn get_captured_piece(&self, to_bb: Bitboard) -> Piece {
        for index in 0..6 {
            if self.bb_pieces[opposite_color(self.to_move) as usize][index] & to_bb != 0 {
                return number_to_piece(index as u32).unwrap_or_else(|err| {
                    println!("{}", err);
                    exit(1);
                });
            }
        }

        Piece::None
    }

    pub fn encode_move(
        &self,
        from_pos: u8,
        to_pos: u8,
        piece: Piece,
        color: Color,
        captured_piece: Piece,
        is_promotion: bool,
    ) -> Result<EncodedMove, &str> {
        if from_pos & !63 != 0 || to_pos & !63 != 0 {
            return Err("Incorrect move positions!");
        }

        // those ands are just another layer of error hanlding
        Ok((from_pos as u32) & 63
            | ((to_pos as u32) & 63) << 6
            | ((piece as u32) & 7) << 12
            | (color as u32) << 15
            | (captured_piece as u32) << 18
            | (is_promotion as u32) << 19)
    }

    pub fn decode_move(&self, piece_move: EncodedMove) -> Result<DecodedMove, &str> {
        let start_bb: Bitboard = BOARD_SQUARES[(piece_move & 63) as usize];
        let end_bb: Bitboard = BOARD_SQUARES[((piece_move >> 6) & 63) as usize];
        let piece: Piece = number_to_piece((piece_move >> 12) & 7).unwrap_or_else(|err| {
            println!("{}", err);
            exit(1);
        });
        let color: Color = number_to_color((piece_move >> 15) & 1).unwrap_or_else(|err| {
            println!("{}", err);
            exit(1);
        });
        let captured_piece: Piece = number_to_piece((piece_move >> 16) & 7).unwrap_or_else(|err| {
            println!("{}", err);
            exit(1);
        });
        let is_promotion = (piece_move >> 19) & 1 == 1;

        Ok((start_bb, end_bb, piece, color, captured_piece, is_promotion))
    }

    pub fn is_in_check(&self, color: &Color) -> bool {
        let all_attacks = self
            .generate_moves_by_color(&opposite_color(*color))
            .iter()
            .fold(0, |all_moves, cur_move| {
                all_moves | BOARD_SQUARES[((cur_move >> 6) & 63) as usize]
            });

        all_attacks & self.get_piece_bb(*color, Piece::King) != 0
    }

    pub fn make_move(&mut self, piece_move: EncodedMove) {
        let (start_bb, end_bb, piece, color, captured_piece, _) =
            self.decode_move(piece_move).unwrap_or_else(|err| {
                println!("{}", err);
                exit(1);
            });

        // delete piece on the move square if there is one
        if !matches!(captured_piece, Piece::None) {
            match color {
                Color::White => {
                    self.bb_pieces[1][captured_piece as usize] &= !end_bb;
                }
                Color::Black => {
                    self.bb_pieces[0][captured_piece as usize] &= !end_bb;
                }
            }
        }

        // delete piece from color bitboards
        match color {
            Color::White => {
                self.bb_colors[Color::White as usize] &= !start_bb;
                self.bb_colors[Color::White as usize] |= end_bb;

                self.bb_colors[Color::Black as usize] &= !start_bb;
                self.bb_colors[Color::Black as usize] &= !end_bb;
            }
            Color::Black => {
                self.bb_colors[Color::Black as usize] &= !start_bb;
                self.bb_colors[Color::Black as usize] |= end_bb;

                self.bb_colors[Color::White as usize] &= !start_bb;
                self.bb_colors[Color::White as usize] &= !end_bb;
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
                self.bb_castling_rigths[color as usize][1] &= !start_bb;
                self.bb_castling_rigths[color as usize][0] &= !start_bb;
            }
            Piece::Pawn => {
                if start_bb >> 16 == end_bb && matches!(color, Color::White) {
                    self.bb_en_passant |= start_bb >> 8;
                }
                if start_bb << 16 == end_bb && matches!(color, Color::Black) {
                    self.bb_en_passant |= start_bb << 8;
                }
            }
            _ => (),
        }

        // make a move
        self.bb_pieces[color as usize][piece as usize] |= end_bb;
        self.bb_pieces[color as usize][piece as usize] ^= start_bb;
        self.bb_colors[color as usize] |= end_bb;
        self.bb_fullboard |= end_bb;

        self.move_history.push(piece_move);

        // println!("{}", format!("{:016b}", encoded_move));
        // println!("{:?}", piece);
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

        let (start_bb, end_bb, piece, color, captured_piece, is_promotion) =
            self.decode_move(last_move).unwrap_or_else(|err| {
                println!("{}", err);
                exit(1);
            });

        // check for castling and castling avaliability
        match piece {
            Piece::Pawn => {
                match color {
                    Color::White => {
                        // if piece was previosly of fourth rank
                        if start_bb & FOURTH_RANK != 0 {
                            self.bb_en_passant |= start_bb << 8;
                        }
                    }
                    Color::Black => {
                        // if piece was previosly of fifth rank
                        if start_bb & FIFTH_RANK != 0 {
                            self.bb_en_passant |= start_bb >> 8;
                        }
                    }
                }
            }
            Piece::Rook => {
                for index in 0..2 {
                    if (self.bb_castling_rigths[color as usize][index] | start_bb).count_ones() == 1
                    {
                        self.bb_castling_rigths[color as usize][index] = start_bb;
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
        self.bb_pieces[color as usize][piece as usize] |= start_bb;
        self.bb_pieces[color as usize][piece as usize] ^= end_bb;

        // undo move in colors bb
        self.bb_colors[color as usize] |= start_bb;
        self.bb_colors[color as usize] ^= end_bb;

        // if move captured piece
        if !matches!(captured_piece, Piece::None) {
            // if captured piece is not empty
            self.bb_pieces[opposite_color(color) as usize][captured_piece as usize] |= end_bb;
            self.bb_colors[opposite_color(color) as usize] |= end_bb;
            self.bb_fullboard |= end_bb;
        }

        // undo move in fullboard
        self.bb_fullboard =
            self.bb_colors[color as usize] | self.bb_colors[opposite_color(color) as usize];

        // halfmove undo
        if self.halfmove == 0 {
            // undo fullmove count only if white currently to move
            self.halfmove = 1;
            self.fullmove -= 1;
        } else {
            self.halfmove = 0;
        }

        self.to_move = opposite_color(self.to_move);

        Ok(())
    }

    pub fn generate_moves_by_color(&self, color: &Color) -> Vec<EncodedMove> {
        let opposite_color: &Color = match *color {
            Color::White => &Color::Black,
            Color::Black => &Color::White,
        };

        let mut moves_vec: Vec<EncodedMove> = vec![];

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

                moves_vec.push(
                    self.encode_move(
                        piece_move.0.trailing_zeros() as u8,
                        least_sign_bit as u8,
                        Piece::Pawn,
                        *color,
                        self.get_captured_piece(BOARD_SQUARES[least_sign_bit as usize]),
                        false,
                        // later implement promotion
                    )
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        exit(1);
                    }),
                );

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

                moves_vec.push(
                    self.encode_move(
                        king.0.trailing_zeros() as u8,
                        least_sign_bit as u8,
                        Piece::King,
                        *color,
                        self.get_captured_piece(BOARD_SQUARES[least_sign_bit as usize]),
                        false,
                        // later implement promotion
                    )
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        exit(1);
                    }),
                );

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

                moves_vec.push(
                    self.encode_move(
                        piece_move.0.trailing_zeros() as u8,
                        least_sign_bit as u8,
                        Piece::Knight,
                        *color,
                        self.get_captured_piece(BOARD_SQUARES[least_sign_bit as usize]),
                        false,
                        // later implement promotion
                    )
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        exit(1);
                    }),
                );

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

                moves_vec.push(
                    self.encode_move(
                        piece_move.0.trailing_zeros() as u8,
                        least_sign_bit as u8,
                        Piece::Bishop,
                        *color,
                        self.get_captured_piece(BOARD_SQUARES[least_sign_bit as usize]),
                        false,
                        // later implement promotion
                    )
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        exit(1);
                    }),
                );
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

                moves_vec.push(
                    self.encode_move(
                        piece_move.0.trailing_zeros() as u8,
                        least_sign_bit as u8,
                        Piece::Rook,
                        *color,
                        self.get_captured_piece(BOARD_SQUARES[least_sign_bit as usize]),
                        false,
                        // later implement promotion
                    )
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        exit(1);
                    }),
                );

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

                moves_vec.push(
                    self.encode_move(
                        piece_move.0.trailing_zeros() as u8,
                        least_sign_bit as u8,
                        Piece::Queen,
                        *color,
                        self.get_captured_piece(BOARD_SQUARES[least_sign_bit as usize]),
                        false,
                        // later implement promotion
                    )
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        exit(1);
                    }),
                );

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
