use crate::{Grid, Piece, TilePos};

#[derive(Debug, Clone)]
pub struct Reversi {
    grid: Grid,
    /// The player whose turn it is currently
    current_player: Piece,
    /// The valid moves for the current player
    valid_moves: Vec<TilePos>,
}

impl Default for Reversi {
    /// Creates a new reversi game with the default pieces placed
    fn default() -> Self {
        let mut grid = Grid::default();
        // The default piece are placed in a 2x2 grid of alternating colors
        grid.place(TilePos {row: 3, col: 3}, Piece::X);
        grid.place(TilePos {row: 3, col: 4}, Piece::O);
        grid.place(TilePos {row: 4, col: 3}, Piece::O);
        grid.place(TilePos {row: 4, col: 4}, Piece::X);

        // X always goes first
        let current_player = Piece::X;
        let valid_moves = compute_valid_moves(&grid, current_player);

        Self {
            grid,
            current_player,
            valid_moves,
        }
    }
}

impl Reversi {
    /// Returns the grid
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Returns the current player
    pub fn current_player(&self) -> Piece {
        self.current_player
    }

    /// Returns the current scores for each player as a tuple: (x score, o score)
    pub fn scores(&self) -> (u32, u32) {
        let mut x_score = 0;
        let mut o_score = 0;

        for row in self.grid.rows() {
            for tile in row {
                match tile {
                    Some(Piece::X) => x_score += 1,
                    Some(Piece::O) => o_score += 1,
                    None => {},
                }
            }
        }

        (x_score, o_score)
    }

    /// Returns all valid moves for the current player
    pub fn valid_moves(&self) -> &[TilePos] {
        &self.valid_moves
    }

    /// Advances the turn by changing the current player, leave the board unmodified
    pub fn advance_turn(&mut self) {
        self.current_player = self.current_player.opposite();
        self.valid_moves = compute_valid_moves(self.grid(), self.current_player);
    }

    /// Places a tile for the current player at the given position, updating any surrounding tiles
    /// that were affected by this move.
    ///
    /// # Panics
    ///
    /// Panics if the move is not valid for the current player.
    pub fn make_move(&mut self, pos: TilePos) {
        let flips = compute_flips(self.grid(), self.current_player, pos);
        assert!(!flips.is_empty(), "bug: attempt to make a move that would result in zero flips");

        let player = self.current_player();
        for flip_pos in flips {
            self.grid.place(flip_pos, player.clone());
        }
        self.grid.place(pos.clone(), player.clone());

        self.advance_turn();
    }
}

fn compute_valid_moves(grid: &Grid, player: Piece) -> Vec<TilePos> {
    // Algorithm: Find all tiles that are empty and would result in at least one flip if the
    // current piece was placed there.

    let mut valid_moves = Vec::new();
    for (row, row_tiles) in grid.rows().iter().enumerate() {
        for (col, tile) in row_tiles.iter().enumerate() {
            // Only empty tiles can be valid moves
            if tile.is_some() {
                continue;
            }

            let pmove = TilePos {row, col};
            if !compute_flips(grid, player, pmove).is_empty() {
                valid_moves.push(pmove);
            }
        }
    }

    valid_moves
}

/// Computes the tiles that would have to flip if the current piece was placed at the given
/// position
fn compute_flips(grid: &Grid, player: Piece, pos: TilePos) -> Vec<TilePos> {
    // Algorithm: Search each of the 8 cardinal directions. A tile is considered a valid move
    // if it is empty and if while searching in a direction we find at least one opponent piece
    // and then a player piece with no empty tiles in between. The "flips" are all opponent
    // pieces found between the given tile and another tile belonging to the player.
    //
    // For player = x, opponent = o,
    //     Finding "oooox" is a valid move for x
    //     Finding "oo" is *not* a valid move for x
    //     Finding "oo x" is *not* a valid move for x
    //     Finding "x" is *not* a valid move for x

    debug_assert!(grid.tile(pos).is_none(),
        "bug: cannot compute flips for a tile that is non-empty");

    let opponent = player.opposite();

    let nrows = grid.col_len() as isize;
    let ncols = grid.row_len() as isize;

    let mut flips = Vec::new();
    let directions = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for &(drow, dcol) in directions {
        // Opponents that can potentially be flipped
        let mut found_opponents = Vec::new();
        for i in 1.. {
            let row = pos.row as isize + drow * i;
            let col = pos.col as isize + dcol * i;
            if row >= 0 && row < nrows && col >= 0 && col < ncols {
                let current_pos = TilePos {
                    row: row as usize,
                    col: col as usize,
                };

                match grid.tile(current_pos) {
                    Some(piece) => {
                        if piece == opponent {
                            found_opponents.push(current_pos);

                        } else if piece == player {
                            // If we didn't find any opponent pieces, this will not add any flips
                            flips.extend(found_opponents);
                            // Stop searching
                            break;
                        }
                    },

                    // Found empty, stop searching and do not add found opponents
                    None => break,
                }

            } else {
                // hit one of the boundaries of the board
                break;
            }
        }
    }

    flips
}
