use crate::{atoms::Coordinates, board::Square, moves::SpecialMove, pieces::Piece};

/// Represents a single completed move by one player (a "half-move").
///
/// In computer chess, this is standardly called a "Ply". It stores all information
/// required to transition the board state forward *and* backward (for undoing moves).
///
/// # Fields
/// * `starting_square`: Where the piece came from.
/// * `ending_square`: Where the piece landed.
/// * `piece_moved`: The specific piece that moved.
/// * `piece_captured`: The piece that was removed, if any.
/// * `special_move`: Metadata for En Passant, Castling, or Promotion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ply {
    starting_square: Coordinates,
    ending_square: Coordinates,

    piece_moved: Piece,
    piece_captured: Square,

    special_move: Option<SpecialMove>,
}

impl Ply {
    /// Creates a new `Ply` representing a move.
    #[must_use]
    pub const fn new(
        starting_square: Coordinates,
        ending_square: Coordinates,

        piece_moved: Piece,
        piece_captured: Square,

        special_move: Option<SpecialMove>,
    ) -> Self {
        Self {
            starting_square,
            ending_square,
            piece_moved,
            piece_captured,
            special_move,
        }
    }

    /// Returns the coordinate the piece moved from.
    #[must_use]
    pub const fn starting_square(&self) -> Coordinates {
        self.starting_square
    }

    /// Returns the coordinate the piece moved to.
    #[must_use]
    pub const fn ending_square(&self) -> Coordinates {
        self.ending_square
    }

    /// Returns the piece that was moved.
    #[must_use]
    pub const fn piece_moved(&self) -> Piece {
        self.piece_moved
    }

    /// Returns the piece that was captured, if any.
    #[must_use]
    pub const fn piece_captured(&self) -> Square {
        self.piece_captured
    }

    /// Returns the special move details, if applicable.
    #[must_use]
    pub const fn special_move(&self) -> Option<SpecialMove> {
        self.special_move
    }
}
