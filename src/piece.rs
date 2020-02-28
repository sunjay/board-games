use std::fmt;

use yansi::Paint;

/// Represents the different colors/types of pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    X,
    O,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::X => write!(f, "{}", Paint::red("\u{25CF}")),
            Piece::O => write!(f, "{}", Paint::blue("\u{25CF}")),
        }
    }
}

impl Piece {
    /// Returns the piece opposite to this piece
    pub fn opposite(self) -> Self {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}
