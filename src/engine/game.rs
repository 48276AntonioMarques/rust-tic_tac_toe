pub mod movement;
pub mod player;

pub struct Game {
    pub winner: Option<player::Player>,
    pub current_player: player::Player,
    pub board: [[movement::Move; 3]; 3],
}

impl Game {
    pub fn new() -> Game {
        Game {
            winner: Option::None,
            current_player: player::Player::X,
            board: [[movement::Move::Empty; 3]; 3],
        }
    }

    pub fn play(&self, number: usize) -> Result<Game, String> {
        if !matches!(self.winner, Option::None) {
            return Result::Ok(Game {
                winner: self.winner,
                current_player: self.current_player,
                board: self.board,
            });
        }
        let square = self.board[Game::int2row(number)][Game::int2col(number)];
        if !matches!(square, movement::Move::Empty) {
            return Result::Err("Invalid move. Square is already taken.".to_string());
        }
        let new_square = match self.current_player {
            player::Player::X => movement::Move::X,
            player::Player::O => movement::Move::O,
        };
        let mut new_board = self.board;
        new_board[Game::int2row(number)][Game::int2col(number)] = new_square;
        let winner = self.check_winner();
        Result::Ok(Game {
            winner,
            current_player: self.current_player.switch(),
            board: new_board,
        })
    }

    fn int2row(number: usize) -> usize {
        (number - 1) / 3
    }

    fn int2col(number: usize) -> usize {
        (number - 1) % 3
    }

    pub fn check_winner(&self) -> Option<player::Player> {
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
        for row in board {
            for square in row {
                if matches!(square, movement::Move::Empty) {
                    return Option::None;
                }
            }
        }

        for condition in winning_conditions {
            let row = Game::int2row(condition[0]);
            let col = Game::int2col(condition[0]);
            let square0: movement::Move = board[row][col];
            let row1 = Game::int2row(condition[1]);
            let col1 = Game::int2col(condition[1]);
            let square1 = board[row1][col1];
            let row2 = Game::int2row(condition[2]);
            let col2 = Game::int2col(condition[2]);
            let square2 = board[row2][col2];
            let squares = [square1, square2];
            let mut all_same = 0;
            for sqr in squares {
                all_same = match sqr {
                    movement::Move::Empty => 0,
                    square0 => all_same + 1,
                    _ => 0,
                }
            }
            if all_same == 2 {
                return match square0 {
                    movement::Move::X => Option::Some(player::Player::X),
                    movement::Move::O => Option::Some(player::Player::O),
                    _ => Option::None,
                };
            }
        }
        Option::None
    }
}
