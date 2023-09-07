use derive_more::{Deref, DerefMut};

pub use super::game_error::GameError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Ex,
    Oh,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameState {
    Ongoing,
    Won(Player),
    Drawn,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SquareState {
    // marked by a player
    Marked(Player),
    // not empty, but belongs to no player and cannot be marked
    Closed,
    // empty, can be marked by a player
    Empty,
}

// what it means to be a square in a Tic-tac-toe-like game
trait TTTSquare {
    fn state(&self) -> SquareState;
}

#[derive(Copy, Clone, Deref, DerefMut)]
struct Board<T: TTTSquare> ([[T; 3]; 3]);

// Contains logic for obtaining the current state of a Tic-tac-toe-like game.
// Since it's generic over TTTSquares, we can use it both for regular Tic-tac-toe
// boards and Ultimate-tic-tac-toe boards
impl<S: TTTSquare> Board<S> {
    fn eval(&self) -> GameState {
        if self.won_by(Player::Ex) {
            GameState::Won(Player::Ex)
        } else if self.won_by(Player::Oh) {
            GameState::Won(Player::Oh)
        } else if self.is_full() {
            GameState::Drawn
        } else {
            GameState::Ongoing
        }
    }

    fn won_by(&self, player: Player) -> bool {
        // checking rows
        for i in 0..3 {
            let mut won_row = true;
            for j in 0..3 {
                won_row &= self[i][j].state() == SquareState::Marked(player);
            }
            if won_row {
                return true;
            }
        }

        // checking cols
        for i in 0..3 {
            let mut won_col = true;
            for j in 0..3 {
                won_col &= self[j][i].state() == SquareState::Marked(player);
            }
            if won_col {
                return true;
            }
        }

        // checking diag
        let is_diag_won  = (0..3).all(|idx| 
            self[idx][idx].state() == SquareState::Marked(player)
        );
            
        // checking anti-diag
        let is_antidiag_won  = (0..3).all(|idx| 
            self[idx][2 - idx].state() == SquareState::Marked(player)
        );

        is_diag_won || is_antidiag_won
    }

    fn is_full(&self) -> bool {
        self.iter()
            .flatten()
            .all(|square| square.state() != SquareState::Empty)
    }
}

impl TTTSquare for Option<Player> {
    fn state(&self) -> SquareState {
        match *self {
            Some(p) => SquareState::Marked(p),
            None    => SquareState::Empty,
        }
    }
}

#[derive(Copy, Clone)]
struct TicTacToe {
    board: Board<Option<Player>>,
    game_state: GameState,
}

enum TTTError {
    InvalidIndex,
    NonEmptySquare(Player),
    GameOver,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: Board([[None; 3]; 3]),
            game_state: GameState::Ongoing,
        }
    }

    fn place(&mut self, player: Player, idx: usize) -> Result<(), TTTError> {
        if self.game_state != GameState::Ongoing {
            return Err(TTTError::GameOver);
        } else if idx >= 9 {
            return Err(TTTError::InvalidIndex);
        }

        let row = idx / 3;
        let col = idx % 3;

        if let Some(resident) = self.board[row][col] {
            Err(TTTError::NonEmptySquare(resident))
        } else {
            self.board[row][col] = Some(player);
            // updating the game state
            self.game_state = self.board.eval();
            Ok(())
        }
    }
}

impl TTTSquare for TicTacToe {
    fn state(&self) -> SquareState {
        match self.game_state {
            GameState::Ongoing => SquareState::Empty,
            GameState::Won(p)  => SquareState::Marked(p),
            GameState::Drawn   => SquareState::Closed,
        }
    }
}

pub struct UltimateTicTacToe {
    board: Board<TicTacToe>,
    pub focus: Option<usize>,
    pub game_state: GameState,
}

// in the range of 1..=9
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoardPos {
    WithFocus(usize, usize),
    WithoutFocus(usize),
}

impl BoardPos {
    fn is_illegal(&self) -> bool {
        match *self {
            BoardPos::WithFocus(f, s) => f == 0 || f > 9 || s == 0 || s > 9,
            BoardPos::WithoutFocus(s) => s == 0 || s > 9,
        }
    }
}

impl std::default::Default for UltimateTicTacToe {
    fn default() -> Self {
        UltimateTicTacToe {
            board: Board([[TicTacToe::new(); 3]; 3]),
            focus: None,
            game_state: GameState::Ongoing,
        }
    }
}

impl UltimateTicTacToe {
    pub fn new() -> Self {
        UltimateTicTacToe {
            board: Board([[TicTacToe::new(); 3]; 3]),
            focus: None,
            game_state: GameState::Ongoing,
        }
    }

    fn loc_from(&self, pos: BoardPos) -> Result<(usize, usize), GameError> {
        // converting from 1-based indexing to 0-based as well
        match (self.focus, pos) {
            (Some(f), BoardPos::WithoutFocus(i)) => Ok((f, i - 1)),
            (None,    BoardPos::WithFocus(f, i)) => Ok((f - 1, i - 1)),
            _ => Err(GameError::IncorrectInputVariant),
        }
    }

    pub fn place(&mut self, player: Player, pos: BoardPos) -> Result<(), GameError> {
        if self.game_state != GameState::Ongoing {
            return Err(GameError::GameOver);
        } else if pos.is_illegal() {
            return Err(GameError::IllegalIndex);
        }

        let (focus, square) = self.loc_from(pos)?;
        let sub_ttt = &mut self.board[focus / 3][focus % 3];

        match sub_ttt.place(player, square) {
            Err(TTTError::NonEmptySquare(p)) => Err(GameError::SquareNotEmpty(p)),
            Err(TTTError::GameOver) => Err(GameError::SquareNotOpen),
            Err(TTTError::InvalidIndex) => Err(GameError::IllegalIndex),
            Ok(()) => {
                self.game_state = self.board.eval();
                let next = self.board[square / 3][square % 3];
                self.focus = if next.state() == SquareState::Empty {
                    Some(square)
                } else {
                    None
                };
                Ok(())
            }
        }
    }
}

//
// Display Stuff
//

use std::fmt;
use itertools::Itertools;

const SMALL_LINE: &'static str = "---+---+--- # ---+---+--- # ---+---+---";
const BIG_LINE: &'static str =   "#######################################";

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Ex => write!(f, " X "),
            Player::Oh => write!(f, " O "),
        }
    }
}

impl TicTacToe {
    pub fn get_line(&self, idx: usize) -> String {
        assert!(idx < 3);

        self.board[idx]
            .iter()
            .map(|e| 
                if let Some(p) = e {
                    p.to_string()
                } else {
                    "   ".to_string()
                }
            )
            .format("|")
            .to_string()
    }
}

impl UltimateTicTacToe {
    pub fn board_as_string(&self) -> String {
        let mut display = String::new();
        for row_idx in 0..3 {
            for line_idx in 0..3 {
                for col_idx in 0..3 {
                    let game = &self.board[row_idx][col_idx];
                    display.push_str(&game.get_line(line_idx));
                    
                    if col_idx != 2 {
                        display.push_str(" # ");
                    }
                }
                display.push('\n');

                if line_idx != 2 {
                    display.push_str(SMALL_LINE);
                    display.push('\n');
                }
            }
            if row_idx != 2 {
                display.push_str(BIG_LINE);
                display.push('\n');
            }
        }
        display
    }
}
                


