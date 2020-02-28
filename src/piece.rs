/// Represents the different colors/types of pieces
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Piece {
    X,
    O,
}

impl Piece {
    /// Returns the piece opposite to this piece
    pub fn opposite(&self) -> Self {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}
