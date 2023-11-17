use bincode::{Encode, Decode};

#[derive(Debug, Encode, Decode)]
pub enum GameUpdate {
    // start
    Start,
    // opponent has made this move (implicitly also saying that previous move made was valid)
    Move(BoardPos),
    // client won game, yay
    GameWon,
    // opponent has won with this finishing move
    GameWonByOpponent(BoardPos),
    // game drawn by client's move
    GameDrawn,
    // game drawn by opponent's move
    GameDrawnByOpponent(BoardPos),
    // bad move made
    BadMove,
}

// in the range of 1..=9
#[derive(Debug, Copy, Clone, Encode, Decode)]
pub enum BoardPos {
    WithFocus(usize, usize),
    WithoutFocus(usize),
}

impl BoardPos {
    pub fn is_illegal(&self) -> bool {
        match *self {
            BoardPos::WithFocus(f, s) => f == 0 || f > 9 || s == 0 || s > 9,
            BoardPos::WithoutFocus(s) => s == 0 || s > 9,
        }
    }
}
