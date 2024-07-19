use iced::widget::{column, container, row, text};
use iced::Length;

use crate::contest::logic::message::Message;
use crate::contest::ui::{
    buttons::{exit_button, new_game_button},
    scoreboard::scoreboard,
};
use crate::contest::Contest;
use crate::game::movement::Move;
use crate::localization::{get_localizad_string, resource::Resource};

pub mod board;
pub mod buttons;
pub mod scoreboard;
pub mod square;

pub fn view(contest: &Contest) -> iced::Element<'_, Message> {
    let message = match &contest.game {
        Some(game) => match &game.message {
            Some(message) => message.clone(),
            None => {
                get_localizad_string(Resource::NowPlaying) + ": " + &game.current_player.to_string()
            }
        },
        None => String::from(""),
    };
    let board = match &contest.game {
        Some(game) => game.board,
        None => [[Move::Empty; 3]; 3],
    };
    let page = column![
        scoreboard(&contest.score),
        row![board::board(board)],
        row![text(message)].padding(10),
        row![new_game_button(), exit_button()]
            .spacing(10)
            .padding(10)
    ]
    .align_items(iced::Alignment::Center);
    container(page)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
