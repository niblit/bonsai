#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValidPromotions {
    Queen,
    Rook,
    Bishop,
    Knight,
}
