//! # Ply (Half-Move)
//!
//! This module defines the [`Ply`] struct, which represents a single, concrete
//! half-move in a chess game. It acts as a historical record of a move, containing
//! all the information necessary to execute the move on a board or to reverse it
//! during an `undo` operation.

use crate::{
    atoms::Coordinates,
    board::Square,
    moves::SpecialMove,
    pieces::{Kind, Piece},
};

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
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{Coordinates, Team, Kind, Piece, Ply};
///
/// let e2 = Coordinates::from_algebraic_notation("e2").unwrap();
/// let e4 = Coordinates::from_algebraic_notation("e4").unwrap();
/// let white_pawn = Piece::new(Team::White, Kind::Pawn);
///
/// // Represents a standard e2-e4 pawn opening, with no captures or special moves.
/// let ply = Ply::new(e2, e4, white_pawn, None, None);
/// ```
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
    ///
    /// # Arguments
    ///
    /// * `starting_square` - The coordinate where the piece originated.
    /// * `ending_square` - The coordinate where the piece landed.
    /// * `piece_moved` - The actual `Piece` making the move.
    /// * `piece_captured` - The piece that was on the destination square (or captured via en passant), if any.
    /// * `special_move` - An `Option` containing context for castling, en passant, or promotion.
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Assuming `ply` is the move e2-e4
    /// // assert_eq!(ply.starting_square().to_algebraic_notation(), "e2");
    /// ```
    #[must_use]
    pub const fn starting_square(&self) -> Coordinates {
        self.starting_square
    }

    /// Returns the coordinate the piece moved to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Assuming `ply` is the move e2-e4
    /// // assert_eq!(ply.ending_square().to_algebraic_notation(), "e4");
    /// ```
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
    ///
    /// This returns a `Square` (which is a type alias for `Option<Piece>`).
    /// It is `None` for quiet moves.
    #[must_use]
    pub const fn piece_captured(&self) -> Square {
        self.piece_captured
    }

    /// Returns the special move details, if applicable.
    ///
    /// This will be `Some` if the move was a castling maneuver, an en passant
    /// capture, or a pawn promotion. Otherwise, it returns `None`.
    #[must_use]
    pub const fn special_move(&self) -> Option<SpecialMove> {
        self.special_move
    }
}

/// Formats the ply using a variant of Long Algebraic Notation (LAN).
///
/// Format: `[PieceSymbol][Origin]-[Destination][Promotion]` for quiet moves,
/// or `[PieceSymbol][Origin]x[Destination][Promotion]` for captures.
///
/// *Note: Pawns do not display a piece symbol.*
///
/// # Examples
///
/// * `e2-e4` (Pawn push)
/// * `Ng1xf3` (Knight capture)
/// * `e7-e8=Q` (Pawn promotion)
impl std::fmt::Display for Ply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Pawns are not denoted by a letter in standard algebraic notation
        let piece = match self.piece_moved.kind() {
            Kind::Pawn => String::new(),
            _ => self.piece_moved.kind().to_string(),
        };

        let origin = self.starting_square.to_algebraic_notation();

        // Determine if this is a capture ('x') or a quiet move ('-')
        let capture_or_not = if self.piece_captured.is_some()
            || matches!(self.special_move, Some(SpecialMove::EnPassant(_)))
        {
            "x"
        } else {
            "-"
        };

        let destination = self.ending_square.to_algebraic_notation();

        // Append promotion suffix (e.g., "=Q") if applicable
        let promotion = if let Some(SpecialMove::Promotion(promoted_piece)) = self.special_move {
            format!("={promoted_piece}")
        } else {
            String::new()
        };

        let long_algebraic_notation =
            format!("{piece}{origin}{capture_or_not}{destination}{promotion}");

        write!(f, "{long_algebraic_notation}")
    }
}
