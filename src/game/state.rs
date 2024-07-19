use std::fmt;

use crate::localization;

pub enum State {
    XWon,
    OWon,
    Draw,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::XWon => write!(
                f,
                "X {}",
                localization::get_localizad_string(localization::resource::Resource::Wins)
            ),
            State::OWon => write!(
                f,
                "O {}",
                localization::get_localizad_string(localization::resource::Resource::Wins)
            ),
            State::Draw => write!(
                f,
                "{}",
                localization::get_localizad_string(localization::resource::Resource::Draw)
            ),
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for State {}
