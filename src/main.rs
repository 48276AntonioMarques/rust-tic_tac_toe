use std::io;

mod engine;

fn main() {
    // Initialize game
    let mut game: engine::game::Game = engine::game::Game::new();
    loop {
        // Render game
        engine::render_game(&game);
        let mut awaiting_input = true;
        while awaiting_input {
            // Check for winner
            if let Some(winner) = engine::check_winner(&game) {
                println!("{}!", winner);
                return;
            }
            // Get input from player
            engine::ask_input(&game);
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let input: String = buffer;
            // Update game state
            let update = engine::update_game(input, &game);
            match update {
                Ok(new_game) => {
                    game = new_game;
                    awaiting_input = false;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
