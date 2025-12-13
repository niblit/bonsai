use crate::{atoms::Coordinates, board::Square, moves::SpecialMove, pieces::Piece};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ply {
    starting_square: Coordinates,
    ending_square: Coordinates,

    piece_moved: Piece,
    piece_captured: Square,

    special_move: Option<SpecialMove>,
}

impl Ply {
    #[must_use]
    pub const fn new(
        starting_square: Coordinates,
        ending_square: Coordinates,

        piece_moved: Piece,
        piece_captured: Square,

        special_move: Option<SpecialMove>,
    ) -> Self {
        Self {
            starting_square,
            ending_square,
            piece_moved,
            piece_captured,
            special_move,
        }
    }

    #[must_use]
    pub const fn starting_square(&self) -> Coordinates {
        self.starting_square
    }

    #[must_use]
    pub const fn ending_square(&self) -> Coordinates {
        self.ending_square
    }

    #[must_use]
    pub const fn piece_moved(&self) -> Piece {
        self.piece_moved
    }

    #[must_use]
    pub const fn piece_captured(&self) -> Square {
        self.piece_captured
    }

    #[must_use]
    pub const fn special_move(&self) -> Option<SpecialMove> {
        self.special_move
    }
}
