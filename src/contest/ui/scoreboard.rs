use iced::widget::{row, text};
use iced::{Alignment, Element, Renderer, Theme};

use crate::contest::logic::message::Message;
use crate::score::Score;

pub fn scoreboard(score: &Score) -> Element<'static, Message, Theme, Renderer> {
    row![text(format!("X:{} - O:{}", score.x_wins, score.o_wins)).size(30)]
        .align_items(Alignment::Center)
        .padding(10)
        .into()
}
