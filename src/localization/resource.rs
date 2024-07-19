pub enum Resource {
    PlayAgain,
    NowPlaying,
    InvalidMove,
    Wins,
    Draw,
    Exit,
    UnableToStartNewGame,
}

impl Resource {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Clone for Resource {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Resource {}
