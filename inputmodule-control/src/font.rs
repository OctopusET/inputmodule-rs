/// 5x6 symbol font. Leaves 2 pixels on each side empty
/// We can leave one row empty below and then the display fits 5 of these digits.
#[rustfmt::skip]
pub fn convert_symbol(symbol: &str) -> Vec<u8> {
    match symbol {
        "degC" => vec![
            1, 1, 0, 0, 0,
            1, 1, 0, 0, 0,
            0, 0, 1, 1, 1,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 1, 1,
        ],
        "degF" => vec![
            1, 1, 0, 0, 0,
            1, 1, 0, 0, 0,
            0, 0, 1, 1, 1,
            0, 0, 1, 0, 0,
            0, 0, 1, 1, 1,
            0, 0, 1, 0, 0,
        ],
        "snow" => vec![
            0, 0, 0, 0, 0,
            1, 0, 1, 0, 1,
            0, 1, 1, 1, 0,
            1, 1, 1, 1, 1,
            0, 1, 1, 1, 0,
            1, 0, 1, 0, 1,
        ],
        "sun" => vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            0, 1, 1, 1, 0,
        ],
        "cloud" => vec![
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
        "rain" => vec![
            0, 1, 1, 1, 0,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            0, 1, 0, 0, 1,
            0, 0, 1, 0, 0,
            1, 0, 0, 1, 0,
        ],
        "thunder" => vec![
            0, 1, 1, 1, 0,
            1, 1, 1, 1, 1,
            1, 1, 1, 1, 1,
            0, 0, 1, 0, 0,
            0, 1, 0, 0, 0,
            0, 0, 1, 0, 0,
        ],
        "batteryLow" => vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            1, 1, 1, 1, 0,
            1, 0, 0, 1, 1,
            1, 0, 0, 1, 1,
            1, 1, 1, 1, 0,
        ],
        "!!" => vec![
            0, 1, 0, 1, 0,
            0, 1, 0, 1, 0,
            0, 1, 0, 1, 0,
            0, 0, 0, 0, 0,
            0, 1, 0, 1, 0,
            0, 1, 0, 1, 0,
        ],
        "heart" => vec![
            0, 0, 0, 0, 0,
            1, 1, 0, 1, 1,
            1, 1, 1, 1, 1,
            0, 1, 1, 1, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
        ],
        "heart0" => vec![
            1, 1, 0, 1, 1,
            1, 1, 1, 1, 1,
            0, 1, 1, 1, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
        "heart2" => vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            1, 1, 0, 1, 1,
            1, 1, 1, 1, 1,
            0, 1, 1, 1, 0,
            0, 0, 1, 0, 0,
        ],
        ":)" => vec![
            0, 0, 0, 0, 0,
            0, 1, 0, 1, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],
        ":|" => vec![
            0, 0, 0, 0, 0,
            0, 1, 0, 1, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 0,
        ],
        ":(" => vec![
            0, 0, 0, 0, 0,
            0, 1, 0, 1, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
        ],
        ";)" => vec![
            0, 0, 0, 0, 0,
            1, 1, 0, 1, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],
        _ => {
            if symbol.len() == 1 {
                convert_font(symbol.chars().next().unwrap())
            } else {
                convert_font('?')
            }
        },
    }
}

