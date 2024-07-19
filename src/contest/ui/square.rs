use iced::widget::{button, text, Button};
use iced::{alignment::Horizontal, alignment::Vertical, Renderer, Theme};

use crate::contest::logic::message::Message;
use crate::game::movement::Move;

pub fn square(board: [[Move; 3]; 3], index: usize) -> Button<'static, Message, Theme, Renderer> {
    let size = 100;
    let movement = board[index / 3][index % 3];
    let value = match movement {
        Move::Empty => " ",
        Move::X => "X",
        Move::O => "O",
    };
    button(
        text(value)
            .size(size / 2)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .on_press(Message::Play(index + 1))
    .height(size)
    .width(size)
}
