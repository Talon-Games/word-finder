pub mod letter_input;
pub mod number_input;

use crossterm::{cursor, terminal, ExecutableCommand};
use letter_input::LetterInput;
use number_input::NumberInput;
use std::fs::read_to_string;
use std::path::Path;
use std::{env, io};

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

#[derive(PartialEq)]
enum WordList {
    English,
    Latin,
    Custom,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut word_list = WordList::English;

    if args.len() > 1 {
        if args[1] == "latin" {
            word_list = WordList::Latin;
        } else {
            word_list = WordList::Custom;
        }
    }

    let file = if word_list == WordList::English {
        include_str!("./words.txt")
    } else if word_list == WordList::Latin {
        include_str!("./latin_words.txt")
    } else {
        &validate_custom_word_list(&args[1])
    };

    println!("Welcome to word finder!");
    if word_list == WordList::English || word_list == WordList::Custom {
        println!("Find the perfect word for you.");
    } else {
        println!("Find the perfect Latin word for you.");
    }
    println!("Use * for any vowels or - for any consonant");
    println!("Press \"esc\" at any time to exit the program.");
    loop {
        search(file)
    }
}

fn validate_custom_word_list(users_path: &str) -> String {
    let path = Path::new(users_path);
    if !path.exists() {
        eprint!(
            "Failed to load custom word list, no file found at: {}",
            users_path
        );
        std::process::exit(1);
    }
    if path.is_dir() {
        eprintln!("Failed to load custom word list, path must lead to a file");
        std::process::exit(1);
    }

    let file = match read_to_string(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to read custom word list: {}", err.to_string());
            std::process::exit(1);
        }
    };

    let mut line = 1;
    for content in file.lines() {
        if content.is_empty() {
            eprintln!(
                "Failed to read custom word list, no empty lines are allowed: line {}",
                line
            );
            std::process::exit(1);
        }

        if content.starts_with(" ") {
            eprintln!(
                "Failed to read custom word list, no spaces allowed at the start of a line: line {}",
                line
            );
            std::process::exit(1);
        }

        if !content.contains("::") {
            eprintln!(
                "Failed to read custom word list, all lines must have a definition seperator: line {}",
                line
            );
            std::process::exit(1);
        }
        line += 1;
    }

    println!("Loaded {} words from custom file", line - 1);

    return file;
}

fn search(file: &str) {
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

    let words = find_words_from_list(word_length, letter_requirements, file);
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
    file: &str,
) -> Vec<String> {
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
            if letter_requirements[i] == "*" && VOWELS.contains(&letter) {
                continue;
            }
            if letter_requirements[i] == "-" && !VOWELS.contains(&letter) {
                continue;
            }
            if letter_requirements[i] != letter {
                valid = false;
                break;
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
