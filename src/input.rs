pub enum InputType {
    Number,
    Char,
}

pub enum OutputType {
    Char(char),
    Number(char),
    ToggleOnlyDef,
    Backspace,
    Enter,
    Invalid,
}

pub fn listen_to_input(desired_input_type: InputType) -> OutputType {
    use crossterm::{
        event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
        terminal,
    };

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
                if c == '.' {
                    terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    return OutputType::ToggleOnlyDef;
                }
                match desired_input_type {
                    InputType::Char => {
                        if c.is_alphabetic() {
                            terminal::disable_raw_mode().expect("Failed to disable raw mode");
                            return OutputType::Char(c);
                        }
                    }
                    InputType::Number => {
                        if c.is_numeric() {
                            terminal::disable_raw_mode().expect("Failed to disable raw mode");
                            return OutputType::Number(c);
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                terminal::disable_raw_mode().expect("Failed to disable raw mode");
                return OutputType::Backspace;
            }
            KeyCode::Enter => {
                terminal::disable_raw_mode().expect("Failed to disable raw mode");
                return OutputType::Enter;
            }
            _ => {}
        },
        _ => {}
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    return OutputType::Invalid;
}
