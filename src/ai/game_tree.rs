use crate::Reversi;

/// Represents the tree of every possible reversi game
///
/// The nodes in the tree are generated on-demand (lazily) as needed.
#[derive(Debug, Clone)]
pub struct GameTree {
    game: Reversi,
    skipped: bool,
}

impl GameTree {
    pub fn new(game: Reversi) -> Self {
        Self {
            game,
            skipped: false,
        }
    }

    pub fn children(&self) -> GameTreeChildren {
        GameTreeChildren {
            root: self,
            next: 0,
        }
    }
}

pub struct GameTreeChildren<'a> {
    root: &'a GameTree,
    next: usize,
}

impl<'a> Iterator for GameTreeChildren<'a> {
    type Item = GameTree;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {root, next} = self;
        let GameTree {game, skipped} = root;
        let valid_moves = game.valid_moves();

        // Check if the game is complete
        if game.grid().is_full() || (*skipped && valid_moves.is_empty()) {
            return None;
        }
        todo!()
    }
}
