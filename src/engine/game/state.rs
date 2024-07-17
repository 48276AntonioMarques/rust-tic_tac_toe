use std::fmt;

pub enum State {
    XWon,
    OWon,
    Draw,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::XWon => write!(f, "X wins"),
            State::OWon => write!(f, "O wins"),
            State::Draw => write!(f, "Draw"),
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for State {}
