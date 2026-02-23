//! # Piece Kinds
//!
//! This module defines the raw types of chess pieces ([`Kind`]) independent of
//! their team/color, as well as the specific subset of pieces that a pawn is
//! legally allowed to promote into ([`ValidPromotions`]).

/// Represents the distinct types of chess pieces, independent of their team.
///
/// This enum covers all six standard piece types defined in chess.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    /// The most important piece. The game ends when it is in check and has no legal moves.
    King,
    /// The most powerful piece, combining the movement capabilities of a Rook and a Bishop.
    Queen,
    /// Moves in straight lines along ranks and files. Participates in castling.
    Rook,
    /// Moves diagonally. A bishop is permanently bound to the color of the square it starts on.
    Bishop,
    /// Moves in an 'L' shape (two squares in one direction, one square in a perpendicular direction) and can jump over other pieces.
    Knight,
    /// The most numerous piece. Moves forward but captures diagonally, and has special rules like en passant and promotion.
    Pawn,
}

impl Kind {
    /// Converts a specific promotion choice back into a general piece `Kind`.
    ///
    /// This is used when a pawn reaches the final rank and transforms into the selected piece.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Kind, ValidPromotions};
    ///
    /// let promotion = ValidPromotions::Queen;
    /// let piece_kind = Kind::from_valid_promotions(promotion);
    ///
    /// assert_eq!(piece_kind, Kind::Queen);
    /// ```
    #[must_use]
    pub const fn from_valid_promotions(vp: ValidPromotions) -> Self {
        match vp {
            ValidPromotions::Queen => Self::Queen,
            ValidPromotions::Rook => Self::Rook,
            ValidPromotions::Bishop => Self::Bishop,
            ValidPromotions::Knight => Self::Knight,
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Standard algebraic notation piece symbols (English)
        let symbol = match self {
            Self::King => "K",
            Self::Queen => "Q",
            Self::Rook => "R",
            Self::Bishop => "B",
            Self::Knight => "N",
            Self::Pawn => "P",
        };

        write!(f, "{symbol}")
    }
}

/// Represents the subset of piece types that a Pawn is allowed to promote into.
///
/// According to FIDE Laws of Chess, a pawn cannot promote into a King or another Pawn.
/// By using a distinct enum, we enforce this restriction at the type system level,
/// making illegal promotions (like promoting to a King) impossible to represent in safe code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValidPromotions {
    /// Promote to a Queen.
    Queen,
    /// Promote to a Rook.
    Rook,
    /// Promote to a Bishop.
    Bishop,
    /// Promote to a Knight.
    Knight,
}

impl std::fmt::Display for ValidPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Defer to the standard Kind display implementation
        let kind = Kind::from_valid_promotions(*self);

        write!(f, "{kind}")
    }
}
