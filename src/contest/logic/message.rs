#[derive(Debug, Clone, Copy)]
pub enum Message {
    Play(usize), // The square where the player wants to play
    NewGame,
    Exit,
}
