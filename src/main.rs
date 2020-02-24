use std::thread;
use std::time::Duration;
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
#[derive(Debug, Clone, PartialEq, Eq)]
enum Piece {
    X,
    O,
}

impl Piece {
    /// Returns the piece opposite to this piece
    fn opposite(&self) -> Self {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

/// A non-empty grid with rows and columns of tables
#[derive(Debug, Default, Clone)]
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

        true
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

    /// Returns the tile at the given position
    fn tile(&self, pos: &TilePos) -> &Option<Piece> {
        &self.tiles[pos.row][pos.col]
    }

    /// Places the given piece on the tile at the given position, overwriting the piece that was
    /// previously at that position (if any)
    ///
    /// # Panics
    ///
    /// This method panics if the position is outside the boundary of the board
    fn place(&mut self, pos: TilePos, piece: Piece) {
        self.tiles[pos.row][pos.col] = Some(piece);
    }
}

#[derive(Debug, Clone)]
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
        // Algorithm: Find all tiles that are empty and would result in at least one flip if the
        // current piece was placed there.

        let mut valid_moves = Vec::new();
        for (row, row_tiles) in self.grid().rows().iter().enumerate() {
            for (col, tile) in row_tiles.iter().enumerate() {
                // Only empty tiles can be valid moves
                if tile.is_some() {
                    continue;
                }

                let pmove = TilePos {row, col};
                if !self.compute_flips(&pmove).is_empty() {
                    valid_moves.push(pmove);
                }
            }
        }

