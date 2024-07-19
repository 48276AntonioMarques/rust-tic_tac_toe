use crate::contest::Contest;
use crate::game::Game;

use crate::contest::logic::message::Message;
use crate::localization::{get_localizad_string, resource::Resource};

pub mod message;

pub fn update(contest: &mut Contest, message: Message) {
    match message {
        Message::Play(number) => {
            if let Some(game) = &mut contest.game {
                if game.winner.is_some() {
                    return;
                }
                match game.play(number) {
                    Ok(new_game) => {
                        if let Some(winner) = new_game.winner {
                            contest.score = contest.score.update(winner);
                        }
                        contest.game = Option::Some(new_game);
                    }
                    Err(message) => {
                        contest.game = Option::Some(Game {
                            winner: game.winner,
                            current_player: game.current_player,
                            board: game.board,
                            message: Option::Some(message),
                        });
                    }
                }
            }
        }
        Message::NewGame => {
            if let Some(game) = &mut contest.game {
                if game.winner.is_some() {
                    // Score is already set only thing to do is to reset the game
                    contest.game = Option::Some(Game::new());
                    return;
                }
                contest.game = Option::Some(Game {
                    winner: game.winner,
                    current_player: game.current_player,
                    board: game.board,
                    message: Option::Some(get_localizad_string(Resource::UnableToStartNewGame)),
                })
            }
        }
        Message::Exit => std::process::exit(0),
    }
}
