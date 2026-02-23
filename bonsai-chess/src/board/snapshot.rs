//! # Position Snapshot
//!
//! This module provides the [`PositionSnapshot`] struct, which captures the exact,
//! reproducible state of a chess board at a specific point in time. It is primarily
//! used for hashing and tracking game history to enforce the FIDE Threefold
//! Repetition draw rule.

use crate::{
    atoms::{CastlingRights, Coordinates, Team},
    board::Grid,
};

/// A hashable representation of the board state used to detect Threefold Repetition.
///
/// This struct captures only the essential data required to uniquely identify a position
/// according to FIDE rules (piece placement, active color, castling rights, and en passant).
/// It excludes move counters or history logs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PositionSnapshot {
    pieces_positions: Grid,
    turn: Team,
    remaining_castling_rights: CastlingRights,
    en_passant: Option<Coordinates>,
}

impl PositionSnapshot {
    /// Creates a new `PositionSnapshot`.
    ///
    /// # Arguments
    ///
    /// * `pieces_positions` - The current 8x8 grid of pieces.
    /// * `turn` - The team whose turn it is to move.
    /// * `remaining_castling_rights` - The castling rights available in this position.
    /// * `en_passant` - The coordinate of a valid en passant target, if any.
    #[must_use]
    pub const fn new(
        pieces_positions: Grid,
        turn: Team,
        remaining_castling_rights: CastlingRights,
        en_passant: Option<Coordinates>,
    ) -> Self {
        Self {
            pieces_positions,
            turn,
            remaining_castling_rights,
            en_passant,
        }
    }

    /// Returns the grid representing the piece placements for this snapshot.
    #[must_use]
    pub const fn get_grid(&self) -> Grid {
        self.pieces_positions
    }

    /// Returns the team whose turn it is to move in this snapshot.
    #[must_use]
    pub const fn get_turn(&self) -> Team {
        self.turn
    }

    /// Returns the castling rights available in this snapshot.
    #[must_use]
    pub const fn get_castling_rights(&self) -> CastlingRights {
        self.remaining_castling_rights
    }

    /// Returns the valid en passant target coordinate, if one exists in this snapshot.
    #[must_use]
    pub const fn get_en_passant(&self) -> Option<Coordinates> {
        self.en_passant
    }
}
