use crate::{atoms::Coordinates, pieces::ValidPromotions};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMove {
    Castle,
    EnPassant(Coordinates), // The coordinates of the pawn that got captured
    Promotion(ValidPromotions), // The piece kind the pawn has promoted into
}
