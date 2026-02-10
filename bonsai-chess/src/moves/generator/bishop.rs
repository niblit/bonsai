use crate::{
    board::BoardBackend,
    moves::{
        Ply,
        generator::{
            directions::{
                DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT, DIAGONALLY_UP_LEFT,
                DIAGONALLY_UP_RIGHT,
            },
            sliding::slide,
        },
    },
    pieces::LocatedPiece,
};

/// Generates pseudo-legal moves for a Bishop.
///
/// The Bishop moves any number of squares along the diagonals, stopping at the first
/// obstacle. It remains bound to squares of the color it started on.
///
/// # Movement Logic
/// * **Directions**: Four diagonals (Up-Left, Up-Right, Down-Left, Down-Right).
/// * **Distance**: Unlimited (up to 7 squares).
pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend, buffer: &mut Vec<Ply>) {
    let directions = [
        DIAGONALLY_UP_LEFT,
        DIAGONALLY_UP_RIGHT,
        DIAGONALLY_DOWN_LEFT,
        DIAGONALLY_DOWN_RIGHT,
    ];
    slide(what_to_move, 7, &directions, backend, buffer);
}
