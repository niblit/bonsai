//! # King Move Generator
//!
//! This module contains the move generation logic for Kings.
//! The King can step exactly one square in any of the 8 directions (orthogonal
//! and diagonal). Additionally, this module handles the complex rules for
//! castling (both Kingside and Queenside), ensuring the King does not castle
//! out of, through, or into check.

use crate::{
    atoms::{CastlingRights, Coordinates, Team},
    board::BoardBackend,
    moves::{
        CastlingSide, LegalityContext, Ply, SpecialMove, directions, generator::sliding::slide,
    },
    pieces::{Kind, LocatedPiece},
};

/// Generates strictly legal moves for a King.
///
/// The King moves exactly one square in any direction. It cannot step onto
/// squares controlled by enemy pieces (danger squares). Additionally, if
/// conditions are met, it can perform a special "Castling" move.
///
/// This function relies on the shared `slide` utility with a maximum distance
/// of 1, and delegates castling checks to a specialized helper.
///
/// # Arguments
///
/// * `what_to_move` - The King being moved and its starting location.
/// * `backend` - The board state used to check for occupancy and path clearance.
/// * `castling_rights` - The current castling permissions to determine if castling is a candidate.
/// * `context` - The pre-calculated legality constraints (danger squares and current checks).
/// * `buffer` - A mutable vector where the generated [`Ply`] instances will be appended.
pub fn legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    castling_rights: CastlingRights,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    slide(
        what_to_move,
        1,
        &directions::KING_DIRECTIONS,
        backend,
        context,
        buffer,
    );

    if castling_rights != CastlingRights::no_rights() {
        get_castling_moves(what_to_move, backend, castling_rights, context, buffer);
    }
}

/// Helper to generate Castling moves if valid.
///
/// According to FIDE Laws of Chess, castling is valid only if all of the following are true:
/// 1. The King is not currently in check.
/// 2. The player still has the right to castle (neither the King nor the participating Rook has moved).
/// 3. All squares between the King and the Rook are unoccupied.
/// 4. The King does not pass through, nor land on, a square attacked by an enemy piece.
///
/// *Note: For Queenside castling, the b-file square adjacent to the Rook must be empty,
/// but it is permitted to be under attack, as the King does not pass through it.*
///
/// # Arguments
///
/// * `what_to_move` - The King attempting to castle.
/// * `backend` - The board state used to verify empty paths and safe squares.
/// * `castling_rights` - The specific short/long rights for the current player.
/// * `context` - Used to quickly determine if the King is currently in check.
/// * `buffer` - The vector to append the castling `Ply` to if valid.
fn get_castling_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    castling_rights: CastlingRights,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    // File Indices (0-7 for A-H)
    const FILE_B: usize = 1; // Occupancy check on long castle
    const FILE_C: usize = 2; // Long Castle Destination
    const FILE_D: usize = 3; // Long Castle Cross / Rook Destination
    const FILE_F: usize = 5; // Short Castle Cross
    const FILE_G: usize = 6; // Short Castle Destination

    const FILE_A: usize = 0; // where long side rook is
    const FILE_H: usize = 7; // where short side rook is

    let ally = what_to_move.piece().team();
    let enemy = ally.opposite();

    // 1. Cannot castle if currently in check
    if !context.checkers().is_empty() {
        return;
    }

    let castling_row = match ally {
        Team::White => 7,
        Team::Black => 0,
    };

    // Helper to get Coordinate safely
    let to_coordinates =
        |column: usize| -> Coordinates { Coordinates::new(castling_row, column).unwrap() };

    // --- Kingside (Short) ---
    // Checks: Rights + Path Empty (F, G) + Path Safe (F, G)
    let can_castle_short = match ally {
        Team::White => castling_rights.white_king_side(),
        Team::Black => castling_rights.black_king_side(),
    };

    if can_castle_short {
        let f_square = to_coordinates(FILE_F);
        let g_square = to_coordinates(FILE_G);

        // You cannot go through other pieces
        let is_path_clear = backend.get(f_square).is_none() && backend.get(g_square).is_none();

        // FIDE Rule: You cannot castle *through* check.
        let is_path_safe = !backend.is_square_under_attack(f_square, enemy)
            && !backend.is_square_under_attack(g_square, enemy);

        let is_rook_in_place = backend
            .get(Coordinates::new(castling_row, FILE_H).unwrap())
            .is_some_and(|potential_rook| {
                potential_rook.kind() == Kind::Rook && potential_rook.team() == ally
            });

        if is_path_clear && is_path_safe && is_rook_in_place {
            buffer.push(Ply::new(
                what_to_move.position(),
                g_square,
                what_to_move.piece(),
                None,
                Some(SpecialMove::Castle(CastlingSide::Short)),
            ));
        }
    }

    // --- Queenside (Long) ---
    // Checks: Rights + Path Empty (B, C, D) + Path Safe (C, D)
    // Note: B-file must be empty, but does NOT need to be safe from attack.
    let can_castle_long = match ally {
        Team::White => castling_rights.white_queen_side(),
        Team::Black => castling_rights.black_queen_side(),
    };

    if can_castle_long {
        let b_square = to_coordinates(FILE_B);
        let c_square = to_coordinates(FILE_C);
        let d_square = to_coordinates(FILE_D);

        // B, C, and D must be empty
        let is_path_clear = backend.get(b_square).is_none()
            && backend.get(c_square).is_none()
            && backend.get(d_square).is_none();

        // Only C and D (where King moves) must be safe
        // The Rook passes through B, but the King does not, so B need not be safe.
        let is_path_safe = !backend.is_square_under_attack(c_square, enemy)
            && !backend.is_square_under_attack(d_square, enemy);

        let is_rook_in_place = backend
            .get(Coordinates::new(castling_row, FILE_A).unwrap())
            .is_some_and(|potential_rook| {
                potential_rook.kind() == Kind::Rook && potential_rook.team() == ally
            });

        if is_path_clear && is_path_safe && is_rook_in_place {
            buffer.push(Ply::new(
                what_to_move.position(),
                c_square,
                what_to_move.piece(),
                None,
                Some(SpecialMove::Castle(CastlingSide::Long)),
            ));
        }
    }
}
