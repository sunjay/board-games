use std::io::{self, Write};

use thiserror::Error;

use crate::{TilePos};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Reached end of input")]
    EndOfInput,
    #[error("Invalid input: `{0}`")]
    InvalidInput(String),
    #[error(transparent)]
    IOError(io::Error),
}

pub fn prompt(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    // Need to flush because output is line buffered
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    Ok(line)
}

/// Repeatedly prompt for the move until a valid one is returned or EOF is recieved
pub fn prompt_move(valid_moves: &[TilePos]) -> Result<TilePos, ParseError> {
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
