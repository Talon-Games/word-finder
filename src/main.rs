pub mod letter_input;
pub mod number_input;

use crossterm::{cursor, terminal, ExecutableCommand};
use letter_input::LetterInput;
use number_input::NumberInput;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut search_english_list = true;

    if args.len() > 1 && args[1] == "latin" {
        search_english_list = false;
    }

    println!("Welcome to word finder!");
    if search_english_list {
        println!("Find the perfect word for you.");
    } else {
        println!("Find the perfect Latin word for you.");
    }
    println!("Press \"esc\" at any time to exit the program.");
    loop {
        search(search_english_list)
    }
}

fn search(search_english_list: bool) {
    let word_length = NumberInput::new()
        .message("Word length: ")
        .min(1)
        .max(50)
        .ask();

    let mut letter_requirements = Vec::new();

    for letter_index in 0..word_length {
        let letter_index_with_ordinal_suffix = get_ordinal_suffix(letter_index + 1);
        let letter = LetterInput::new()
            .message(&format!("{} letter: ", letter_index_with_ordinal_suffix))
            .ask();

        letter_requirements.push(letter);
    }

    let words = find_words_from_list(word_length, letter_requirements, search_english_list);
    println!("--------------------------------------");
    if words.is_empty() {
        println!("No Words Found")
    } else {
        for word in words {
            println!("| {}", word)
        }
    }
    println!("--------------------------------------");
}

fn find_words_from_list(
    word_length: i32,
    letter_requirements: Vec<String>,
    search_english_list: bool,
) -> Vec<String> {
    let file = if search_english_list == true {
        include_str!("./words.txt")
    } else {
        include_str!("./latin_words.txt")
    };

    let words_file: Vec<&str> = file
        .lines()
        .map(|line| line.trim())
        .filter(|line| {
            !line.is_empty()
                && line.split("::").collect::<Vec<&str>>()[0].trim().len() == word_length as usize
        })
        .collect();

    let mut fitting_words = Vec::new();

    for line in words_file {
        let mut valid = true;
        let line: Vec<&str> = line.split("::").collect();
        let word = line[0].trim();
        let mut definition = line[1].trim();
        for (i, letter) in word
            .trim()
            .split("")
            .into_iter()
            .filter(|s| !s.is_empty())
            .enumerate()
        {
            if letter_requirements[i] == "" {
                continue;
            }
            if letter_requirements[i] != letter {
                valid = false;
            }
        }
        if !valid {
            continue;
        }

        if definition.len() == 0 {
            definition = "[no definition]"
        }

        fitting_words.push(format!("{} - {}", word, definition))
    }

    fitting_words
}

fn get_ordinal_suffix(number: i32) -> String {
    let suffix = match number % 100 {
        11 | 12 | 13 => "th",
        _ => match number % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    format!("{}{}", number, suffix)
}

fn refresh_display(lines: i32) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}
