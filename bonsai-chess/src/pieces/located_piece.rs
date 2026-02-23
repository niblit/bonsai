//! # Located Piece
//!
//! This module provides the [`LocatedPiece`] struct, which anchors a concrete
//! [`Piece`] to a specific set of [`Coordinates`] on the chess board. This is
//! particularly useful for move generation and board iteration where both identity
//! and location are simultaneously required.

use crate::{atoms::Coordinates, pieces::Piece};

/// A composite struct representing a specific piece at a specific location.
///
/// While `Piece` describes *what* an object is (e.g., "White Pawn"), and `Coordinates`
/// describes *where* a square is (e.g., "e4"), `LocatedPiece` combines them.
///
/// This is particularly useful during move generation, where iterators need to yield
/// not just the piece being evaluated, but also its origin square on the grid.
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{Coordinates, Team, Kind, Piece, LocatedPiece};
///
/// let e4 = Coordinates::from_algebraic_notation("e4").unwrap();
/// let white_pawn = Piece::new(Team::White, Kind::Pawn);
///
/// let located_pawn = LocatedPiece::new(white_pawn, e4);
/// ```
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
    /// * `piece` - The underlying piece (Team + Kind) being placed.
    /// * `position` - The specific coordinates on the board where the piece resides.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Coordinates, Team, Kind, Piece, LocatedPiece};
    ///
    /// let a1 = Coordinates::new(7, 0).unwrap();
    /// let white_rook = Piece::new(Team::White, Kind::Rook);
    ///
    /// let located_rook = LocatedPiece::new(white_rook, a1);
    /// ```
    #[must_use]
    pub const fn new(piece: Piece, position: Coordinates) -> Self {
        Self { piece, position }
    }

    /// Returns the piece component (Team + Kind).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Coordinates, Team, Kind, Piece, LocatedPiece};
    ///
    /// let h8 = Coordinates::new(0, 7).unwrap();
    /// let black_rook = Piece::new(Team::Black, Kind::Rook);
    /// let located = LocatedPiece::new(black_rook, h8);
    ///
    /// assert_eq!(located.piece(), black_rook);
    /// ```
    #[must_use]
    pub const fn piece(&self) -> Piece {
        self.piece
    }

    /// Returns the coordinate component (Row + Column).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Coordinates, Team, Kind, Piece, LocatedPiece};
    ///
    /// let e8 = Coordinates::from_algebraic_notation("e8").unwrap();
    /// let black_king = Piece::new(Team::Black, Kind::King);
    /// let located = LocatedPiece::new(black_king, e8);
    ///
    /// assert_eq!(located.position(), e8);
    /// ```
    #[must_use]
    pub const fn position(&self) -> Coordinates {
        self.position
    }
}
