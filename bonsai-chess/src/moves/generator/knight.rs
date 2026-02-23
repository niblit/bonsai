//! # Knight Move Generator
//!
//! This module contains the move generation logic for Knights.
//! Knights are unique pieces in chess that move in an "L" shape and are the
//! only pieces capable of jumping over other pieces to reach their destination.

use crate::{
    board::BoardBackend,
    moves::{LegalityContext, Ply, directions, generator::sliding::slide},
    pieces::LocatedPiece,
};

/// Generates strictly legal moves for a Knight.    
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
///
/// # Arguments
///
/// * `what_to_move` - The Knight being moved and its starting location.
/// * `backend` - The board state used to check for occupancy and captures at the destination.
/// * `context` - The pre-calculated legality constraints (pins, checks, and danger squares).
/// * `buffer` - A mutable vector where the generated [`Ply`] instances will be appended.
pub fn legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    slide(
        what_to_move,
        1,
        &directions::KNIGHT_DIRECTIONS,
        backend,
        context,
        buffer,
    );
}
