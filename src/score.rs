pub struct Score {
    pub x_wins: u32,
    pub o_wins: u32,
}

use crate::game::state::State;

impl Score {
    pub fn new() -> Score {
        Score {
            x_wins: 0,
            o_wins: 0,
        }
    }

    pub fn update(&self, winner: State) -> Score {
        match winner {
            State::XWon => Score {
                x_wins: self.x_wins + 1,
                o_wins: self.o_wins,
            },
            State::OWon => Score {
                x_wins: self.x_wins,
                o_wins: self.o_wins + 1,
            },
            State::Draw => Score {
                x_wins: self.x_wins,
                o_wins: self.o_wins,
            },
        }
    }
}

impl Clone for Score {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Score {}
