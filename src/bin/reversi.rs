use std::thread;
use std::time::Duration;

use board_games::{
    Reversi,
    Piece,
    ParseError,
    prompt,
    prompt_move,
    print_game,
    format_piece,
    compute_ai_move,
};

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
