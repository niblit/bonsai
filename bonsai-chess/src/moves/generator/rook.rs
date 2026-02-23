//! # Rook Move Generator
//!
//! This module contains the move generation logic for Rooks.
//! Rooks are sliding pieces that can move any number of squares orthogonally
//! (up, down, left, right) until they hit the edge of the board, are blocked
//! by a friendly piece, or capture an enemy piece.

use crate::{
    board::BoardBackend,
    moves::{
        LegalityContext, Ply,
        generator::{directions, sliding},
    },
    pieces::LocatedPiece,
};

/// Generates strictly legal moves for a Rook.
///
/// This function relies on the shared [`sliding::slide`] utility, passing it the
/// 4 orthogonal directions and a maximum slide distance of 7 squares (the maximum
/// possible distance across a standard 8x8 board). It respects all board boundaries,
/// piece blocking, pins, and checks.
///
/// # Arguments
///
/// * `what_to_move` - The Rook being moved and its starting location.
/// * `backend` - The board state used to check for occupancy and captures.
/// * `context` - The pre-calculated legality constraints (pins, checks, and danger squares).
/// * `buffer` - A mutable vector where the generated [`Ply`] instances will be appended.
pub fn legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    sliding::slide(
        what_to_move,
        7,
        &directions::ORTHOGONAL_DIRECTIONS,
        backend,
        context,
        buffer,
    );
}
