//! # Queen Move Generator
//!
//! This module contains the move generation logic for Queens.
//! Because a Queen's movement is simply a combination of a Rook's orthogonal
//! movement and a Bishop's diagonal movement, this generator simply delegates
//! the logic to the respective sub-modules for those pieces.

use crate::{
    board::BoardBackend,
    moves::{
        LegalityContext, Ply,
        generator::{bishop, rook},
    },
    pieces::LocatedPiece,
};

/// Generates strictly legal moves for a Queen.
///
/// The Queen combines the power of the Rook and the Bishop, moving any number of
/// squares orthogonally or diagonally. This function respects all board boundaries,
/// friendly piece blocking, enemy captures, pins, and checks.
///
/// # Implementation
///
/// This function aggregates the results from:
/// * [`rook::legal_moves`]
/// * [`bishop::legal_moves`]
///
/// # Arguments
///
/// * `what_to_move` - The Queen being moved and its starting location.
/// * `backend` - The board state used to check for occupancy and captures.
/// * `context` - The pre-calculated legality constraints (pins, checks, and danger squares).
/// * `buffer` - A mutable vector where the generated [`Ply`] instances will be appended.
pub fn legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    rook::legal_moves(what_to_move, backend, context, buffer);
    bishop::legal_moves(what_to_move, backend, context, buffer);
}
