use crate::atoms::Coordinates;

/// Provides the legality constraints for the current position before move generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LegalityContext {
    /// Squares containing enemy pieces currently checking our King.
    checkers: Vec<Coordinates>,
    /// Friendly pieces pinned to the King, and the (dr, dc) direction ray of the pin.
    pinned_pieces: Vec<(Coordinates, (isize, isize))>,
    /// Squares adjacent to the King that are controlled by the enemy.
    danger_squares: Vec<Coordinates>,
}

impl LegalityContext {
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

    #[must_use]
    pub fn checkers(&self) -> &[Coordinates] {
        &self.checkers
    }

    #[must_use]
    pub fn pinned_pieces(&self) -> &[(Coordinates, (isize, isize))] {
        &self.pinned_pieces
    }

    #[must_use]
    pub fn danger_squares(&self) -> &[Coordinates] {
        &self.danger_squares
    }

    #[must_use]
    pub const fn in_check(&self) -> bool {
        !self.checkers.is_empty()
    }

    #[must_use]
    pub const fn in_single_check(&self) -> bool {
        self.checkers.len() == 1
    }

    #[must_use]
    pub const fn in_double_check(&self) -> bool {
        self.checkers.len() > 1
    }

    /// Fast check if a piece is allowed to move along a specific ray.
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

    /// Checks if a non-king target square resolves a single check.
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
        if let Some(ep_sq) = captured_ep_pawn
            && ep_sq == checker
        {
            return true;
        }

        // 2. Block the checker (target must lie exactly between King and Checker)
        let ray_row_delta = checker.row().cast_signed() - king_position.row().cast_signed();
        let ray_column_delta = checker.column().cast_signed() - king_position.column().cast_signed();

        let target_row_delta = target.row().cast_signed() - king_position.row().cast_signed();
        let target_column_delta = target.column().cast_signed() - king_position.column().cast_signed();

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