/// 5x6 font. Leaves 2 pixels on each side empty
/// We can leave one row empty below and then the display fits 5 of these digits.
#[rustfmt::skip]
pub fn convert_font(c: char) -> Vec<u8> {
    match c {
        '0' => vec![
            0, 1, 1, 0, 0,
            1, 0, 0, 1, 0,
            1, 0, 0, 1, 0,
            1, 0, 0, 1, 0,
            1, 0, 0, 1, 0,
            0, 1, 1, 0, 0,
        ],

        '1' => vec![
            0, 0, 1, 0, 0,
            0, 1, 1, 0, 0,
            1, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            1, 1, 1, 1, 1,
        ],

        '2' => vec![
            1, 1, 1, 1, 0,
            0, 0, 0, 0, 1,
            1, 1, 1, 1, 1,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
        ],

        '3' => vec![
            1, 1, 1, 1, 0,
            0, 0, 0, 0, 1,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 1,
            1, 1, 1, 1, 0,
        ],

        '4' => vec![
            0, 0, 0, 1, 0,
            0, 0, 1, 1, 0,
            0, 1, 0, 1, 0,
            1, 1, 1, 1, 1,
            0, 0, 0, 1, 0,
            0, 0, 0, 1, 0,
        ],

        '5' => vec![
            1, 1, 1, 1, 1,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 1,
            1, 1, 1, 1, 0,
        ],

        '6' => vec![
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],

        '7' => vec![
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 1,
            0, 0, 0, 1, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
        ],

        '8' => vec![
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],

        '9' => vec![
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],

        ':' => vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
        ],

        ' ' => vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],

        '?' => vec![
            0, 1, 1, 0, 0,
            0, 0, 0, 1, 0,
            0, 0, 0, 1, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
        ],

        '.' => vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],

        ',' => vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],

        '!' => vec![
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 1, 0, 0,
        ],

        '/' => vec![
            0, 0, 0, 0, 1,
            0, 0, 0, 1, 1,
            0, 0, 1, 1, 0,
            0, 1, 1, 0, 0,
            1, 1, 0, 0, 0,
            1, 0, 0, 0, 0,
        ],

        '*' => vec![
            0, 0, 0, 0, 0,
            0, 1, 0, 1, 0,
            0, 0, 1, 0, 0,
            0, 1, 0, 1, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],

        '%' => vec![
            1, 1, 0, 0, 1,
            1, 1, 0, 1, 1,
            0, 0, 1, 1, 0,
            0, 1, 1, 0, 0,
            1, 1, 0, 1, 1,
            1, 0, 0, 1, 1,
        ],

        '+' => vec![
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            1, 1, 1, 1, 1,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 0, 0, 0,
        ],

        '-' => vec![
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
        '=' => vec![
            0, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
            0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
        'A' => vec![
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
            1, 1, 1, 1, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
        ],
        'B' => vec![
            1, 1, 1, 0, 0,
            1, 0, 0, 1, 0,
            1, 1, 1, 0, 0,
            1, 0, 0, 1, 0,
            1, 0, 0, 1, 0,
            1, 1, 1, 0, 0,
        ],
        'C' => vec![
            1, 1, 1, 1, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 0,
        ],
        'D' => vec![
            1, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 1, 1, 1, 0,
        ],
        'E' => vec![
            1, 1, 1, 1, 1,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
        ],
        'F' => vec![
            1, 1, 1, 1, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 0,
        ],
        'G' => vec![
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 0,
            1, 0, 1, 1, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],
        'H' => vec![
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 1, 1, 1, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
        ],
        'I' => vec![
            1, 1, 1, 1, 1,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            1, 1, 1, 1, 1,
        ],
        'J' => vec![
            0, 1, 1, 1, 1,
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 1,
            0, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],
        'K' => vec![
            1, 0, 0, 1, 0,
            1, 0, 1, 0, 0,
            1, 1, 0, 0, 0,
            1, 1, 0, 0, 0,
            1, 0, 1, 0, 0,
            1, 0, 0, 1, 0,
        ],
        'L' => vec![
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
        ],
        'M' => vec![
            0, 0, 0, 0, 0,
            0, 1, 0, 1, 0,
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
        ],
        'N' => vec![
            1, 0, 0, 0, 1,
            1, 1, 0, 0, 1,
            1, 0, 1, 0, 1,
            1, 0, 0, 1, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
        ],
        'O' => vec![
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],
        'P' => vec![
            1, 1, 1, 0, 0,
            1, 0, 0, 1, 0,
            1, 0, 0, 1, 0,
            1, 1, 1, 0, 0,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
        ],
        'Q' => vec![
            0, 1, 1, 1, 0,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 1, 0, 1,
            1, 0, 0, 1, 0,
            0, 1, 1, 0, 1,
        ],
        'R' => vec![
            1, 1, 1, 1, 0,
            1, 0, 0, 1, 0,
            1, 1, 1, 1, 0,
            1, 1, 0, 0, 0,
            1, 0, 1, 0, 0,
            1, 0, 0, 1, 0,
        ],
        'S' => vec![
            0, 1, 1, 1, 1,
            1, 0, 0, 0, 0,
            1, 0, 0, 0, 0,
            0, 1, 1, 1, 0,
            0, 0, 0, 0, 1,
            1, 1, 1, 1, 0
        ],
        'T' => vec![
            1, 1, 1, 1, 1,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
        ],
        'U' => vec![
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            0, 1, 1, 1, 0,
        ],
        'V' => vec![
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            0, 1, 0, 1, 0,
            0, 1, 0, 1, 0,
            0, 0, 1, 0, 0,
        ],
        'W' => vec![
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
            0, 1, 0, 1, 0,
            0, 1, 0, 1, 0
        ],
        'X' => vec![
            0, 0, 0, 0, 0,
            1, 0, 0, 0, 1,
            0, 1, 0, 1, 0,
            0, 0, 1, 0, 0,
            0, 1, 0, 1, 0,
            1, 0, 0, 0, 1,
        ],
        'Y' => vec![
            1, 0, 0, 0, 1,
            1, 0, 0, 0, 1,
            0, 1, 0, 1, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
            0, 0, 1, 0, 0,
        ],
        'Z' => vec![
            1, 1, 1, 1, 1,
            0, 0, 0, 1, 0,
            0, 0, 1, 0, 0,
            0, 1, 0, 0, 0,
            1, 0, 0, 0, 0,
            1, 1, 1, 1, 1,
        ],
        _ => convert_font('?'),
    }
}
