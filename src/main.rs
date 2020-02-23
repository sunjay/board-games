use std::io::{self, Write};
use std::error::Error;

/// Represents the position of a tile on the grid
#[derive(Debug, Clone, PartialEq)]
struct TilePos {
    row: usize,
    col: usize,
}

impl TilePos {
    fn to_string(&self) -> String {
        format!("{}{}", (self.col as u8 - b'A') as char, self.row)
    }
}

/// Represents the different colors/types of pieces
#[derive(Debug, Clone)]
enum Piece {
    X,
    O,
}

/// A non-empty grid with rows and columns of tables
#[derive(Debug)]
struct Grid {
    /// The tiles of the grid, stored row-by-row. Each tile is either empty (`None`), or contains
    /// a single `Piece`.
    ///
    /// `tiles[r]` represents row r
    /// `tiles[r][c]` represents the tile at row r and column c
    tiles: Vec<Vec<Option<Piece>>>,
}

impl Grid {
    /// Creates a new grid with the given size
    fn new(rows: usize, cols: usize) -> Self {
        assert_ne!(rows, 0, "bug: grid must be non-empty (rows == 0)");
        assert_ne!(cols, 0, "bug: grid must be non-empty (cols == 0)");

        Self {
            tiles: vec![vec![None; cols]; rows],
        }
    }

    /// Returns the length of each row (i.e. the number of columns)
    fn row_len(&self) -> usize {
        self.tiles[0].len()
    }

    /// Returns the length of each column (i.e. the number of rows)
    fn col_len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns a slice of the tiles of the grid
    fn rows(&self) -> &Vec<Vec<Option<Piece>>> {
        &self.tiles
    }

    /// Returns the positions that are horizontally, vertically, and diagonally adjacent to the
    /// given position
    fn adjacents(&self, pos: TilePos) -> Vec<TilePos> {
        let mut adjs = Vec::new();

        let directions = &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for &(drow, dcol) in directions {
            let row = pos.row as isize + drow;
            let col = pos.col as isize + dcol;
            if row >= 0 && row < self.col_len() as isize && col >= 0 && col < self.row_len() as isize {
                adjs.push(TilePos {
                    row: row as usize,
                    col: col as usize,
                });
            }
        }

        adjs
    }

    /// Places the given piece on the tile at the given position
    ///
    /// # Panics
    ///
    /// This method panics if the position is outside the boundary of the board or if the tile
    /// already contained a piece.
    fn place(&mut self, pos: TilePos, piece: Piece) {
        let tile = &mut self.tiles[pos.row][pos.col];
        assert!(tile.is_none(),
            "bug: attempt to place a piece on a non-empty tile");

        *tile = Some(piece);
    }
}

#[derive(Debug)]
struct Reversi {
    grid: Grid,
    /// The player whose turn it is currently
    current_player: Piece,
}

impl Reversi {
    /// Creates a new reversi game with the default pieces placed
    fn new() -> Self {
        let mut grid = Grid::new(8, 8);
        // The default piece are placed in a 2x2 grid of alternating colors
        grid.place(TilePos {row: 3, col: 3}, Piece::X);
        grid.place(TilePos {row: 3, col: 4}, Piece::O);
        grid.place(TilePos {row: 4, col: 3}, Piece::O);
        grid.place(TilePos {row: 4, col: 4}, Piece::X);

        Self {
            grid,
            // X always goes first
            current_player: Piece::X,
        }
    }

    /// Returns the current scores for each player as a tuple: (x score, o score)
    fn scores(&self) -> (u32, u32) {
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
    fn valid_moves(&self) -> Vec<TilePos> {
        todo!()
    }

    /// Places a tile for the current player at the given position, updating any surrounding tiles
    /// that were affected by this move.
    ///
    /// # Panics
    ///
    /// Panics if the move is not valid for the current player.
    fn make_move(&mut self, pos: TilePos) {
        todo!()
    }
}

fn print_game(game: &Reversi, valid_moves: &[TilePos]) {
}

#[derive(Debug)]
enum ParseError {
    IOError(io::Error),
}

/// Parses a move from an input string in the format "A1" or "1A" where "A" is the column and "1"
/// is the row. The move string is not case-sensitive.
///
/// Returns `Ok(None)` if EOF was received.
/// Returns `Err(...)` if something went wrong.
fn parse_move(line: &str) -> Result<Option<TilePos>, ParseError> {
    todo!()
}

/// Repeatedly prompt for the move until a valid one is returned or EOF is recieved
///
/// See `parse_move` for more information.
fn prompt_move(valid_moves: &[TilePos]) -> Result<Option<TilePos>, ParseError> {
    loop {
        print!("Enter your move (e.g. A1): ");
        // Need to flush because output is line buffered
        io::stdout().flush().map_err(ParseError::IOError)?;

        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(ParseError::IOError)?;

        match parse_move(&line) {
            Ok(Some(pmove)) => {
                if !valid_moves.contains(&pmove) {
                    println!("Invalid move: {}", pmove.to_string());
                    continue;
                }

                return Ok(Some(pmove));
            },

            Ok(None) => break Ok(None),

            err@Err(ParseError::IOError(_)) => return err,
        }
    }
}

fn main() {
    let mut game = Reversi::new();

    loop {
        let valid_moves = game.valid_moves();
        print_game(&game, &valid_moves);
        println!();

        println!("Score: ", );
        println!("The current piece is: ", );

        let pmove = prompt_move(&valid_moves).unwrap();
        match pmove {
            Some(pmove) => game.make_move(pmove),
            None => break,
        }
    }
}
