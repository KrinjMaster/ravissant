use crate::board::{Bitboard, Color};

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
