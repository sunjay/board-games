use crate::{Piece, TilePos};

/// A non-empty grid with rows and columns of tables
#[derive(Debug, Default, Clone)]
pub struct Grid {
    /// The tiles of the grid, stored row-by-row. Each tile is either empty (`None`), or contains
    /// a single `Piece`.
    ///
    /// `tiles[r]` represents row r
    /// `tiles[r][c]` represents the tile at row r and column c
    tiles: [[Option<Piece>; 8]; 8],
}

impl Grid {
    /// Returns true if the grid is completely full (no empty tiles left)
    pub fn is_full(&self) -> bool {
        for row in &self.tiles {
            for tile in row {
                if tile.is_none() {
                    return false;
                }
            }
        }

        true
    }

    /// Returns the length of each row (i.e. the number of columns)
    pub fn row_len(&self) -> usize {
        self.tiles[0].len()
    }

    /// Returns the length of each column (i.e. the number of rows)
    pub fn col_len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns a slice of the tiles of the grid
    pub fn rows(&self) -> &[[Option<Piece>; 8]] {
        &self.tiles
    }

    /// Returns the tile at the given position
    pub fn tile(&self, pos: &TilePos) -> &Option<Piece> {
        &self.tiles[pos.row][pos.col]
    }

    /// Places the given piece on the tile at the given position, overwriting the piece that was
    /// previously at that position (if any)
    ///
    /// # Panics
    ///
    /// This method panics if the position is outside the boundary of the board
    pub fn place(&mut self, pos: TilePos, piece: Piece) {
        self.tiles[pos.row][pos.col] = Some(piece);
    }
}
