use std::fmt;

pub enum Player {
    X,
    O,
}

impl Player {
    pub fn switch(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Player {}
