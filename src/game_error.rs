use super::game::Player;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameError {
    // we needed the other input
    IncorrectInputVariant,
    // indices are invalid
    IllegalIndex,
    SquareNotOpen,
    SquareNotEmpty(Player),
    GameOver,
}

