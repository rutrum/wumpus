pub enum Action {
    Move(usize),
    Shoot(usize),
    Quit,
    Help,
}

impl Action {
    pub fn chad_from(s: String) -> Result<Action, ActionError> {
        use ActionError::*;

        // Constructs an iterator
        let mut word_iter = s.split_whitespace();

        // .next() gets the first element, but may not exist.
        // if it's not okay, return EmptyCommand and return early
        let first = word_iter.next().ok_or(EmptyCommand)?;

        // Closure for getting second word as a room number
        let mut parse_room_number = || {
            let loc = word_iter.next().ok_or(NoNumber)?;
            loc.parse().map_err(|_| InvalidNumber)
        };

        // Lowercase command and compare against possibilities
        Ok(match first.to_lowercase().as_ref() {
            "shoot" | "s" => Action::Shoot(parse_room_number()?),
            "move" | "m" => Action::Move(parse_room_number()?),
            "quit" | "q" => Action::Quit,
            "help" | "h" => Action::Help,
            _ => return Err(InvalidCommand),
        })
    }

    pub fn from(s: String) -> Result<Action, String> {
        // Constructs an iterator
        let mut word_iter = s.split_whitespace();

        // .next() gets the first element, but it may not exist
        match word_iter.next() {
            None => Err("please type in a command".to_string()),

            Some(word) => match word.to_lowercase().as_ref() {
                "shoot" | "s" => match word_iter.next() {
                    None => Err("Please also type a room number.".to_string()),
                    Some(loc) => match loc.parse::<usize>() {
                        Err(_) => Err("Please type a valid number.".to_string()),
                        Ok(num) => Ok(Action::Shoot(num)),
                    },
                },

                "move" | "m" => match word_iter.next() {
                    None => Err("Please also type a room number.".to_string()),
                    Some(loc) => match loc.parse::<usize>() {
                        Err(_) => Err("Please type a valid number.".to_string()),
                        Ok(num) => Ok(Action::Move(num)),
                    },
                },

                "quit" | "q" => Ok(Action::Quit),
                "help" | "h" => Ok(Action::Help),
                _ => Err("Please enter a valid command.".to_string()),
            },
        }
    }
}

pub enum ActionError {
    EmptyCommand,
    InvalidNumber,
    InvalidCommand,
    NoNumber,
}

use std::fmt;

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ActionError::*;
        match self {
            EmptyCommand => write!(f, "Please enter a command."),
            InvalidCommand => write!(f, "That command does not exist."),
            NoNumber => write!(f, "Please enter a room number after command."),
            InvalidNumber => write!(f, "Please enter a valid positive integer."),
        }
    }
}