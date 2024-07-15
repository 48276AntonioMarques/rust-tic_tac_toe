use std::fmt;

pub enum Move {
    X,
    O,
    Empty,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::X => write!(f, "X"),
            Move::O => write!(f, "O"),
            Move::Empty => write!(f, " "),
        }
    }
}

impl Clone for Move {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Move {}
