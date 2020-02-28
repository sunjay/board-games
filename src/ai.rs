use rand::{thread_rng, Rng, rngs::ThreadRng, seq::SliceRandom};

use crate::{Reversi, TilePos, Piece};

/// Returns a move for the current player computed automatically
pub fn compute_ai_move(game: &Reversi, valid_moves: &[TilePos]) -> TilePos {
    enum AIType {
        Random,
        Negamax,
    }

    let mut rng = thread_rng();
    match AIType::Negamax {
        AIType::Random => random_ai(&mut rng, game, valid_moves),
        AIType::Negamax => negamax_ai(&mut rng, game, valid_moves),
    }
}

/// Randomly chooses a move from the set of valid moves
fn random_ai(rng: &mut ThreadRng, _game: &Reversi, valid_moves: &[TilePos]) -> TilePos {
    *valid_moves.choose(rng).expect("bug: no valid moves to choose from")
}

/// Chooses a move based on the negamax algorithm
fn negamax_ai(rng: &mut ThreadRng, game: &Reversi, valid_moves: &[TilePos]) -> TilePos {
    let (pmove, _score) = negamax(rng, game, valid_moves, false, 0);
    pmove.unwrap()
}

/// The negamax algorithm
///
/// Based on: https://en.wikipedia.org/wiki/Negamax
fn negamax(
    rng: &mut ThreadRng,
    game: &Reversi,
    valid_moves: &[TilePos],
    skipped: bool,
    depth: usize,
) -> (Option<TilePos>, i32) {
    const MAX_DEPTH: usize = 4;

    if depth >= MAX_DEPTH || game.grid().is_full() || (skipped && valid_moves.is_empty()) {
        let score = negamax_score(rng, game, game.current_player());
        return (None, score);
    }

    // No valid moves, so skip the turn
    if valid_moves.is_empty() {
        let mut mgame = game.clone();
        mgame.advance_turn();
        let mvalid_moves = mgame.valid_moves();
        return negamax(rng, &mgame, &mvalid_moves, true, depth + 1);
    }

    let mut max_move = None;
    let mut max_score = i32::min_value();
    for &pmove in valid_moves {
        let mut mgame = game.clone();
        mgame.make_move(pmove);
        let mvalid_moves = mgame.valid_moves();

        // Skipped is always false because we just made a move
        let (_, score) = negamax(rng, &mgame, &mvalid_moves, false, depth + 1);
        // Negate score because the returned score is from the perspective of the opponent
        // We want to find the score that is *lowest* from their perspective
        let score = -score;
        if score > max_score {
            max_move = Some(pmove);
            max_score = score;
        }
    }

    (max_move, max_score)
}

/// Computes the negamax score for the given player. A higher score means that the current state of
/// the board is better for the given player.
fn negamax_score(rng: &mut ThreadRng, game: &Reversi, player: Piece) -> i32 {
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

    // Adds the given value to the score. Setting the sign of the value based on whether the piece
    // this value is being awarded for is the current player or the opponent.
    let mut add_score = |piece: Piece, value: i32| if piece == player {
        score += value;
    } else {
        score -= value;
    };

    let grid = game.grid();

    // Adds to score based on the piece at the given position (if any)
    let mut add_tile_score = |pos, value| if let Some(piece) = grid.tile(pos) {
        add_score(piece, value);
    };

    let nrows = grid.col_len();
    let ncols = grid.row_len();

    let corners = &[
        TilePos {row: 0, col: 0},
        TilePos {row: 0, col: ncols - 1},
        TilePos {row: nrows - 1, col: 0},
        TilePos {row: nrows - 1, col: ncols - 1},
    ];
    for &corner in corners {
        add_tile_score(corner, CORNER_BONUS);
    }

    for row in 0..nrows {
        let side = TilePos {row, col: 0};
        add_tile_score(side, SIDE_BONUS);

        let side = TilePos {row, col: ncols - 1};
        add_tile_score(side, SIDE_BONUS);
    }

    for col in 0..ncols {
        let side = TilePos {row: 0, col};
        add_tile_score(side, SIDE_BONUS);

        let side = TilePos {row: nrows - 1, col};
        add_tile_score(side, SIDE_BONUS);
    }

    // A perfectly deterministic AI is pretty boring...
    let score_error = rng.gen_range(-100, 100);

    score + score_error
}
