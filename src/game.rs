use crate::localization;

pub mod movement;
pub mod player;
pub mod state;

pub struct Game {
    pub winner: Option<state::State>,
    pub current_player: player::Player,
    pub board: [[movement::Move; 3]; 3],
    pub message: Option<String>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            winner: Option::None,
            current_player: player::Player::X,
            board: [[movement::Move::Empty; 3]; 3],
            message: Option::None,
        }
    }

    pub fn play(&self, number: usize) -> Result<Game, String> {
        // Check if game is over
        if !matches!(self.winner, Option::None) {
            return Result::Ok(Game {
                winner: self.winner,
                current_player: self.current_player,
                board: self.board,
                message: Option::None,
            });
        }
        // Check if input is valid
        let square = self.board[Game::int2row(number)][Game::int2col(number)];
        if !matches!(square, movement::Move::Empty) {
            return Result::Err(localization::get_localizad_string(
                localization::resource::Resource::InvalidMove,
            ));
        }
        // Make move
        let new_square = match self.current_player {
            player::Player::X => movement::Move::X,
            player::Player::O => movement::Move::O,
        };
        // Updating board
        let mut new_board = self.board;
        new_board[Game::int2row(number)][Game::int2col(number)] = new_square;
        let new_game = Game {
            winner: self.winner,
            current_player: self.current_player.switch(),
            board: new_board,
            message: Option::None,
        };
        // Check for winner
        // FIXME: Don't create a new game just to check for the winner
        let winner = new_game.check_winner();
        // Return new game state
        Result::Ok(Game {
            winner,
            current_player: self.current_player.switch(),
            board: new_board,
            message: Option::None,
        })
    }

    fn int2row(number: usize) -> usize {
        (number - 1) / 3
    }

    fn int2col(number: usize) -> usize {
        (number - 1) % 3
    }

    pub fn check_winner(&self) -> Option<state::State> {
        let winning_conditions = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
            [1, 5, 9],
            [3, 5, 7],
        ];
        let board = self.board;
        // Check if all square are empty
        let entries = {
            let mut entries = 9;
            for row in board {
                for square in row {
                    if matches!(square, movement::Move::Empty) {
                        entries -= 1;
                    }
                }
            }
            if entries == 0 {
                return Option::None;
            }
            entries
        };

        for condition in winning_conditions {
            // Check if there's any line
            let row = Game::int2row(condition[0]);
            let col = Game::int2col(condition[0]);
            let square0: movement::Move = board[row][col];
            let row1 = Game::int2row(condition[1]);
            let col1 = Game::int2col(condition[1]);
            let square1 = board[row1][col1];
            let row2 = Game::int2row(condition[2]);
            let col2 = Game::int2col(condition[2]);
            let square2 = board[row2][col2];
            let squares = [square0, square1, square2];
            let winner = squares.into_iter().reduce(|acc, _sqr| match acc {
                movement::Move::X => match _sqr {
                    movement::Move::X => movement::Move::X,
                    _ => movement::Move::Empty,
                },
                movement::Move::O => match _sqr {
                    movement::Move::O => movement::Move::O,
                    _ => movement::Move::Empty,
                },
                _ => movement::Move::Empty,
            });
            match winner {
                Option::Some(movement::Move::X) => return Option::Some(state::State::XWon),
                Option::Some(movement::Move::O) => return Option::Some(state::State::OWon),
                _ => (),
            }
        }
        if entries == 9 {
            Option::Some(state::State::Draw)
        } else {
            Option::None
        }
    }
}
