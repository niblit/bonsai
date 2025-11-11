use crate::{coordinates::Coordinates, piece::Piece};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMove {
    EnPassant(Coordinates),
    Castle,
    Promotion(Piece)
}