use crate::{atoms::Coordinates, pieces::ValidPromotions};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMove {
    EnPassant(Coordinates), // The coordinates of the pawn that got captured
    Castle,
    Promotion(ValidPromotions), // The piece kind the pawn has promoted into
}
