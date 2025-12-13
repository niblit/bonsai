use crate::{atoms::Coordinates, pieces::Piece};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocatedPiece {
    piece: Piece,
    position: Coordinates,
}

impl LocatedPiece {
    #[must_use]
    pub const fn new(piece: Piece, position: Coordinates) -> Self {
        Self { piece, position }
    }

    #[must_use]
    pub const fn piece(&self) -> Piece {
        self.piece
    }

    #[must_use]
    pub const fn position(&self) -> Coordinates {
        self.position
    }
}
