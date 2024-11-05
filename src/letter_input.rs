use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};

use crate::refresh_display;

/**
 * Full size: the number of lines the element will take up
 * Reset size: the number of lines the user input section will take up
 */
pub struct LetterInput {
    message: String,
    pub full_size: i32,
    reset_size: i32,
    manual_clear: bool,
}

impl LetterInput {
    pub fn new() -> Self {
        LetterInput {
            message: String::new(),
            full_size: 2,
            reset_size: 1,
            manual_clear: false,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn manual_clear(mut self) -> Self {
        self.manual_clear = true;
        self
    }

    pub fn ask(&self) -> String {
        println!("{}", self.message);

        let mut current_char = String::new();

        loop {
            println!("> {}", current_char);
            terminal::enable_raw_mode().expect("Failed to enable raw mode");

            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Esc => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Char(c) => {
                        if c.is_alphabetic() || c == '-' || c == '*' {
                            current_char = c.to_string().to_lowercase();
                        }
                    }
                    KeyCode::Backspace => current_char = "".to_string(),
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        if !self.manual_clear {
                            refresh_display(self.full_size);
                        }
                        return current_char;
                    }
                    _ => {}
                },
                _ => {}
            }

            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            refresh_display(self.reset_size);
        }
    }
}
