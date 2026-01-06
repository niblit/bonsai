/// Represents the distinct types of chess pieces, independent of their team.
///
/// This enum covers all six standard piece types defined in chess.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Kind {
    /// Converts a specific promotion choice back into a general piece `Kind`.
    ///
    /// This is used when a pawn reaches the final rank and transforms into the selected piece.
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

/// Represents the subset of piece types that a Pawn is allowed to promote into.
///
/// According to FIDE Laws of Chess, a pawn cannot promote into a King or another Pawn.
/// By using a distinct enum, we enforce this restriction at the type system level,
/// making illegal promotions (like promoting to a King) impossible to represent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValidPromotions {
    Queen,
    Rook,
    Bishop,
    Knight,
}
