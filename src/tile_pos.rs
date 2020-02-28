/// Represents the position of a tile on the grid
#[derive(Debug, Clone, PartialEq)]
pub struct TilePos {
    pub row: usize,
    pub col: usize,
}

impl TilePos {
    pub fn to_string(&self) -> String {
        format!("{}{}", (b'A' + self.col as u8) as char, self.row + 1)
    }
}
