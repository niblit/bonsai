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

    #[must_use]
    pub const fn get_grid(&self) -> Grid {
        self.pieces_positions
    }
    #[must_use]
    pub const fn get_turn(&self) -> Team {
        self.turn
    }
    #[must_use]
    pub const fn get_castling_rights(&self) -> CastlingRights {
        self.remaining_castling_rights
    }
    #[must_use]
    pub const fn get_en_passant(&self) -> Option<Coordinates> {
        self.en_passant
    }
}
