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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValidPromotions {
    Queen,
    Rook,
    Bishop,
    Knight,
}
