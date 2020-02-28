use yansi::Paint;

use crate::{Reversi, TilePos, Piece};

pub fn print_game(game: &Reversi, valid_moves: &[TilePos]) {
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

pub fn format_piece(piece: Piece) -> Paint<&'static str> {
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
