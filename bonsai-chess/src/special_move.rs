use crate::{coordinates::Coordinates, kind::Kind};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMove {
    EnPassant(Coordinates), //The coordinates of the pawn that got captured
    Castle,
    Promotion(Kind), // The piece kind the pawn has promoted into
}
