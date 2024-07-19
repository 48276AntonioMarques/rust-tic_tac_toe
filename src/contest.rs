use iced::{Sandbox, Theme};

use crate::contest::ui::view;
use crate::game;
use crate::score;

mod logic;
mod ui;

pub struct Contest {
    score: score::Score,
    game: Option<game::Game>,
}

impl Sandbox for Contest {
    type Message = logic::message::Message;

    fn new() -> Self {
        Self {
            score: score::Score::new(),
            game: Option::Some(game::Game::new()),
        }
    }

    fn title(&self) -> String {
        String::from("Tic Tac Toe")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: logic::message::Message) {
        logic::update(self, message)
    }

    fn view(&self) -> iced::Element<'_, logic::message::Message> {
        view(self)
    }
}
