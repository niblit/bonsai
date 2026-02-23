//! # Bishop Move Generator
//!
//! This module contains the move generation logic for Bishops.
//! Bishops are sliding pieces that can move any number of squares diagonally
//! until they hit the edge of the board, are blocked by a friendly piece,
//! or capture an enemy piece. Because they only move diagonally, a Bishop
//! is permanently restricted to squares of the same color as its starting square.

use crate::{
    board::BoardBackend,
    moves::{LegalityContext, Ply, directions, generator::sliding::slide},
    pieces::LocatedPiece,
};

/// Generates strictly legal moves for a Bishop.
///
/// The Bishop moves any number of squares along the diagonals, stopping at the first
/// obstacle. It remains bound to squares of the color it started on.
///
/// This function relies on the shared `slide` utility, passing it the
/// 4 diagonal directions and a maximum slide distance of 7 squares. It respects
/// all board boundaries, piece blocking, pins, and checks.
///
/// # Movement Logic
/// * **Directions**: Four diagonals (Up-Left, Up-Right, Down-Left, Down-Right).
/// * **Distance**: Unlimited (up to 7 squares).
///
/// # Arguments
///
/// * `what_to_move` - The Bishop being moved and its starting location.
/// * `backend` - The board state used to check for occupancy and captures.
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
        7,
        &directions::DIAGONAL_DIRECTIONS,
        backend,
        context,
        buffer,
    );
}