        valid_moves
    }

    /// Advances the turn by changing the current player, leave the board unmodified
    fn advance_turn(&mut self) {
        self.current_player = self.current_player.opposite();
    }

    /// Places a tile for the current player at the given position, updating any surrounding tiles
    /// that were affected by this move.
    ///
    /// # Panics
    ///
    /// Panics if the move is not valid for the current player.
    fn make_move(&mut self, pos: &TilePos) {
        let flips = self.compute_flips(pos);
        assert!(!flips.is_empty(), "bug: attempt to make a move that would result in zero flips");

        let player = self.current_player();
        for flip_pos in flips {
            self.grid.place(flip_pos, player.clone());
        }
        self.grid.place(pos.clone(), player.clone());

        self.advance_turn();
    }

    /// Computes the tiles that would have to flip if the current piece was placed at the given
    /// position
    fn compute_flips(&self, pos: &TilePos) -> Vec<TilePos> {
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

        let grid = self.grid();
        debug_assert!(grid.tile(pos).is_none(),
            "bug: cannot compute flips for a tile that is non-empty");

        let player = self.current_player();
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

                    match grid.tile(&current_pos) {
                        Some(piece) => {
                            if *piece == opponent {
                                found_opponents.push(current_pos);

                            } else if *piece == player {
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
}

/// Returns a move for the current player computed automatically
fn compute_ai_move(game: &Reversi, valid_moves: &[TilePos]) -> TilePos {
    enum AIType {
        Random,
        Negamax,
    }

    match AIType::Negamax {
        AIType::Random => random_ai(game, valid_moves),
        AIType::Negamax => negamax_ai(game, valid_moves),
    }
}

/// Randomly chooses a move from the set of valid moves
fn random_ai(_game: &Reversi, valid_moves: &[TilePos]) -> TilePos {
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    valid_moves.choose(&mut rng).expect("bug: no valid moves to choose from").clone()
}

/// Chooses a move based on the negamax algorithm
fn negamax_ai(game: &Reversi, valid_moves: &[TilePos]) -> TilePos {
    let (pmove, _score) = negamax(game, valid_moves, game.current_player(), false, 0);
    pmove.unwrap()
}

fn negamax(
    game: &Reversi,
    valid_moves: &[TilePos],
    player: Piece,
    skipped: bool,
    depth: usize,
) -> (Option<TilePos>, i32) {
    const MAX_DEPTH: usize = 5;

    if depth >= MAX_DEPTH || game.grid().is_full() || (skipped && valid_moves.is_empty()) {
        let score = negamax_score(game, player.clone());
        return (None, score);
    }

    let mut max_move = None;
    let mut max_score = i32::min_value();
    for pmove in valid_moves {
        let mut mgame = game.clone();
        mgame.make_move(pmove);

        let mvalid_moves = mgame.valid_moves();
        let skipped = mvalid_moves.is_empty();

        let (_, score) = negamax(&mgame, &mvalid_moves, player.clone(), skipped, depth + 1);
        if score > max_score {
            max_move = Some(pmove.clone());
            max_score = score;
        }
    }

    (max_move, max_score)
}

/// Computes the negamax score for the given player. A higher score means that the current state of
/// the board is better for the given player.
fn negamax_score(game: &Reversi, player: Piece) -> i32 {
    // Computes the normal score of the game, then awards bonuses for corners and sides. Corners
    // are more important than sides so they get a bigger bonus.
    const CORNER_BONUS: i32 = 4;
    const SIDE_BONUS: i32 = 2;

    let (x_score, o_score) = game.scores();

    let mut score = if player == Piece::X {
        x_score as i32 - o_score as i32
    } else {
        o_score as i32 - x_score as i32
    };

    let grid = game.grid();
    let nrows = grid.col_len();
    let ncols = grid.row_len();

    let corners = &[
        TilePos {row: 0, col: 0},
        TilePos {row: 0, col: ncols - 1},
        TilePos {row: nrows - 1, col: 0},
        TilePos {row: nrows - 1, col: ncols - 1},
    ];
    for corner in corners {
        match grid.tile(corner) {
            Some(piece) => if *piece == player {
                score += CORNER_BONUS;
            } else {
                score -= CORNER_BONUS;
            },

            None => {},
        }
    }

    for row in 0..nrows {
        let side = TilePos {row, col: 0};
        match grid.tile(&side) {
            Some(piece) => if *piece == player {
                score += SIDE_BONUS;
            } else {
                score -= SIDE_BONUS;
            },

            None => {},
        }

        let side = TilePos {row, col: ncols - 1};
        match grid.tile(&side) {
            Some(piece) => if *piece == player {
                score += SIDE_BONUS;
            } else {
                score -= SIDE_BONUS;
            },

            None => {},
        }
    }

    for col in 0..ncols {
        let side = TilePos {row: 0, col};
        match grid.tile(&side) {
            Some(piece) => if *piece == player {
                score += SIDE_BONUS;
            } else {
                score -= SIDE_BONUS;
            },

            None => {},
        }

        let side = TilePos {row: nrows - 1, col};
        match grid.tile(&side) {
            Some(piece) => if *piece == player {
                score += SIDE_BONUS;
            } else {
                score -= SIDE_BONUS;
            },

            None => {},
        }
    }

    score
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

    // Set this variable to control the game type
    //let ai_controlled = &[]; // Human vs Human
    let ai_controlled = &[Piece::O]; // Human vs AI
    //let ai_controlled = &[Piece::X, Piece::O]; // AI vs AI

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

        let player = game.current_player();
        let is_ai = ai_controlled.contains(&player);

        println!();
        print_game(&game, &valid_moves);
        println!();
        println!("Score: {} {} | {} {}", format_piece(Piece::X), x_score, format_piece(Piece::O), o_score);
        println!("The current piece is: {}", format_piece(player));

        if valid_moves.is_empty() {
            if is_ai {
                println!("No moves available. Skipping turn. Press enter to continue...");
            } else {
                prompt("No moves available. Skipping turn. Press enter to continue...").unwrap();
            }

            skipped = true;
            game.advance_turn();
            continue;
        }
        // If the previous turn was skipped, we can reset that now
        skipped = false;

        if is_ai {
            let pmove = compute_ai_move(&game, &valid_moves);
            game.make_move(&pmove);
            // Slow down the game a bit so it's easier to follow
            thread::sleep(Duration::from_millis(200));
            continue;
        }

        let pmove = prompt_move(&valid_moves);
        match pmove {
            Ok(pmove) => game.make_move(&pmove),

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
