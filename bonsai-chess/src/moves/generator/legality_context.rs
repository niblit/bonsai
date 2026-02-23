//! # Legality Context
//!
//! This module provides the [`LegalityContext`] struct, a critical component
//! in the move generation pipeline. Rather than generating all pseudo-legal
//! moves and strictly validating them after the fact (which is slow), the engine
//! pre-calculates the "context" of the board: which pieces are pinned, where
//! the King is in danger, and whether the King is currently in check.
//!
//! Move generators for individual pieces then use this context to only yield
//! strictly legal moves, significantly improving performance.

use crate::atoms::Coordinates;

/// Provides the legality constraints for the current position before move generation.
///
/// This structure holds pre-calculated masks and lists that dictate whether a
/// theoretically valid piece movement is actually legal under the rules of chess
/// (e.g., preventing a pinned piece from moving off its pin ray).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LegalityContext {
    /// Squares containing enemy pieces currently checking our King.
    checkers: Vec<Coordinates>,
    /// Friendly pieces pinned to the King, and the `(delta_row, delta_column)` direction ray of the pin.
    pinned_pieces: Vec<(Coordinates, (isize, isize))>,
    /// Squares adjacent to the King that are controlled by the enemy.
    danger_squares: Vec<Coordinates>,
}

impl LegalityContext {
    /// Creates a new `LegalityContext` from pre-calculated board constraints.
    ///
    /// # Arguments
    ///
    /// * `checkers` - A list of coordinates containing enemy pieces attacking the King.
    /// * `pinned_pieces` - A list of tuples containing the coordinate of a friendly pinned piece and the `(d_row, d_col)` ray of the pin.
    /// * `danger_squares` - A list of coordinates around the King that cannot be stepped onto.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Coordinates, LegalityContext};
    ///
    /// let context = LegalityContext::from(vec![], vec![], vec![]);
    /// assert!(!context.in_check());
    /// ```
    #[must_use]
    pub const fn from(
        checkers: Vec<Coordinates>,
        pinned_pieces: Vec<(Coordinates, (isize, isize))>,
        danger_squares: Vec<Coordinates>,
    ) -> Self {
        Self {
            checkers,
            pinned_pieces,
            danger_squares,
        }
    }

    /// Returns a slice of coordinates representing the position of enemy pieces delivering check.
    #[must_use]
    pub fn checkers(&self) -> &[Coordinates] {
        &self.checkers
    }

    /// Returns a slice of tuples representing pinned pieces and their pin direction vectors.
    #[must_use]
    pub fn pinned_pieces(&self) -> &[(Coordinates, (isize, isize))] {
        &self.pinned_pieces
    }

    /// Returns a slice of coordinates representing squares the King cannot move to.
    #[must_use]
    pub fn danger_squares(&self) -> &[Coordinates] {
        &self.danger_squares
    }

    /// Returns `true` if the King is currently in check (attacked by 1 or more pieces).
    #[must_use]
    pub const fn in_check(&self) -> bool {
        !self.checkers.is_empty()
    }

    /// Returns `true` if the King is attacked by exactly one piece.
    ///
    /// In a single check, the king can move out of the check, or friendly pieces can attempt to capture the checker or
    /// block the attack ray.
    #[must_use]
    pub const fn in_single_check(&self) -> bool {
        self.checkers.len() == 1
    }

    /// Returns `true` if the King is attacked by two or more pieces.
    ///
    /// In a double check, it is impossible to block or capture both checkers
    /// simultaneously. Therefore, the only legal response is for the King to move.
    #[must_use]
    pub const fn in_double_check(&self) -> bool {
        self.checkers.len() > 1
    }

    /// Determines if a pinned piece is allowed to move along a specific directional ray.
    ///
    /// A pinned piece can only move along the ray of its pin (e.g., a piece pinned
    /// horizontally can still move horizontally to capture the pinning piece, but
    /// cannot move vertically).
    ///
    /// This function uses the cross product of the intended movement vector and
    /// the pin vector to quickly determine collinearity.
    ///
    /// # Arguments
    ///
    /// * `location_of_piece_to_move` - The coordinate of the piece attempting to move.
    /// * `row_delta` - The row direction of the intended move.
    /// * `column_delta` - The column direction of the intended move.
    #[must_use]
    pub fn is_direction_allowed_for_pin(
        &self,
        location_of_piece_to_move: Coordinates,
        row_delta: isize,
        column_delta: isize,
    ) -> bool {
        for &(pinned_square, (pin_row_delta, pin_column_delta)) in &self.pinned_pieces {
            if pinned_square == location_of_piece_to_move {
                // Direction vectors must be parallel. Cross product == 0.
                return (row_delta * pin_column_delta) - (column_delta * pin_row_delta) == 0;
            }
        }
        true // Not pinned
    }

    /// Checks if a proposed target square resolves an ongoing single check.
    ///
    /// A non-king piece can resolve a check in one of two ways:
    /// 1. Capturing the piece delivering the check.
    /// 2. Moving to a square that blocks the attack ray between the King and the checker.
    ///
    /// # Arguments
    ///
    /// * `target` - The coordinate the friendly piece intends to move to.
    /// * `king_position` - The coordinate of the friendly King.
    /// * `captured_ep_pawn` - The coordinate of a pawn that would be captured via en passant, if applicable.
    ///
    /// # Returns
    ///
    /// `true` if the move resolves the check or if there is no single check. `false` otherwise.
    #[must_use]
    pub fn resolves_single_check(
        &self,
        target: Coordinates,
        king_position: Coordinates,
        captured_ep_pawn: Option<Coordinates>,
    ) -> bool {
        if !self.in_single_check() {
            return true;
        }

        let checker = self.checkers[0];

        // 1. Capture the checker
        if target == checker {
            return true;
        }

        // Handle En Passant resolving a check from a pawn
        if let Some(ep_sq) = captured_ep_pawn
            && ep_sq == checker
        {
            return true;
        }

        // 2. Block the checker (target must lie exactly between King and Checker)
        let ray_row_delta = checker.row().cast_signed() - king_position.row().cast_signed();
        let ray_column_delta =
            checker.column().cast_signed() - king_position.column().cast_signed();

        let target_row_delta = target.row().cast_signed() - king_position.row().cast_signed();
        let target_column_delta =
            target.column().cast_signed() - king_position.column().cast_signed();

        // Cross product must be 0 (must be collinear)
        if ray_row_delta * target_column_delta - ray_column_delta * target_row_delta != 0 {
            return false;
        }

        // Must be in the same direction (dot product > 0)
        if ray_row_delta * target_row_delta + ray_column_delta * target_column_delta <= 0 {
            return false;
        }

        // Must be closer to the king than the checker is (Chebyshev distance)
        let checker_dist = ray_row_delta.abs().max(ray_column_delta.abs());
        let target_dist = target_row_delta.abs().max(target_column_delta.abs());

        target_dist < checker_dist
    }
}
