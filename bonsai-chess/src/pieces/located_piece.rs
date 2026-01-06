use crate::{atoms::Coordinates, pieces::Piece};

/// A composite struct representing a specific piece at a specific location.
///
/// While `Piece` describes *what* an object is (e.g., "White Pawn"), and `Coordinates`
/// describes *where* a square is (e.g., "E4"), `LocatedPiece` combines them.
///
/// This is particularly useful during move generation, where iterators need to yield
/// not just the piece being evaluated, but also its origin square on the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocatedPiece {
    piece: Piece,
    position: Coordinates,
}

impl LocatedPiece {
    /// Creates a new `LocatedPiece`.
    ///
    /// # Arguments
    ///
    /// * `piece` - The piece (Team + Kind).
    /// * `position` - The coordinates on the board.
    #[must_use]
    pub const fn new(piece: Piece, position: Coordinates) -> Self {
        Self { piece, position }
    }

    /// Returns the piece component (Team + Kind).
    #[must_use]
    pub const fn piece(&self) -> Piece {
        self.piece
    }

    /// Returns the coordinate component (Row + Column).
    #[must_use]
    pub const fn position(&self) -> Coordinates {
        self.position
    }
}
