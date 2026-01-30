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

impl std::fmt::Display for Ply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self.piece_moved.kind() {
            Kind::King => "K",
            Kind::Queen => "Q",
            Kind::Rook => "R",
            Kind::Bishop => "B",
            Kind::Knight => "N",
            Kind::Pawn => "",
        };

        let origin = self.starting_square.to_algebraic_notation();
        let capture_or_not = if self.piece_captured.is_some()
            || matches!(self.special_move, Some(SpecialMove::EnPassant(_)))
        {
            "x"
        } else {
            "-"
        };
        let destination = self.ending_square.to_algebraic_notation();
        let promotion = if let Some(SpecialMove::Promotion(promoted_piece)) = self.special_move {
            format!("{promoted_piece:?}")
        } else {
            String::new()
        };

        let long_algebraic_notation =
            format!("{piece}{origin}{capture_or_not}{destination}{promotion}");
        write!(f, "{long_algebraic_notation}")
    }
}
