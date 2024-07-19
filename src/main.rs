mod engine;
mod localization;

use std::io::{self, Write};

fn main() {
    // Main game loop
    let mut score = engine::score::Score::new();
    loop {
        // Initialize game
        let mut game: engine::game::Game = engine::game::Game::new();
        loop {
            // Render game
            engine::render_game(&game, &score);
            // Check for winner
            if let Some(winner) = engine::check_winner(&game) {
                println!("{}", winner);
                let new_score = engine::update_score(winner, &score);
                if let Some(s) = new_score {
                    score = s;
                }
                // Finish current game
                break;
            }
            // Get input from player
            engine::ask_input();
            let input = engine::get_input();
            // Update game state
            let update = engine::update_game(input, &game);
            match update {
                Ok(new_game) => {
                    game = new_game;
                }
                Err(e) => {
                    game.message = Some(e);
                }
            }
        }
        localization::print(localization::resource::Resource::PlayAgain);
        print!(" > ");
        io::stdout().flush().unwrap();
        let input = engine::get_input();
        if input.trim().to_ascii_lowercase() == "x" {
            break;
        }
    }
}
