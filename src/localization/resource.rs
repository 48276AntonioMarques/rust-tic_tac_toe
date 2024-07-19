pub enum Resource {
    PlayAgain,
    NowPlaying,
    EnterNumber,
    InvalidInput,
    InvalidMove,
    Wins,
    Draw,
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
