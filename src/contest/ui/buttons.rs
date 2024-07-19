use iced::widget::{button, text, Button};
use iced::{Renderer, Theme};

use crate::contest::logic::message::Message::{self, Exit, NewGame};
use crate::localization::{self, resource::Resource};

pub fn new_game_button() -> Button<'static, Message, Theme, Renderer> {
    button(text(localization::get_localizad_string(
        Resource::PlayAgain,
    )))
    .on_press(NewGame)
}

pub fn exit_button() -> Button<'static, Message, Theme, Renderer> {
    button(text(localization::get_localizad_string(Resource::Exit))).on_press(Exit)
}
