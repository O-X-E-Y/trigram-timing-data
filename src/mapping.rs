use std::{fmt::Display, str::FromStr};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        match key {
            "Escape" => Ok(Pos { row: 9, col: 0 }),
            "Backquote" => Ok(Pos { row: 0, col: 0 }),
            "Digit1" => Ok(Pos { row: 0, col: 1 }),
            "Digit2" => Ok(Pos { row: 0, col: 2 }),
            "Digit3" => Ok(Pos { row: 0, col: 3 }),
            "Digit4" => Ok(Pos { row: 0, col: 4 }),
            "Digit5" => Ok(Pos { row: 0, col: 5 }),
            "Digit6" => Ok(Pos { row: 0, col: 6 }),
            "Digit7" => Ok(Pos { row: 0, col: 7 }),
            "Digit8" => Ok(Pos { row: 0, col: 8 }),
            "Digit9" => Ok(Pos { row: 0, col: 9 }),
            "Digit0" => Ok(Pos { row: 0, col: 10 }),
            "Minus" => Ok(Pos { row: 0, col: 11 }),
            "Equal" => Ok(Pos { row: 0, col: 12 }),
            "Backspace" => Ok(Pos { row: 0, col: 13 }),
            "Tab" => Ok(Pos { row: 1, col: 0 }),
            "KeyQ" => Ok(Pos { row: 1, col: 1 }),
            "KeyW" => Ok(Pos { row: 1, col: 2 }),
            "KeyE" => Ok(Pos { row: 1, col: 3 }),
            "KeyR" => Ok(Pos { row: 1, col: 4 }),
            "KeyT" => Ok(Pos { row: 1, col: 5 }),
            "KeyY" => Ok(Pos { row: 1, col: 6 }),
            "KeyU" => Ok(Pos { row: 1, col: 7 }),
            "KeyI" => Ok(Pos { row: 1, col: 8 }),
            "KeyO" => Ok(Pos { row: 1, col: 9 }),
            "KeyP" => Ok(Pos { row: 1, col: 10 }),
            "BracketRight" => Ok(Pos { row: 1, col: 11 }),
            "BracketLeft" => Ok(Pos { row: 1, col: 12 }),
            "Enter" => Ok(Pos { row: 1, col: 13 }),
            "CapsLock" => Ok(Pos { row: 2, col: 0 }),
            "KeyA" => Ok(Pos { row: 2, col: 1 }),
            "KeyS" => Ok(Pos { row: 2, col: 2 }),
            "KeyD" => Ok(Pos { row: 2, col: 3 }),
            "KeyF" => Ok(Pos { row: 2, col: 4 }),
            "KeyG" => Ok(Pos { row: 2, col: 5 }),
            "KeyH" => Ok(Pos { row: 2, col: 6 }),
            "KeyJ" => Ok(Pos { row: 2, col: 7 }),
            "KeyK" => Ok(Pos { row: 2, col: 8 }),
            "KeyL" => Ok(Pos { row: 2, col: 9 }),
            "Semicolon" => Ok(Pos { row: 2, col: 10 }),
            "Quote" => Ok(Pos { row: 2, col: 11 }),
            "Backslash" => Ok(Pos { row: 2, col: 12 }),
            "ShiftLeft" => Ok(Pos { row: 3, col: 0 }),
            "IntlBackslash" => Ok(Pos { row: 3, col: 1 }),
            "KeyZ" => Ok(Pos { row: 3, col: 2 }),
            "KeyX" => Ok(Pos { row: 3, col: 3 }),
            "KeyC" => Ok(Pos { row: 3, col: 4 }),
            "KeyV" => Ok(Pos { row: 3, col: 5 }),
            "KeyB" => Ok(Pos { row: 3, col: 6 }),
            "KeyN" => Ok(Pos { row: 3, col: 7 }),
            "KeyM" => Ok(Pos { row: 3, col: 8 }),
            "Comma" => Ok(Pos { row: 3, col: 9 }),
            "Period" => Ok(Pos { row: 3, col: 10 }),
            "Slash" => Ok(Pos { row: 3, col: 11 }),
            "ShiftRight" => Ok(Pos { row: 3, col: 12 }),
            "ControlLeft" => Ok(Pos { row: 4, col: 0 }),
            "OSLeft" => Ok(Pos { row: 4, col: 1 }),
            "AltLeft" => Ok(Pos { row: 4, col: 2 }),
            "Space" => Ok(Pos { row: 4, col: 3 }),
            "AltRight" => Ok(Pos { row: 4, col: 4 }),
            "ContextMenu" => Ok(Pos { row: 4, col: 5 }),
            "ControlRight" => Ok(Pos { row: 4, col: 6 }),
            _ => Err(format!("invalid key: {}", key)),
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pos { row: 9, col: 0 } => write!(f, "Escape"),
            Pos { row: 0, col: 0 } => write!(f, "Backquote"),
            Pos { row: 0, col: 1 } => write!(f, "Digit1"),
            Pos { row: 0, col: 2 } => write!(f, "Digit2"),
            Pos { row: 0, col: 3 } => write!(f, "Digit3"),
            Pos { row: 0, col: 4 } => write!(f, "Digit4"),
            Pos { row: 0, col: 5 } => write!(f, "Digit5"),
            Pos { row: 0, col: 6 } => write!(f, "Digit6"),
            Pos { row: 0, col: 7 } => write!(f, "Digit7"),
            Pos { row: 0, col: 8 } => write!(f, "Digit8"),
            Pos { row: 0, col: 9 } => write!(f, "Digit9"),
            Pos { row: 0, col: 10 } => write!(f, "Digit0"),
            Pos { row: 0, col: 11 } => write!(f, "Minus"),
            Pos { row: 0, col: 12 } => write!(f, "Equal"),
            Pos { row: 0, col: 13 } => write!(f, "Backspace"),
            Pos { row: 1, col: 0 } => write!(f, "Tab"),
            Pos { row: 1, col: 1 } => write!(f, "KeyQ"),
            Pos { row: 1, col: 2 } => write!(f, "KeyW"),
            Pos { row: 1, col: 3 } => write!(f, "KeyE"),
            Pos { row: 1, col: 4 } => write!(f, "KeyR"),
            Pos { row: 1, col: 5 } => write!(f, "KeyT"),
            Pos { row: 1, col: 6 } => write!(f, "KeyY"),
            Pos { row: 1, col: 7 } => write!(f, "KeyU"),
            Pos { row: 1, col: 8 } => write!(f, "KeyI"),
            Pos { row: 1, col: 9 } => write!(f, "KeyO"),
            Pos { row: 1, col: 10 } => write!(f, "KeyP"),
            Pos { row: 1, col: 11 } => write!(f, "BracketRight"),
            Pos { row: 1, col: 12 } => write!(f, "BracketLeft"),
            Pos { row: 1, col: 13 } => write!(f, "Enter"),
            Pos { row: 2, col: 0 } => write!(f, "CapsLock"),
            Pos { row: 2, col: 1 } => write!(f, "KeyA"),
            Pos { row: 2, col: 2 } => write!(f, "KeyS"),
            Pos { row: 2, col: 3 } => write!(f, "KeyD"),
            Pos { row: 2, col: 4 } => write!(f, "KeyF"),
            Pos { row: 2, col: 5 } => write!(f, "KeyG"),
            Pos { row: 2, col: 6 } => write!(f, "KeyH"),
            Pos { row: 2, col: 7 } => write!(f, "KeyJ"),
            Pos { row: 2, col: 8 } => write!(f, "KeyK"),
            Pos { row: 2, col: 9 } => write!(f, "KeyL"),
            Pos { row: 2, col: 10 } => write!(f, "Semicolon"),
            Pos { row: 2, col: 11 } => write!(f, "Quote"),
            Pos { row: 2, col: 12 } => write!(f, "Backslash"),
            Pos { row: 3, col: 0 } => write!(f, "ShiftLeft"),
            Pos { row: 3, col: 1 } => write!(f, "IntlBackslash"),
            Pos { row: 3, col: 2 } => write!(f, "KeyZ"),
            Pos { row: 3, col: 3 } => write!(f, "KeyX"),
            Pos { row: 3, col: 4 } => write!(f, "KeyC"),
            Pos { row: 3, col: 5 } => write!(f, "KeyV"),
            Pos { row: 3, col: 6 } => write!(f, "KeyB"),
            Pos { row: 3, col: 7 } => write!(f, "KeyN"),
            Pos { row: 3, col: 8 } => write!(f, "KeyM"),
            Pos { row: 3, col: 9 } => write!(f, "Comma"),
            Pos { row: 3, col: 10 } => write!(f, "Period"),
            Pos { row: 3, col: 11 } => write!(f, "Slash"),
            Pos { row: 3, col: 12 } => write!(f, "ShiftRight"),
            Pos { row: 4, col: 0 } => write!(f, "ControlLeft"),
            Pos { row: 4, col: 1 } => write!(f, "OSLeft"),
            Pos { row: 4, col: 2 } => write!(f, "AltLeft"),
            Pos { row: 4, col: 3 } => write!(f, "Space"),
            Pos { row: 4, col: 4 } => write!(f, "AltRight"),
            Pos { row: 4, col: 5 } => write!(f, "ContextMenu"),
            Pos { row: 4, col: 6 } => write!(f, "ControlRight"),
            _ => unreachable!("Invalid keyboard position: {self:?}"),
        }
    }
}
