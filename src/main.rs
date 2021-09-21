use std::io;

mod alphabet_tool;

fn main() {
    // let string = String::from("Abiria is babos");
    //
    // let upper = alphabet_tool::to_upper_case(&string);

    // println!("{}", upper);

    // let mut muts = String::from("word");

    // muts.remove(0);
    // muts.pop();

    // println!("{}", muts.chars().next().unwrap());

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input);

        let mut save_vec: Vec<String> = Vec::new();

        for mut word in input.split_whitespace() {
            let mut string = String::from(word);

            to_pig_latin(&mut string);

            save_vec.push(string);
        }

        println!("{}", save_vec.join(" "));
    }
}

fn to_pig_latin(word: &mut String) {
    let first_char = word.chars().next().unwrap();

    let will_insert = if !alphabet_tool::is_vowel(first_char) {
        word.remove(0);

        first_char
    } else {
        'h'
    };

    word.insert(word.len(), will_insert);

    // println!("{}", word);

    word.push_str("ay");
}
