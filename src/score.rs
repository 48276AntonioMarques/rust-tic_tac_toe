pub struct Score {
    pub x_wins: u32,
    pub o_wins: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            x_wins: 0,
            o_wins: 0,
        }
    }
}
