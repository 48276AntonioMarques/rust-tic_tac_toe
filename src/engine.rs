use game::Game;

use std::io::{self, Write};

use crate::localization;

pub mod game;
pub mod score;

pub fn render_game(game: &game::Game, score: &score::Score) {
    // Print ANSI to clear the screen
    println!("\x1B[2J");
    // Print the score
    let screen_width = 30;
    let padding = (screen_width - 13) / 2;
    for _ in 0..padding {
        print!("-");
    }
    print!("  X {} - {} O  ", score.x_wins, score.o_wins);
    for _ in 0..padding {
        print!("-");
    }
    println!();
    println!();
    for i in 0..3 {
        let row: [game::movement::Move; 3] = game.board[i];
        let pos0: String = row[0].to_string();
        let text0 = if pos0 == " " {
            (i * 3 + 1).to_string()
        } else {
            pos0
        };
        let pos1: String = row[1].to_string();
        let text1 = if pos1 == " " {
            (i * 3 + 2).to_string()
        } else {
            pos1
        };
        let pos2: String = row[2].to_string();
        let text2 = if pos2 == " " {
            (i * 3 + 3).to_string()
        } else {
            pos2
        };
        // Print spaces to center the board
        for _ in 0..10 {
            print!(" ");
        }
        println!("{} | {} | {}", text0, text1, text2);
    }
    println!();
    if game.winner.is_none() {
        // Print the current player
        localization::print(localization::resource::Resource::NowPlaying);
        println!(": {}", game.current_player);
    } else {
        println!();
    }
    // Print the status message
    if let Some(message) = &game.message {
        println!("{}", message);
    }
}

pub fn ask_input() {
    localization::print(localization::resource::Resource::EnterNumber);
    print!(" > ");
    io::stdout().flush().unwrap();
}

pub fn update_game(input: String, game: &game::Game) -> Result<Game, String> {
    let number = input.trim().parse::<usize>().unwrap_or(10);
    match number {
        1..=9 => Game::play(game, number),
        _ => Result::Err(localization::get_localizad_string(
            localization::resource::Resource::InvalidInput,
        )),
    }
}

pub fn check_winner(game: &game::Game) -> Option<game::state::State> {
    game.winner
}

pub fn update_score(winner: game::state::State, score: &score::Score) -> Option<score::Score> {
    match winner {
        game::state::State::XWon => Some(score::Score {
            x_wins: score.x_wins + 1,
            o_wins: score.o_wins,
        }),
        game::state::State::OWon => Some(score::Score {
            x_wins: score.x_wins,
            o_wins: score.o_wins + 1,
        }),
        _ => None,
    }
}

pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
