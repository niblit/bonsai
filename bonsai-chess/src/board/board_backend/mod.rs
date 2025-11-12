mod grid;
pub use grid::BoardGrid;

use crate::{board::square::Square, coordinates::Coordinates, located_piece::LocatedPiece, piece::Piece};

pub trait BoardBackend {
    #[must_use]
    fn from_starting_position() -> Self where Self: std::marker::Sized;

    fn set(&mut self, piece: Piece, coordinates: Coordinates);

    fn unset(&mut self, coordinates: Coordinates);

    #[must_use]
    fn get(&self, coordinates: Coordinates) -> Square;

    #[must_use]
    fn get_white_pieces(&self) -> Vec<LocatedPiece>;

    #[must_use]
    fn get_black_pieces(&self) -> Vec<LocatedPiece>;
}