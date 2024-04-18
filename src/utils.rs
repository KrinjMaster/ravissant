use crate::board::{Bitboard, Color, Piece};

pub fn print_bitboard(bb: Bitboard) {
    let formatted_bb: String = format!("{:064b}", bb);
    let form_bb: Vec<&str> = formatted_bb
        .split("")
        .filter(|&str| !str.is_empty())
        .collect();

    for rank in 0..8 {
        let rank: String = match rank {
            0 => "8  ".to_string(),
            1 => "7  ".to_string(),
            2 => "6  ".to_string(),
            3 => "5  ".to_string(),
            4 => "4  ".to_string(),
            5 => "3  ".to_string(),
            6 => "2  ".to_string(),
            7 => "1  ".to_string(),
            _ => continue,
        } + &form_bb[(7 - rank) * 8..(7 - rank) * 8 + 8]
            .join(" ")
            .chars()
            .rev()
            .collect::<String>();

        println!("{}", rank);
    }
    println!("\n   a b c d e f g h\n");
    println!("bitboard is: {}", bb);
}

pub fn opposite_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}

pub fn number_to_piece(piece: u32) -> Result<Piece, &'static str> {
    match piece {
        0 => Ok(Piece::Pawn),
        1 => Ok(Piece::Knight),
        2 => Ok(Piece::Bishop),
        3 => Ok(Piece::Rook),
        4 => Ok(Piece::Queen),
        5 => Ok(Piece::King),
        6 => Ok(Piece::None),
        _ => Err("Piece wasn't found when converting!"),
    }
}

pub fn number_to_color(color: u32) -> Result<Color, &'static str> {
    match color {
        0 => Ok(Color::White),
        1 => Ok(Color::Black),
        _ => Err("Color wasn't found when converting!"),
    }
}
