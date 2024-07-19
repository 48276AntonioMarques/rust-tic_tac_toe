use iced::{
    window::{self, Position},
    Application, Settings, Size,
};

mod contest;
mod game;
mod localization;
mod score;

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: window::Settings {
            position: Position::Centered,
            size: Size {
                width: 900.0,
                height: 600.0,
            },
            min_size: Option::Some(Size {
                width: 500.0,
                height: 500.0,
            }),
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    contest::Contest::run(settings)
}
