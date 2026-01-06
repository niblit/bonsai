use crate::{
    board::BoardBackend,
    moves::Ply,
    moves::generator::{
        directions::{DOWN, LEFT, RIGHT, UP},
        sliding::slide,
    },
    pieces::LocatedPiece,
};

/// Generates pseudo-legal moves for a Rook.
///
/// The Rook moves any number of squares along a rank or file (orthogonally),
/// stopping at the first obstacle. It can capture enemy pieces but cannot jump
/// over friendly ones.
///
/// # Movement Logic
/// * **Directions**: Up, Down, Left, Right.
/// * **Distance**: Unlimited (up to 7 squares).
pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend) -> Vec<Ply> {
    let directions = [UP, DOWN, LEFT, RIGHT];
    slide(what_to_move, 7, &directions, backend)
}
