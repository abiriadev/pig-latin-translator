const ASCII_ALPHABET_CAPITAL_START: u8 = 65;
const ASCII_ALPHABET_CAPITAL_END: u8 = 90;
const ASCII_ALPHABET_LOWER_START: u8 = 97;
const ASCII_ALPHABET_LOWER_END: u8 = 122;

const BETWEEN: u8 = ASCII_ALPHABET_LOWER_START - ASCII_ALPHABET_CAPITAL_START;

pub fn to_upper_case(string: &String) -> String {
    let mut save = String::new();

    for char in string.chars() {
        // println!("{}", char);

        save.push(if is_lower(char) {
            // println!("{}", char);
            upper(char)
        } else {
            char
        })
    }

    save
}

pub fn upper(char: char) -> char {
    // println!(
    //     "{}: {} -> {}: {}",
    //     char,
    //     (char as u8),
    //     (char as u8) + BETWEEN,
    //     ((char as u8) + BETWEEN) as char
    // );

    ((char as u8) - BETWEEN) as char
}

pub fn is_alphabet(char: char) -> bool {
    is_lower(char) && is_capital(char)
}

pub fn is_lower(char: char) -> bool {
    ASCII_ALPHABET_LOWER_START <= char as u8 && char as u8 <= ASCII_ALPHABET_LOWER_END
}

pub fn is_capital(char: char) -> bool {
    ASCII_ALPHABET_CAPITAL_START <= char as u8 && char as u8 <= ASCII_ALPHABET_CAPITAL_END
}

// pub enum Vowel {
//     A,
//     E,
//     I,
//     O,
//     U,
// }

const VOWEL_ARR: [u8; 5] = [65u8, 69u8, 73u8, 79u8, 85u8];

pub fn is_vowel(ch: char) -> bool {
    for asc in VOWEL_ARR {
        if asc == (ch as u8) || asc + BETWEEN == (ch as u8) {
            return true;
        }
    }
    false
}
