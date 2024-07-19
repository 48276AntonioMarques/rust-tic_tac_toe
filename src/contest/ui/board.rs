use iced::widget::{column, row, Column};
use iced::{Renderer, Theme};

use crate::contest::logic::message::Message;
use crate::contest::ui::square::square;
use crate::game::movement::Move;

pub fn board(board: [[Move; 3]; 3]) -> Column<'static, Message, Theme, Renderer> {
    column![
        row![square(board, 0), square(board, 1), square(board, 2),],
        row![square(board, 3), square(board, 4), square(board, 5),],
        row![square(board, 6), square(board, 7), square(board, 8),],
    ]
}
