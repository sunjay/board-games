use std::fmt;

/// Represents the position of a tile on the grid
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TilePos {
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for TilePos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (b'A' + self.col as u8) as char, self.row + 1)
    }
}
