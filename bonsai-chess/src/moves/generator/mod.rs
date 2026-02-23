//! # Move Generator
//!
//! This module contains the core logic for calculating valid moves for every
//! piece type on the board. It handles the specific movement patterns of
//! sliding pieces (Queens, Rooks, Bishops), leaping pieces (Knights),
//! step pieces (Kings), and the complex rules governing pawns.

/// Bishop move generation logic (diagonal sliding).
mod bishop;

/// Directional vectors (offsets) used for calculating piece movements.
pub mod directions;

/// King move generation logic, including castling rules.
mod king;

/// Knight move generation logic (L-shaped jumps).
mod knight;

/// Contextual information used to determine if a move is strictly legal (e.g., pin masks, check masks).
mod legality_context;

/// Pawn move generation logic, including en passant, double pushes, and promotion.
mod pawn;

/// Queen move generation logic (combination of straight and diagonal sliding).
mod queen;

/// Rook move generation logic (straight sliding).
mod rook;

/// Shared logic for generating ray attacks for sliding pieces.
mod sliding;

use crate::{
    atoms::{CastlingRights, Coordinates},
    board::BoardBackend,
    moves::Ply,
    pieces::{Kind, LocatedPiece},
};

pub use legality_context::LegalityContext;

/// Calculates all valid moves for a specific piece and pushes them into a buffer.
///
/// This function acts as a dispatcher, routing the generation logic to the appropriate
/// submodule based on the piece's `Kind`. It also provides an early exit for double-checks,
/// as only the King can legally move when under attack by two pieces simultaneously.
///
/// # Arguments
///
/// * `what_to_move`: The piece being evaluated and its current location.
/// * `backend`: The current state of the board grid (used to check occupancy).
/// * `en_passant_target`: The coordinate of a pawn that can be captured en passant (if any).
/// * `castling_rights`: The current castling permissions for both players.
/// * `context`: The pre-calculated data regarding checks and pins to filter illegal moves.
/// * `buffer`: A mutable vector where the generated [`Ply`] instances will be appended.
///
/// # Examples
///
/// ```rust
/// // Conceptual example of generating moves for a single piece:
/// // use bonsai_chess::prelude::*;
/// // use bonsai_chess::moves::generator::{generate_legal_moves, LegalityContext};
/// //
/// // let mut moves = Vec::new();
/// //
/// // // Assuming we have our board state and context already built:
/// // generate_legal_moves(
/// //     located_knight,
/// //     &board_backend,
/// //     None,
/// //     castling_rights,
/// //     &context,
/// //     &mut moves
/// // );
/// //
/// // assert!(!moves.is_empty());
/// ```
pub fn generate_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    en_passant_target: Option<Coordinates>,
    castling_rights: CastlingRights,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    if context.in_double_check() && what_to_move.piece().kind() != Kind::King {
        // Double check: Only the king can move.
        return;
    }

    match what_to_move.piece().kind() {
        Kind::King => king::legal_moves(what_to_move, backend, castling_rights, context, buffer),
        Kind::Queen => queen::legal_moves(what_to_move, backend, context, buffer),
        Kind::Rook => rook::legal_moves(what_to_move, backend, context, buffer),
        Kind::Bishop => bishop::legal_moves(what_to_move, backend, context, buffer),
        Kind::Knight => knight::legal_moves(what_to_move, backend, context, buffer),
        Kind::Pawn => pawn::legal_moves(what_to_move, backend, en_passant_target, context, buffer),
    }
}
