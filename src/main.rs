use std::io::{self, Write};

use yansi::Paint;

/// Represents the position of a tile on the grid
#[derive(Debug, Clone, PartialEq)]
struct TilePos {
    row: usize,
    col: usize,
}

impl TilePos {
    fn to_string(&self) -> String {
        format!("{}{}", (b'A' + self.col as u8) as char, self.row + 1)
    }
}

/// Represents the different colors/types of pieces
#[derive(Debug, Clone)]
enum Piece {
    X,
    O,
}

/// A non-empty grid with rows and columns of tables
#[derive(Debug, Default)]
struct Grid {
    /// The tiles of the grid, stored row-by-row. Each tile is either empty (`None`), or contains
    /// a single `Piece`.
    ///
    /// `tiles[r]` represents row r
    /// `tiles[r][c]` represents the tile at row r and column c
    tiles: [[Option<Piece>; 8]; 8],
}

impl Grid {
    /// Returns true if the grid is completely full (no empty tiles left)
    fn is_full(&self) -> bool {
        for row in &self.tiles {
            for tile in row {
                if tile.is_none() {
                    return false;
                }
            }
        }

        return true;
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
    fn rows(&self) -> &[[Option<Piece>; 8]] {
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
        let mut grid = Grid::default();
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

    /// Returns the grid
    fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Returns the current player
    fn current_player(&self) -> Piece {
        self.current_player.clone()
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

    /// Skips the turn of the current player, leave the board unmodified
    fn skip_turn(&mut self) {
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
    let grid = game.grid();

    print_cell(Paint::new(" "));
    for col_i in 0..grid.row_len() {
        print_cell(Paint::new(&format!("{}", (b'A' + col_i as u8) as char)));
    }
    println!();

    print_row_sep(grid.row_len());

    for (row, row_tiles) in grid.rows().iter().enumerate() {
        print_cell(Paint::new(&format!("{}", row+1)));
        for (col, tile) in row_tiles.iter().enumerate() {
            print_tile(tile, valid_moves.contains(&TilePos {row, col}));
        }
        println!();

        print_row_sep(grid.row_len());
    }
}

fn print_tile(tile: &Option<Piece>, is_valid_move: bool) {
    match tile {
        Some(piece) => print_cell(format_piece(piece.clone())),

        None if is_valid_move => print_cell(Paint::yellow("\u{25CB}")),
        None => print_cell(Paint::new(" ")),
    }
}

fn format_piece(piece: Piece) -> Paint<&'static str> {
    match piece {
        Piece::X => Paint::red("\u{25CF}"),
        Piece::O => Paint::blue("\u{25CF}"),
    }
}

fn print_cell(value: Paint<&str>) {
    print!(" {} \u{2502}", value);
}

fn print_row_sep(cols: usize) {
    const CELL_SIZE: usize = 4;

    for _ in 0..=cols {
        for _ in 0..CELL_SIZE {
            print!("\u{2500}");
        }
    }
    println!();
}

#[derive(Debug)]
enum ParseError {
    EndOfInput,
    InvalidInput(String),
    IOError(io::Error),
}

/// Parses a move from an input string in the format "A1" or "1A" where "A" is the column and "1"
/// is the row. The move string is not case-sensitive.
fn parse_move(line: String) -> Result<TilePos, ParseError> {
    fn byte_to_usize(byte: u8, start: u8) -> usize {
        (byte - start) as usize
    }

    let bytes = line.as_bytes();
    // Leave off the newline when matching
    match &bytes[0..bytes.len()-1] {
        [b'A' ..= b'H', b'1' ..= b'8'] => Ok(TilePos {
            row: byte_to_usize(bytes[1], b'1'),
            col: byte_to_usize(bytes[0], b'A'),
        }),
        [b'a' ..= b'h', b'1' ..= b'8'] => Ok(TilePos {
            row: byte_to_usize(bytes[1], b'1'),
            col: byte_to_usize(bytes[0], b'a'),
        }),
        [b'1' ..= b'8', b'A' ..= b'H'] => Ok(TilePos {
            row: byte_to_usize(bytes[0], b'1'),
            col: byte_to_usize(bytes[1], b'A'),
        }),
        [b'1' ..= b'8', b'a' ..= b'h'] => Ok(TilePos {
            row: byte_to_usize(bytes[0], b'1'),
            col: byte_to_usize(bytes[1], b'a'),
        }),

        _ => Err(ParseError::InvalidInput(line)),
    }
}

/// Repeatedly prompt for the move until a valid one is returned or EOF is recieved
fn prompt_move(valid_moves: &[TilePos]) -> Result<TilePos, ParseError> {
    loop {
        let line = prompt("Enter your move (e.g. A1): ").map_err(ParseError::IOError)?;
        if line.is_empty() {
            // Reached EOF, quit
            break Err(ParseError::EndOfInput);
        }

        match parse_move(line) {
            Ok(pmove) => {
                if !valid_moves.contains(&pmove) {
                    println!("Invalid move: `{}`. Your move must flip at least one tile.\n", pmove.to_string());
                    continue;
                }

                return Ok(pmove);
            },

            Err(ParseError::InvalidInput(inp)) => println!("Invalid input: `{}`. Enter something like 'A1'.\n", inp.trim_end_matches('\n')),
            err@Err(ParseError::EndOfInput) |
            err@Err(ParseError::IOError(_)) => return err,
        }
    }
}

fn prompt(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    // Need to flush because output is line buffered
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    Ok(line)
}

fn main() {
    let mut game = Reversi::new();

    let mut skipped = false;
    loop {
        let (x_score, o_score) = game.scores();
        let valid_moves = game.valid_moves();

        // If the grid is full or the turn is skipped twice, the game ends
        if game.grid().is_full() || (skipped && valid_moves.is_empty()) {
            // Game has been completed
            println!();
            print_game(&game, &valid_moves);
            println!();
            println!("Score: {} {} | {} {}", format_piece(Piece::X), x_score, format_piece(Piece::O), o_score);

            use std::cmp::Ordering::*;
            match x_score.cmp(&o_score) {
                Greater => println!("The winner is: {}", format_piece(Piece::X)),
                Less => println!("The winner is: {}", format_piece(Piece::O)),
                Equal => println!("The game ended with a tie"),
            }

            break;
        }

        println!();
        print_game(&game, &valid_moves);
        println!();
        println!("Score: {} {} | {} {}", format_piece(Piece::X), x_score, format_piece(Piece::O), o_score);
        println!("The current piece is: {}", format_piece(game.current_player()));

        if valid_moves.is_empty() {
            prompt("No moves available. Skipping turn. Press enter to continue...").unwrap();
            skipped = true;
            game.skip_turn();
            continue;
        }
        // If the previous turn was skipped, we can reset that now
        skipped = false;

        let pmove = prompt_move(&valid_moves);
        match pmove {
            Ok(pmove) => game.make_move(pmove),

            Err(ParseError::EndOfInput) => {
                // Print a final newline
                println!();
                break;
            },

            Err(ParseError::InvalidInput(_)) => unreachable!(),

            Err(ParseError::IOError(err)) => {
                eprintln!("Error: {}", err);
                break;
            },
        }
    }
}
