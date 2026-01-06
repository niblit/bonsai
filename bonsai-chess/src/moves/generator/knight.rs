use crate::{
    board::BoardBackend,
    moves::Ply,
    moves::generator::{
        directions::{
            L_DOWN_LEFT, L_DOWN_RIGHT, L_LEFT_DOWN, L_LEFT_UP, L_RIGHT_DOWN, L_RIGHT_UP, L_UP_LEFT,
            L_UP_RIGHT,
        },
        sliding::slide,
    },
    pieces::LocatedPiece,
};

/// Generates pseudo-legal moves for a Knight.
///
/// The Knight moves in an "L" shape: two squares in a cardinal direction, then one
/// square perpendicular to that.
///
/// # Unique Properties
/// * **Jumping**: The Knight is the only piece that can jump over other pieces.
///   The `slide` function handles this naturally when the `distance` is set to 1,
///   as it only checks the *destination* square for occupancy, ignoring the path.
///
/// # Movement Logic
/// * **Directions**: 8 possible L-shapes.
/// * **Distance**: 1 (It "teleports" to the target square).
pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend) -> Vec<Ply> {
    let directions = [
        L_UP_LEFT,
        L_UP_RIGHT,
        L_DOWN_LEFT,
        L_DOWN_RIGHT,
        L_LEFT_UP,
        L_LEFT_DOWN,
        L_RIGHT_UP,
        L_RIGHT_DOWN,
    ];
    slide(what_to_move, 1, &directions, backend)
}
