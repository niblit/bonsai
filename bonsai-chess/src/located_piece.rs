use crate::{coordinates::Coordinates, piece::Piece};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocatedPiece {
    pub piece: Piece,
    pub position: Coordinates,
}
